class AddButton extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {};

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            :host {
                display: block;
            }

            .add-button {
                background: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin: 0rem;
            }

            .add-button-icon-content {
                margin: 0rem;
                padding: 0rem;
                width: 1.5rem;
                height: 1.5rem;
                border-bottom: 0.15rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
            }

            .add-button-icon {
                margin: 0rem;
                padding: 0rem;
                width: 1.5rem;
                height: 1.5rem;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .add-button:hover .add-button-icon {
                color: var(--preprocessor-menu-content-buttons-hover-color);
            }

            .active:hover .add-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .add-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-color);
            }

            .active:hover .add-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .add-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-color);
            }
        </style>

        <button class="add-button">
            <div class="add-button-icon-content">
                <svg class="add-button-icon" width="35" height="35" viewBox="0 0 35 35" fill="none" 
                    xmlns="http://www.w3.org/2000/svg"
                >  
                    <title>Add ${this.getAttribute("name")}</title>
                    <g stroke="currentColor">
                        <line x1="7" y1="17.5" x2="28" y2="17.5" stroke-width="2"/>
                        <line x1="17.5" y1="7" x2="17.5" y2="28" stroke-width="2"/>
                    </g>
                </svg>
            </div>
        </button>
        `;
    }

    connectedCallback() {
    }

    disconnectedCallback() {
    }

    static get observedAttributes() {
        return ["is-active"];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        this.changeButtonActivity();
    }

    adoptedCallback() {
    }

    changeButtonActivity() {
        const addButton = this.shadowRoot.querySelector(".add-button");
        if (this.getAttribute("is-active") === "true") {
            if (addButton.classList.contains("active") === false) {
                addButton.classList.add("active");
            }
        }
        if (this.getAttribute("is-active") === "false") {
            if (addButton.classList.contains("active") === true) {
                addButton.classList.remove("active");
            }
        }
    }
}

export default AddButton;
