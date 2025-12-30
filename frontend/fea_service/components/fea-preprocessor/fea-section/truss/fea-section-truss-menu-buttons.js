class FeaSectionTrussMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "section-add-truss-menu-button",
                "section-update-truss-menu-button",
                "section-delete-truss-menu-button",
            ],

            menuNames: {
                "section-add-truss-menu-button": "section-add-truss-menu",
                "section-update-truss-menu-button": "section-update-truss-menu",
                "section-delete-truss-menu-button": "section-delete-truss-menu",
            },

            captions: {
                "section-add-truss-menu-button": "Add",
                "section-update-truss-menu-button": "Update",
                "section-delete-truss-menu-button": "Delete",
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

            .section-truss-menu-buttons-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                align-items: center;
            }

            .section-truss-menu-buttons-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 4rem;
            }

            .section-add-truss-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 2.5rem;
                margin-right: 0rem;
            }

            .section-update-truss-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }

            .section-delete-truss-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }
        </style>

        <div class=wrapper>
            <div class="section-truss-menu-buttons-content">
                <p class="section-truss-menu-buttons-caption">Add</p>
                <add-button class="section-add-truss-menu-button" name="truss section"></add-button>
                <update-button class="section-update-truss-menu-button" name="truss section"></update-button>
                <delete-button class="section-delete-truss-menu-button" name="truss section"></delete-button>
            </div>
        </div>
        `;

        this.shadowRoot.querySelector(".section-add-truss-menu-button").addEventListener("click", 
            () => this.activate("section-add-truss-menu-button"));

        this.shadowRoot.querySelector(".section-update-truss-menu-button").addEventListener("click", 
            () => this.activate("section-update-truss-menu-button"));

        this.shadowRoot.querySelector(".section-delete-truss-menu-button").addEventListener("click", 
            () => this.activate("section-delete-truss-menu-button"));
    }

    connectedCallback() {
        this.activate("section-add-truss-menu-button");
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
            this.shadowRoot.querySelector(".section-truss-menu-buttons-caption").innerHTML = caption;
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

export default FeaSectionTrussMenuButtons;
