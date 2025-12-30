class FeaPropertiesAssignPropertiesMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null, // u32 
        };

        this.state = {
            activeMenuName: null,
            menuNames: {
                "assign-to-line-menu": "fea-properties-assign-properties-to-lines-menu",
                "assign-to-surface-menu": "fea-properties-assign-properties-to-surfaces-menu",
            },
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            :host {
                display: flex;
            }

            .wrapper {
                display: flex;
                flex-direction: column;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                overflow-y: auto;
                overflow-x: hidden;
                scrollbar-color: var(--renderer-menu-content-scrollbar-thumb-color) var(--preprocessor-menu-buttons-active-button-bg-color);
                scrollbar-width: thin;
            }

            ::-webkit-scrollbar {
                width: 0.5rem;
            }

            ::-webkit-scrollbar-track {
                background: var(--preprocessor-menu-buttons-active-button-bg-color);
            }

            ::-webkit-scrollbar-thumb {
                background: var(--renderer-menu-content-scrollbar-thumb-color);
            }

            ::-webkit-scrollbar-thumb:hover {
                background: var(--renderer-menu-content-scrollbar-thumb-hover-color);
            }
        </style>

        <div class=wrapper>
            <fea-properties-assign-properties-menu-buttons></fea-properties-assign-properties-menu-buttons>
            <slot></slot>
        </div>
        `;

        this.addEventListener("activate-menu", (event) => this.activateMenu(event));

        this.addEventListener("deactivate-menu", (event) => this.deactivateMenu(event));
    }

    connectedCallback() {
    }

    disconnectedCallback() {
    }

    static get observedAttributes() {
        return ["action-id", "error-message"];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "action-id") {
            this.props.actionId = newValue;
            if (this.state.activeMenuName !== null) {
                this.querySelector(this.state.menuNames[this.state.activeMenuName])
                    .setAttribute("action-id", this.props.actionId);
            }
        }

        if (name === "error-message") {
            if (this.state.activeMenuName !== null) {
                this.querySelector(this.state.menuNames[this.state.activeMenuName]).feModelError = newValue;
            }
        }
    }

    adoptedCallback() {
    }

    activateMenu(event) {
        const menuName = event.detail.menuName;
        const menu = document.createElement(this.state.menuNames[menuName]);
        this.append(menu);
        menu.setAttribute("action-id", this.props.actionId);
        this.state.activeMenuName = menuName;
        event.stopPropagation();
    }

    deactivateMenu(event) {
        const menuName = event.detail.menuName;
        this.querySelector(this.state.menuNames[menuName]).remove();
        this.state.activeMenuName = null;
        event.stopPropagation();
    }
}

export default FeaPropertiesAssignPropertiesMenu;
