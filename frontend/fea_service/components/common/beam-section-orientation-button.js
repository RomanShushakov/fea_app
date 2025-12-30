class BeamSectionOrientationButton extends HTMLElement {
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

            .beam-section-orientation-button {
                background: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin: 0rem;
            }

            .beam-section-orientation-button-icon-content {
                margin: 0rem;
                padding: 0rem;
                width: 1.5rem;
                height: 1.5rem;
                border-bottom: 0.15rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
            }

            .beam-section-orientation-button-icon {
                margin: 0rem;
                padding: 0rem;
                width: 1.5rem;
                height: 1.5rem;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .beam-section-orientation-button:hover .beam-section-orientation-button-icon {
                color: var(--preprocessor-menu-content-buttons-hover-color);
            }

            .active:hover .beam-section-orientation-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .beam-section-orientation-button-icon {
                color: var(--preprocessor-menu-content-active-buttons-color);
            }

            .active:hover .beam-section-orientation-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .active .beam-section-orientation-button-icon-content {
                border-bottom: 0.15rem solid var(--preprocessor-menu-content-active-buttons-color);
            }
        </style>
        <button class="beam-section-orientation-button">
            <div class="beam-section-orientation-button-icon-content">

                <svg class="beam-section-orientation-button-icon" width="36" height="35" 
                    viewBox="0 0 36 35" fill="none" xmlns="http://www.w3.org/2000/svg"
                >
                    <title>Beam section orientation</title>
                    <path d="M26.9635 27.5397V28.8922H28.0573V29.3708H26.9635V32.8913C26.9635
                        33.1842 27.0156 33.4023 27.1198 33.5456C27.2272 33.6888 27.4046 33.7604 
                        27.652 33.7604C27.7496 33.7604 27.9075 33.7441 28.1256 33.7116L28.15 
                        34.1901C27.997 34.2454 27.7887 34.2731 27.525 34.2731C27.1246 34.2731 
                        26.8333 34.1575 26.651 33.9264C26.4687 33.692 26.3776 33.3486 26.3776 
                        32.8961V29.3708H25.4059V28.8922H26.3776V27.5397H26.9635Z" fill="var(--preprocessor-menu-content-caption-color)"
                    />
                    <path d="M25.8459 15.9975L25.8654 16.9008C26.064 16.5753 26.3114 16.3279 
                        26.6076 16.1586C26.9038 15.9861 27.231 15.8998 27.589 15.8998C28.1554 
                        15.8998 28.577 16.0593 28.8537 16.3784C29.1304 16.6974 29.2704 17.1759 
                        29.2736 17.8139V21.2807H28.6926V17.809C28.6893 17.337 28.5884 16.9855 
                        28.3898 16.7543C28.1945 16.5232 27.8804 16.4077 27.4474 16.4077C27.0861 
                        16.4077 26.7655 16.5216 26.4855 16.7495C26.2088 16.9741 26.0038 17.2784 
                        25.8703 17.6625V21.2807H25.2892V15.9975H25.8459ZM33.4142 
                        21.2807H32.8283V14.8745L30.8947 15.5922V15.0453L33.3166 14.1469H33.4142V21.2807Z" 
                        fill="var(--preprocessor-menu-content-caption-color)"
                    />
                    <path d="M16.0459 3.7168L16.0654 4.62012C16.264 4.2946 16.5114 4.0472 
                        16.8076 3.87793C17.1038 3.7054 17.431 3.61914 17.7891 3.61914C18.3555 
                        3.61914 18.777 3.77865 19.0537 4.09766C19.3304 4.41667 19.4704 4.89518 
                        19.4736 5.5332V9H18.8926V5.52832C18.8893 5.05632 18.7884 4.70475 18.5898 
                        4.47363C18.3945 4.24251 18.0804 4.12695 17.6474 4.12695C17.2861 4.12695 
                        16.9655 4.24089 16.6855 4.46875C16.4088 4.69336 16.2038 4.99772 16.0703 
                        5.38184V9H15.4892V3.7168H16.0459ZM25.3476 9H20.8945V8.55078L23.331 
                        5.78711C23.7347 5.32161 24.0146 4.9375 24.1709 4.63477C24.3271 4.33203 
                        24.4053 4.03743 24.4053 3.75098C24.4053 3.30176 24.2734 2.94694 24.0098 
                        2.68652C23.7461 2.42285 23.3831 2.29102 22.9209 2.29102C22.4424 2.29102 
                        22.055 2.44401 21.7588 2.75C21.4658 3.05599 21.3193 3.45312 21.3193 
                        3.94141H20.7383C20.7383 3.54102 20.8278 3.17643 21.0068 2.84766C21.1891 
                        2.51888 21.4463 2.26172 21.7783 2.07617C22.1136 1.88737 22.4945 1.79297 
                        22.9209 1.79297C23.5719 1.79297 24.0797 1.96387 24.4443 2.30566C24.8122 
                        2.64746 24.9961 3.11296 24.9961 3.70215C24.9961 4.03418 24.8919 4.39714 
                        24.6836 4.79102C24.4785 5.1849 24.1155 5.67643 23.5947 6.26562L21.6123 
                        8.50684H25.3476V9Z" fill="var(--preprocessor-menu-content-caption-color)"
                    />
                    <g fill="currentColor">
                        <path d="M23.4 29.3259C23.6936 29.3259 23.9316 29.1172 23.9316 
                            28.8597V24.663C23.9316 24.4055 23.6936 24.1967 23.4 24.1967C23.1064 24.1967 
                            22.8684 24.4055 22.8684 24.663V28.3934H18.6158C18.3223 28.3934 18.0843 
                            28.6021 18.0843 28.8597C18.0843 29.1172 18.3223 29.3259 18.6158 
                            29.3259L23.4 29.3259ZM0.62412 9.54024L23.0241 29.1894L23.7759 
                            28.5299L1.37588 8.88081L0.62412 9.54024Z"
                        />
                        <path d="M23.7536 19.3886C23.9488 19.1934 23.9488 18.8768 23.7536 18.6815L20.5716 
                            15.4996C20.3763 15.3043 20.0597 15.3043 19.8645 15.4996C19.6692 15.6948 
                            19.6692 16.0114 19.8645 16.2067L22.6929 19.0351L19.8645 21.8635C19.6692 
                            22.0588 19.6692 22.3754 19.8645 22.5706C20.0597 22.7659 20.3763 22.7659 
                            20.5716 22.5706L23.7536 19.3886ZM12.2 19.5351L23.4 19.5351V18.5351L12.2 
                            18.5351V19.5351Z"
                        />
                        <path d="M12.5536 8.85698C12.3583 8.66172 12.0417 8.66172 11.8465 8.85698L8.66448 
                            12.039C8.46922 12.2342 8.46922 12.5508 8.66448 12.7461C8.85974 12.9413 
                            9.17632 12.9413 9.37159 12.7461L12.2 9.91764L15.0284 12.7461C15.2237 
                            12.9413 15.5403 12.9413 15.7355 12.7461C15.9308 12.5508 15.9308 12.2342 
                            15.7355 12.039L12.5536 8.85698ZM12.7 19.0351V9.21053H11.7V19.0351H12.7Z"
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
        const beamSectionOrientationButton = this.shadowRoot.querySelector(".beam-section-orientation-button");
        if (this.getAttribute("is-active") === "true") {
            if (beamSectionOrientationButton.classList.contains("active") === false) {
                beamSectionOrientationButton.classList.add("active");
            }
        }
        if (this.getAttribute("is-active") === "false") {
            if (beamSectionOrientationButton.classList.contains("active") === true) {
                beamSectionOrientationButton.classList.remove("active");
            }
        }
    }
}

export default BeamSectionOrientationButton;
