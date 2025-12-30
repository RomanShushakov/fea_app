import { UPDATE_BEAM_ELEMENT_VISIBILITY_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";


class FeaAppShowHideBeamElementButton extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            isBeamElementVisible: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .show-hide-beam-element-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .show-hide-beam-element-button:hover .show-hide-beam-element-button-icon {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .show-hide-beam-element-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .active:hover .show-hide-beam-element-button-icon {
                background: var(--toolbar-active-button-icon-hover-bg-color);
                border: var(--toolbar-active-button-icon-hover-bg-color);
            }

            .active-icon {
                background: var(--toolbar-active-button-icon-bg-color);
                border: var(--toolbar-active-button-icon-bg-color);
            }
        </style>
        <button class="show-hide-beam-element-button">
            <svg class="show-hide-beam-element-button-icon" width="26" height="26" viewBox="0 0 26 26" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Show/Hide beam element</title>
                <circle cx="2.18182" cy="2.18182" r="2.18182" transform="matrix(-1 0 0 1 25 1)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <circle cx="2.18182" cy="2.18182" r="2.18182" transform="matrix(-1 0 0 1 5.36365 20.6364)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <line x1="5.01009" y1="21.3737" x2="21.3737" y2="5.0101" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <line x1="12.6464" y1="13.7374" x2="18.101" y2="8.28284" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" stroke-width="1.5"
                />
                <line x1="12.5" y1="14" x2="12.5" y2="6" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-5)" stroke-width="1.5"
                />
                <line x1="13.1516" y1="13.6145" x2="19.1516" y2="15.5235" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-4)" stroke-width="1.5"
                />
                <path d="M19.5901 6.68737L18.4326 10.1531L16.1296 7.86031L19.5901 6.68737Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" stroke-width="1.5"
                />
                <path d="M12.5 4L13.799 7H11.201L12.5 4Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-5)" stroke-width="1.5"
                />
                <path d="M21.6326 16.4177L18.0383 16.8102L19.0353 13.9024L21.6326 16.4177Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-4)" stroke-width="1.5"
                />
            </svg>
        </button>
        `;

        this.shadowRoot.querySelector(".show-hide-beam-element-button").addEventListener(
            "click", () => this.updateBeamElementVisibility(),
        );

    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.updateBeamElementVisibility();
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
        const button = this.shadowRoot.querySelector(".show-hide-beam-element-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-beam-element-button-icon");
        if (button.classList.contains("active") == false) {
            button.classList.add("active");
            buttonIcon.classList.add("active-icon");
        }
    }

    deactivate() {
        const button = this.shadowRoot.querySelector(".show-hide-beam-element-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-beam-element-button-icon");
        if (button.classList.contains("active") == true) {
            button.classList.remove("active");
            buttonIcon.classList.remove("active-icon");
        }
    }

    updateBeamElementVisibility() {
        if (this.state.isBeamElementVisible == true) {
            this.state.isBeamElementVisible = false;
            this.activate();
        } else {
            this.state.isBeamElementVisible = true;
            this.deactivate();
        }

        this.dispatchEvent(new CustomEvent(UPDATE_BEAM_ELEMENT_VISIBILITY_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: { "is_beam_element_visible": this.state.isBeamElementVisible },
        }));   
    }
}

export default FeaAppShowHideBeamElementButton;
