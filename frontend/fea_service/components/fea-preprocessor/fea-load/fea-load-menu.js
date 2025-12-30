class FeaLoadMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null, // u32 
        };

        this.state = {
            activeMenuName: null,
            menuNames: {
                "load-concentrated-load-menu": "fea-load-concentrated-load-menu",
                "load-uniformly-distributed-load-menu": "fea-load-uniformly-distributed-load-menu",
            },

            loadTypes: ["Concentrated load", "Distributed load"],
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

            .load-type-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
                margin-bottom: 0;
                margin-left: 0;
                margin-right: 0;
                align-items: baseline;
            }

            .load-menu-caption {
                margin: 0rem;
                padding-top: 0rem;
                padding-bottom: 0.3rem;
                padding-left: 0rem;
                padding-right: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                font-size: 85%;
            }

            .load-type-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 3rem;
            }

            .load-type-select-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: column;
            }

            .load-type {
                width: 8rem;
                margin-top: 0.5rem;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-content-caption-border-color);
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                outline: none;
                color: var(--preprocessor-menu-content-caption-color);
                -webkit-appearance: none;
                -moz-appearance: none;
                background: url('data:image/svg+xml,<svg width="4" height="4" viewBox="0 0 4 4" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M1 1L2 2L3 1" stroke="rgb(112, 112, 114)" stroke-width="0.5"/></svg>') right / contain no-repeat;
            }

            .load-type option {
                background-color: var(--preprocessor-menu-content-dropdown-menu-bg-color);
            }

            .load-type:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }
        </style>

        <div class=wrapper>
            <p class="load-menu-caption">Load</p>

            <div class="load-type-field-content">
                <p class="load-type-caption">Type</p>
                <div class="load-type-select-content">
                    <select class="load-type"></select>
                </div>
            </div>
            <slot></slot>
        </div>
        `;

        this.shadowRoot.querySelector(".load-type").addEventListener("change", () => this.defineLoadTypeForLoadMenuButtons());
    }

    connectedCallback() {
        this.defineLoadType();
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

    defineLoadType() {
        const loadTypeSelect = this.shadowRoot.querySelector(".load-type");
        for (let i = loadTypeSelect.length - 1; i >= 0; i--) {
            loadTypeSelect.options[i] = null;
        }
        for (let i = 0; i < this.state.loadTypes.length; i++) {
            let updateOption = document.createElement("option");
            updateOption.value = this.state.loadTypes[i];
            updateOption.innerHTML = this.state.loadTypes[i];
            loadTypeSelect.appendChild(updateOption);
        }
        this.defineLoadTypeForLoadMenuButtons();
    }

    defineLoadTypeForLoadMenuButtons() {
        const loadTypeSelect = this.shadowRoot.querySelector(".load-type");
        switch (loadTypeSelect.value) {
            case "Concentrated load":
                this.deactivateMenu("load-uniformly-distributed-load-menu");
                this.activateMenu("load-concentrated-load-menu");
                break;
            case "Distributed load":
                this.deactivateMenu("load-concentrated-load-menu");
                this.activateMenu("load-uniformly-distributed-load-menu");
                break;
        }
    }

    activateMenu(menuName) {
        const menu = document.createElement(this.state.menuNames[menuName]);
        this.append(menu);
        menu.setAttribute("action-id", this.props.actionId);
        this.state.activeMenuName = menuName;
    }

    deactivateMenu(menuName) {
        if (this.querySelector(this.state.menuNames[menuName])) {
            this.querySelector(this.state.menuNames[menuName]).remove();
            this.state.activeMenuName = null;
        }
    }
}

export default FeaLoadMenu;
