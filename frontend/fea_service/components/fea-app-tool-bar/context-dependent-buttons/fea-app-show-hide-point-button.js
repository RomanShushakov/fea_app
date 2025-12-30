import { UPDATE_POINT_VISIBILITY_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";


class FeaAppShowHidePointButton extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            isPointVisible: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .show-hide-point-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .show-hide-point-button:hover .show-hide-point-button-icon {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .show-hide-point-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .active:hover .show-hide-point-button-icon {
                background: var(--toolbar-active-button-icon-hover-bg-color);
                border: var(--toolbar-active-button-icon-hover-bg-color);
            }

            .active-icon {
                background: var(--toolbar-active-button-icon-bg-color);
                border: var(--toolbar-active-button-icon-bg-color);
            }
        </style>
        <button class="show-hide-point-button">

            <svg class="show-hide-point-button-icon" width="24" height="24" viewBox="0 0 24 24" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Show/Hide point</title>
                <rect x="10.2857" y="10.2858" width="3.42857" height="3.42857" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
            </svg>
        </button>
        `;

        this.shadowRoot.querySelector(".show-hide-point-button").addEventListener("click", () => this.updatePointVisibility());

    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.updatePointVisibility();
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
        const button = this.shadowRoot.querySelector(".show-hide-point-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-point-button-icon");
        if (button.classList.contains("active") == false) {
            button.classList.add("active");
            buttonIcon.classList.add("active-icon");
        }
    }

    deactivate() {
        const button = this.shadowRoot.querySelector(".show-hide-point-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-point-button-icon");
        if (button.classList.contains("active") == true) {
            button.classList.remove("active");
            buttonIcon.classList.remove("active-icon");
        }
    }

    updatePointVisibility() {
        if (this.state.isPointVisible == true) {
            this.state.isPointVisible = false;
            this.activate();
        } else {
            this.state.isPointVisible = true;
            this.deactivate();
        }

        this.dispatchEvent(new CustomEvent(UPDATE_POINT_VISIBILITY_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: { "is_point_visible": this.state.isPointVisible },
        }));   
    }
}

export default FeaAppShowHidePointButton;
