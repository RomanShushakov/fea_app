class FeaLoadUniformlyDistributedLineLoadMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null, // u32 
        };

        this.state = {
            activeMenuName: null,
            menuNames: {
                "load-add-uniformly-distributed-line-load-menu": "fea-load-add-uniformly-distributed-line-load-menu",
                "load-update-uniformly-distributed-line-load-menu": "fea-load-update-uniformly-distributed-line-load-menu",
                "load-delete-uniformly-distributed-line-load-menu": "fea-load-delete-uniformly-distributed-line-load-menu",
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
                padding: 0rem;
            }
        </style>

        <div class=wrapper>
            <fea-load-uniformly-distributed-line-load-menu-buttons></fea-load-uniformly-distributed-line-load-menu-buttons>
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

export default FeaLoadUniformlyDistributedLineLoadMenu;
