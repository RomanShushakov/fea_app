class FeaMeshMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "mesh-seed-global-menu-button",
                "mesh-seed-local-menu-button",
            ],

            menuNames: {
                "mesh-seed-global-menu-button": "mesh-seed-global-menu",
                "mesh-seed-local-menu-button": "mesh-seed-local-menu",
            },

            captions: {
                "mesh-seed-global-menu-button": "Global mesh seed",
                "mesh-seed-local-menu-button": "Local mesh seed",
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
                width: 12rem;
            }

            .mesh-seed-menu-button-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                align-items: center;
            }

            .mesh-seed-menu-button-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 3.5rem;
            }

            .mesh-seed-global-menu-button {
                background: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 2.5rem;
                margin-right: 0rem;
            }

            .mesh-seed-global-menu-button-icon-content {
                margin: 0rem;
                padding: 0rem;
                width: 1.5rem;
                height: 1.5rem;
                border-bottom: 0.15rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
            }

            .mesh-seed-global-menu-button-icon {
                margin: 0rem;
                padding: 0rem;
                width: 1.5rem;
                height: 1.5rem;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .mesh-seed-global-menu-button:hover .mesh-seed-global-menu-button-icon {
                color: var(--preprocessor-menu-content-buttons-hover-color);
            }

            .active:hover .mesh-seed-global-menu-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .mesh-seed-global-menu-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-color);
            }

            .active:hover .mesh-seed-global-menu-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .mesh-seed-global-menu-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-color);
            }

            .mesh-seed-local-menu-button {
                background: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }

            .mesh-seed-local-menu-button-icon-content {
                margin: 0rem;
                padding: 0rem;
                width: 1.5rem;
                height: 1.5rem;
                border-bottom: 0.15rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
            }

            .mesh-seed-local-menu-button-icon {
                margin: 0rem;
                padding: 0rem;
                width: 1.5rem;
                height: 1.5rem;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .mesh-seed-local-menu-button:hover .mesh-seed-local-menu-button-icon {
                color: var(--preprocessor-menu-content-buttons-hover-color);
            }

            .active:hover .mesh-seed-local-menu-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .mesh-seed-local-menu-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-color);
            }

            .active:hover .mesh-seed-local-menu-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .mesh-seed-local-menu-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-color);
            }
        </style>

        <div class=wrapper>
            <div class="mesh-seed-menu-button-content">

                <p class="mesh-seed-menu-button-caption">Global mesh seed</p>

                <button class="mesh-seed-global-menu-button">
                    <div class="mesh-seed-global-menu-button-icon-content">

                        <svg class="mesh-seed-global-menu-button-icon" width="37" height="37" viewBox="0 0 37 37" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Global mesh seed</title>
                            <g stroke="currentColor">
                                <rect x="1" y="24.3333" width="11.6667" height="11.6667"/>
                                <rect x="12.6667" y="24.3333" width="11.6667" height="11.6667"/>
                                <rect x="24.3333" y="24.3333" width="11.6667" height="11.6667"/>
                                <rect x="1" y="12.6667" width="11.6667" height="11.6667"/>
                                <rect x="1" y="12.6667" width="11.6667" height="11.6667" transform="rotate(-90 1 12.6667)"/>
                            </g>
                        </svg>
                    </div>
                </button>

                <button class="mesh-seed-local-menu-button">
                    <div class="mesh-seed-local-menu-button-icon-content">
                        <svg class="mesh-seed-local-menu-button-icon" width="37" height="37" viewBox="0 0 37 37" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Local mesh seed</title>
                            <g stroke="currentColor">
                                <rect x="1" y="24.3333" width="35" height="11.6667"/>
                                <rect x="1" y="12.6667" width="11.6667" height="11.6667"/>
                                <rect x="1" y="12.6667" width="11.6667" height="11.6667" transform="rotate(-90 1 12.6667)"/>
                            </g>
                        </svg>
                    </div>
                </button>
            </div>
        </div>
        `;

        this.shadowRoot.querySelector(".mesh-seed-global-menu-button").addEventListener("click", 
            () => this.activate("mesh-seed-global-menu-button"));
        
        this.shadowRoot.querySelector(".mesh-seed-local-menu-button").addEventListener("click", 
            () => this.activate("mesh-seed-local-menu-button"));
    }

    connectedCallback() {
        this.activate("mesh-seed-global-menu-button");
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
            const caption = this.state.captions[buttonName];
            this.shadowRoot.querySelector(".mesh-seed-menu-button-caption").innerHTML = caption;
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

export default FeaMeshMenuButtons;
