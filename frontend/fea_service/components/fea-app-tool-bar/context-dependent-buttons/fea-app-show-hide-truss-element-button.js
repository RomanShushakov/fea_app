import { UPDATE_TRUSS_ELEMENT_VISIBILITY_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";


class FeaAppShowHideTrussElementButton extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            isTrussElementVisible: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .show-hide-truss-element-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .show-hide-truss-element-button:hover .show-hide-truss-element-button-icon {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .show-hide-truss-element-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .active:hover .show-hide-truss-element-button-icon {
                background: var(--toolbar-active-button-icon-hover-bg-color);
                border: var(--toolbar-active-button-icon-hover-bg-color);
            }

            .active-icon {
                background: var(--toolbar-active-button-icon-bg-color);
                border: var(--toolbar-active-button-icon-bg-color);
            }
        </style>
        <button class="show-hide-truss-element-button">
            <svg class="show-hide-truss-element-button-icon" width="26" height="26" viewBox="0 0 26 26" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Show/Hide truss element</title>
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
                <path d="M19.5901 6.68737L18.4326 10.1531L16.1296 7.86031L19.5901 6.68737Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" stroke-width="1.5"
                />
            </svg>
        </button>
        `;

        this.shadowRoot.querySelector(".show-hide-truss-element-button").addEventListener(
            "click", () => this.updateTrussElementVisibility(),
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
        this.updateTrussElementVisibility();
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
        const button = this.shadowRoot.querySelector(".show-hide-truss-element-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-truss-element-button-icon");
        if (button.classList.contains("active") == false) {
            button.classList.add("active");
            buttonIcon.classList.add("active-icon");
        }
    }

    deactivate() {
        const button = this.shadowRoot.querySelector(".show-hide-truss-element-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-truss-element-button-icon");
        if (button.classList.contains("active") == true) {
            button.classList.remove("active");
            buttonIcon.classList.remove("active-icon");
        }
    }

    updateTrussElementVisibility() {
        if (this.state.isTrussElementVisible == true) {
            this.state.isTrussElementVisible = false;
            this.activate();
        } else {
            this.state.isTrussElementVisible = true;
            this.deactivate();
        }

        this.dispatchEvent(new CustomEvent(UPDATE_TRUSS_ELEMENT_VISIBILITY_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: { "is_truss_element_visible": this.state.isTrussElementVisible },
        }));   
    }
}

export default FeaAppShowHideTrussElementButton;
