import { UPDATE_SURFACE_NORMAL_VISIBILITY_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";

class FeaAppShowHideSurfaceNormalButton extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            isSurfaceNormalVisible: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .show-hide-surface-normal-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .show-hide-surface-normal-button:hover .show-hide-surface-normal-button-icon {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .show-hide-surface-normal-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .active:hover .show-hide-surface-normal-button-icon {
                background: var(--toolbar-active-button-icon-hover-bg-color);
                border: var(--toolbar-active-button-icon-hover-bg-color);
            }

            .active-icon {
                background: var(--toolbar-active-button-icon-bg-color);
                border: var(--toolbar-active-button-icon-bg-color);
            }
        </style>
        <button class="show-hide-surface-normal-button">

            <svg class="show-hide-surface-normal-button-icon" width="38" height="39" viewBox="0 0 38 39" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Show/Hide surface normal</title>

                <path d="M19.6144 3.64647C19.4192 3.4512 19.1026 3.4512 18.9073 3.64647L15.7253 6.82845C15.5301 
                    7.02371 15.5301 7.34029 15.7253 7.53555C15.9206 7.73082 16.2372 7.73082 16.4324 7.53555L19.2609 
                    4.70713L22.0893 7.53555C22.2846 7.73082 22.6011 7.73082 22.7964 7.53555C22.9917 7.34029 22.9917 
                    7.02371 22.7964 6.82845L19.6144 3.64647ZM19.7609 24.4167L19.7609 4.00002H18.7609L18.7609 
                    24.4167H19.7609Z" fill="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" stroke-width="1.5" 
                />
                <line y1="-0.5" x2="11.8853" y2="-0.5" 
                    transform="matrix(-0.51214 0.858902 -0.415292 -0.909688 30.6739 20.0417)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <line y1="-0.5" x2="11.8853" y2="-0.5" 
                    transform="matrix(-0.51214 0.858902 -0.415292 -0.909688 11.6522 20.0417)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <line x1="17.7391" y1="35.125" x2="7.08696" y2="35.125" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <line x1="29.9131" y1="14.7083" x2="19.2609" y2="14.7083" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <rect x="1" y="33" width="5" height="5" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <rect x="19" y="33" width="5" height="5" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <rect x="32" y="12" width="5" height="5" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <rect x="12" y="12" width="5" height="5" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <path d="M26.8486 4.84473V9H26.2676V3.7168H26.8242L26.8486 4.84473ZM26.7168 6.03125L26.458 
                    5.88477C26.4775 5.57227 26.541 5.2793 26.6484 5.00586C26.7559 4.73242 26.9007 4.49154 27.083 
                    4.2832C27.2686 4.07487 27.485 3.91211 27.7324 3.79492C27.9831 3.67773 28.2598 3.61914 28.5625 
                    3.61914C28.8262 3.61914 29.0622 3.65495 29.2705 3.72656C29.4788 3.79818 29.6562 3.91211 29.8027 
                    4.06836C29.9492 4.22461 30.0599 4.4248 30.1348 4.66895C30.2129 4.91309 30.252 5.21094 30.252 
                    5.5625V9H29.666V5.55762C29.666 5.19303 29.6156 4.9082 29.5146 4.70312C29.4137 4.49479 29.2705 
                    4.34668 29.085 4.25879C28.8994 4.1709 28.6797 4.12695 28.4258 4.12695C28.1458 4.12695 27.9001 
                    4.1888 27.6885 4.3125C27.4801 4.43294 27.3044 4.59082 27.1611 4.78613C27.0212 4.97819 26.9137 
                    5.1849 26.8389 5.40625C26.764 5.62435 26.7233 5.83268 26.7168 6.03125Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
            </svg>
        </button>
        `;

        this.shadowRoot.querySelector(".show-hide-surface-normal-button").addEventListener("click", 
            () => this.updateSurfaceNormalVisibility());

    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.updateSurfaceNormalVisibility();
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
        const button = this.shadowRoot.querySelector(".show-hide-surface-normal-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-surface-normal-button-icon");
        if (button.classList.contains("active") == false) {
            button.classList.add("active");
            buttonIcon.classList.add("active-icon");
        }
    }

    deactivate() {
        const button = this.shadowRoot.querySelector(".show-hide-surface-normal-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-surface-normal-button-icon");
        if (button.classList.contains("active") == true) {
            button.classList.remove("active");
            buttonIcon.classList.remove("active-icon");
        }
    }

    updateSurfaceNormalVisibility() {
        if (this.state.isSurfaceNormalVisible == true) {
            this.state.isSurfaceNormalVisible = false;
            this.activate();
        } else {
            this.state.isSurfaceNormalVisible = true;
            this.deactivate();
        }

        this.dispatchEvent(new CustomEvent(UPDATE_SURFACE_NORMAL_VISIBILITY_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: { "is_surface_normal_visible": this.state.isSurfaceNormalVisible },
        }));    
    }
}

export default FeaAppShowHideSurfaceNormalButton;
