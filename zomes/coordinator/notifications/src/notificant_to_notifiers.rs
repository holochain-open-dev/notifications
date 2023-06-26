use hdk::prelude::*;
use notifications_integrity::*;

#[hdk_extern]
pub fn list_notifiers(_: ()) -> ExternResult<Vec<AgentPubKey>> {
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
        .map(|link| AgentPubKey::from(EntryHash::from(link.target)))
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
        .map(|link| AgentPubKey::from(EntryHash::from(link.target)))
        .collect();
    Ok(agents)
}
#[hdk_extern]
pub fn get_my_notifier(_: ()) -> ExternResult<AgentPubKey> {
    let me: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    let links = get_links(me, LinkTypes::NotificantToNotifiers, None)?;
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::from(link.target)))
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
        if AgentPubKey::from(EntryHash::from(link.target.clone()))
            .eq(&input.target_notifier)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
