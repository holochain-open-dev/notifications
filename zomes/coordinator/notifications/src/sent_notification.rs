use hdk::prelude::*;
use notifications_integrity::*;

#[hdk_extern]
pub fn retrieve_sent_notifications(_:()) -> ExternResult<Vec<Record>> {
  let sent_notifications_entry_type: EntryType = UnitEntryTypes::SentNotification.try_into()?;
  let filter = ChainQueryFilter::new()
      .entry_type(sent_notifications_entry_type)
      .include_entries(true);
  let all_sent_notifications = query(filter)?;

  Ok(all_sent_notifications)
}

#[hdk_extern]
pub fn was_it_sent(message_id: String) -> ExternResult<bool> {
  let sent_notifications_entry_type: EntryType = UnitEntryTypes::SentNotification.try_into()?;
  let filter = ChainQueryFilter::new()
      .entry_type(sent_notifications_entry_type)
      .include_entries(true);
  let all_sent_notifications_records = query(filter)?;

  emit_signal(all_sent_notifications_records.clone())?;

  let output: bool;

  let all_sent_notifications = all_sent_notifications_records
    .into_iter()
    .map(|record| {
        let sent_notification: SentNotification = record
            .entry
            .clone()
            .into_option()
            .ok_or(
                wasm_error!(WasmErrorInner::Guest(
                    String::from("Could not find the SentNotification")
                )),
            )?
            .try_into()?;
        Ok(sent_notification)
    })
    .collect::<ExternResult<Vec<SentNotification>>>()?;

    emit_signal(all_sent_notifications.clone())?;

    let filtered_sent_notifications = all_sent_notifications
    .into_iter()
    .filter(|sent_notification| {
        sent_notification.unique_data == message_id
    })
    .collect::<Vec<SentNotification>>();

    emit_signal(filtered_sent_notifications.clone())?;

    if filtered_sent_notifications.len() > 0 {
      output = true;
    } else {
      output = false;
    }

  Ok(output)
}

#[hdk_extern]
pub fn create_sent_notification(
    sent_notification: SentNotification,
) -> ExternResult<Record> {
    let sent_notification_hash = create_entry(
        &EntryTypes::SentNotification(sent_notification.clone()),
    )?;
    let record = get(sent_notification_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created SentNotification"))
            ),
        )?;
    Ok(record)
}
// #[hdk_extern]
// pub fn get_sent_notification(
//     original_sent_notification_hash: ActionHash,
// ) -> ExternResult<Option<Record>> {
//     let links = get_links(
//         original_sent_notification_hash.clone(),
//         LinkTypes::SentNotificationUpdates,
//         None,
//     )?;
//     let latest_link = links
//         .into_iter()
//         .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
//     let latest_sent_notification_hash = match latest_link {
//         Some(link) => ActionHash::from(link.target.clone()),
//         None => original_sent_notification_hash.clone(),
//     };
//     get(latest_sent_notification_hash, GetOptions::default())
// }
// #[derive(Serialize, Deserialize, Debug)]
// pub struct UpdateSentNotificationInput {
//     pub original_sent_notification_hash: ActionHash,
//     pub previous_sent_notification_hash: ActionHash,
//     pub updated_sent_notification: SentNotification,
// }
// #[hdk_extern]
// pub fn update_sent_notification(
//     input: UpdateSentNotificationInput,
// ) -> ExternResult<Record> {
//     let updated_sent_notification_hash = update_entry(
//         input.previous_sent_notification_hash.clone(),
//         &input.updated_sent_notification,
//     )?;
//     create_link(
//         input.original_sent_notification_hash.clone(),
//         updated_sent_notification_hash.clone(),
//         LinkTypes::SentNotificationUpdates,
//         (),
//     )?;
//     let record = get(updated_sent_notification_hash.clone(), GetOptions::default())?
//         .ok_or(
//             wasm_error!(
//                 WasmErrorInner::Guest(String::from("Could not find the newly updated SentNotification"))
//             ),
//         )?;
//     Ok(record)
// }
// #[hdk_extern]
// pub fn delete_sent_notification(
//     original_sent_notification_hash: ActionHash,
// ) -> ExternResult<ActionHash> {
//     delete_entry(original_sent_notification_hash)
// }
