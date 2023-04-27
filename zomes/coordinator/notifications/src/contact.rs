use hdk::prelude::*;
use notifications_integrity::*;
#[hdk_extern]
pub fn send_contact(contact: Contact) -> ExternResult<()> {
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
        .map(|link| AgentPubKey::from(EntryHash::from(link.target)))
        .collect();
    let notifier = agents[0].clone();
    let zome_call_response = call_remote(
        notifier.clone(),
        "notifications",
        FunctionName(String::from("create_contact")),
        None,
        contact,
    )?;
    Ok(())
    // match zome_call_response {
    //     ZomeCallResponse::Ok(result) => {
    //         let action_hash: ActionHash = result
    //             .decode()
    //             .map_err(|err| wasm_error!(err))?;
    //         let me: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    //         create_link(me, notifier, LinkTypes::NotificantToNotifiers, ())?;
    //         Ok(action_hash)
    //     }
    //     ZomeCallResponse::NetworkError(err) => {
    //         Err(
    //             wasm_error!(
    //                 WasmErrorInner::Guest(format!("There was a network error: {:?}",
    //                 err))
    //             ),
    //         )
    //     }
    //     _ => {
    //         Err(
    //             wasm_error!(WasmErrorInner::Guest("Failed to handle remote call".into())),
    //         )
    //     }
    // }
}
#[hdk_extern]
pub fn create_contact(contact: Contact) -> ExternResult<Record> {
    let contact_hash = create_entry(&EntryTypes::Contact(contact.clone()))?;
    let record = get(contact_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Contact"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_contacts(agent_pub_key: AgentPubKey) -> ExternResult<Vec<Record>> {
    let contact_entry_type: EntryType = UnitEntryTypes::Contact.try_into()?;
    let filter = ChainQueryFilter::new()
        .entry_type(contact_entry_type)
        .include_entries(true);
    let all_contacts = query(filter)?;

    Ok(all_contacts)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateContactInput {
    pub original_contact_hash: ActionHash,
    pub previous_contact_hash: ActionHash,
    pub updated_contact: Contact,
}
#[hdk_extern]
pub fn update_contact(input: UpdateContactInput) -> ExternResult<Record> {
    let updated_contact_hash = update_entry(
        input.previous_contact_hash.clone(),
        &input.updated_contact,
    )?;
    create_link(
        input.original_contact_hash.clone(),
        updated_contact_hash.clone(),
        LinkTypes::ContactUpdates,
        (),
    )?;
    let record = get(updated_contact_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Contact"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_contact(original_contact_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_contact_hash)
}
