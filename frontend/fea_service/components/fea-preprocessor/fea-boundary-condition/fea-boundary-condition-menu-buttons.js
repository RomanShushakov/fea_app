class FeaBoundaryConditionMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "boundary-condition-add-boundary-condition-menu-button",
                "boundary-condition-update-boundary-condition-menu-button",
                "boundary-condition-delete-boundary-condition-menu-button",
            ],

            menuNames: {
                "boundary-condition-add-boundary-condition-menu-button": "boundary-condition-add-boundary-condition-menu",
                "boundary-condition-update-boundary-condition-menu-button": "boundary-condition-update-boundary-condition-menu",
                "boundary-condition-delete-boundary-condition-menu-button": "boundary-condition-delete-boundary-condition-menu",
            },

            captions: {
                "boundary-condition-add-boundary-condition-menu-button": "Add",
                "boundary-condition-update-boundary-condition-menu-button": "Update",
                "boundary-condition-delete-boundary-condition-menu-button": "Delete",
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

            .boundary-condition-menu-buttons-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                align-items: center;
            }

            .boundary-condition-menu-buttons-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 4rem;
            }

            .boundary-condition-add-boundary-condition-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 2.5rem;
                margin-right: 0rem;
            }

            .boundary-condition-update-boundary-condition-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }

            .boundary-condition-delete-boundary-condition-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }

        </style>
        <div class=wrapper>
            <div class="boundary-condition-menu-buttons-content">
                <p class="boundary-condition-menu-buttons-caption">Add</p>
                <add-button class="boundary-condition-add-boundary-condition-menu-button" name="boundary condition"></add-button>
                <update-button class="boundary-condition-update-boundary-condition-menu-button" name="boundary condition"></update-button>
                <delete-button class="boundary-condition-delete-boundary-condition-menu-button" name="boundary condition"></delete-button>
            </div>
        </div>
        `;

        this.shadowRoot.querySelector(".boundary-condition-add-boundary-condition-menu-button").addEventListener("click", 
            () => this.activate("boundary-condition-add-boundary-condition-menu-button"));

        this.shadowRoot.querySelector(".boundary-condition-update-boundary-condition-menu-button").addEventListener("click", 
            () => this.activate("boundary-condition-update-boundary-condition-menu-button"));

        this.shadowRoot.querySelector(".boundary-condition-delete-boundary-condition-menu-button").addEventListener("click", 
            () => this.activate("boundary-condition-delete-boundary-condition-menu-button"));
    }

    set activateButton(buttonName) {
        this.activate(buttonName);
    }

    connectedCallback() {
        this.activate("boundary-condition-add-boundary-condition-menu-button");
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
            this.shadowRoot.querySelector(".boundary-condition-menu-buttons-caption").innerHTML = caption;
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

export default FeaBoundaryConditionMenuButtons;
