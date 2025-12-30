import { LOCAL_AXIS_1_DIRECTION_INPUT_INFO_MESSAGE_HEADER } from "../../../consts/common_consts.js";


class FeaPropertiesMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null, // u32 
        };

        this.state = {
            activeMenuName: null,
            menuNames: {
                "properties-add-properties-menu": "fea-properties-add-properties-menu",
                "properties-update-properties-menu": "fea-properties-update-properties-menu",
                "properties-delete-properties-menu": "fea-properties-delete-properties-menu",
                "properties-assign-properties-menu": "fea-properties-assign-properties-menu",
                "properties-beam-section-orientation-menu": "fea-properties-beam-section-orientation-menu",
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

            .properties-menu-caption {
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
            <p class="properties-menu-caption">Properties</p>
            <fea-properties-menu-buttons></fea-properties-menu-buttons>
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
                if (this.state.activeMenuName === "properties-assign-properties-menu") {
                    this.querySelector(this.state.menuNames[this.state.activeMenuName]).setAttribute("error-message", 
                        newValue);
                } else if (this.state.activeMenuName === "properties-beam-section-orientation-menu") {
                    let errorData = Object.create(null);
                    if (newValue.startsWith(LOCAL_AXIS_1_DIRECTION_INPUT_INFO_MESSAGE_HEADER)) {
                        errorData.dst = "local_axis_1_direction_input_info";
                        errorData.error = newValue.slice(LOCAL_AXIS_1_DIRECTION_INPUT_INFO_MESSAGE_HEADER.length);
                    } else {
                        errorData.dst = "analysis_info";
                        errorData.error = newValue;
                    }
                    this.querySelector(this.state.menuNames[this.state.activeMenuName]).feModelError = errorData;
                } else {
                    this.querySelector(this.state.menuNames[this.state.activeMenuName]).feModelError = newValue;
                }
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

export default FeaPropertiesMenu;
