import { RESET_PREPROCESSOR_DATA_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";

class FeaAppResetButton extends HTMLElement {
    constructor() {
        super();

        this.props = {
            isPreprocessorActive: null, // bool
        };

        this.state = {};

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .open-reset-menu-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .open-reset-menu-button :hover {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .open-reset-menu-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .reset-menu {
                display: none;
                position: fixed;
                inset: 0;
                z-index: 20;
                background-color: rgba(0.0, 0.0, 0.0, 0.75);
                padding: 3rem;
                overflow: auto;
            }

            .reset-menu.open {
                display: block;
            }

            .reset-menu-body {
                margin: 0;
                padding: 0;
                display: flex;
                flex-direction: column;
                align-items: center;
                width: 12rem;
                background: var(--menubar-bg-color);
            }

            .dialog-caption {
                color: var(--preprocessor-menu-buttons-button-icon-caption-color);
                margin-top: 0.25rem;
                margin-bottom: 0.25rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
                width: 9rem;
            }

            .reset-menu-buttons {
                margin-top: 0.25rem;
                margin-bottom: 0.25rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0;
                display: flex;
                flex-direction: row;
                justify-content: space-between;
                width: 7.5rem;
            }

            .reset-button {
                background: var(--preprocessor-menu-content-aux-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 3.5rem;
                height: 1.7rem;
                font-size: 70%;
            }

            .reset-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .close-reset-menu-button {
                background: var(--preprocessor-menu-content-aux-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 3.5rem;
                height: 1.7rem;
                font-size: 70%;
            }

            .close-reset-menu-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }
        </style>
        <button class="open-reset-menu-button">
            <svg class="open-reset-menu-button-icon" width="24" height="24" viewBox="0 0 24 24" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Reset</title>
                <path d="M16.7301 19.8958C15.5994 20.4347 14.3529 20.7872 13.0618 20.9329C11.7708 21.0786 10.4605 
                    21.0148 9.20581 20.7451C7.95109 20.4754 6.7765 20.0051 5.7491 19.3611C4.72171 18.717 3.86162 
                    17.9118 3.21796 16.9914" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <path d="M18.2147 19.188L16.1279 21.2989L14.9569 19.6245L18.2147 19.188Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)" stroke-width="2"
                />
                <path d="M7.26992 4.10423C8.4007 3.56526 9.64719 3.21284 10.9382 3.06711C12.2292 2.92138 13.5395 
                    2.98518 14.7942 3.25488C16.049 3.52457 17.2236 3.99488 18.251 4.63894C19.2783 5.283 20.1384 
                    6.08821 20.7821 7.00859" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <path d="M5.78529 4.81174L7.87212 2.70084L9.04308 4.37521L5.78529 4.81174Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)" stroke-width="2"
                />
                <path d="M9.88068 15.5356V8.26292H12.3381C12.9063 8.26292 13.3726 8.35998 13.7372 8.55411C14.1018 
                    8.74587 14.3717 9.00984 14.5469 9.34601C14.7221 9.68219 14.8097 10.0645 14.8097 10.493C14.8097 
                    10.9215 14.7221 11.3015 14.5469 11.6329C14.3717 11.9644 14.103 12.2248 13.7408 12.4142C13.3786 
                    12.6012 12.9157 12.6947 12.3523 12.6947H10.3636V11.8993H12.3239C12.7121 11.8993 13.0246 11.8425 
                    13.2614 11.7288C13.5005 11.6152 13.6733 11.4542 13.7798 11.2459C13.8887 11.0352 13.9432 10.7842 
                    13.9432 10.493C13.9432 10.2018 13.8887 9.94734 13.7798 9.72954C13.6709 9.51173 13.4969 9.34365 
                    13.2578 9.22528C13.0187 9.10454 12.7027 9.04417 12.3097 9.04417H10.7614V15.5356H9.88068ZM13.304 
                    12.2686L15.0938 15.5356H14.071L12.3097 12.2686H13.304Z" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-2)"
                />
            </svg>
        </button>

        <div id="reset-menu" class="reset-menu">
            <div class="reset-menu-body">
                <p class="dialog-caption">Do you really want to reset model?</p>
                <div class="reset-menu-buttons">
                    <button class="reset-button">Yes</button>
                    <button class="close-reset-menu-button">No</button>
                </div>
            </div>
        </div>
        `;

        this.shadowRoot.querySelector(".open-reset-menu-button").addEventListener("click", () => this.openResetMenu());
        this.shadowRoot.querySelector(".close-reset-menu-button").addEventListener(
            "click", () => this.closeResetMenu(),
        );
        this.shadowRoot.querySelector(".reset-button").addEventListener("click", () => this.reset());
    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
    }

    disconnectedCallback() {
    }

    static get observedAttributes() {
        return ["is-preprocessor-active"];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "is-preprocessor-active") {
            this.props.isPreprocessorActive = newValue;
        }
    }

    adoptedCallback() {
    }

    openResetMenu() {
        this.shadowRoot.getElementById("reset-menu").classList.add("open");
    }
    
    closeResetMenu() {
        this.shadowRoot.querySelector('.reset-menu.open').classList.remove('open');
    }

    reset() {
        if (this.props.isPreprocessorActive === "true") {
            this.dispatchEvent(new CustomEvent(RESET_PREPROCESSOR_DATA_MESSAGE_HEADER, {
                bubbles: true,
                composed: true,
            }));   
        } 
        this.closeResetMenu();
    }
}

export default FeaAppResetButton;
