import { UPDATE_BOUNDARY_CONDITION_VISIBILITY_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";


class FeaAppShowHideBoundaryConditionButton extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            isBoundaryConditionVisible: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .show-hide-boundary-condition-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .show-hide-boundary-condition-button:hover .show-hide-boundary-condition-button-icon {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .show-hide-boundary-condition-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .active:hover .show-hide-boundary-condition-button-icon {
                background: var(--toolbar-active-button-icon-hover-bg-color);
                border: var(--toolbar-active-button-icon-hover-bg-color);
            }

            .active-icon {
                background: var(--toolbar-active-button-icon-bg-color);
                border: var(--toolbar-active-button-icon-bg-color);
            }
        </style>
        <button class="show-hide-boundary-condition-button">

            <svg class="show-hide-boundary-condition-button-icon" width="26" height="27" 
                viewBox="0 0 26 27" fill="none" xmlns="http://www.w3.org/2000/svg"
            >
                <title>Show/Hide boundary condition</title>
                <path d="M10.1909 9.3133L1.84461 17.6726C1.83832 17.6789 1.84279 17.6897 1.85169 
                    17.6897H15.802C15.8047 17.6897 15.8073 17.6886 15.8091 17.6867L24.1554 9.32743C24.1617 
                    9.32113 24.1572 9.31036 24.1483 9.31036H10.1979C10.1953 9.31036 10.1927 9.31142 
                    10.1909 9.3133Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <path d="M12.5345 7.4483L10.5187 3.95692L14.5502 3.95692L12.5345 7.4483Z" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke-width="2"
                />
                <ellipse cx="14.3965" cy="1.39655" rx="1.39655" ry="1.39655" 
                    transform="rotate(90 14.3965 1.39655)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <ellipse cx="10.6724" cy="1.39655" rx="1.39655" ry="1.39655" 
                    transform="rotate(90 10.6724 1.39655)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <path d="M21.8448 7.4483L19.8291 3.95692L23.8606 3.95692L21.8448 7.4483Z" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke-width="2"
                />
                <ellipse cx="23.7069" cy="1.39655" rx="1.39655" ry="1.39655" 
                    transform="rotate(90 23.7069 1.39655)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <ellipse cx="19.9828" cy="1.39655" rx="1.39655" ry="1.39655" 
                    transform="rotate(90 19.9828 1.39655)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <path d="M4.15516 19.5517L6.17091 23.0431L2.13941 23.0431L4.15516 19.5517Z" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke-width="2"
                />
                <ellipse cx="2.2931" cy="25.6034" rx="1.39655" ry="1.39655" 
                    transform="rotate(-90 2.2931 25.6034)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <ellipse cx="6.01725" cy="25.6034" rx="1.39655" ry="1.39655" 
                    transform="rotate(-90 6.01725 25.6034)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <path d="M13.4655 19.5517L15.4813 23.0431L11.4498 23.0431L13.4655 19.5517Z" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke-width="2"
                />
                <ellipse cx="11.6034" cy="25.6034" rx="1.39655" ry="1.39655" 
                    transform="rotate(-90 11.6034 25.6034)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <ellipse cx="15.3276" cy="25.6034" rx="1.39655" ry="1.39655" 
                    transform="rotate(-90 15.3276 25.6034)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
            </svg>
        </button>
        `;

        this.shadowRoot.querySelector(".show-hide-boundary-condition-button").addEventListener("click", 
            () => this.updateBoundaryConditionVisibility());

    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.updateBoundaryConditionVisibility();
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
        const button = this.shadowRoot.querySelector(".show-hide-boundary-condition-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-boundary-condition-button-icon");
        if (button.classList.contains("active") == false) {
            button.classList.add("active");
            buttonIcon.classList.add("active-icon");
        }
    }

    deactivate() {
        const button = this.shadowRoot.querySelector(".show-hide-boundary-condition-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-boundary-condition-button-icon");
        if (button.classList.contains("active") == true) {
            button.classList.remove("active");
            buttonIcon.classList.remove("active-icon");
        }
    }

    updateBoundaryConditionVisibility() {
        if (this.state.isBoundaryConditionVisible == true) {
            this.state.isBoundaryConditionVisible = false;
            this.activate();
        } else {
            this.state.isBoundaryConditionVisible = true;
            this.deactivate();
        }

        this.dispatchEvent(new CustomEvent(UPDATE_BOUNDARY_CONDITION_VISIBILITY_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: { "is_boundary_condition_visible": this.state.isBoundaryConditionVisible },
        }));    
    }
}

export default FeaAppShowHideBoundaryConditionButton;
