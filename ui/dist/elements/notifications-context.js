import { __decorate } from "tslib";
import { css, html, LitElement } from 'lit';
import { provide } from '@lit-labs/context';
import { customElement, property } from 'lit/decorators.js';
import { notificationsStoreContext } from '../context.js';
let NotificationsContext = class NotificationsContext extends LitElement {
    render() {
        return html `<slot></slot>`;
    }
};
NotificationsContext.styles = css `
    :host {
      display: contents;
    }
  `;
__decorate([
    provide({ context: notificationsStoreContext }),
    property({ type: Object })
], NotificationsContext.prototype, "store", void 0);
NotificationsContext = __decorate([
    customElement('notifications-context')
], NotificationsContext);
export { NotificationsContext };
//# sourceMappingURL=notifications-context.js.map