use hdk::prelude::*;
use notifications_integrity::*;


#[hdk_extern]
pub fn send_contact(contact: Contact) -> ExternResult<()> {
    // let path = Path::from(format!("all_notifiers"));
    // let typed_path = path.typed(LinkTypes::AnchorToNotifiers)?;
    // typed_path.ensure()?;
    // let links = get_links(
    //     typed_path.path_entry_hash()?,
    //     LinkTypes::AnchorToNotifiers,
    //     None,
    // )?;
    // let agents: Vec<AgentPubKey> = links
    //     .into_iter()
    //     .map(|link| AgentPubKey::from(EntryHash::from(link.target)))
    //     .collect();
    // let notifier = agents[0].clone();

    let me: AgentPubKey = agent_info()?.agent_latest_pubkey;
    let info = call_info()?;
    let caller: AgentPubKey = info.provenance;
    if caller != contact.agent_pub_key {
        debug!("Contact agent did not match caller");
        debug!("     me: {}", me);
        debug!(" caller: {}", caller);
        debug!("contact: {}", contact.agent_pub_key);
        // return Err(
        //     wasm_error!(WasmErrorInner::Guest("Contact agent did not match sender".into())),
        // )
    }
    let links = get_links(me, LinkTypes::NotificantToNotifiers, None)?;
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(
            EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap()
        ))
        .collect();
    let notifier = agents[0].clone();

    debug!("Calling notifier: {}", notifier);
    let zome_call_response = call_remote(
        notifier.clone(),
        "notifications",
        "create_contact".to_string().into(),
        None,
        contact,
    )?;
    // let zome_call_response = call(
    //     CallTargetCell::OtherCell(CellId::new(dna_info()?.hash, notifier)),
    //     "notifications",
    //     "create_contact".to_string().into(),
    //     None,
    //     contact,
    // )?;
    // Ok(())
    match zome_call_response {
        ZomeCallResponse::Ok(_result) => Ok(()),
        ZomeCallResponse::NetworkError(err) => Err(wasm_error!(WasmErrorInner::Guest(format!("There was a network error: {:?}",err)))),
        ZomeCallResponse::Unauthorized(authorization, cellId, zomeName, fnName, agent) => {
            let msg = format!("Remote call Unauthorized: {:?} | {:?} | {:?} | {:?} | {:?}", authorization, cellId, zomeName, fnName, agent);
            debug!(msg);
            Err(wasm_error!(WasmErrorInner::Guest(msg.into())))
        },
        _ => Err(wasm_error!(WasmErrorInner::Guest("Failed to handle remote call".into()))),
    }
}


#[hdk_extern]
pub fn send_update_contact(contact: Contact) -> ExternResult<()> {
    let info = call_info()?;
    let caller: AgentPubKey = info.provenance;
    if caller != contact.agent_pub_key {
        return Err(
            wasm_error!(WasmErrorInner::Guest("Contact did not match sender".into())),
        )
    }
    let me: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    let links = get_links(me, LinkTypes::NotificantToNotifiers, None)?;
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap()))
        .collect();
    let notifier = agents[0].clone();

    let zome_call_response = call_remote(
        notifier.clone(),
        "notifications",
        FunctionName(String::from("update_contact")),
        None,
        contact,
    )?;
    Ok(())
}


#[hdk_extern]
pub fn send_delete_contact(contact: Contact) -> ExternResult<()> {
    let info = call_info()?;
    let caller: AgentPubKey = info.provenance;
    if caller != contact.agent_pub_key {
        return Err(
            wasm_error!(WasmErrorInner::Guest("Contact did not match sender".into())),
        )
    }
    let me: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    let links = get_links(me, LinkTypes::NotificantToNotifiers, None)?;
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap()))
        .collect();
    let notifier = agents[0].clone();

    let zome_call_response = call_remote(
        notifier.clone(),
        "notifications",
        FunctionName(String::from("delete_contact")),
        None,
        contact,
    )?;
    Ok(())
}


#[hdk_extern]
pub fn create_contact(contact: Contact) -> ExternResult<Record> {
    debug!("=====> create contact {:?}", contact);
    debug!("     me: {}", agent_info()?.agent_latest_pubkey);
    debug!(" caller: {}", AgentPubKey = call_info()?.provenance);
    debug!("contact: {}", contact.agent_pub_key);

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
pub fn get_contacts(agent_pub_keys: Vec<AgentPubKey>) -> ExternResult<Vec<Contact>> {
    emit_signal("agent pub keys below")?;
    emit_signal(agent_pub_keys.clone())?;

    let contact_entry_type: EntryType = UnitEntryTypes::Contact.try_into()?;
    let filter = ChainQueryFilter::new()
        .entry_type(contact_entry_type)
        .include_entries(true);
    let all_contact_records = query(filter)?;

    emit_signal(all_contact_records.clone())?;

    let all_contacts: Vec<Contact> = all_contact_records
        .into_iter()
        .map(|record| {
            let contact: Contact = record
                .entry
                .clone()
                .into_option()
                .ok_or(
                    wasm_error!(WasmErrorInner::Guest(
                        String::from("Could not find the Contact")
                    )),
                )?
                .try_into()?;
            Ok(contact)
        })
        .collect::<ExternResult<Vec<Contact>>>()?;

    emit_signal(all_contacts.clone())?;

    let all_contacts = all_contacts
        .into_iter()
        .filter(|contact| {
            agent_pub_keys.contains(&contact.agent_pub_key)
        })
        .collect::<Vec<Contact>>();

    emit_signal(all_contacts.clone())?;

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
