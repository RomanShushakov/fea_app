class FeaLoadConcentratedLoadMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "load-add-concentrated-load-menu-button",
                "load-update-concentrated-load-menu-button",
                "load-delete-concentrated-load-menu-button",
            ],

            menuNames: {
                "load-add-concentrated-load-menu-button": "load-add-concentrated-load-menu",
                "load-update-concentrated-load-menu-button": "load-update-concentrated-load-menu",
                "load-delete-concentrated-load-menu-button": "load-delete-concentrated-load-menu",
            },

            captions: {
                "load-add-concentrated-load-menu-button": "Add",
                "load-update-concentrated-load-menu-button": "Update",
                "load-delete-concentrated-load-menu-button": "Delete",
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

            .load-concentrated-load-menu-buttons-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                align-items: center;
            }

            .load-concentrated-load-menu-buttons-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 4rem;
            }

            .load-add-concentrated-load-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 2.5rem;
                margin-right: 0rem;
            }

            .load-update-concentrated-load-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }

            .load-delete-concentrated-load-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }
        </style>

        <div class=wrapper>
            <div class="load-concentrated-load-menu-buttons-content">
                <p class="load-concentrated-load-menu-buttons-caption">Add</p>
                <add-button class="load-add-concentrated-load-menu-button" name="concentrated load"></add-button>
                <update-button class="load-update-concentrated-load-menu-button" name="concentrated load"></update-button>
                <delete-button class="load-delete-concentrated-load-menu-button" name="concentrated load"></delete-button>
            </div>
        </div>
        `;

        this.shadowRoot.querySelector(".load-add-concentrated-load-menu-button").addEventListener("click", 
            () => this.activate("load-add-concentrated-load-menu-button"));

        this.shadowRoot.querySelector(".load-update-concentrated-load-menu-button").addEventListener("click", 
            () => this.activate("load-update-concentrated-load-menu-button"));

        this.shadowRoot.querySelector(".load-delete-concentrated-load-menu-button").addEventListener("click", 
            () => this.activate("load-delete-concentrated-load-menu-button"));
    }

    connectedCallback() {
        this.activate("load-add-concentrated-load-menu-button");
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
        for (let i = 0; i < this.state.buttonNames.length; i++) {
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
            this.shadowRoot.querySelector(".load-concentrated-load-menu-buttons-caption").innerHTML = caption;
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

export default FeaLoadConcentratedLoadMenuButtons;
