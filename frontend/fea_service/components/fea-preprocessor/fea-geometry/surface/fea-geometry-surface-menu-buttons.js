class FeaGeometrySurfaceMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "geometry-add-surface-menu-button",
                "geometry-update-surface-menu-button",
                "geometry-delete-surface-menu-button",
            ],

            menuNames: {
                "geometry-add-surface-menu-button": "geometry-add-surface-menu",
                "geometry-update-surface-menu-button": "geometry-update-surface-menu",
                "geometry-delete-surface-menu-button": "geometry-delete-surface-menu",
            },

            captions: {
                "geometry-add-surface-menu-button": "Add",
                "geometry-update-surface-menu-button": "Update",
                "geometry-delete-surface-menu-button": "Delete",
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

            .geometry-surface-menu-buttons-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                align-items: center;
            }

            .geometry-surface-menu-buttons-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 4rem;
            }

            .geometry-add-surface-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 2.5rem;
                margin-right: 0rem;
            }

            .geometry-update-surface-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }

            .geometry-delete-surface-menu-button {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }
        </style>

        <div class=wrapper>
            <div class="geometry-surface-menu-buttons-content">
                <p class="geometry-surface-menu-buttons-caption">Add</p>
                <add-button class="geometry-add-surface-menu-button" name="surface"></add-button>
                <update-button class="geometry-update-surface-menu-button" name="surface"></update-button>
                <delete-button class="geometry-delete-surface-menu-button" name="surface"></delete-button>
            </div>
        </div>
        `;

        this.shadowRoot.querySelector(".geometry-add-surface-menu-button").addEventListener("click", 
            () => this.activate("geometry-add-surface-menu-button"));

        this.shadowRoot.querySelector(".geometry-update-surface-menu-button").addEventListener("click", 
            () => this.activate("geometry-update-surface-menu-button"));

        this.shadowRoot.querySelector(".geometry-delete-surface-menu-button").addEventListener("click", 
            () => this.activate("geometry-delete-surface-menu-button"));
    }

    set activateButton(buttonName) {
        this.activate(buttonName);
    }

    connectedCallback() {
        this.activate("geometry-add-surface-menu-button");
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
            this.shadowRoot.querySelector(".geometry-surface-menu-buttons-caption").innerHTML = caption;
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

export default FeaGeometrySurfaceMenuButtons;
