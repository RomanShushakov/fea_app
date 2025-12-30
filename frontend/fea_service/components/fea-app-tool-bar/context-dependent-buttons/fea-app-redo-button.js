import { REDO_MESSAGE_HEADER } from "../../../consts/actions_router_consts.js";
import { CLIENT_MESSAGE_HEADER } from "../../../consts/fea_app_consts.js";


class FeaAppRedoButton extends HTMLElement {
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
            .redo-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .redo-button :hover {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .redo-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }
        </style>
        <button class="redo-button">
            <svg class="redo-button-icon" width="31" height="22" viewBox="0 0 31 22" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Redo</title>
                <path d="M12 14H7.6875C5.58057 14 2.709 14.52148.99561 17.00684l-.7544 1.09423L.062 
                    16.57959A5.59289 5.59289 0 0 1 0 15.86035C0 11.62891 3.12939 9 8.1665 9H12V4l12 
                    7.5L12 19Zm1-8.124V10H8.1665c-4.30664 0-6.94775 2.01807-7.15332 5.42432A9.08669 
                    9.08669 0 0 1 7.6875 13H13v4.124L22 11.5Z" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
            </svg>
        </button>
        `;

        this.shadowRoot.querySelector(".redo-button").addEventListener("click", () => this.redo());

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

    redo() {
        const message = { [REDO_MESSAGE_HEADER]: { "actionId": this.props.actionId } };
        this.dispatchEvent(new CustomEvent([CLIENT_MESSAGE_HEADER], {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));   
    }
}

export default FeaAppRedoButton;
