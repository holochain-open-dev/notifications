use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct TwilioCredentials {
    pub account_sid: String,
    pub auth_token: String,
    pub from_number_text: String,
    pub from_number_whatsapp: String,
}
pub fn validate_create_twilio_credentials(
    _action: EntryCreationAction,
    _twilio_credentials: TwilioCredentials,
) -> ExternResult<ValidateCallbackResult> {
    // debug!("-----------------------> validate create twilio: {:?}", _action);
    // Ok(ValidateCallbackResult::Invalid("testing".into()))
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_twilio_credentials(
    _action: Update,
    _twilio_credentials: TwilioCredentials,
    _original_action: EntryCreationAction,
    _original_twilio_credentials: TwilioCredentials,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_twilio_credentials(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_twilio_credentials: TwilioCredentials,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_twilio_credentials_updates(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::try_from(base_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into())))?;
    let record = must_get_valid_record(action_hash)?;
    let _twilio_credentials: crate::TwilioCredentials = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    let action_hash = ActionHash::try_from(target_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into())))?;
    let record = must_get_valid_record(action_hash)?;
    let _twilio_credentials: crate::TwilioCredentials = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_twilio_credentials_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("TwilioCredentialsUpdates links cannot be deleted"),
        ),
    )
}
