import 
{ 
    AUTO_FIT_MESSAGE_HEADER, CHANGE_VIEW_MESSAGE_HEADER, SAVE_PREPROCESSOR_DATA_MESSAGE_HEADER,
    SAVE_POSTPROCESSOR_DATA_MESSAGE_HEADER, 
} from "../../consts/tool_bar_consts.js";


class FeaAppToolBar extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,
            isPreprocessorActive: null,
            isPostprocessorActive: null,
            jobName: null,
        };

        this.state = { };

        this.attachShadow({ mode: "open" });

        this.shadowRoot.innerHTML = 
        /*html*/
        `
        <style>
            :host {
                display: block;
            }

            .wrapper {
                padding-top: 0.5rem;
                background: var(--toolbar-bg-color);
                border-left: 0.1rem solid var(--toolbar-border-color);
                border-right: 0.1rem solid var(--toolbar-border-color);
                border-bottom: 0.1rem solid var(--toolbar-border-color);
            }

            .open-save-menu-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .open-save-menu-button :hover {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .open-save-menu-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .save-menu {
                display: none;
                position: fixed;
                inset: 0;
                z-index: 20;
                background-color: rgba(0.0, 0.0, 0.0, 0.75);
                padding: 3rem;
                overflow: auto;
            }

            .save-menu.open {
                display: block;
            }

            .save-menu-body {
                margin: 0;
                padding: 0;
                display: flex;
                flex-direction: column;
                align-items: center;
                width: 12rem;
                background: var(--menubar-bg-color);
            }

            .file-name-caption {
                color: var(--preprocessor-menu-buttons-button-icon-caption-color);
                margin-top: 0.25rem;
                margin-bottom: 0.25rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
                width: 5rem;
            }

            .file-name {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
                width: 10rem;
                background-color: var(--toolbar-bg-color);
                border: var(--preprocessor-menu-content-caption-border-color);
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                outline: none;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .save-menu-buttons {
                margin-top: 0.25rem;
                margin-bottom: 0.25rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0;
                display: flex;
                flex-direction: row;
                width: 8.5rem;
                justify-content: space-between;
            }

            .save-button {
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 4rem;
                height: 1.7rem;
            }

            .save-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .close-save-menu-button {
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 4rem;
                height: 1.7rem;
            }

            .close-save-menu-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .highlighted {
                box-shadow: 0rem 0.1rem 0rem var(--preprocessor-menu-content-active-buttons-hover-color);
            }

            .x-y-view-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .x-y-view-button :hover {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .z-y-view-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .z-y-view-button :hover {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .x-z-view-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .x-z-view-button :hover {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .view-button-icon {
                width: 1.7rem;
                height: 1.7rem;
                padding: 0.25rem;
            }

            .isometric-view-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .isometric-view-button :hover {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .isometric-view-button-icon {
                width: 2.1rem;
                height: 2.1rem;
            }

            .auto-fit-button {
                background: var(--toolbar-button-bg-color);
                border: var(--toolbar-button-bg-color);
            }

            .auto-fit-button :hover {
                background: var(--toolbar-button-hover-bg-color);
                border: var(--toolbar-button-hover-bg-color);
            }

            .tool-bars {
                display: flex;
                list-style-type: none;
                padding: 0rem;
                margin: 0rem;
            }
        </style>
        <div class="wrapper">
            <nav>
                <ul class="tool-bars">
                    <li>
                        <button class="open-save-menu-button">
                            <svg class="open-save-menu-button-icon" width="22" height="25" viewBox="0 0 22 25" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Save</title>
                                <path d="M1 4.01V23.99C1 23.9955 1.00448 24 1.01 24H20.99C20.9955 
                                    24 21 23.9955 21 23.99V8.50414C21 8.50149 20.9989 8.49895 20.9971 
                                    8.49707L19 6.5L16.5029 4.00293C16.5011 4.00105 16.4985 4 16.4959 
                                    4H1.01C1.00448 4 1 4.00448 1 4.01Z" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                                />
                                <rect x="6" y="14" width="10" height="10" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                                />
                                <rect x="6" y="4" width="10" height="5" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                                />
                                <rect x="8" y="16" width="2" height="5" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                                />
                                <path d="M17 1C15 1.5 11 2 11 7.49999" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-2)"
                                />
                                <line x1="8.35355" y1="4.64645" x2="11.3536" y2="7.64645" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-2)"
                                />
                                <line x1="10.6464" y1="7.64645" x2="13.6464" y2="4.64645" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-2)"
                                />
                            </svg>
                        </button>

                        <div id="save-menu" class="save-menu">
                            <div class="save-menu-body">
                                <p class="file-name-caption">File name</p>
                                <input class="file-name" type="text"/>
                                <div class="save-menu-buttons">
                                    <button class="save-button">Save</button>
                                    <button class="close-save-menu-button">Cancel</button>
                                </div>
                            </div>
                        </div>

                    </li>
                    <li>
                        <slot name="load-button"></slot>
                    </li>
                    <li>
                        <slot name="reset-button"></slot>
                    </li>
                    <li>
                        <slot name="undo-button"></slot>
                    </li>
                    <li>
                        <slot name="redo-button"></slot>
                    </li>
                    <li>
                        <slot name="show-hide-point-button"></slot>
                    </li>
                    <li>
                        <slot name="show-hide-line-button"></slot>
                    </li>
                    <li>
                        <slot name="show-hide-surface-button"></slot>
                    </li>
                    <li>
                        <slot name="show-hide-beam-section-orientation-button"></slot>
                    </li>
                    <li>
                        <slot name="show-hide-surface-edges-1-3-button"></slot>
                    </li>
                    <li>
                        <slot name="show-hide-surface-edges-2-4-button"></slot>
                    </li>
                    <li>
                        <slot name="show-hide-surface-normal-button"></slot>
                    </li>
                    <li>
                        <slot name="show-hide-load-button"></slot>
                    </li>
                    <li>
                        <slot name="show-hide-boundary-condition-button"></slot>
                    </li>
                    <li>
                        <slot name="show-hide-mesh-seed-button"></slot>
                    </li>
                    <li>
                        <slot name="show-hide-node-button"></slot>
                    </li>
                    <li>
                        <slot name="show-hide-truss-element-button"></slot>
                    </li>
                    <li>
                        <slot name="show-hide-beam-element-button"></slot>
                    </li>
                    <li>
                        <slot name="show-hide-plate-element-button"></slot>
                    </li>
                    <li>
                        <slot name="show-hide-local-axes-button"></slot>
                    </li>
                    <li>
                        <slot name="show-hide-mesh-button"></slot>
                    </li>
                    <li>
                        <button class="auto-fit-button">

                            <svg class="view-button-icon" width="38" height="38" viewBox="0 0 38 38" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Auto-fit</title>
                                <rect x="1" y="1" width="36" height="36" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)"
                                />
                                <path d="M34.6522 35.6522C35.2045 35.6522 35.6522 35.2045 35.6522 
                                    34.6522V25.6522C35.6522 25.0999 35.2045 24.6522 34.6522 24.6522C34.0999 24.6522 
                                    33.6522 25.0999 33.6522 25.6522V33.6522H25.6522C25.0999 33.6522 24.6522 34.0999 
                                    24.6522 34.6522C24.6522 35.2045 25.0999 35.6522 25.6522 35.6522H34.6522ZM23.7711 
                                    25.1854L33.9451 35.3593L35.3593 33.9451L25.1854 23.7712L23.7711 25.1854Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                                />
                                <path d="M35.6522 3.34782C35.6522 2.79553 35.2045 2.34782 34.6522 
                                    2.34782H25.6522C25.0999 2.34782 24.6522 2.79553 24.6522 3.34782C24.6522 3.9001 
                                    25.0999 4.34782 25.6522 4.34782H33.6522V12.3478C33.6522 12.9001 34.0999 13.3478 
                                    34.6522 13.3478C35.2045 13.3478 35.6522 12.9001 35.6522 12.3478V3.34782ZM25.1854 
                                    14.2288L35.3593 4.05492L33.9451 2.64071L23.7711 12.8146L25.1854 14.2288Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                                />
                                <path d="M3.34783 2.34782C2.79555 2.34782 2.34783 2.79553 2.34783 
                                    3.34782V12.3478C2.34783 12.9001 2.79555 13.3478 3.34783 13.3478C3.90012 13.3478 
                                    4.34783 12.9001 4.34783 12.3478V4.34782H12.3478C12.9001 4.34782 13.3478 3.9001 
                                    13.3478 3.34782C13.3478 2.79553 12.9001 2.34782 12.3478 2.34782H3.34783ZM14.2289 
                                    12.8146L4.05494 2.64071L2.64072 4.05492L12.8146 14.2288L14.2289 12.8146Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                                />
                                <path d="M2.34783 34.6522C2.34783 35.2045 2.79555 35.6522 3.34783 
                                    35.6522H12.3478C12.9001 35.6522 13.3478 35.2045 13.3478 34.6522C13.3478 34.0999 
                                    12.9001 33.6522 12.3478 33.6522H4.34783V25.6522C4.34783 25.0999 3.90012 24.6522 
                                    3.34783 24.6522C2.79555 24.6522 2.34783 25.0999 2.34783 25.6522V34.6522ZM12.8146 
                                    23.7712L2.64072 33.9451L4.05494 35.3593L14.2289 25.1854L12.8146 23.7712Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                                />
                            </svg>
                        </button>
                    </li>
                    <li>
                        <button class="x-y-view-button">
                            <svg class="view-button-icon" width="67" height="64" viewBox="0 0 67 64" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Plane-XY</title>
                                <line x1="12" y1="54.5" x2="42" y2="54.5" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-4)" 
                                    stroke-width="5"
                                />
                                <path d="M9.49999 10L16 27L2.99999 27L9.49999 10Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-5)" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-5)" 
                                    stroke-width="3"
                                />
                                <path d="M59 54.5L42 61V48L59 54.5Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-4)" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-4)" 
                                    stroke-width="3"
                                />
                                <line x1="9.85602" y1="56.7927" x2="9.85602" y2="26.7927" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-5)" 
                                    stroke-width="5"
                                />
                                <path d="M60.2891 36.2305L63.6973 30.7812H65.9043L61.4121 37.832L66.0117 
                                    45H63.7852L60.2891 39.4531L56.7734 45H54.5566L59.166 37.832L54.6641 
                                    30.7812H56.8613L60.2891 36.2305Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                                />
                                <path d="M26.9863 10.9199L30.6973 3.78125H32.8262L27.9238 
                                    12.6973V18H26.0488V12.6973L21.1465 3.78125H23.2949L26.9863 10.9199Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                                />
                            </svg>
                        </button>
                    </li>
                    <li>
                        <button class="z-y-view-button">
                        <svg class="view-button-icon" width="66" height="64" viewBox="0 0 66 64" fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <title>Plane-ZY</title>
                            <line x1="12" y1="54.5" x2="42" y2="54.5" 
                                stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                                stroke-width="5"
                            />
                            <path d="M9.49999 10L16 27L2.99999 27L9.49999 10Z" 
                                fill="var(--toolbar-menu-buttons-button-icon-content-color-5)" 
                                stroke="var(--toolbar-menu-buttons-button-icon-content-color-5)" 
                                stroke-width="3"
                            />
                            <path d="M59 54.5L42 61V48L59 54.5Z" 
                                fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                                stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                                stroke-width="3"
                            />
                            <line x1="9.85602" y1="56.7927" x2="9.85602" y2="26.7927" 
                                stroke="var(--toolbar-menu-buttons-button-icon-content-color-5)" 
                                stroke-width="5"
                            />
                            <path d="M57.0566 43.4668H65.1914V45H54.8398V43.5938L62.6328 
                                32.3242H54.9668V30.7812H64.8789V32.1582L57.0566 43.4668Z" 
                                fill="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                            />
                            <path d="M26.9863 10.9199L30.6973 3.78125H32.8262L27.9238 
                                12.6973V18H26.0488V12.6973L21.1465 3.78125H23.2949L26.9863 10.9199Z" 
                                fill="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                            >
                        </svg>
                        </button>
                    </li>
                    <li>
                        <button class="x-z-view-button">
                            <svg class="view-button-icon" width="67" height="64" viewBox="0 0 67 64" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Plane-XZ</title>
                                <line x1="12" y1="54.5" x2="42" y2="54.5" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-4)" 
                                    stroke-width="5"
                                />
                                <path d="M9.49999 10L16 27L2.99999 27L9.49999 10Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                                    stroke-width="3"
                                />
                                <path d="M59 54.5L42 61V48L59 54.5Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-4)" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-4)" 
                                    stroke-width="3"
                                />
                                <line x1="9.85602" y1="56.7927" x2="9.85602" y2="26.7927" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                                    stroke-width="5"
                                />
                                <path d="M60.2891 36.2305L63.6973 30.7812H65.9043L61.4121 37.832L66.0117 
                                    45H63.7852L60.2891 39.4531L56.7734 45H54.5566L59.166 37.832L54.6641 
                                    30.7812H56.8613L60.2891 36.2305Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                                />
                                <path d="M24.0566 16.4668H32.1914V18H21.8398V16.5938L29.6328 
                                    5.32422H21.9668V3.78125H31.8789V5.1582L24.0566 16.4668Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                                />
                            </svg>
                        </button>
                    </li>
                    <li>
                        <button class="isometric-view-button">
                            <svg class="isometric-view-button-icon" width="101" height="83" viewBox="0 0 101 83" fill="none" 
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <title>Isometric</title>
                                <line x1="51.0761" y1="52.6001" x2="81.7653" y2="67.2347" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-4)" 
                                    stroke-width="5"
                                />
                                <path d="M49.5 8L56 25L43 25L49.5 8Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-5)" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-5)" 
                                    stroke-width="3"
                                />
                                <path d="M92.5968 72.3999L74.4544 70.9497L80.05 59.2156L92.5968 72.3999Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-4)" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-4)" 
                                    stroke-width="3"
                                />
                                <line y1="-2.5" x2="34" y2="-2.5" transform="matrix(-0.902623 0.430431 0.430431 
                                    0.902623 50.3946 54.8566)" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                                    stroke-width="5"
                                />
                                <path d="M7.7978 72.3999L25.9402 70.9497L20.3446 59.2156L7.7978 72.3999Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-3)" 
                                    stroke-width="3"
                                />
                                <line x1="49.856" y1="53.7927" x2="49.856" y2="19.7927" 
                                    stroke="var(--toolbar-menu-buttons-button-icon-content-color-5)" 
                                    stroke-width="5"
                                />
                                <path d="M90.5469 44.4766L94.6367 37.9375H97.2852L91.8945 46.3984L97.4141 
                                    55H94.7422L90.5469 48.3438L86.3281 55H83.668L89.1992 46.3984L83.7969 
                                    37.9375H86.4336L90.5469 44.4766Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                                />
                                <path d="M6.66797 54.1602H16.4297V56H4.00781V54.3125L13.3594 
                                    40.7891H4.16016V38.9375H16.0547V40.5898L6.66797 54.1602Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                                />
                                <path d="M69.1836 13.5039L73.6367 4.9375H76.1914L70.3086 
                                    15.6367V22H68.0586V15.6367L62.1758 4.9375H64.7539L69.1836 13.5039Z" 
                                    fill="var(--toolbar-menu-buttons-button-icon-content-color-1)"
                                />
                            </svg>
                        </button>
                    </li>
                </ul>
            </nav>
        </div>
        `;

        this.shadowRoot.querySelector(".open-save-menu-button").addEventListener("click", () => this.openSaveMenu());
        this.shadowRoot.querySelector(".close-save-menu-button").addEventListener(
            "click", () => this.closeSaveMenu(),
        );
        this.shadowRoot.querySelector(".save-button").addEventListener("click", () => this.save());

        this.shadowRoot.querySelector(".x-y-view-button").addEventListener("click", () => this.changeView("planeXY"));
        this.shadowRoot.querySelector(".z-y-view-button").addEventListener("click", () => this.changeView("planeZY"));
        this.shadowRoot.querySelector(".x-z-view-button").addEventListener("click", () => this.changeView("planeXZ"));
        this.shadowRoot.querySelector(".isometric-view-button").addEventListener("click", () => this.changeView("isometric"));
        this.shadowRoot.querySelector(".auto-fit-button").addEventListener("click", () => this.autoFit());
    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
    }

    disconnectedCallback() {
    }
    
    static get observedAttributes() {
        return ["is-preprocessor-active", "is-postprocessor-active", "action-id", "job-name"];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "action-id") {
            this.props.actionId = newValue;
            const feaAppUndoButton = this.querySelector("fea-app-undo-button");
            if (feaAppUndoButton !== null) {
                feaAppUndoButton.setAttribute("action-id", newValue);
            }

            const feaAppRedoButton = this.querySelector("fea-app-redo-button");
            if (feaAppRedoButton !== null) {
                feaAppRedoButton.setAttribute("action-id", newValue);
            }
        }

        if (name === "is-preprocessor-active") {
            this.props.isPreprocessorActive = newValue;
            if (newValue === "true") {
                if (this.querySelector("fea-app-show-hide-node-button") !== null) {
                    this.querySelector("fea-app-show-hide-node-button").remove();
                }

                if (this.querySelector("fea-app-show-hide-truss-element-button") !== null) {
                    this.querySelector("fea-app-show-hide-truss-element-button").remove();
                }

                if (this.querySelector("fea-app-show-hide-beam-element-button") !== null) {
                    this.querySelector("fea-app-show-hide-beam-element-button").remove();
                }

                if (this.querySelector("fea-app-show-hide-plate-element-button") !== null) {
                    this.querySelector("fea-app-show-hide-plate-element-button").remove();
                }

                if (this.querySelector("fea-app-show-hide-local-axes-button") !== null) {
                    this.querySelector("fea-app-show-hide-local-axes-button").remove();
                }

                const feaAppLoadButton = document.createElement("fea-app-load-button");
                feaAppLoadButton.setAttribute("slot", "load-button");
                if (this.props.isPreprocessorActive !== null) {
                    feaAppLoadButton.setAttribute("is-preprocessor-active", this.props.isPreprocessorActive);
                }
                this.append(feaAppLoadButton);

                const feaAppResetButton = document.createElement("fea-app-reset-button");
                feaAppResetButton.setAttribute("slot", "reset-button");
                if (this.props.isPreprocessorActive !== null) {
                    feaAppResetButton.setAttribute("is-preprocessor-active", this.props.isPreprocessorActive);
                }
                this.append(feaAppResetButton);

                const feaAppUndoButton = document.createElement("fea-app-undo-button");
                feaAppUndoButton.setAttribute("slot", "undo-button");
                if (this.props.actionId !== null) {
                    feaAppUndoButton.setAttribute("action-id", this.props.actionId);
                }
                this.append(feaAppUndoButton);

                const feaAppRedoButton = document.createElement("fea-app-redo-button");
                feaAppRedoButton.setAttribute("slot", "redo-button");
                if (this.props.actionId !== null) {
                    feaAppRedoButton.setAttribute("action-id", this.props.actionId);
                }
                this.append(feaAppRedoButton);

                const feaShowHidePointButton = document.createElement("fea-app-show-hide-point-button");
                feaShowHidePointButton.setAttribute("slot", "show-hide-point-button");
                this.append(feaShowHidePointButton);

                const feaShowHideLineButton = document.createElement("fea-app-show-hide-line-button");
                feaShowHideLineButton.setAttribute("slot", "show-hide-line-button");
                this.append(feaShowHideLineButton);

                const feaShowHideSurfaceButton = document.createElement("fea-app-show-hide-surface-button");
                feaShowHideSurfaceButton.setAttribute("slot", "show-hide-surface-button");
                this.append(feaShowHideSurfaceButton);

                const feaShowHideSurfaceEdges13Button = document.createElement("fea-app-show-hide-surface-edges-1-3-button");
                feaShowHideSurfaceEdges13Button.setAttribute("slot", "show-hide-surface-edges-1-3-button");
                this.append(feaShowHideSurfaceEdges13Button);

                const feaShowHideSurfaceEdges24Button = document.createElement("fea-app-show-hide-surface-edges-2-4-button");
                feaShowHideSurfaceEdges24Button.setAttribute("slot", "show-hide-surface-edges-2-4-button");
                this.append(feaShowHideSurfaceEdges24Button);

                const feaShowHideSurfaceNormalButton = document.createElement("fea-app-show-hide-surface-normal-button");
                feaShowHideSurfaceNormalButton.setAttribute("slot", "show-hide-surface-normal-button");
                this.append(feaShowHideSurfaceNormalButton);

                const feaShowHideBeamSectionOrientationButton = 
                    document.createElement("fea-app-show-hide-beam-section-orientation-button");
                feaShowHideBeamSectionOrientationButton.setAttribute("slot", 
                    "show-hide-beam-section-orientation-button");
                this.append(feaShowHideBeamSectionOrientationButton);

                const feaShowHideLoadButton = document.createElement("fea-app-show-hide-load-button");
                feaShowHideLoadButton.setAttribute("slot", "show-hide-load-button");
                this.append(feaShowHideLoadButton);

                const feaShowHideBoundaryConditionButton = 
                    document.createElement("fea-app-show-hide-boundary-condition-button");
                feaShowHideBoundaryConditionButton.setAttribute("slot", "show-hide-boundary-condition-button");
                this.append(feaShowHideBoundaryConditionButton);

                const feaShowHideMeshSeedButton = document.createElement("fea-app-show-hide-mesh-seed-button");
                feaShowHideMeshSeedButton.setAttribute("slot", "show-hide-mesh-seed-button");
                this.append(feaShowHideMeshSeedButton);
            }
        }

        if (name === "is-postprocessor-active") {
            this.props.isPostprocessorActive = newValue;
            if (newValue === "true") {
                if (this.querySelector("fea-app-load-button") !== null) {
                    this.querySelector("fea-app-load-button").remove();
                }

                if (this.querySelector("fea-app-reset-button") !== null) {
                    this.querySelector("fea-app-reset-button").remove();
                }

                if (this.querySelector("fea-app-undo-button") !== null) {
                    this.querySelector("fea-app-undo-button").remove();
                }

                if (this.querySelector("fea-app-redo-button") !== null) {
                    this.querySelector("fea-app-redo-button").remove();
                }

                if (this.querySelector("fea-app-show-hide-point-button") !== null) {
                    this.querySelector("fea-app-show-hide-point-button").remove();
                }

                if (this.querySelector("fea-app-show-hide-line-button") !== null) {
                    this.querySelector("fea-app-show-hide-line-button").remove();
                }

                if (this.querySelector("fea-app-show-hide-surface-button") !== null) {
                    this.querySelector("fea-app-show-hide-surface-button").remove();
                }

                if (this.querySelector("fea-app-show-hide-surface-edges-1-3-button") !== null) {
                    this.querySelector("fea-app-show-hide-surface-edges-1-3-button").remove();
                }

                if (this.querySelector("fea-app-show-hide-surface-edges-2-4-button") !== null) {
                    this.querySelector("fea-app-show-hide-surface-edges-2-4-button").remove();
                }

                if (this.querySelector("fea-app-show-hide-surface-normal-button") !== null) {
                    this.querySelector("fea-app-show-hide-surface-normal-button").remove();
                }

                if (this.querySelector("fea-app-show-hide-beam-section-orientation-button") !== null) {
                    this.querySelector("fea-app-show-hide-beam-section-orientation-button").remove();
                }

                if (this.querySelector("fea-app-show-hide-load-button") !== null) {
                    this.querySelector("fea-app-show-hide-load-button").remove();
                }

                if (this.querySelector("fea-app-show-hide-boundary-condition-button") !== null) {
                    this.querySelector("fea-app-show-hide-boundary-condition-button").remove();
                }

                if (this.querySelector("fea-app-show-hide-mesh-seed-button") !== null) {
                    this.querySelector("fea-app-show-hide-mesh-seed-button").remove();
                }

                const feaShowHideNodeButton = document.createElement("fea-app-show-hide-node-button");
                feaShowHideNodeButton.setAttribute("slot", "show-hide-node-button");
                this.append(feaShowHideNodeButton);

                const feaShowHideTrussElementButton = document.createElement("fea-app-show-hide-truss-element-button");
                feaShowHideTrussElementButton.setAttribute("slot", "show-hide-truss-element-button");
                this.append(feaShowHideTrussElementButton);

                const feaShowHideBeamElementButton = document.createElement("fea-app-show-hide-beam-element-button");
                feaShowHideBeamElementButton.setAttribute("slot", "show-hide-beam-element-button");
                this.append(feaShowHideBeamElementButton);

                const feaShowHidePlateElementButton = document.createElement("fea-app-show-hide-plate-element-button");
                feaShowHidePlateElementButton.setAttribute("slot", "show-hide-plate-element-button");
                this.append(feaShowHidePlateElementButton);

                const feaShowHideLocalAxesButton = document.createElement("fea-app-show-hide-local-axes-button");
                feaShowHideLocalAxesButton.setAttribute("slot", "show-hide-local-axes-button");
                this.append(feaShowHideLocalAxesButton);
            }
        }

        if (name === "job-name") {
            if (newValue !== "null") {
                this.props.jobName = newValue;
            }
        }
    }

    adoptedCallback() {
    }

    changeView(viewName) {
        this.dispatchEvent(new CustomEvent(CHANGE_VIEW_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: { "selectedView": viewName },
        }));   
    }

    autoFit() {
        this.dispatchEvent(new CustomEvent(AUTO_FIT_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
        }));   
    }

    openSaveMenu() {
        this.shadowRoot.getElementById("save-menu").classList.add("open");
    }
    
    closeSaveMenu() {
        this.shadowRoot.querySelector('.file-name').value = "";
        if (this.shadowRoot.querySelector('.file-name').classList.contains("highlighted") === true) {
            this.shadowRoot.querySelector('.file-name').classList.remove("highlighted");
        }
        this.shadowRoot.querySelector('.save-menu.open').classList.remove('open');
    }

    save() {
        const fileNameInput = this.shadowRoot.querySelector('.file-name');
        if (fileNameInput.value === "") {
            if (fileNameInput.classList.contains("highlighted") === false) {
                fileNameInput.classList.add("highlighted");
            }
        } else {
            if (fileNameInput.classList.contains("highlighted") === true) {
                fileNameInput.classList.remove("highlighted");
            }

            if (this.props.isPreprocessorActive === "true") {
                this.dispatchEvent(new CustomEvent(SAVE_PREPROCESSOR_DATA_MESSAGE_HEADER, {
                    bubbles: true,
                    composed: true,
                    detail: { file_name: fileNameInput.value }
                }));   
            } 
            if (this.props.isPostprocessorActive === "true") {
                this.dispatchEvent(new CustomEvent(SAVE_POSTPROCESSOR_DATA_MESSAGE_HEADER, {
                    bubbles: true,
                    composed: true,
                    detail: { file_name: fileNameInput.value, job_name: this.props.jobName },
                }));   
            } 

            this.closeSaveMenu();
        }
    }
}

export default FeaAppToolBar;
