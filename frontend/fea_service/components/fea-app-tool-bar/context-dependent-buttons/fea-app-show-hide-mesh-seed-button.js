import { UPDATE_MESH_SEED_VISIBILITY_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";


class FeaAppShowHideMeshSeedButton extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            isMeshSeedVisible: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .show-hide-mesh-seed-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .show-hide-mesh-seed-button:hover .show-hide-mesh-seed-button-icon {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .show-hide-mesh-seed-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .active:hover .show-hide-mesh-seed-button-icon {
                background: var(--toolbar-active-button-icon-hover-bg-color);
                border: var(--toolbar-active-button-icon-hover-bg-color);
            }

            .active-icon {
                background: var(--toolbar-active-button-icon-bg-color);
                border: var(--toolbar-active-button-icon-bg-color);
            }
        </style>
        <button class="show-hide-mesh-seed-button">
            <svg class="show-hide-mesh-seed-button-icon" width="32" height="32" viewBox="0 0 32 32" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Show/Hide mesh seed</title>


                <path d="M2 2H11V21H30L30 30H2V2Z" stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"/>
                <circle r="2" transform="matrix(-1 0 0 1 30 21)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <circle r="2" transform="matrix(-1 0 0 1 30 30)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <circle r="2" transform="matrix(-1 0 0 1 11 21)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <circle r="2" transform="matrix(-1 0 0 1 2 21)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <circle r="2" transform="matrix(-1 0 0 1 11 11)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <circle r="2" transform="matrix(-1 0 0 1 2 11)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <circle r="2" transform="matrix(-1 0 0 1 11 2)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <circle r="2" transform="matrix(-1 0 0 1 2 2)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <circle r="2" transform="matrix(-1 0 0 1 11 30)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <circle r="2" transform="matrix(-1 0 0 1 2 30)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <circle r="2" transform="matrix(-1 0 0 1 21 21)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <circle r="2" transform="matrix(-1 0 0 1 21 30)" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
            </svg>
        </button>
        `;

        this.shadowRoot.querySelector(".show-hide-mesh-seed-button").addEventListener("click", () => this.updateMeshSeedVisibility());

    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.updateMeshSeedVisibility();
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
        const button = this.shadowRoot.querySelector(".show-hide-mesh-seed-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-mesh-seed-button-icon");
        if (button.classList.contains("active") == false) {
            button.classList.add("active");
            buttonIcon.classList.add("active-icon");
        }
    }

    deactivate() {
        const button = this.shadowRoot.querySelector(".show-hide-mesh-seed-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-mesh-seed-button-icon");
        if (button.classList.contains("active") == true) {
            button.classList.remove("active");
            buttonIcon.classList.remove("active-icon");
        }
    }

    updateMeshSeedVisibility() {
        if (this.state.isMeshSeedVisible == true) {
            this.state.isMeshSeedVisible = false;
            this.activate();
        } else {
            this.state.isMeshSeedVisible = true;
            this.deactivate();
        }

        this.dispatchEvent(new CustomEvent(UPDATE_MESH_SEED_VISIBILITY_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: { "is_mesh_seed_visible": this.state.isMeshSeedVisible },
        }));    
    }
}

export default FeaAppShowHideMeshSeedButton;
