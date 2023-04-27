import { lazyLoadAndPoll, AsyncReadable } from "@holochain-open-dev/stores";
import { EntryRecord, LazyHoloHashMap } from "@holochain-open-dev/utils";
import { NewEntryAction, Record, ActionHash, EntryHash, AgentPubKey } from '@holochain/client';

import { NotificationsClient } from './notifications-client';

export class NotificationsStore {

  constructor(public client: NotificationsClient) {}
  
}
