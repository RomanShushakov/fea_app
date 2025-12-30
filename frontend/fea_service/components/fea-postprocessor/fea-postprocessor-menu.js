import { RESIZE_MESSAGE_HEADER } from "../../consts/fea_app_consts.js";

class FeaPostprocessorMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            jobName: null,
        };

        this.state = {
            activeMenuName: null,
            menuNames: {
                "contours-menu": "fea-contours-menu",
                "symbols-menu": "fea-symbols-menu",
            },
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            :host {
                display: block;
            }

            .wrapper {
                background-color: var(--postprocessor-menu-bg-color);
                display: flex;
                flex-direction: row;
                border-bottom: 0.1rem solid var(--postprocessor-menu-border-color);
                border-left: 0.1rem solid var(--postprocessor-menu-border-color);
            }
        </style>

        <div class=wrapper>
            <fea-postprocessor-menu-buttons></fea-postprocessor-menu-buttons>
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
        return ["menu-height", "job-name"];
    }
    
    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "menu-height") {
            this.shadowRoot.querySelector(".wrapper").style.height = `${newValue}px`;
        }

        if (name === "job-name") {
            this.props.jobName = newValue;
            if (this.state.activeMenuName !== null) {
                this.querySelector(this.state.menuNames[this.state.activeMenuName])
                    .setAttribute("job-name", this.props.jobName);
            }
        }
    }
    
    adoptedCallback() {
    }

    updateCanvasSize() {
        this.dispatchEvent(new CustomEvent(RESIZE_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
        }));
    }

    activateMenu(event) {
        const menuName = event.detail.menuName;
        const menu = document.createElement(this.state.menuNames[menuName]);
        this.append(menu);
        menu.setAttribute("job-name", this.props.jobName);
        this.state.activeMenuName = menuName;
        event.stopPropagation();
        this.updateCanvasSize();
    }

    deactivateMenu(event) {
        const menuName = event.detail.menuName;
        this.querySelector(this.state.menuNames[menuName]).remove();
        this.state.activeMenuName = null;
        event.stopPropagation();
        this.updateCanvasSize();
    }
}

export default FeaPostprocessorMenu;