import { UPDATE_BEAM_SECTION_ORIENTATION_VISIBILITY_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";


class FeaAppShowHideBeamSectionOrientationButton extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            isBeamSectionOrientationVisible: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .show-hide-beam-section-orientation-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .show-hide-beam-section-orientation-button:hover .show-hide-beam-section-orientation-button-icon {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .show-hide-beam-section-orientation-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .active:hover .show-hide-beam-section-orientation-button-icon {
                background: var(--toolbar-active-button-icon-hover-bg-color);
                border: var(--toolbar-active-button-icon-hover-bg-color);
            }

            .active-icon {
                background: var(--toolbar-active-button-icon-bg-color);
                border: var(--toolbar-active-button-icon-bg-color);
            }
        </style>
        <button class="show-hide-beam-section-orientation-button">
            <svg class="show-hide-beam-section-orientation-button-icon" width="36" height="35" viewBox="0 0 36 35" 
                fill="none" xmlns="http://www.w3.org/2000/svg"
            >
                <title>Show/Hide beam section orientation</title>

                <path d="M28.0524 28.8922V29.3707H25.4059V28.8922H28.0524ZM26.3776 27.5348H26.9586V32.8912C26.9586 
                    33.1354 26.9912 33.3193 27.0563 33.443C27.1214 33.5667 27.206 33.6497 27.3102 33.692C27.4143 
                    33.7343 27.5267 33.7555 27.6471 33.7555C27.735 33.7555 27.8196 33.7506 27.901 33.7408C27.9824 
                    33.7278 28.0556 33.7148 28.1207 33.7018L28.1451 34.1949C28.0735 34.2177 27.9808 34.2356 27.8668 
                    34.2487C27.7529 34.2649 27.639 34.2731 27.525 34.2731C27.3004 34.2731 27.1018 34.2324 26.9293 
                    34.151C26.7568 34.0664 26.6217 33.9231 26.524 33.7213C26.4264 33.5162 26.3776 33.2379 26.3776 
                    32.8864V27.5348Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)" stroke-width="0.75"
                />
                <path d="M25.8752 17.1254V21.2807H25.2941V15.9975H25.8508L25.8752 17.1254ZM25.7433 18.312L25.4845 
                    18.1655C25.5041 17.853 25.5676 17.56 25.675 17.2866C25.7824 17.0131 25.9273 16.7722 26.1095 
                    16.5639C26.2951 16.3556 26.5116 16.1928 26.759 16.0756C27.0096 15.9584 27.2863 15.8998 27.589 
                    15.8998C27.8527 15.8998 28.0887 15.9356 28.297 16.0073C28.5054 16.0789 28.6828 16.1928 28.8293 
                    16.3491C28.9758 16.5053 29.0864 16.7055 29.1613 16.9496C29.2394 17.1938 29.2785 17.4916 29.2785 
                    17.8432V21.2807H28.6926V17.8383C28.6926 17.4737 28.6421 17.1889 28.5412 16.9838C28.4403 16.7755 
                    28.297 16.6274 28.1115 16.5395C27.926 16.4516 27.7062 16.4077 27.4523 16.4077C27.1724 16.4077 
                    26.9266 16.4695 26.715 16.5932C26.5067 16.7136 26.3309 16.8715 26.1877 17.0668C26.0477 17.2589 
                    25.9403 17.4656 25.8654 17.687C25.7905 17.905 25.7498 18.1134 25.7433 18.312ZM33.4142 
                    14.142V21.2807H32.8332V14.8793L30.8996 15.5971V15.0453L33.3166 14.142H33.4142Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)" stroke-width="0.75"
                />
                <path d="M16.0752 4.84473V9H15.4941V3.7168H16.0508L16.0752 4.84473ZM15.9434 6.03125L15.6846 
                    5.88477C15.7041 5.57227 15.7676 5.2793 15.875 5.00586C15.9824 4.73242 16.1273 4.49154 16.3096 
                    4.2832C16.4951 4.07487 16.7116 3.91211 16.959 3.79492C17.2096 3.67773 17.4863 3.61914 17.7891 
                    3.61914C18.0527 3.61914 18.2887 3.65495 18.4971 3.72656C18.7054 3.79818 18.8828 3.91211 19.0293 
                    4.06836C19.1758 4.22461 19.2865 4.4248 19.3613 4.66895C19.4395 4.91309 19.4785 5.21094 19.4785
                    5.5625V9H18.8926V5.55762C18.8926 5.19303 18.8421 4.9082 18.7412 4.70312C18.6403 4.49479 18.4971 
                    4.34668 18.3115 4.25879C18.126 4.1709 17.9062 4.12695 17.6523 4.12695C17.3724 4.12695 17.1266 
                    4.1888 16.915 4.3125C16.7067 4.43294 16.5309 4.59082 16.3877 4.78613C16.2477 4.97819 16.1403 
                    5.1849 16.0654 5.40625C15.9906 5.62435 15.9499 5.83268 15.9434 6.03125ZM25.3477 
                    8.49707V9H20.8994V8.5459L23.2285 5.9043C23.5182 5.57552 23.7493 5.28743 23.9219 5.04004C24.0944 
                    4.78939 24.2181 4.56152 24.293 4.35645C24.3711 4.14811 24.4102 3.94629 24.4102 3.75098C24.4102 
                    3.46452 24.3532 3.21224 24.2393 2.99414C24.1286 2.77604 23.9626 2.60514 23.7412 2.48145C23.5199 
                    2.35775 23.2464 2.2959 22.9209 2.2959C22.5954 2.2959 22.3122 2.36589 22.0713 2.50586C21.8304 
                    2.64583 21.6449 2.84115 21.5146 3.0918C21.3877 3.33919 21.3242 3.62077 21.3242 
                    3.93652H20.7432C20.7432 3.5459 20.8294 3.18783 21.002 2.8623C21.1777 2.53678 21.4284 2.27799 
                    21.7539 2.08594C22.0794 1.89062 22.4684 1.79297 22.9209 1.79297C23.3506 1.79297 23.7201 1.86947 
                    24.0293 2.02246C24.3385 2.1722 24.5762 2.38867 24.7422 2.67188C24.9115 2.95508 24.9961 3.29688 
                    24.9961 3.69727C24.9961 3.91211 24.957 4.12858 24.8789 4.34668C24.804 4.56478 24.6999 4.78288 
                    24.5664 5.00098C24.4362 5.21582 24.2865 5.42904 24.1172 5.64062C23.9512 5.85221 23.777 6.06055 
                    23.5947 6.26562L21.6123 8.49707H25.3477Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)" stroke-width="0.75"
                />
                <path d="M23.4 29.3259C23.6936 29.3259 23.9316 29.1172 23.9316 28.8596V24.663C23.9316 24.4055 23.6936 
                    24.1967 23.4 24.1967C23.1064 24.1967 22.8684 24.4055 22.8684 24.663V28.3933H18.6158C18.3223 
                    28.3933 18.0843 28.6021 18.0843 28.8596C18.0843 29.1172 18.3223 29.3259 18.6158 29.3259L23.4 
                    29.3259ZM0.62412 9.54023L23.0241 29.1894L23.7759 28.5299L1.37588 8.88079L0.62412 9.54023Z" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" stroke-width="1.5"
                />
                <path d="M23.7536 19.3886C23.9488 19.1934 23.9488 18.8768 23.7536 18.6815L20.5716 15.4996C20.3763 
                    15.3043 20.0597 15.3043 19.8645 15.4996C19.6692 15.6948 19.6692 16.0114 19.8645 16.2067L22.6929 
                    19.0351L19.8645 21.8635C19.6692 22.0588 19.6692 22.3754 19.8645 22.5706C20.0597 22.7659 20.3763 
                    22.7659 20.5716 22.5706L23.7536 19.3886ZM12.2 19.5351L23.4 19.5351V18.5351L12.2 18.5351V19.5351Z" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" stroke-width="1.5"
                />
                <path d="M12.5536 8.85698C12.3583 8.66172 12.0417 8.66172 11.8465 8.85698L8.66448 12.039C8.46922 
                    12.2342 8.46922 12.5508 8.66448 12.7461C8.85974 12.9413 9.17632 12.9413 9.37159 12.7461L12.2 
                    9.91764L15.0284 12.7461C15.2237 12.9413 15.5403 12.9413 15.7355 12.7461C15.9308 12.5508 15.9308 
                    12.2342 15.7355 12.039L12.5536 8.85698ZM12.7 19.0351V9.21053H11.7V19.0351H12.7Z" 
                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" stroke-width="1.5"
                />
            </svg>
        </button>
        `;

        this.shadowRoot.querySelector(".show-hide-beam-section-orientation-button").addEventListener("click", 
            () => this.updateBeamSectionOrientationVisibility());

    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.updateBeamSectionOrientationVisibility();
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
        const button = this.shadowRoot.querySelector(".show-hide-beam-section-orientation-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-beam-section-orientation-button-icon");
        if (button.classList.contains("active") == false) {
            button.classList.add("active");
            buttonIcon.classList.add("active-icon");
        }
    }

    deactivate() {
        const button = this.shadowRoot.querySelector(".show-hide-beam-section-orientation-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-beam-section-orientation-button-icon");
        if (button.classList.contains("active") == true) {
            button.classList.remove("active");
            buttonIcon.classList.remove("active-icon");
        }
    }

    updateBeamSectionOrientationVisibility() {
        if (this.state.isBeamSectionOrientationVisible == true) {
            this.state.isBeamSectionOrientationVisible = false;
            this.activate();
        } else {
            this.state.isBeamSectionOrientationVisible = true;
            this.deactivate();
        }

        this.dispatchEvent(new CustomEvent(UPDATE_BEAM_SECTION_ORIENTATION_VISIBILITY_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: { "is_beam_section_orientation_visible": this.state.isBeamSectionOrientationVisible },
        }));    
    }
}

export default FeaAppShowHideBeamSectionOrientationButton;
