class FeaMaterialMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "material-add-material-menu-button",
                "material-update-material-menu-button",
                "material-delete-material-menu-button",
            ],

            menuNames: {
                "material-add-material-menu-button": "material-add-material-menu",
                "material-update-material-menu-button": "material-update-material-menu",
                "material-delete-material-menu-button": "material-delete-material-menu",
            },

            captions: {
                "material-add-material-menu-button": "Add",
                "material-update-material-menu-button": "Update",
                "material-delete-material-menu-button": "Delete",
            }
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
                margin-top: 1rem;
            }

            .material-menu-buttons-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                align-items: center;
            }

            .material-menu-buttons-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 4rem;
            }

            .material-add-material-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 2.5rem;
                margin-right: 0rem;
            }

            .material-update-material-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }

            .material-delete-material-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }

        </style>
        <div class=wrapper>
            <div class="material-menu-buttons-content">
                <p class="material-menu-buttons-caption">Add</p>
                <add-button class="material-add-material-menu-button" name="material"></add-button>
                <update-button class="material-update-material-menu-button" name="material"></update-button>
                <delete-button class="material-delete-material-menu-button" name="material"></delete-button>
            </div>
        </div>
        `;

        this.shadowRoot.querySelector(".material-add-material-menu-button").addEventListener("click", 
            () => this.activate("material-add-material-menu-button"));

        this.shadowRoot.querySelector(".material-update-material-menu-button").addEventListener("click", 
            () => this.activate("material-update-material-menu-button"));

        this.shadowRoot.querySelector(".material-delete-material-menu-button").addEventListener("click", 
            () => this.activate("material-delete-material-menu-button"));
    }

    connectedCallback() {
        this.activate("material-add-material-menu-button");
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

    activate(buttonName) {
        for (let i = 0; i < this.state.buttonNames.length; i ++) {
            if (this.state.buttonNames[i] !== buttonName && 
                this.shadowRoot.querySelector(`.${this.state.buttonNames[i]}`)
                    .classList.contains("active") === true) {
                this.shadowRoot.querySelector(`.${this.state.buttonNames[i]}`).classList.remove("active");
                this.shadowRoot.querySelector(`.${this.state.buttonNames[i]}`).setAttribute("is-active", false);
                const menuName = this.state.menuNames[this.state.buttonNames[i]];
                this.dispatchEvent(new CustomEvent("deactivate-menu", {
                    bubbles: true,
                    composed: true,
                    detail: {
                        "menuName": menuName,
                    }
                }));
            }
        } 
        const currentButton = this.shadowRoot.querySelector(`.${buttonName}`);
        if (currentButton.classList.contains("active") === false) {
            currentButton.classList.add("active");
            currentButton.setAttribute("is-active", true);
            const caption = this.state.captions[buttonName];
            this.shadowRoot.querySelector(".material-menu-buttons-caption").innerHTML = caption;
            const menuName = this.state.menuNames[buttonName];
            this.dispatchEvent(new CustomEvent("activate-menu", {
                bubbles: true,
                composed: true,
                detail: {
                    "menuName": menuName,
                }
            }));
        }
    }
}

export default FeaMaterialMenuButtons;
