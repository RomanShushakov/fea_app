class FeaMeshLocalMeshSeedMenuButtons extends HTMLElement {
    constructor() {
        super();

        this.props = {};

        this.state = {
            buttonNames: [
                "lines-mesh-seed-menu-button", "surfaces-mesh-seed-menu-button",
            ],

            menuNames: {
                "lines-mesh-seed-menu-button": "lines-mesh-seed-menu",
                "surfaces-mesh-seed-menu-button": "surfaces-mesh-seed-menu",
            },

            captions: {
                "lines-mesh-seed-menu-button": "Lines",
                "surfaces-mesh-seed-menu-button": "Surfaces",
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

            .local-mesh-seed-menu-buttons-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                align-items: center;
            }

            .local-mesh-seed-menu-buttons-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .lines-mesh-seed-menu-button {
                background: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }

            .lines-mesh-seed-menu-button-icon-content {
                margin: 0rem;
                padding: 0rem;
                width: 1.5rem;
                height: 1.5rem;
                border-bottom: 0.15rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
            }

            .lines-mesh-seed-menu-button-icon {
                margin: 0rem;
                padding: 0rem;
                width: 1.5rem;
                height: 1.5rem;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .lines-mesh-seed-menu-button:hover .lines-mesh-seed-menu-button-icon {
                color: var(--preprocessor-menu-content-buttons-hover-color);
            }

            .active:hover .lines-mesh-seed-menu-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .lines-mesh-seed-menu-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-color);
            }

            .active:hover .lines-mesh-seed-menu-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .lines-mesh-seed-menu-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-color);
            }

            .surfaces-mesh-seed-menu-button {
                background: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0.5rem;
                margin-right: 0rem;
            }

            .surfaces-mesh-seed-menu-button-icon-content {
                margin: 0rem;
                padding: 0rem;
                width: 1.5rem;
                height: 1.5rem;
                border-bottom: 0.15rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
            }

            .surfaces-mesh-seed-menu-button-icon {
                margin: 0rem;
                padding: 0rem;
                width: 1.5rem;
                height: 1.5rem;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .surfaces-mesh-seed-menu-button:hover .surfaces-mesh-seed-menu-button-icon {
                color: var(--preprocessor-menu-content-buttons-hover-color);
            }

            .active:hover .surfaces-mesh-seed-menu-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .surfaces-mesh-seed-menu-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-color);
            }

            .active:hover .surfaces-mesh-seed-menu-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .surfaces-mesh-seed-menu-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-color);
            }
        </style>

        <div class=wrapper>
            <div class="local-mesh-seed-menu-buttons-content">

                <p class="local-mesh-seed-menu-buttons-caption">Lines</p>

                <button class="lines-mesh-seed-menu-button">
                    <div class="lines-mesh-seed-menu-button-icon-content">
                        <svg class="lines-mesh-seed-menu-button-icon" width="35" height="35" viewBox="0 0 35 35" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Line</title>
                            <g stroke="currentColor">
                                <rect x="28" y="2" width="5" height="5"/>
                                <rect x="3" y="27" width="5" height="5"/>
                                <line x1="8.64645" y1="26.6464" x2="27.6464" y2="7.64645"/>
                            </g>
                        </svg>
                    </div>
                </button>

                <button class="surfaces-mesh-seed-menu-button">
                    <div class="surfaces-mesh-seed-menu-button-icon-content">
                        <svg class="surfaces-mesh-seed-menu-button-icon" width="35" height="35" viewBox="0 0 35 35" fill="none" 
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Surfaces</title>
                            <g stroke="currentColor">
                                <rect x="9" y="2" width="5" height="5"/>
                                <rect x="27" y="5" width="5" height="5"/>
                                <rect x="4" y="24" width="5" height="5"/>
                                <rect x="22" y="27" width="5" height="5"/>
                                <line x1="24.8915" y1="7.48809" x2="15.8915" y2="5.48809"/>
                                <line x1="19.8915" y1="28.4881" x2="10.8915" y2="26.4881"/>
                                <line x1="25.5128" y1="24.8876" x2="28.5128" y2="11.8876"/>
                                <line x1="7.5128" y1="21.8876" x2="10.5128" y2="8.88757"/>
                            </g>
                        </svg>
                    </div>
                </button>
            </div>
        </div>
        `;
        
        this.shadowRoot.querySelector(".lines-mesh-seed-menu-button").addEventListener("click", 
            () => this.activate("lines-mesh-seed-menu-button"));
        this.shadowRoot.querySelector(".surfaces-mesh-seed-menu-button").addEventListener("click", 
            () => this.activate("surfaces-mesh-seed-menu-button"));
    }

    set activateButton(buttonName) {
        this.activate(buttonName);
    }

    connectedCallback() {
        this.activate("lines-mesh-seed-menu-button");
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
            this.shadowRoot.querySelector(".local-mesh-seed-menu-buttons-caption").innerHTML = caption;
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

export default FeaMeshLocalMeshSeedMenuButtons;
