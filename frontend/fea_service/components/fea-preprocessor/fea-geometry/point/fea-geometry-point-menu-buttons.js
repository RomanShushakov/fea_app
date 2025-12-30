class FeaGeometryPointMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "geometry-add-point-menu-button",
                "geometry-update-point-menu-button",
                "geometry-delete-point-menu-button",
            ],

            menuNames: {
                "geometry-add-point-menu-button": "geometry-add-point-menu",
                "geometry-update-point-menu-button": "geometry-update-point-menu",
                "geometry-delete-point-menu-button": "geometry-delete-point-menu",
            },

            captions: {
                "geometry-add-point-menu-button": "Add",
                "geometry-update-point-menu-button": "Update",
                "geometry-delete-point-menu-button": "Delete",
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

            .geometry-point-menu-buttons-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                align-items: center;
            }

            .geometry-point-menu-buttons-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 4rem;
            }

            .geometry-add-point-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 2.5rem;
                margin-right: 0rem;
            }

            .geometry-update-point-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }

            .geometry-delete-point-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }
        </style>

        <div class=wrapper>
            <div class="geometry-point-menu-buttons-content">
                <p class="geometry-point-menu-buttons-caption">Add</p>
                <add-button class="geometry-add-point-menu-button" name="point"></add-button>
                <update-button class="geometry-update-point-menu-button" name="point"></update-button>
                <delete-button class="geometry-delete-point-menu-button" name="point"></delete-button>
            </div>
        </div>
        `;

        this.shadowRoot.querySelector(".geometry-add-point-menu-button").addEventListener("click", 
            () => this.activate("geometry-add-point-menu-button"));

        this.shadowRoot.querySelector(".geometry-update-point-menu-button").addEventListener("click", 
            () => this.activate("geometry-update-point-menu-button"));

        this.shadowRoot.querySelector(".geometry-delete-point-menu-button").addEventListener("click", 
            () => this.activate("geometry-delete-point-menu-button"));
    }

    set activateButton(buttonName) {
        this.activate(buttonName);
    }

    connectedCallback() {
        this.activate("geometry-add-point-menu-button");
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
            this.shadowRoot.querySelector(".geometry-point-menu-buttons-caption").innerHTML = caption;
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

export default FeaGeometryPointMenuButtons;
