use hdk::prelude::*;
use notifications_integrity::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddNotifierForNotificantInput {
    pub base_notificant: AgentPubKey,
    pub target_notifier: AgentPubKey,
}
#[hdk_extern]
pub fn add_notifier_for_notificant(
    input: AddNotifierForNotificantInput,
) -> ExternResult<()> {
    create_link(
        input.base_notificant.clone(),
        input.target_notifier.clone(),
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
