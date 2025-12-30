import { UPDATE_SURFACE_EDGES_2_4_VISIBILITY_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";

class FeaAppShowHideSurfaceEdges24Button extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            isSurfaceEdges24Visible: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .show-hide-surface-edges-2-4-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .show-hide-surface-edges-2-4-button:hover .show-hide-surface-edges-2-4-button-icon {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .show-hide-surface-edges-2-4-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .active:hover .show-hide-surface-edges-2-4-button-icon {
                background: var(--toolbar-active-button-icon-hover-bg-color);
                border: var(--toolbar-active-button-icon-hover-bg-color);
            }

            .active-icon {
                background: var(--toolbar-active-button-icon-bg-color);
                border: var(--toolbar-active-button-icon-bg-color);
            }
        </style>
        <button class="show-hide-surface-edges-2-4-button">

            <svg class="show-hide-surface-edges-2-4-button-icon" width="38" height="37" viewBox="0 0 38 37" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Show/Hide surface edges: 2 4</title>
                
                <line y1="-0.75" x2="11.8853" y2="-0.75" 
                    transform="matrix(-0.51214 0.858902 -0.415292 -0.909688 30.6739 18.0417)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" stroke-width="1.5"
                />
                <line y1="-0.75" x2="11.8853" y2="-0.75" 
                    transform="matrix(-0.51214 0.858902 -0.415292 -0.909688 11.6522 18.0417)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" stroke-width="1.5"
                />
                <line x1="17.7391" y1="33.125" x2="7.08696" y2="33.125" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <line x1="29.9131" y1="12.7083" x2="19.2609" y2="12.7083" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <rect x="1" y="31" width="5" height="5" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <rect x="19" y="31" width="5" height="5" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <rect x="32" y="10" width="5" height="5" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <rect x="12" y="10" width="5" height="5" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <path d="M8.59863 21.4971V22H4.15039V21.5459L6.47949 18.9043C6.7692 18.5755 7.00032 18.2874 7.17285 
                    18.04C7.34538 17.7894 7.46907 17.5615 7.54394 17.3564C7.62207 17.1481 7.66113 16.9463 7.66113 
                    16.751C7.66113 16.4645 7.60417 16.2122 7.49023 15.9941C7.37956 15.776 7.21354 15.6051 6.99219 
                    15.4814C6.77083 15.3577 6.49739 15.2959 6.17187 15.2959C5.84635 15.2959 5.56315 15.3659 5.32226 
                    15.5059C5.08138 15.6458 4.89583 15.8411 4.76562 16.0918C4.63867 16.3392 4.57519 16.6208 4.57519 
                    16.9365H3.99414C3.99414 16.5459 4.0804 16.1878 4.25293 15.8623C4.42871 15.5368 4.67936 15.278 
                    5.00488 15.0859C5.3304 14.8906 5.7194 14.793 6.17187 14.793C6.60156 14.793 6.97103 14.8695 7.28027 
                    15.0225C7.58952 15.1722 7.82715 15.3887 7.99316 15.6719C8.16243 15.9551 8.24707 16.2969 8.24707 
                    16.6973C8.24707 16.9121 8.20801 17.1286 8.12988 17.3467C8.05501 17.5648 7.95085 17.7829 7.81738 
                    18.001C7.68717 18.2158 7.53743 18.429 7.36816 18.6406C7.20215 18.8522 7.02799 19.0605 6.8457 
                    19.2656L4.86328 21.4971H8.59863Z" stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <path d="M27.2129 7.25195C27.2129 7.05664 27.1803 6.88249 27.1152 6.72949C27.0501 6.5765 26.9427 
                    6.43978 26.793 6.31934C26.6465 6.19564 26.4479 6.08171 26.1973 5.97754C25.9499 5.87012 25.6406 
                    5.76432 25.2695 5.66016C24.9147 5.55924 24.5941 5.44857 24.3076 5.32812C24.0212 5.20768 23.7754 
                    5.06934 23.5703 4.91309C23.3685 4.75684 23.2139 4.57454 23.1064 4.36621C22.999 4.15462 22.9453 
                    3.90885 22.9453 3.62891C22.9453 3.35547 23.0039 3.10645 23.1211 2.88184C23.2383 2.65723 23.4027 
                    2.46354 23.6143 2.30078C23.8291 2.13802 24.0814 2.0127 24.3711 1.9248C24.6641 1.83691 24.9847 
                    1.79297 25.333 1.79297C25.8311 1.79297 26.2607 1.88737 26.6221 2.07617C26.9867 2.26497 27.2682 
                    2.52051 27.4668 2.84277C27.6686 3.16178 27.7695 3.51986 27.7695 3.91699H27.1689C27.1689 3.60775 
                    27.0957 3.33268 26.9492 3.0918C26.806 2.84766 26.5977 2.65723 26.3242 2.52051C26.054 2.38053 
                    25.7236 2.31055 25.333 2.31055C24.9424 2.31055 24.6136 2.37077 24.3467 2.49121C24.083 2.6084 
                    23.8828 2.76628 23.7461 2.96484C23.6126 3.16016 23.5459 3.37826 23.5459 3.61914C23.5459 3.78841 
                    23.5784 3.94466 23.6436 4.08789C23.7087 4.22786 23.8145 4.3597 23.9609 4.4834C24.1107 4.60384 
                    24.3076 4.71777 24.5518 4.8252C24.7959 4.92936 25.097 5.02865 25.4551 5.12305C25.8327 5.22721 
                    26.168 5.34277 26.4609 5.46973C26.7539 5.59668 27.0013 5.74316 27.2031 5.90918C27.4049 6.07194 
                    27.5579 6.26237 27.6621 6.48047C27.7663 6.69857 27.8184 6.95247 27.8184 7.24219C27.8184 7.5319 
                    27.7565 7.79232 27.6328 8.02344C27.5124 8.2513 27.3415 8.44499 27.1201 8.60449C26.902 8.764 26.6432
                    8.88607 26.3437 8.9707C26.0475 9.05534 25.7236 9.09766 25.3721 9.09766C25.0563 9.09766 24.7422 
                    9.05697 24.4297 8.97559C24.1172 8.89421 23.8307 8.76888 23.5703 8.59961C23.3132 8.42708 23.1064 
                    8.20573 22.9502 7.93555C22.7972 7.66536 22.7207 7.3431 22.7207 6.96875H23.3164C23.3164 7.26823 
                    23.3766 7.52051 23.4971 7.72559C23.6175 7.93066 23.777 8.09668 23.9756 8.22363C24.1774 8.35059 
                    24.3988 8.44336 24.6396 8.50195C24.8838 8.55729 25.1279 8.58496 25.3721 8.58496C25.7464 8.58496 
                    26.0703 8.52962 26.3437 8.41895C26.6204 8.30827 26.8337 8.15365 26.9834 7.95508C27.1364 7.75326 
                    27.2129 7.51888 27.2129 7.25195Z" stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <path d="M24.8428 22.7051V23.2031H19.8086V22.8662L23.0947 17.8906H23.583L22.8896 19.0967L20.5215 
                    22.7051H24.8428ZM23.7344 17.8906V25H23.1533V17.8906H23.7344Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
            </svg>

        </button>
        `;

        this.shadowRoot.querySelector(".show-hide-surface-edges-2-4-button").addEventListener("click", 
            () => this.updateSurfaceEdges24Visibility());

    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.updateSurfaceEdges24Visibility();
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
        const button = this.shadowRoot.querySelector(".show-hide-surface-edges-2-4-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-surface-edges-2-4-button-icon");
        if (button.classList.contains("active") == false) {
            button.classList.add("active");
            buttonIcon.classList.add("active-icon");
        }
    }

    deactivate() {
        const button = this.shadowRoot.querySelector(".show-hide-surface-edges-2-4-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-surface-edges-2-4-button-icon");
        if (button.classList.contains("active") == true) {
            button.classList.remove("active");
            buttonIcon.classList.remove("active-icon");
        }
    }

    updateSurfaceEdges24Visibility() {
        if (this.state.isSurfaceEdges24Visible == true) {
            this.state.isSurfaceEdges24Visible = false;
            this.activate();
        } else {
            this.state.isSurfaceEdges24Visible = true;
            this.deactivate();
        }

        this.dispatchEvent(new CustomEvent(UPDATE_SURFACE_EDGES_2_4_VISIBILITY_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: { "is_surface_edges_2_4_visible": this.state.isSurfaceEdges24Visible },
        }));    
    }
}

export default FeaAppShowHideSurfaceEdges24Button;
