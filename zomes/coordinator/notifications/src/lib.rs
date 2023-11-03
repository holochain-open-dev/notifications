pub mod sent_notification;
pub mod contact;
pub mod notificant_to_notifiers;
pub mod twilio_credentials;
use hdk::prelude::*;
use notifications_integrity::*;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Contact {
    pub agent_pub_key: AgentPubKey,
    pub text_number: Option<String>,
    pub whatsapp_number: Option<String>,
    pub email_address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationTip {
  pub retry_count: i32,
  pub status: String,
  pub message: String,
  pub notificants: Vec<AgentPubKey>,
  pub contacts: Vec<Contact>,
  pub extra_context: String,
  pub message_id: String,
  pub destination: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Signal {
    LinkCreated { action: SignedActionHashed, link_type: LinkTypes },
    LinkDeleted { action: SignedActionHashed, link_type: LinkTypes },
    EntryCreated { action: SignedActionHashed, app_entry: EntryTypes },
    EntryUpdated {
        action: SignedActionHashed,
        app_entry: EntryTypes,
        original_app_entry: EntryTypes,
    },
    EntryDeleted { action: SignedActionHashed, original_app_entry: EntryTypes },
}

#[hdk_extern]
pub fn handle_notification_tip(data: NotificationTip) -> ExternResult<()> {
    emit_signal("tip received")?;

    let zome_call_response = call(
        CallTargetCell::Local,
        ZomeName::from(String::from("notifications")),
        FunctionName(String::from("custom_handle_notification_tip")),
        None,
        data.clone(),
    )?;

    emit_signal(zome_call_response.clone())?;

    match zome_call_response {
        ZomeCallResponse::Ok(result) => {
            emit_signal("custom handle received")?;

            let tip: NotificationTip = result.decode().map_err(|err| wasm_error!(String::from(err)))?; // Deserialize byte array
            emit_signal(tip.clone())?;
            // check if validated
            let validated = tip.status != String::from("stop");
            emit_signal(validated.clone())?;
            if validated {
                emit_signal("validated")?;
                // check if sent
                let message_id = tip.message_id;
                let was_it_sent_response = call_remote(
                    agent_info().unwrap().agent_latest_pubkey.into(),
                    "notifications",
                    FunctionName(String::from("was_it_sent")),
                    None,
                    message_id.clone(),
                )?;
                emit_signal(was_it_sent_response.clone())?;

                match was_it_sent_response {
                    ZomeCallResponse::Ok(was_it_sent) => {
                        emit_signal("was it sent received")?;

                        let was_it_sent: bool = was_it_sent.decode().map_err(|err| wasm_error!(String::from(err)))?; // Deserialize byte array
                        if !was_it_sent {
                            // find contacts
                            let mut contacts: Vec<Contact> = vec![];
                            let get_contacts_response = call(
                                CallTargetCell::Local,
                                ZomeName::from(String::from("notifications")),
                                FunctionName(String::from("get_contacts")),
                                None,
                                tip.notificants.clone(),
                            )?;
                            match get_contacts_response {
                            ZomeCallResponse::Ok(contacts_result) => {
                                emit_signal("contacts received")?;
                                emit_signal(contacts_result.clone())?;
                                contacts = contacts_result.decode().map_err(|err| wasm_error!(String::from(err)))?; // Deserialize byte array
                                emit_signal(contacts.clone())?;
                            }_ => {}};

                            // save as sent and send
                            let output: NotificationTip = NotificationTip {
                                retry_count: tip.retry_count,
                                status: tip.status,
                                message: tip.message,
                                notificants: tip.notificants,
                                contacts: contacts,
                                extra_context: tip.extra_context,
                                message_id: message_id.clone(),
                                destination: String::from("notifier_service"),
                            };

                            emit_signal("this is what is sent to js client")?;
                            emit_signal(output.clone())?;
                            emit_signal("this is what is sent to js client end")?;
                            
                            if output.status == String::from("send") && output.message_id != String::from("") {
                                // save as sent
                                let sent_notification: SentNotification = SentNotification {
                                    unique_data: message_id,
                                };
                                call(
                                    CallTargetCell::Local,
                                    ZomeName::from(String::from("notifications")),
                                    FunctionName(String::from("create_sent_notification")),
                                    None,
                                    sent_notification,
                                )?;
                            }
                        }
                    }_ => {},
                }
            } else {
                emit_signal("not validated")?;
            }
            Ok(())
        }
        ZomeCallResponse::NetworkError(err) => {
            Err(
                wasm_error!(
                    WasmErrorInner::Guest(format!("There was a network error: {:?}",
                    err))
                ),
            )
        }
        ZomeCallResponse::Unauthorized(a,b,c,d,e) => {
            Err(
                wasm_error!(
                    WasmErrorInner::Guest(format!("There was an unauthorized error: {:?}{:?}{:?}{:?}{:?}",
                    a,b,c,d,e))
                ),
            )
        }
        _ => {
            Err(wasm_error!(WasmErrorInner::Guest(format!("There was an error by call: {:?}", zome_call_response))))
        },
    }

    // match zome_call_response {
    //     ZomeCallResponse::Ok(result) => { // ExternIO is a wrapper around a byte array
    //       let validated: bool = result.decode().map_err(|err| wasm_error!(String::from(err)))?; // Deserialize byte array
    //     //   Ok(entry_hash)
    //         emit_signal(validated)?;
    //     },
    //     ZomeCallResponse::Unauthorized(cell_id, zome_name, function_name, callee, agent_pubkey) => {
    //       Err(wasm_error!(WasmErrorInner::Guest("Agent revoked the capability".into())))
    //     },
    //     _ => {
    //     //   Err(wasm_error!(WasmErrorInner::Guest(format!("There was an error by call: {:?}", zome_call_response))))
    //     },
    // }
    
    // Ok(())
}
#[hdk_extern]
pub fn send_notification_tip(data: NotificationTip) -> ExternResult<()> {
    let path = Path::from(format!("all_notifiers"));
    let typed_path = path.typed(LinkTypes::AnchorToNotifiers)?;
    typed_path.ensure()?;
    let links = get_links(
        typed_path.path_entry_hash()?,
        LinkTypes::AnchorToNotifiers,
        None,
    )?;
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap()))
        .collect();
    let notifier = agents[0].clone();
    let zome_call_response = call_remote(
        notifier.clone(),
        "notifications",
        FunctionName(String::from("handle_notification_tip")),
        None,
        data,
    )?;

    emit_signal("tip send attempted")?;

    // ZomeCallResponse::NetworkError(err) => {
    //     Err(
    //         wasmerror!(
    //             WasmErrorInner::Guest(format!("There was a network error: {:?}",
    //             err))
    //         ),
    //     )
    // }
    //  => {
    //     Err(
    //         wasm_error!(WasmErrorInner::Guest(format!("Failed to handle remote call {:?}", response))),
    //     )
    // } 

    match zome_call_response {
        ZomeCallResponse::Ok(result) => {
            emit_signal("tip sent")?;
            // let me: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
            // create_link(me, notifier, LinkTypes::NotificantToNotifiers, ())?;
            Ok(())
        }
        ZomeCallResponse::NetworkError(err) => {
            Err(
                wasm_error!(
                    WasmErrorInner::Guest(format!("There was a network error: {:?}",
                    err))
                ),
            )
        }
        ZomeCallResponse::Unauthorized(a,b,c,d,e) => {
            Err(
                wasm_error!(
                    WasmErrorInner::Guest(format!("There was a network error: {:?}{:?}{:?}{:?}{:?}",
                    a,b,c,d,e))
                ),
            )
        }
        _ => {
            Err(
                wasm_error!(WasmErrorInner::Guest("Failed to handle remote call".into())),
            )
        }
    }
}
#[hdk_extern]
pub fn claim_notifier(description: String) -> ExternResult<()> {
    let path = Path::from(format!("all_notifiers"));
    let typed_path = path.typed(LinkTypes::AnchorToNotifiers)?;
    typed_path.ensure()?;
    let my_agent_pub_key: AgentPubKey = agent_info()?.agent_latest_pubkey.into();

    let tag_str = description;
    let tag_bytes = tag_str.as_bytes().to_vec();
    let tag = LinkTag(tag_bytes);

    create_link(
        typed_path.path_entry_hash()?,
        my_agent_pub_key,
        LinkTypes::AnchorToNotifiers,
        tag,
    )?;
    Ok(())
}
#[hdk_extern]
pub fn find_a_notifier(_: ()) -> ExternResult<AgentPubKey> {
    let path = Path::from(format!("all_notifiers"));
    let typed_path = path.typed(LinkTypes::AnchorToNotifiers)?;
    typed_path.ensure()?;
    let links = get_links(
        typed_path.path_entry_hash()?,
        LinkTypes::AnchorToNotifiers,
        None,
    )?;
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap()))
        .collect();
    Ok(agents[0].clone())
}
#[hdk_extern(infallible)]
pub fn post_commit(committed_actions: Vec<SignedActionHashed>) {
    for action in committed_actions {
        if let Err(err) = signal_action(action) {
            error!("Error signaling new action: {:?}", err);
        }
    }
}
fn signal_action(action: SignedActionHashed) -> ExternResult<()> {
    match action.hashed.content.clone() {
        Action::CreateLink(create_link) => {
            if let Ok(Some(link_type))
                = LinkTypes::from_type(create_link.zome_index, create_link.link_type) {
                emit_signal(Signal::LinkCreated {
                    action,
                    link_type,
                })?;
            }
            Ok(())
        }
        Action::DeleteLink(delete_link) => {
            let record = get(
                    delete_link.link_add_address.clone(),
                    GetOptions::default(),
                )?
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest("Failed to fetch CreateLink action"
                        .to_string())
                    ),
                )?;
            match record.action() {
                Action::CreateLink(create_link) => {
                    if let Ok(Some(link_type))
                        = LinkTypes::from_type(
                            create_link.zome_index,
                            create_link.link_type,
                        ) {
                        emit_signal(Signal::LinkDeleted {
                            action,
                            link_type,
                        })?;
                    }
                    Ok(())
                }
                _ => {
                    return Err(
                        wasm_error!(
                            WasmErrorInner::Guest("Create Link should exist".to_string())
                        ),
                    );
                }
            }
        }
        Action::Create(_create) => {
            if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                emit_signal(Signal::EntryCreated {
                    action,
                    app_entry,
                })?;
            }
            Ok(())
        }
        Action::Update(update) => {
            if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                if let Ok(Some(original_app_entry))
                    = get_entry_for_action(&update.original_action_address) {
                    emit_signal(Signal::EntryUpdated {
                        action,
                        app_entry,
                        original_app_entry,
                    })?;
                }
            }
            Ok(())
        }
        Action::Delete(delete) => {
            if let Ok(Some(original_app_entry))
                = get_entry_for_action(&delete.deletes_address) {
                emit_signal(Signal::EntryDeleted {
                    action,
                    original_app_entry,
                })?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
fn get_entry_for_action(action_hash: &ActionHash) -> ExternResult<Option<EntryTypes>> {
    let record = match get_details(action_hash.clone(), GetOptions::default())? {
        Some(Details::Record(record_details)) => record_details.record,
        _ => {
            return Ok(None);
        }
    };
    let entry = match record.entry().as_option() {
        Some(entry) => entry,
        None => {
            return Ok(None);
        }
    };
    let (zome_index, entry_index) = match record.action().entry_type() {
        Some(EntryType::App(AppEntryDef { zome_index, entry_index, .. })) => {
            (zome_index, entry_index)
        }
        _ => {
            return Ok(None);
        }
    };
    Ok(
        EntryTypes::deserialize_from_type(
            zome_index.clone(),
            entry_index.clone(),
            entry,
        )?,
    )
}
