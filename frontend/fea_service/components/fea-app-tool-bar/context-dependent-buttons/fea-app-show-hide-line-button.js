import { UPDATE_LINE_VISIBILITY_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";


class FeaAppShowHideLineButton extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            isLineVisible: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .show-hide-line-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .show-hide-line-button:hover .show-hide-line-button-icon {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .show-hide-line-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .active:hover .show-hide-line-button-icon {
                background: var(--toolbar-active-button-icon-hover-bg-color);
                border: var(--toolbar-active-button-icon-hover-bg-color);
            }

            .active-icon {
                background: var(--toolbar-active-button-icon-bg-color);
                border: var(--toolbar-active-button-icon-bg-color);
            }
        </style>
        <button class="show-hide-line-button">

            <svg class="show-hide-line-button-icon" width="24" height="24" viewBox="0 0 24 24" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Show/Hide line</title>
                <rect x="19.2" y="1.37146" width="3.42857" height="3.42857" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <rect x="2.05713" y="18.5143" width="3.42857" height="3.42857" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <line x1="5.81789" y1="18.1607" x2="18.8465" y2="5.13216" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
            </svg>
        </button>
        `;

        this.shadowRoot.querySelector(".show-hide-line-button").addEventListener("click", 
            () => this.updateLineVisibility());

    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.updateLineVisibility();
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
        const button = this.shadowRoot.querySelector(".show-hide-line-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-line-button-icon");
        if (button.classList.contains("active") == false) {
            button.classList.add("active");
            buttonIcon.classList.add("active-icon");
        }
    }

    deactivate() {
        const button = this.shadowRoot.querySelector(".show-hide-line-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-line-button-icon");
        if (button.classList.contains("active") == true) {
            button.classList.remove("active");
            buttonIcon.classList.remove("active-icon");
        }
    }

    updateLineVisibility() {
        if (this.state.isLineVisible == true) {
            this.state.isLineVisible = false;
            this.activate();
        } else {
            this.state.isLineVisible = true;
            this.deactivate();
        }

        this.dispatchEvent(new CustomEvent(UPDATE_LINE_VISIBILITY_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: { "is_line_visible": this.state.isLineVisible },
        }));   
    }
}

export default FeaAppShowHideLineButton;
