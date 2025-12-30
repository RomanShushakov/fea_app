class FeaMeshMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null, // u32 
        };

        this.state = {
            activeMenuName: null,
            menuNames: {
                "mesh-seed-global-menu": "fea-mesh-global-mesh-seed-menu",
                "mesh-seed-local-menu": "fea-mesh-local-mesh-seed-menu",
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
                padding: 1rem;
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

            .mesh-menu-caption {
                margin: 0rem;
                padding-top: 0rem;
                padding-bottom: 0.3rem;
                padding-left: 0rem;
                padding-right: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                font-size: 85%;
            }
        </style>

        <div class=wrapper>
            <p class="mesh-menu-caption">Mesh</p>
            <fea-mesh-menu-buttons></fea-mesh-menu-buttons>
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
                this.querySelector(this.state.menuNames[this.state.activeMenuName]).setAttribute("error-message", 
                    newValue);
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

export default FeaMeshMenu;
