class AssignButton extends HTMLElement {
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

            .assign-button {
                background: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin: 0rem;
            }

            .assign-button-icon-content {
                margin: 0rem;
                padding: 0rem;
                width: 1.5rem;
                height: 1.5rem;
                border-bottom: 0.15rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
            }

            .assign-button-icon {
                margin: 0rem;
                padding: 0rem;
                width: 1.5rem;
                height: 1.5rem;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .assign-button:hover .assign-button-icon {
                color: var(--preprocessor-menu-content-buttons-hover-color);
            }

            .active:hover .assign-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .assign-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-color);
            }

            .active:hover .assign-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .assign-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-color);
            }
        </style>
        <button class="assign-button">
            <div class="assign-button-icon-content">

                <svg class="assign-button-icon" width="35" height="35" viewBox="0 0 35 35" fill="none" 
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <title>Assign ${this.getAttribute("name")}</title>
                    <g stroke="currentColor">
                        <path d="M7.63328 10.5C9.14299 9.01664 11.0665 8.00646 13.1605 7.5972C15.2546 7.18794 
                            17.4251 7.39799 19.3976 8.20078C21.3702 9.00356 23.0561 10.363 24.2423 12.1073C25.4285 
                            13.8515 26.0616 15.9022 26.0616 18C26.0616 20.0978 25.4285 22.1485 24.2423 23.8927C23.0561 
                            25.637 21.3702 26.9964 19.3976 27.7992C17.4251 28.602 15.2546 28.8121 13.1605 
                            28.4028C11.0665 27.9935 9.14299 26.9834 7.63328 25.5" 
                            stroke-linecap="round"
                        />
                        <path d="M17.6274 7.5972C19.7215 7.18794 21.892 7.39798 23.8646 8.20077C25.8371 9.00356 
                            27.523 10.363 28.7092 12.1073C29.8954 13.8515 30.5285 15.9022 30.5285 18C30.5285 
                            20.0978 29.8954 22.1485 28.7092 23.8927C27.5231 25.637 25.8371 26.9964 23.8646 
                            27.7992C21.892 28.602 19.7215 28.8121 17.6274 28.4028"
                        />
                        <path d="M10.7996 16.6342C10.2473 16.6342 9.79962 17.082 9.79962 17.6342C9.79962 
                            18.1865 10.2473 18.6342 10.7996 18.6342V16.6342ZM20.0683 18.3414C20.4589 
                            17.9508 20.4589 17.3177 20.0683 16.9271L13.7044 10.5632C13.3138 10.1727 
                            12.6807 10.1727 12.2902 10.5632C11.8996 10.9537 11.8996 11.5869 12.2902 
                            11.9774L17.947 17.6342L12.2902 23.2911C11.8996 23.6816 11.8996 24.3148 
                            12.2902 24.7053C12.6807 25.0958 13.3138 25.0958 13.7044 24.7053L20.0683 
                            18.3414ZM10.7996 18.6342L19.3612 18.6342V16.6342L10.7996 16.6342V18.6342Z"
                        />
                        <line x1="7.19391" y1="17.9028" x2="5.47147" y2="17.9028" 
                            stroke-width="2" stroke-linecap="round"
                        />
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
        const assignButton = this.shadowRoot.querySelector(".assign-button");
        if (this.getAttribute("is-active") === "true") {
            if (assignButton.classList.contains("active") === false) {
                assignButton.classList.add("active");
            }
        }
        if (this.getAttribute("is-active") === "false") {
            if (assignButton.classList.contains("active") === true) {
                assignButton.classList.remove("active");
            }
        }
    }
}

export default AssignButton;
