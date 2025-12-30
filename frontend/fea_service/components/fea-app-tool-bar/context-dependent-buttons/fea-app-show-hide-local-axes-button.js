import { UPDATE_LOCAL_AXES_VISIBILITY_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";


class FeaAppShowHideLocalAxesButton extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            isLocalAxesVisible: true,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .show-hide-local-axes-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .show-hide-local-axes-button:hover .show-hide-local-axes-button-icon {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .show-hide-local-axes-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .active:hover .show-hide-local-axes-button-icon {
                background: var(--toolbar-active-button-icon-hover-bg-color);
                border: var(--toolbar-active-button-icon-hover-bg-color);
            }

            .active-icon {
                background: var(--toolbar-active-button-icon-bg-color);
                border: var(--toolbar-active-button-icon-bg-color);
            }
        </style>
        <button class="show-hide-local-axes-button">
            <svg class="show-hide-local-axes-button-icon" width="36" height="35" viewBox="0 0 36 35" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Show/Hide local axes</title>
                <path d="M26.5631 29.7662V34.1754H25.9821V28.8922H26.5485L26.5631 29.7662ZM28.4332 28.8531L28.4186 
                    29.3903C28.3535 29.3805 28.29 29.3724 28.2281 29.3658C28.1663 29.3593 28.0996 29.3561 28.028 
                    29.3561C27.774 29.3561 27.5511 29.4033 27.359 29.4977C27.1702 29.5888 27.0107 29.7174 26.8805 
                    29.8834C26.7503 30.0462 26.651 30.2382 26.5826 30.4596C26.5143 30.6777 26.4752 30.9121 26.4655 
                    31.1627L26.2506 31.2457C26.2506 30.9007 26.2864 30.58 26.358 30.2838C26.4296 29.9876 26.5387 
                    29.7288 26.6852 29.5074C26.8349 29.2828 27.0221 29.1087 27.2467 28.985C27.4746 28.858 27.7415 
                    28.7946 28.0475 28.7946C28.1224 28.7946 28.1956 28.8011 28.2672 28.8141C28.3421 28.8239 28.3974 
                    28.8369 28.4332 28.8531Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)" stroke-width="0.75"
                />
                <path d="M31.2023 15.9976V16.4761H28.5558V15.9976H31.2023ZM29.5275 14.6401H30.1086V19.9966C30.1086 
                    20.2407 30.1411 20.4246 30.2062 20.5483C30.2713 20.672 30.356 20.755 30.4601 20.7974C30.5643
                    20.8397 30.6766 20.8608 30.797 20.8608C30.8849 20.8608 30.9696 20.856 31.051 20.8462C31.1323 
                    20.8332 31.2056 20.8201 31.2707 20.8071L31.2951 21.3003C31.2235 21.3231 31.1307 21.341 31.0168 
                    21.354C30.9028 21.3703 30.7889 21.3784 30.675 21.3784C30.4504 21.3784 30.2518 21.3377 30.0793 
                    21.2563C29.9067 21.1717 29.7717 21.0285 29.674 20.8267C29.5763 20.6216 29.5275 20.3433 29.5275 
                    19.9917V14.6401Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)" stroke-width="0.75"
                />
                <path d="M21.6465 7.65234C21.6465 7.50911 21.6107 7.36751 21.5391 7.22754C21.4675 7.08757 21.3292 
                    6.95898 21.1241 6.8418C20.9222 6.72461 20.6244 6.62533 20.2305 6.54395C19.9375 6.47884 19.6755 
                    6.4056 19.4444 6.32422C19.2165 6.24284 19.0245 6.14518 18.8682 6.03125C18.712 5.91732 18.5931 
                    5.7806 18.5118 5.62109C18.4304 5.46159 18.3897 5.27279 18.3897 5.05469C18.3897 4.85938 18.432 
                    4.67546 18.5167 4.50293C18.6045 4.32715 18.7282 4.17415 18.8877 4.04395C19.0505 3.91048 19.2458 
                    3.80632 19.4737 3.73145C19.7048 3.65658 19.9636 3.61914 20.25 3.61914C20.6569 3.61914 21.0053 
                    3.68913 21.295 3.8291C21.5879 3.96582 21.8109 4.15299 21.9639 4.39062C22.1202 4.62826 22.1983
                    4.89681 22.1983 5.19629H21.6172C21.6172 5.014 21.5635 4.84147 21.4561 4.67871C21.3519 4.51595 
                    21.1973 4.38249 20.9922 4.27832C20.7904 4.17415 20.543 4.12207 20.25 4.12207C19.9473 4.12207 
                    19.7015 4.16764 19.5127 4.25879C19.3239 4.34993 19.1856 4.46549 19.0977 4.60547C19.0131 4.74544 
                    18.9708 4.8903 18.9708 5.04004C18.9708 5.15723 18.9887 5.26465 19.0245 5.3623C19.0635 5.45671 
                    19.1319 5.5446 19.2295 5.62598C19.3305 5.70736 19.4737 5.78385 19.6592 5.85547C19.8448 5.92708 
                    20.0857 5.99707 20.3819 6.06543C20.8083 6.15983 21.1566 6.27539 21.4268 6.41211C21.7002 6.54557 
                    21.9021 6.71159 22.0323 6.91016C22.1625 7.10547 22.2276 7.3431 22.2276 7.62305C22.2276 7.84115 
                    22.182 8.04134 22.0909 8.22363C21.9997 8.40267 21.8679 8.55729 21.6954 8.6875C21.5261 8.81771 
                    21.321 8.91862 21.0801 8.99023C20.8425 9.06185 20.5756 9.09766 20.2793 9.09766C19.8334 9.09766 
                    19.4558 9.02441 19.1465 8.87793C18.8405 8.72819 18.6078 8.53288 18.4483 8.29199C18.292 8.04785 
                    18.2139 7.78906 18.2139 7.51562H18.795C18.8145 7.78906 18.9008 8.00553 19.0538 8.16504C19.2068 
                    8.32129 19.3939 8.43197 19.6153 8.49707C19.8399 8.56217 20.0612 8.59473 20.2793 8.59473C20.5788 
                    8.59473 20.8295 8.55078 21.0313 8.46289C21.2331 8.375 21.3861 8.25944 21.4903 8.11621C21.5944 
                    7.97298 21.6465 7.81836 21.6465 7.65234Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)" stroke-width="0.75"
                />
                <path d="M23.4 29.3259C23.6936 29.3259 23.9316 29.1171 23.9316 28.8596V24.6629C23.9316 24.4054 23.6936 
                    24.1966 23.4 24.1966C23.1064 24.1966 22.8684 24.4054 22.8684 24.6629V28.3933H18.6158C18.3223 28.3933 
                    18.0843 28.602 18.0843 28.8596C18.0843 29.1171 18.3223 29.3259 18.6158 29.3259L23.4 29.3259ZM0.62412 
                    9.54017L23.0241 29.1893L23.7759 28.5299L1.37588 8.88073L0.62412 9.54017Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" stroke-width="1.5"
                />
                <path d="M23.7535 19.3887C23.9488 19.1934 23.9488 18.8769 23.7535 18.6816L20.5715 15.4996C20.3763 
                    15.3044 20.0597 15.3044 19.8644 15.4996C19.6692 15.6949 19.6692 16.0115 19.8644 16.2067L22.6928 
                    19.0352L19.8644 21.8636C19.6692 22.0588 19.6692 22.3754 19.8644 22.5707C20.0597 22.766 20.3763 
                    22.766 20.5715 22.5707L23.7535 19.3887ZM12.2 19.5352L23.4 19.5352V18.5352L12.2 18.5352V19.5352Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" stroke-width="1.5"
                />
                <path d="M12.5535 8.85704C12.3582 8.66178 12.0417 8.66178 11.8464 8.85704L8.66442 12.039C8.46916 12.2343 
                    8.46916 12.5509 8.66442 12.7461C8.85968 12.9414 9.17626 12.9414 9.37152 12.7461L12.2 9.9177L15.0284 
                    12.7461C15.2236 12.9414 15.5402 12.9414 15.7355 12.7461C15.9307 12.5509 15.9307 12.2343 15.7355 
                    12.039L12.5535 8.85704ZM12.7 19.0352V9.21059H11.7V19.0352H12.7Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" stroke-width="1.5"
                />
            </svg>
        </button>
        `;

        this.shadowRoot.querySelector(".show-hide-local-axes-button").addEventListener(
            "click", () => this.updateLocalAxesVisibility(),
        );

    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.updateLocalAxesVisibility();
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

    activate() {
        const button = this.shadowRoot.querySelector(".show-hide-local-axes-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-local-axes-button-icon");
        if (button.classList.contains("active") == false) {
            button.classList.add("active");
            buttonIcon.classList.add("active-icon");
        }
    }

    deactivate() {
        const button = this.shadowRoot.querySelector(".show-hide-local-axes-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-local-axes-button-icon");
        if (button.classList.contains("active") == true) {
            button.classList.remove("active");
            buttonIcon.classList.remove("active-icon");
        }
    }

    updateLocalAxesVisibility() {
        if (this.state.isLocalAxesVisible == true) {
            this.state.isLocalAxesVisible = false;
            this.activate();
        } else {
            this.state.isLocalAxesVisible = true;
            this.deactivate();
        }

        this.dispatchEvent(new CustomEvent(UPDATE_LOCAL_AXES_VISIBILITY_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: { "is_local_axes_visible": this.state.isLocalAxesVisible },
        }));   
    }
}

export default FeaAppShowHideLocalAxesButton;
