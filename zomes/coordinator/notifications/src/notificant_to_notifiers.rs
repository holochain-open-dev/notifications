use hdk::prelude::*;
use notifications_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentPubKeyWithTag {
    pub agent: AgentPubKey,
    pub tag: String,
}

#[hdk_extern]
pub fn list_notifiers(_: ()) -> ExternResult<Vec<AgentPubKeyWithTag>> {
    let path = Path::from(format!("all_notifiers"));
    let typed_path = path.typed(LinkTypes::AnchorToNotifiers)?;
    typed_path.ensure()?;
    let links = get_links(
        typed_path.path_entry_hash()?,
        LinkTypes::AnchorToNotifiers,
        None,
    )?;
    let agents: Vec<AgentPubKeyWithTag> = links
    .into_iter()
    .map(|link| {
        let tag = link.tag;
        let tag_str = String::from_utf8(tag.0).unwrap();
        let agent = AgentPubKey::from(EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap());
        let agent_with_tag = AgentPubKeyWithTag {
            agent: agent.clone(),
            tag: tag_str,
        };
        agent_with_tag
    })
    .collect();

    // let agents: Vec<AgentPubKey> = links
    //     .into_iter()
    //     .map(|link| AgentPubKey::from(EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap()))
    //     .collect();
    Ok(agents)
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct AddNotifierForNotificantInput {
//     pub base_notificant: AgentPubKey,
//     pub target_notifier: AgentPubKey,
// }
#[hdk_extern]
pub fn select_notifier(input: AgentPubKey) -> ExternResult<()> {
    create_link(
        agent_info()?.agent_latest_pubkey.clone(),
        input.clone(),
        LinkTypes::NotificantToNotifiers,
        (),
    )?;
    Ok(())
}

#[hdk_extern]
pub fn select_first_notifier(_: ()) -> ExternResult<()> {
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
        .map(|link| AgentPubKey::from(EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected hash".into()))).unwrap()))
        .collect();
    let notifier = agents[0].clone();

    create_link(
        agent_info()?.agent_latest_pubkey.clone(),
        notifier.clone(),
        LinkTypes::NotificantToNotifiers,
        (),
    )?;
    Ok(())
}

#[hdk_extern]
pub fn get_notifiers_for_notificant(
    notificant: AgentPubKey,
) -> ExternResult<Vec<AgentPubKey>> {
    let links = get_links(notificant, LinkTypes::NotificantToNotifiers, None)?;
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap()))
        .collect();
    Ok(agents)
}
#[hdk_extern]
pub fn get_my_notifier(_: ()) -> ExternResult<AgentPubKey> {
    let me: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    let links = get_links(me, LinkTypes::NotificantToNotifiers, None)?;
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap()))
        .collect();
    Ok(agents[0].clone())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveNotifierForNotificantInput {
    pub base_notificant: AgentPubKey,
    pub target_notifier: AgentPubKey,
}
#[hdk_extern]
pub fn remove_notifier_for_notificant(
    input: RemoveNotifierForNotificantInput,
) -> ExternResult<()> {
    let links = get_links(
        input.base_notificant.clone(),
        LinkTypes::NotificantToNotifiers,
        None,
    )?;
    for link in links {
        if AgentPubKey::from(EntryHash::try_from(link.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap())
            .eq(&input.target_notifier)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
