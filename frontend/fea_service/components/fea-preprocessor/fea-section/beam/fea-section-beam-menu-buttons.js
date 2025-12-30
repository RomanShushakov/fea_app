class FeaSectionBeamMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "section-add-beam-menu-button",
                "section-update-beam-menu-button",
                "section-delete-beam-menu-button",
            ],

            menuNames: {
                "section-add-beam-menu-button": "section-add-beam-menu",
                "section-update-beam-menu-button": "section-update-beam-menu",
                "section-delete-beam-menu-button": "section-delete-beam-menu",
            },

            captions: {
                "section-add-beam-menu-button": "Add",
                "section-update-beam-menu-button": "Update",
                "section-delete-beam-menu-button": "Delete",
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

            .section-beam-menu-buttons-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                align-items: center;
            }

            .section-beam-menu-buttons-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 4rem;
            }

            .section-add-beam-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 2.5rem;
                margin-right: 0rem;
            }

            .section-update-beam-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }

            .section-delete-beam-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }
        </style>

        <div class=wrapper>
            <div class="section-beam-menu-buttons-content">
                <p class="section-beam-menu-buttons-caption">Add</p>
                <add-button class="section-add-beam-menu-button"></add-button>
                <update-button class="section-update-beam-menu-button"></update-button>
                <delete-button class="section-delete-beam-menu-button"></delete-button>
            </div>
        </div>
        `;

        this.shadowRoot.querySelector(".section-add-beam-menu-button").addEventListener("click", 
            () => this.activate("section-add-beam-menu-button"));

        this.shadowRoot.querySelector(".section-update-beam-menu-button").addEventListener("click", 
            () => this.activate("section-update-beam-menu-button"));

        this.shadowRoot.querySelector(".section-delete-beam-menu-button").addEventListener("click", 
            () => this.activate("section-delete-beam-menu-button"));
    }

    connectedCallback() {
        this.activate("section-add-beam-menu-button");
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
            this.shadowRoot.querySelector(".section-beam-menu-buttons-caption").innerHTML = caption;
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

export default FeaSectionBeamMenuButtons;
