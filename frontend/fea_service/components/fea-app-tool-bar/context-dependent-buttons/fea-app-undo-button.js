import { DECREASE_ACTION_ID_MESSAGE_HEADER, CLIENT_MESSAGE_HEADER } from "../../../consts/fea_app_consts.js";
import { UNDO_MESSAGE_HEADER } from "../../../consts/actions_router_consts.js";

class FeaAppUndoButton extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null, // u32
        };

        this.state = {};

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .undo-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .undo-button :hover {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .undo-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }
        </style>
        <button class="undo-button">
            <svg class="undo-button-icon" width="30" height="22" viewBox="0 0 30 22" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Undo</title>
                <path d="M12 19 0 11.5 12 4V9h3.8335C20.87061 9 24 11.62891 24 15.86035a5.59289 
                    5.59289 0 0 1-.062.71924l-.1792 1.52148-.7544-1.09423C21.291 14.52148 
                    18.41943 14 16.31249 14H12ZM2 11.5l9 5.624V13h5.31249a9.0867 9.0867 
                    0 0 1 6.67433 2.42432C22.78125 12.01807 20.14014 10 15.8335 10H11V5.876Z" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
            </svg>
        </button>
        `;

        this.shadowRoot.querySelector(".undo-button").addEventListener("click", () => this.undo());

    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
    }

    disconnectedCallback() {
    }

    static get observedAttributes() {
        return ["action-id"];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "action-id") {
            this.props.actionId = parseInt(newValue);
        }
    }

    adoptedCallback() {
    }

    undo() {
        if (this.props.actionId > 1) {
            this.dispatchEvent(new CustomEvent(DECREASE_ACTION_ID_MESSAGE_HEADER, {
                bubbles: true,
                composed: true,
            }));
            const message = { [UNDO_MESSAGE_HEADER]: { actionId: this.props.actionId } };
            this.dispatchEvent(new CustomEvent([CLIENT_MESSAGE_HEADER], {
                bubbles: true,
                composed: true,
                detail: {
                    message: message,
                },
            }));
        }
    }
}

export default FeaAppUndoButton;
