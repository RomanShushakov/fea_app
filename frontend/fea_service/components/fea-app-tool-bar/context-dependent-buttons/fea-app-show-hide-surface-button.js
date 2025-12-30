import { UPDATE_SURFACE_VISIBILITY_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";


class FeaAppShowHideSurfaceButton extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            isSurfaceVisible: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .show-hide-surface-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .show-hide-surface-button:hover .show-hide-surface-button-icon {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .show-hide-surface-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .active:hover .show-hide-surface-button-icon {
                background: var(--toolbar-active-button-icon-hover-bg-color);
                border: var(--toolbar-active-button-icon-hover-bg-color);
            }

            .active-icon {
                background: var(--toolbar-active-button-icon-bg-color);
                border: var(--toolbar-active-button-icon-bg-color);
            }
        </style>
        <button class="show-hide-surface-button">

            <svg class="show-hide-surface-button-icon" width="24" height="24" viewBox="0 0 24 24" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Show/Hide surface</title>
                <rect x="6.17145" y="1.37146" width="3.42857" height="3.42857" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <rect x="18.5143" y="3.42859" width="3.42857" height="3.42857" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <rect x="2.74286" y="16.4572" width="3.42857" height="3.42857" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <rect x="15.0857" y="18.5143" width="3.42857" height="3.42857" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <line x1="17.0344" y1="5.28814" x2="10.863" y2="3.91671" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <line x1="13.6058" y1="19.688" x2="7.4344" y2="18.3166" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <line x1="17.3414" y1="17.0304" x2="19.3985" y2="8.11611" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <line x1="4.99852" y1="14.9733" x2="7.05567" y2="6.05898" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
            </svg>
        </button>
        `;

        this.shadowRoot.querySelector(".show-hide-surface-button").addEventListener("click", 
            () => this.updateSurfaceVisibility());

    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.updateSurfaceVisibility();
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
        const button = this.shadowRoot.querySelector(".show-hide-surface-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-surface-button-icon");
        if (button.classList.contains("active") == false) {
            button.classList.add("active");
            buttonIcon.classList.add("active-icon");
        }
    }

    deactivate() {
        const button = this.shadowRoot.querySelector(".show-hide-surface-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-surface-button-icon");
        if (button.classList.contains("active") == true) {
            button.classList.remove("active");
            buttonIcon.classList.remove("active-icon");
        }
    }

    updateSurfaceVisibility() {
        if (this.state.isSurfaceVisible == true) {
            this.state.isSurfaceVisible = false;
            this.activate();
        } else {
            this.state.isSurfaceVisible = true;
            this.deactivate();
        }

        this.dispatchEvent(new CustomEvent(UPDATE_SURFACE_VISIBILITY_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: { "is_surface_visible": this.state.isSurfaceVisible },
        }));   
    }
}

export default FeaAppShowHideSurfaceButton;
