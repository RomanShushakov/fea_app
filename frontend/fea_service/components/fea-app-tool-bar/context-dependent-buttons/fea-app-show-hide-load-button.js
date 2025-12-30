import { UPDATE_LOAD_VISIBILITY_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";


class FeaAppShowHideLoadButton extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            isLoadVisible: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .show-hide-load-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .show-hide-load-button:hover .show-hide-load-button-icon {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .show-hide-load-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .active:hover .show-hide-load-button-icon {
                background: var(--toolbar-active-button-icon-hover-bg-color);
                border: var(--toolbar-active-button-icon-hover-bg-color);
            }

            .active-icon {
                background: var(--toolbar-active-button-icon-bg-color);
                border: var(--toolbar-active-button-icon-bg-color);
            }
        </style>
        <button class="show-hide-load-button">
            <svg class="show-hide-load-button-icon" width="26" height="24" viewBox="0 0 26 24" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Show/Hide load</title>
                <path d="M9.91153 14.003L1.01688 22.983C1.01062 22.9893 1.01509 23 1.02398 23H15.8905C15.8932 23 
                    15.8957 22.9989 15.8976 22.997L24.7923 14.017C24.7985 14.0107 24.794 14 24.7852 
                    14H9.91863C9.91597 14 9.91341 14.0011 9.91153 14.003Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <path d="M13 16L11.2679 11.5H14.7321L13 16Z" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke-width="2"
                />
                <path d="M13 0L13 13" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke-width="2"
                />
            </svg>
        </button>
        `;

        this.shadowRoot.querySelector(".show-hide-load-button").addEventListener("click", () => this.updateLoadVisibility());

    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.updateLoadVisibility();
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
        const button = this.shadowRoot.querySelector(".show-hide-load-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-load-button-icon");
        if (button.classList.contains("active") == false) {
            button.classList.add("active");
            buttonIcon.classList.add("active-icon");
        }
    }

    deactivate() {
        const button = this.shadowRoot.querySelector(".show-hide-load-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-load-button-icon");
        if (button.classList.contains("active") == true) {
            button.classList.remove("active");
            buttonIcon.classList.remove("active-icon");
        }
    }

    updateLoadVisibility() {
        if (this.state.isLoadVisible == true) {
            this.state.isLoadVisible = false;
            this.activate();
        } else {
            this.state.isLoadVisible = true;
            this.deactivate();
        }

        this.dispatchEvent(new CustomEvent(UPDATE_LOAD_VISIBILITY_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: { "is_load_visible": this.state.isLoadVisible },
        }));    
    }
}

export default FeaAppShowHideLoadButton;
