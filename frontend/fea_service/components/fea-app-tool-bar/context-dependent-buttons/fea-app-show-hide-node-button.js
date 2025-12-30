import { UPDATE_NODE_VISIBILITY_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";


class FeaAppShowHideNodeButton extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            isNodeVisible: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .show-hide-node-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .show-hide-node-button:hover .show-hide-node-button-icon {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .show-hide-node-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .active:hover .show-hide-node-button-icon {
                background: var(--toolbar-active-button-icon-hover-bg-color);
                border: var(--toolbar-active-button-icon-hover-bg-color);
            }

            .active-icon {
                background: var(--toolbar-active-button-icon-bg-color);
                border: var(--toolbar-active-button-icon-bg-color);
            }
        </style>
        <button class="show-hide-node-button">
            <svg class="show-hide-node-button-icon" width="24" height="24" viewBox="0 0 24 24" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Show/Hide node</title>
                <circle cx="2" cy="2" r="2" transform="matrix(-1 0 0 1 14 10)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
            </svg>
        </button>
        `;

        this.shadowRoot.querySelector(".show-hide-node-button").addEventListener("click", () => this.updateNodeVisibility());

    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.updateNodeVisibility();
    }

    disconnectedCallback() {
    }

    static get observedAttributes() {
        return [];
    }

    attributeChangedCallback(name, oldValue, newValue) {
    }

    adoptedCallback() {
    }

    activate() {
        const button = this.shadowRoot.querySelector(".show-hide-node-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-node-button-icon");
        if (button.classList.contains("active") == false) {
            button.classList.add("active");
            buttonIcon.classList.add("active-icon");
        }
    }

    deactivate() {
        const button = this.shadowRoot.querySelector(".show-hide-node-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-node-button-icon");
        if (button.classList.contains("active") == true) {
            button.classList.remove("active");
            buttonIcon.classList.remove("active-icon");
        }
    }

    updateNodeVisibility() {
        if (this.state.isNodeVisible == true) {
            this.state.isNodeVisible = false;
            this.activate();
        } else {
            this.state.isNodeVisible = true;
            this.deactivate();
        }

        this.dispatchEvent(new CustomEvent(UPDATE_NODE_VISIBILITY_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: { "is_node_visible": this.state.isNodeVisible },
        }));   
    }
}

export default FeaAppShowHideNodeButton;
