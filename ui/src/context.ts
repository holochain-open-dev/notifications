import { createContext } from '@lit-labs/context';
import { NotificationsStore } from './notifications-store';

export const notificationsStoreContext = createContext<NotificationsStore>(
  'hc_zome_notifications/store'
);

