import { UPDATE_PLATE_ELEMENT_VISIBILITY_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";


class FeaAppShowHidePlateElementButton extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            isPlateElementVisible: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .show-hide-plate-element-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .show-hide-plate-element-button:hover .show-hide-plate-element-button-icon {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .show-hide-plate-element-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .active:hover .show-hide-plate-element-button-icon {
                background: var(--toolbar-active-button-icon-hover-bg-color);
                border: var(--toolbar-active-button-icon-hover-bg-color);
            }

            .active-icon {
                background: var(--toolbar-active-button-icon-bg-color);
                border: var(--toolbar-active-button-icon-bg-color);
            }
        </style>
        <button class="show-hide-plate-element-button">
            <svg class="show-hide-plate-element-button-icon" width="26" height="26" viewBox="0 0 26 26" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Show/Hide plate element</title>
                <line x1="4.38245" y1="22.753" x2="21.2713" y2="16.5308" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <line x1="4.38245" y1="9.41974" x2="21.2713" y2="3.19752" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <line x1="3.27783" y1="11.6666" x2="3.27783" y2="21.4444" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <line x1="22.8334" y1="4.55554" x2="22.8334" y2="14.3333" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <circle cx="1.77778" cy="1.77778" r="1.77778" transform="matrix(-1 0 0 1 25 1)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <circle cx="1.77778" cy="1.77778" r="1.77778" transform="matrix(-1 0 0 1 4.55554 21.4445)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <circle cx="1.77778" cy="1.77778" r="1.77778" transform="matrix(-1 0 0 1 4.55554 8.11108)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <circle cx="1.77778" cy="1.77778" r="1.77778" transform="matrix(-1 0 0 1 25 14.3334)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <path d="M13.1302 14.1L20.1112 11.2223" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" stroke-width="1.5"
                />
                <line x1="12.9445" y1="14.3334" x2="12.9445" y2="7.22226" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-5)" stroke-width="1.5"
                />
                <line x1="12.7791" y1="13.8862" x2="18.1125" y2="16.5528" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-4)" stroke-width="1.5"
                />
                <path d="M21.2141 10.4818L19.3525 13.1433L17.9687 10.6077L21.2141 10.4818Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" stroke-width="1.5"
                />
                <path d="M13 5.44446L14.1547 8.11112H11.8453L13 5.44446Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-5)" stroke-width="1.5"
                />
                <path d="M20.2067 17.7611L16.9343 17.8346L18.2496 15.1374L20.2067 17.7611Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-4)" stroke-width="1.5"
                />
            </svg>
        </button>
        `;

        this.shadowRoot.querySelector(".show-hide-plate-element-button").addEventListener(
            "click", () => this.updatePlateElementVisibility(),
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
        this.updatePlateElementVisibility();
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
        const button = this.shadowRoot.querySelector(".show-hide-plate-element-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-plate-element-button-icon");
        if (button.classList.contains("active") == false) {
            button.classList.add("active");
            buttonIcon.classList.add("active-icon");
        }
    }

    deactivate() {
        const button = this.shadowRoot.querySelector(".show-hide-plate-element-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-plate-element-button-icon");
        if (button.classList.contains("active") == true) {
            button.classList.remove("active");
            buttonIcon.classList.remove("active-icon");
        }
    }

    updatePlateElementVisibility() {
        if (this.state.isPlateElementVisible == true) {
            this.state.isPlateElementVisible = false;
            this.activate();
        } else {
            this.state.isPlateElementVisible = true;
            this.deactivate();
        }

        this.dispatchEvent(new CustomEvent(UPDATE_PLATE_ELEMENT_VISIBILITY_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: { "is_plate_element_visible": this.state.isPlateElementVisible },
        }));   
    }
}

export default FeaAppShowHidePlateElementButton;
