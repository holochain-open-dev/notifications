pub mod sent_notification;
pub use sent_notification::*;
pub mod contact;
pub use contact::*;
pub mod notificant_to_notifiers;
pub use notificant_to_notifiers::*;
pub mod twilio_credentials;
pub use twilio_credentials::*;
use hdi::prelude::*;
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def(name = "TwilioCredentials", visibility = "private")]
    TwilioCredentials(TwilioCredentials),
    #[entry_def(name = "Contact", visibility = "private")]
    Contact(Contact),
    #[entry_def(name = "SentNotification", visibility = "private")]
    SentNotification(SentNotification),
}
#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    TwilioCredentialsUpdates,
    NotificantToNotifiers,
    ContactUpdates,
    AnchorToNotifiers,
    SentNotificationUpdates,
}
#[hdk_extern]
pub fn genesis_self_check(
    _data: GenesisSelfCheckData,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_agent_joining(
    _agent_pub_key: AgentPubKey,
    _membrane_proof: &Option<MembraneProof>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
