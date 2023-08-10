import { ZomeClient } from '@holochain-open-dev/utils';
export class NotificationsClient extends ZomeClient {
    constructor(client, roleName, zomeName = 'notifications') {
        super(client, roleName, zomeName);
        this.client = client;
        this.roleName = roleName;
        this.zomeName = zomeName;
    }
}
//# sourceMappingURL=notifications-client.js.map