import { LOAD_PREPROCESSOR_DATA_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";

class FeaAppLoadButton extends HTMLElement {
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
            .load-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .load-button :hover {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .load-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }
        </style>
        <button class="load-button">
            <svg class="load-button-icon" width="26" height="21" viewBox="0 0 26 21" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Load</title>
                <path d="M1 5.01V19.9759C1 19.9848 1.01077 19.9892 1.01707 19.9829L9.99707
                    11.0029C9.99895 11.0011 10.0015 11 10.0041 11H15.99C15.9955 11 16 10.9955 
                    16 10.99V5.01C16 5.00448 15.9955 5 15.99 5H9.01C9.00448 5 9 4.99552 
                    9 4.99V3.51C9 3.50448 8.99552 3.5 8.99 3.5H4.51C4.50448 3.5 4.5 3.50448 
                    4.5 3.51V4.99C4.5 4.99552 4.49552 5 4.49 5H1.01C1.00448 5 1 5.00448 1 5.01Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <path d="M9.98299 11.0029L1.01704 19.9829C1.01075 19.9892 1.01521 20 1.02412 
                    20H16.0099C16.0126 20 16.0151 19.9989 16.017 19.9971L24.983 11.0171C24.9893
                    11.0108 24.9848 11 24.9759 11H9.99007C9.98742 11 9.98487 11.0011 9.98299 11.0029Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <path d="M14 1.00001C16 0.5 19 -0.500012 22.5 3.99998" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-2)"
                />
                <path d="M23 1.5L22.6464 4.64645" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-2)"
                />
                <line x1="22.8419" y1="4.47434" x2="19.8419" y2="3.47434" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-2)"
                />
            </svg>
        </button>
        <input type="file" class="load-file" style="display:none">
        `;

        this.shadowRoot.querySelector(".load-button").addEventListener("click", () => this.load());
        this.shadowRoot.querySelector(".load-file").addEventListener("change", (event) => {
            const file = event.target.files[0];
            this.readFile(file);
        });

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

    load() {
        this.shadowRoot.querySelector(".load-file").click();
    }

    async readFile(file) {
        const text = await file.text();
        let fileContentArray = text.split(/\r\n|\n/);
        if (this.props.isPreprocessorActive === "true") {
            this.dispatchEvent(new CustomEvent(LOAD_PREPROCESSOR_DATA_MESSAGE_HEADER, {
                bubbles: true,
                composed: true,
                detail: { activeActions: fileContentArray },
            })); 
        }
    }
}

export default FeaAppLoadButton;
