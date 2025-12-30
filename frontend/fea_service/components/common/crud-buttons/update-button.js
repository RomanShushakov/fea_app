class UpdateButton extends HTMLElement {
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

            .update-button {
                background: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin: 0rem;
            }

            .update-button-icon-content {
                margin: 0rem;
                padding: 0rem;
                width: 1.5rem;
                height: 1.5rem;
                border-bottom: 0.15rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
            }

            .update-button-icon {
                margin: 0rem;
                padding: 0rem;
                width: 1.5rem;
                height: 1.5rem;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .update-button:hover .update-button-icon {
                color: var(--preprocessor-menu-content-buttons-hover-color);
            }

            .active:hover .update-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .update-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-color);
            }

            .active:hover .update-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .update-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-color);
            }
        </style>

        <button class="update-button">
            <div class="update-button-icon-content">
                <svg class="update-button-icon" width="36" height="35" viewBox="0 0 36 35" fill="none" 
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <title>Update ${this.getAttribute("name")}</title>
                    <g stroke="currentColor">
                        <path d="M17.2055 8L16.9178 11.4247L14.9041 12.1096L12.3151 9.91781L10.3014 
                            11.8356L12.4589 14.4384L11.7397 16.2192L8 16.4932V19.2329L11.5959 
                            19.5068L12.4589 21.2877L10.0137 23.8904L12.0274 25.8082L14.7603 
                            23.6164L16.6301 24.4384L16.9178 28H19.7945L20.0822 24.5753L22.2397 
                            23.7534L25.1164 26.0822L27.1301 24.1644L24.5411 21.4247L25.4041 
                            19.6438L29 19.3699V16.7671L25.4041 16.4932L24.6849 14.5753L27.1301 
                            11.8356L25.1164 9.91781L22.2397 12.2466L20.3699 11.5616L20.0822 
                            8H17.2055Z"
                        />
                        <circle cx="18.5" cy="17.5" r="3.5"/>
                        <path d="M34 18.5C34 20.4042 33.6249 22.2897 32.8963 24.0489C32.1676 
                            25.8081 31.0995 27.4066 29.753 28.753C28.4066 30.0995 26.8081 
                            31.1676 25.0489 31.8963C23.2897 32.6249 21.4042 33 19.5 33"
                        />
                        <path d="M32.9962 20L34 17.3919L35.0038 20H32.9962Z"/>
                        <path d="M2.5 16.5C2.5 14.5958 2.87505 12.7103 3.60375 10.9511C4.33244 
                            9.19187 5.4005 7.5934 6.74695 6.24695C8.0934 4.9005 9.69187 
                            3.83244 11.4511 3.10375C13.2103 2.37505 15.0958 2 17 2"
                        />
                        <path d="M3.50384 15L2.5 17.6081L1.49616 15H3.50384Z"/>
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
        const updateButton = this.shadowRoot.querySelector(".update-button");
        if (this.getAttribute("is-active") === "true") {
            if (updateButton.classList.contains("active") === false) {
                updateButton.classList.add("active");
            }
        }
        if (this.getAttribute("is-active") === "false") {
            if (updateButton.classList.contains("active") === true) {
                updateButton.classList.remove("active");
            }
        }
    }
}

export default UpdateButton;
