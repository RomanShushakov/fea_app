import { UPDATE_SURFACE_EDGES_1_3_VISIBILITY_MESSAGE_HEADER } from "../../../consts/tool_bar_consts.js";

class FeaAppShowHideSurfaceEdges13Button extends HTMLElement {
    constructor() {
        super();

        this.props = { };

        this.state = {
            isSurfaceEdges13Visible: false,
        };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            .show-hide-surface-edges-1-3-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .show-hide-surface-edges-1-3-button:hover .show-hide-surface-edges-1-3-button-icon {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .show-hide-surface-edges-1-3-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .active:hover .show-hide-surface-edges-1-3-button-icon {
                background: var(--toolbar-active-button-icon-hover-bg-color);
                border: var(--toolbar-active-button-icon-hover-bg-color);
            }

            .active-icon {
                background: var(--toolbar-active-button-icon-bg-color);
                border: var(--toolbar-active-button-icon-bg-color);
            }
        </style>
        <button class="show-hide-surface-edges-1-3-button">

            <svg class="show-hide-surface-edges-1-3-button-icon" width="38" height="37" viewBox="0 0 38 37" fill="none" 
                xmlns="http://www.w3.org/2000/svg"
            >
                <title>Show/Hide surface edges: 1 3</title>

                <line y1="-0.5" x2="11.8853" y2="-0.5" 
                    transform="matrix(-0.51214 0.858902 -0.415292 -0.909688 30.6739 18.0417)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <line y1="-0.5" x2="11.8853" y2="-0.5" 
                    transform="matrix(-0.51214 0.858902 -0.415292 -0.909688 11.6522 18.0417)" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
                <line x1="17.7391" y1="33.375" x2="7.08696" y2="33.375" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" stroke-width="1.5"
                />
                <line x1="29.9131" y1="12.9583" x2="19.2609" y2="12.9583" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" stroke-width="1.5"
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
                <path d="M26.8652 1.86133V9H26.2842V2.59863L24.3506 3.31641V2.76465L26.7676 1.86133H26.8652Z" 
                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <path d="M14.4492 25.123H15.0449C15.4095 25.123 15.7139 25.0612 15.958 24.9375C16.2054 24.8105 
                    16.3909 24.6429 16.5146 24.4346C16.6416 24.2262 16.7051 23.9951 16.7051 23.7412C16.7051 23.4548 
                    16.6514 23.2041 16.5439 22.9893C16.4365 22.7712 16.2721 22.6019 16.0508 22.4814C15.8327 22.3577 
                    15.5527 22.2959 15.2109 22.2959C14.9115 22.2959 14.6445 22.3545 14.4102 22.4717C14.1758 22.5889 
                    13.9902 22.7565 13.8535 22.9746C13.7168 23.1927 13.6484 23.4531 13.6484 23.7559H13.0674C13.0674 
                    23.3685 13.1602 23.0283 13.3457 22.7354C13.5312 22.4391 13.7868 22.208 14.1123 22.042C14.4378 
                    21.876 14.804 21.793 15.2109 21.793C15.6309 21.793 15.9954 21.8695 16.3047 22.0225C16.6172 22.1755 
                    16.8581 22.3984 17.0273 22.6914C17.1999 22.9844 17.2861 23.3408 17.2861 23.7607C17.2861 23.9756 
                    17.2389 24.1872 17.1445 24.3955C17.0534 24.6038 16.9167 24.7926 16.7344 24.9619C16.5521 25.1279 
                    16.3258 25.2614 16.0557 25.3623C15.7855 25.4632 15.4714 25.5137 15.1133 
                    25.5137H14.4492V25.123ZM14.4492 25.626V25.2402H15.1133C15.5202 25.2402 15.8685 25.2874 16.1582 
                    25.3818C16.4479 25.473 16.6855 25.6032 16.8711 25.7725C17.0599 25.9385 17.1982 26.1322 17.2861 
                    26.3535C17.374 26.5749 17.418 26.8125 17.418 27.0664C17.418 27.3919 17.3643 27.68 17.2568 
                    27.9307C17.1494 28.1813 16.9964 28.3945 16.7979 28.5703C16.6025 28.7428 16.3714 28.8747 16.1045 
                    28.9658C15.8408 29.0537 15.5511 29.0977 15.2354 29.0977C14.9489 29.0977 14.6689 29.0553 14.3955 
                    28.9707C14.1253 28.8861 13.8812 28.7607 13.6631 28.5947C13.4482 28.4255 13.2773 28.2139 13.1504 
                    27.96C13.0234 27.7028 12.96 27.4066 12.96 27.0713H13.541C13.541 27.3708 13.6126 27.6361 13.7559 
                    27.8672C13.8991 28.0951 14.0977 28.2741 14.3516 28.4043C14.6087 28.5312 14.9033 28.5947 15.2354 
                    28.5947C15.5706 28.5947 15.8571 28.5378 16.0947 28.4238C16.3356 28.3066 16.5195 28.1357 16.6465 
                    27.9111C16.7734 27.6865 16.8369 27.4115 16.8369 27.0859C16.8369 26.7441 16.7604 26.4658 16.6074 
                    26.251C16.4577 26.0361 16.2477 25.8783 15.9775 25.7773C15.7106 25.6764 15.3997 25.626 15.0449 
                    25.626H14.4492Z" stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                />
                <path d="M8.21289 20.252C8.21289 20.0566 8.18034 19.8825 8.11523 19.7295C8.05013 19.5765 7.94271 
                    19.4398 7.79297 19.3193C7.64648 19.1956 7.44792 19.0817 7.19726 18.9775C6.94987 18.8701 6.64062 
                    18.7643 6.26953 18.6602C5.91471 18.5592 5.59407 18.4486 5.30762 18.3281C5.02116 18.2077 4.77539 
                    18.0693 4.57031 17.9131C4.36849 17.7568 4.21387 17.5745 4.10644 17.3662C3.99902 17.1546 3.94531 
                    16.9089 3.94531 16.6289C3.94531 16.3555 4.00391 16.1064 4.12109 15.8818C4.23828 15.6572 4.40267 
                    15.4635 4.61426 15.3008C4.8291 15.138 5.08138 15.0127 5.37109 14.9248C5.66406 14.8369 5.9847 14.793 
                    6.33301 14.793C6.83105 14.793 7.26074 14.8874 7.62207 15.0762C7.98665 15.265 8.26823 15.5205 8.4668 
                    15.8428C8.66862 16.1618 8.76953 16.5199 8.76953 16.917H8.16894C8.16894 16.6077 8.0957 16.3327 
                    7.94922 16.0918C7.80599 15.8477 7.59766 15.6572 7.32422 15.5205C7.05404 15.3805 6.72363 15.3105 
                    6.33301 15.3105C5.94238 15.3105 5.61361 15.3708 5.34668 15.4912C5.08301 15.6084 4.88281 15.7663 
                    4.74609 15.9648C4.61263 16.1602 4.5459 16.3783 4.5459 16.6191C4.5459 16.7884 4.57845 16.9447 
                    4.64355 17.0879C4.70866 17.2279 4.81445 17.3597 4.96094 17.4834C5.11068 17.6038 5.30762 17.7178 
                    5.55176 17.8252C5.7959 17.9294 6.097 18.0286 6.45508 18.123C6.83268 18.2272 7.16797 18.3428 7.46094 
                    18.4697C7.75391 18.5967 8.0013 18.7432 8.20312 18.9092C8.40495 19.0719 8.55794 19.2624 8.66211 
                    19.4805C8.76628 19.6986 8.81836 19.9525 8.81836 20.2422C8.81836 20.5319 8.75651 20.7923 8.63281 
                    21.0234C8.51237 21.2513 8.34147 21.445 8.12012 21.6045C7.90202 21.764 7.64323 21.8861 7.34375 
                    21.9707C7.04753 22.0553 6.72363 22.0977 6.37207 22.0977C6.05631 22.0977 5.74219 22.057 5.42969 
                    21.9756C5.11719 21.8942 4.83073 21.7689 4.57031 21.5996C4.31315 21.4271 4.10644 21.2057 3.95019 
                    20.9355C3.7972 20.6654 3.7207 20.3431 3.7207 19.9688H4.31641C4.31641 20.2682 4.37663 20.5205 
                    4.49707 20.7256C4.61751 20.9307 4.77702 21.0967 4.97558 21.2236C5.17741 21.3506 5.39876 21.4434 
                    5.63965 21.502C5.88379 21.5573 6.12793 21.585 6.37207 21.585C6.74642 21.585 7.07031 21.5296 7.34375 
                    21.4189C7.62044 21.3083 7.83366 21.1536 7.9834 20.9551C8.13639 20.7533 8.21289 20.5189 8.21289 
                    20.252Z" stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                />
            </svg>

        </button>
        `;

        this.shadowRoot.querySelector(".show-hide-surface-edges-1-3-button").addEventListener("click", 
            () => this.updateSurfaceEdges13Visibility());

    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        this.updateSurfaceEdges13Visibility();
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
        const button = this.shadowRoot.querySelector(".show-hide-surface-edges-1-3-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-surface-edges-1-3-button-icon");
        if (button.classList.contains("active") == false) {
            button.classList.add("active");
            buttonIcon.classList.add("active-icon");
        }
    }

    deactivate() {
        const button = this.shadowRoot.querySelector(".show-hide-surface-edges-1-3-button");
        const buttonIcon = this.shadowRoot.querySelector(".show-hide-surface-edges-1-3-button-icon");
        if (button.classList.contains("active") == true) {
            button.classList.remove("active");
            buttonIcon.classList.remove("active-icon");
        }
    }

    updateSurfaceEdges13Visibility() {
        if (this.state.isSurfaceEdges13Visible == true) {
            this.state.isSurfaceEdges13Visible = false;
            this.activate();
        } else {
            this.state.isSurfaceEdges13Visible = true;
            this.deactivate();
        }

        this.dispatchEvent(new CustomEvent(UPDATE_SURFACE_EDGES_1_3_VISIBILITY_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: { "is_surface_edges_1_3_visible": this.state.isSurfaceEdges13Visible },
        }));    
    }
}

export default FeaAppShowHideSurfaceEdges13Button;
