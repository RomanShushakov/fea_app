class FeaContoursMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            jobName: null,
        };

        this.state = {
            activeMenuName: null,
            menuNames: {
                "displacement": "fea-contours-displacement-menu",
                // "stress": "fea-contours-stress-menu",
            },

            contourResultsTypes: ["Displacement"],
            // contourResultsTypes: ["Displacement", "Stress"],
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
                background-color: var(--postprocessor-menu-buttons-button-border-color);
                padding: 1rem;
                overflow-y: auto;
                overflow-x: hidden;
                scrollbar-color: var(--renderer-menu-content-scrollbar-thumb-color) var(--preprocessor-menu-buttons-active-button-bg-color);
                scrollbar-width: thin;
            }

            ::-webkit-scrollbar {
                width: 0.5rem;
            }

            ::-webkit-scrollbar-track {
                background: var(--preprocessor-menu-buttons-active-button-bg-color);
            }

            ::-webkit-scrollbar-thumb {
                background: var(--renderer-menu-content-scrollbar-thumb-color);
            }

            ::-webkit-scrollbar-thumb:hover {
                background: var(--renderer-menu-content-scrollbar-thumb-hover-color);
            }

            .contour-results-type-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--postprocessor-menu-buttons-button-border-color);
                padding: 0rem;
                margin-top: 1rem;
                margin-bottom: 0;
                margin-left: 0;
                margin-right: 0;
                align-items: baseline;
            }

            .contour-plot-caption {
                margin: 0rem;
                padding-top: 0rem;
                padding-bottom: 0.3rem;
                padding-left: 0rem;
                padding-right: 0rem;
                color: var(--postprocessor-menu-buttons-button-icon-content-color-1);
                border-bottom: 0.1rem solid var(--postprocessor-menu-buttons-button-border-hover-bg-color-2);
                font-size: 85%;
            }

            .contour-results-type-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--postprocessor-menu-buttons-button-icon-content-color-1);
                font-size: 85%;
                width: 4.5rem;
            }

            .contour-results-type-select-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: column;
            }

            .contour-results-type {
                width: 6.5rem;
                margin-top: 0.5rem;
                background-color: var(--postprocessor-menu-buttons-button-border-color);
                border: var(--postprocessor-menu-buttons-button-border-hover-bg-color-2);
                border-bottom: 0.1rem solid var(--postprocessor-menu-buttons-button-border-hover-bg-color-2);
                outline: none;
                color: var(--postprocessor-menu-buttons-button-icon-content-color-1);
                -webkit-appearance: none;
                -moz-appearance: none;
                background: url('data:image/svg+xml,<svg width="4" height="4" viewBox="0 0 4 4" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M1 1L2 2L3 1" stroke="rgb(112, 112, 114)" stroke-width="0.5"/></svg>') right / contain no-repeat;
            }

            .contour-results-type option {
                background-color: var(--postprocessor-menu-content-dropdown-menu-bg-color);
            }

            .contour-results-type:hover {
                box-shadow: 0rem 0.15rem 0rem var(--postprocessor-menu-buttons-button-border-hover-bg-color-2);
            }
        </style>

        <div class=wrapper>
            <p class="contour-plot-caption">Contour plot</p>

            <div class="contour-results-type-field-content">
                <p class="contour-results-type-caption">Result type</p>
                <div class="contour-results-type-select-content">
                    <select class="contour-results-type"></select>
                </div>
            </div>
            <slot></slot>
        </div>
        `;

        this.shadowRoot.querySelector(".contour-results-type").addEventListener("change", () => this.defineContourResultTypeMenu());
    }

    connectedCallback() {
        this.defineContourResultType();
    }

    disconnectedCallback() {
    }

    static get observedAttributes() {
        return ["job-name"];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "job-name") {
            this.props.jobName = newValue;
            if (this.state.activeMenuName !== null) {
                this.querySelector(this.state.menuNames[this.state.activeMenuName])
                    .setAttribute("job-name", this.props.jobName);
            }
        }
    }

    adoptedCallback() {
    }

    defineContourResultType() {
        const contourResultTypeSelect = this.shadowRoot.querySelector(".contour-results-type");
        for (let i = contourResultTypeSelect.length - 1; i >= 0; i--) {
            contourResultTypeSelect.options[i] = null;
        }
        for (let i = 0; i < this.state.contourResultsTypes.length; i++) {
            let updateOption = document.createElement("option");
            updateOption.value = this.state.contourResultsTypes[i];
            updateOption.innerHTML = this.state.contourResultsTypes[i];
            contourResultTypeSelect.appendChild(updateOption);
        }
        this.defineContourResultTypeMenu();
    }

    defineContourResultTypeMenu() {
        const loadTypeSelect = this.shadowRoot.querySelector(".contour-results-type");
        switch (loadTypeSelect.value) {
            case "Displacement":
                this.deactivateMenu("stress");
                this.activateMenu("displacement");
                break;
            // case "Stress":
            //     this.deactivateMenu("displacement");
            //     this.activateMenu("stress");
            //     break;
        }
    }

    activateMenu(menuName) {
        const menu = document.createElement(this.state.menuNames[menuName]);
        this.append(menu);
        menu.setAttribute("job-name", this.props.jobName);
        this.state.activeMenuName = menuName;
    }

    deactivateMenu(menuName) {
        if (this.querySelector(this.state.menuNames[menuName]) != null) {
            this.querySelector(this.state.menuNames[menuName]).remove();
            this.state.activeMenuName = null;
        }
    }
}

export default FeaContoursMenu;
