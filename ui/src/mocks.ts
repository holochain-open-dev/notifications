import {
  AgentPubKeyMap,
  decodeEntry,
  fakeEntry,
  fakeCreateAction,
  fakeUpdateEntry,
  fakeDeleteEntry,
  fakeRecord,
  fakeAgentPubKey,
  fakeDnaHash,
  fakeActionHash,
  fakeEntryHash,
  pickBy,
  ZomeMock,
  RecordBag,
  entryState,
  HoloHashMap,
  HashType,
  hash
} from "@holochain-open-dev/utils";
import {
  decodeHashFromBase64,
  AgentPubKey,
  ActionHash,
  EntryHash,
  AppAgentClient,
  Record,
} from "@holochain/client";

export class NotificationsZomeMock extends ZomeMock implements AppAgentClient {
  constructor(
    myPubKey?: AgentPubKey
  ) {
    super("notifications_test", "notifications", myPubKey);
  }
  
}
