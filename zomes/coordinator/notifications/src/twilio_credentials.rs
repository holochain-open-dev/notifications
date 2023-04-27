use hdk::prelude::*;
use notifications_integrity::*;
fn functions_to_grant_capability_for() -> ExternResult<GrantedFunctions> {
    let mut functions: BTreeSet<(ZomeName, FunctionName)> = BTreeSet::new();
    functions.insert((zome_info()?.name, FunctionName(String::from("create_contact"))));
    functions
        .insert((
            zome_info()?.name,
            FunctionName(String::from("handle_notification_tip")),
        ));
    Ok(GrantedFunctions::Listed(functions))
}
#[hdk_extern]
fn grant_unrestricted_capability(_: ()) -> ExternResult<()> {
    let functions = functions_to_grant_capability_for()?;
    let access = CapAccess::Unrestricted;
    let capability_grant = CapGrantEntry {
        functions,
        access,
        tag: String::from("unrestricted"),
    };
    create_cap_grant(capability_grant)?;
    Ok(())
}
#[hdk_extern]
pub fn create_twilio_credentials(
    twilio_credentials: TwilioCredentials,
) -> ExternResult<Record> {
    let twilio_credentials_hash = create_entry(
        &EntryTypes::TwilioCredentials(twilio_credentials.clone()),
    )?;
    let record = get(twilio_credentials_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created TwilioCredentials"))
            ),
        )?;
    grant_unrestricted_capability(())?;
    Ok(record)
}
#[hdk_extern]
pub fn get_twilio_credentials(_:(),
) -> ExternResult<Option<Record>> {

    let twilio_credentials_entry_type: EntryType = UnitEntryTypes::TwilioCredentials.try_into()?;
    let filter = ChainQueryFilter::new().entry_type(twilio_credentials_entry_type);
    let all_credentials = query(filter)?;
    let latest_hash = all_credentials[all_credentials.len() - 1].clone();
    let latest_record = get(latest_hash.signed_action.hashed.hash, GetOptions::default());
    latest_record


    // let links = get_links(
    //     original_twilio_credentials_hash.clone(),
    //     LinkTypes::TwilioCredentialsUpdates,
    //     None,
    // )?;
    // let latest_link = links
    //     .into_iter()
    //     .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    // let latest_twilio_credentials_hash = match latest_link {
    //     Some(link) => ActionHash::from(link.target.clone()),
    //     None => original_twilio_credentials_hash.clone(),
    // };
    // get(latest_twilio_credentials_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTwilioCredentialsInput {
    pub original_twilio_credentials_hash: ActionHash,
    pub previous_twilio_credentials_hash: ActionHash,
    pub updated_twilio_credentials: TwilioCredentials,
}
#[hdk_extern]
pub fn update_twilio_credentials(
    input: UpdateTwilioCredentialsInput,
) -> ExternResult<Record> {
    let updated_twilio_credentials_hash = update_entry(
        input.previous_twilio_credentials_hash.clone(),
        &input.updated_twilio_credentials,
    )?;
    create_link(
        input.original_twilio_credentials_hash.clone(),
        updated_twilio_credentials_hash.clone(),
        LinkTypes::TwilioCredentialsUpdates,
        (),
    )?;
    let record = get(updated_twilio_credentials_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated TwilioCredentials"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_twilio_credentials(
    original_twilio_credentials_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_twilio_credentials_hash)
}
