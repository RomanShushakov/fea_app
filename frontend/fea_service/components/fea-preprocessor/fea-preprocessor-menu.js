import { RESIZE_MESSAGE_HEADER } from "../../consts/fea_app_consts.js";

class FeaPreprocessorMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
            isProcessing: null,
        };

        this.state = {
            activeMenuName: null,
            menuNames: {
                "geometry-menu": "fea-geometry-menu",
                "material-menu": "fea-material-menu",
                "section-menu": "fea-section-menu",
                "properties-menu": "fea-properties-menu",
                "load-menu": "fea-load-menu",
                "boundary-condition-menu": "fea-boundary-condition-menu",
                "mesh-menu": "fea-mesh-menu",
                "analysis-menu": "fea-analysis-menu",
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
                background-color: var(--preprocessor-menu-bg-color);
                display: flex;
                flex-direction: row;
                border-bottom: 0.1rem solid var(--preprocessor-menu-border-color);
                border-left: 0.1rem solid var(--preprocessor-menu-border-color);
            }
        </style>
        <div class=wrapper>
            <fea-preprocessor-menu-buttons></fea-preprocessor-menu-buttons>
            <slot></slot>
        </div>
        `;
        
        this.addEventListener("activate-menu", (event) => this.activateMenu(event));

        this.addEventListener("deactivate-menu", (event) => this.deactivateMenu(event));
    }

    delay(t, v) {
        return new Promise(function(resolve) { 
            setTimeout(resolve.bind(null, v), t)
        });
    }

    // set selectPointInClient(pointNumber) {
    //     if (this.querySelector("fea-geometry-menu") === null) {
    //         this.delay(0)
    //             .then(() => { 
    //                 this.shadowRoot.querySelector("fea-preprocessor-menu-buttons").toggleButton = "geometry-menu-button";
    //             })
    //             .then(async () => { this.querySelector("fea-geometry-menu").selectPointInClient = pointNumber });
    //     } else {
    //         this.delay(0)
    //             .then(() => { 
    //                 this.querySelector("fea-geometry-menu").selectPointInClient = pointNumber;
    //             });
    //     }
    // }

    connectedCallback() {
    }

    disconnectedCallback() {
    }
    
    static get observedAttributes() {
        return ["menu-height", "action-id", "error-message", "is-processing"];
    }
    
    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "menu-height") {
            this.shadowRoot.querySelector(".wrapper").style.height = `${newValue}px`;
        }

        if (name === "action-id") {
            this.props.actionId = newValue;
            if (this.state.activeMenuName !== null) {
                this.querySelector(this.state.menuNames[this.state.activeMenuName])
                    .setAttribute("action-id", this.props.actionId);
            }
        }

        if (name === "error-message") {
            if (this.state.activeMenuName !== null) {
                this.querySelector(this.state.menuNames[this.state.activeMenuName])
                    .setAttribute("error-message", newValue);
            }
        }

        if (name === "is-processing") {
            this.props.isProcessing = newValue;
            if (this.state.activeMenuName !== null) {
                this.querySelector(this.state.menuNames[this.state.activeMenuName])
                    .setAttribute("is-processing", this.props.isProcessing);
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
        menu.setAttribute("action-id", this.props.actionId);
        menu.setAttribute("is-processing", this.props.isProcessing);
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

export default FeaPreprocessorMenu;
