import { 
    GET_RENDERER_LOADING_STATUS_MESSAGE_HEADER, PLOT_DISPLACEMENTS_MESSAGE_HEADER,
} from "../../../consts/fea_app_consts.js";

class FeaContoursDisplacementMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            isRendererLoaded: false,     // load status of wasm module "renderer";
            jobName: null,
        };

        this.state = {};

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
                padding: 0rem;
                margin-top: 1rem;
                align-items: center;
            }

            .magnitude-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--postprocessor-menu-buttons-button-border-color);
                padding: 0rem;
                margin: 0rem;
            }

            .magnitude-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--postprocessor-menu-buttons-button-icon-content-color-1);
                font-size: 85%;
                width: 6rem;
            }

            .magnitude {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                width: 5rem;
                background-color: var(--postprocessor-menu-buttons-button-border-color);
                border: var(--postprocessor-menu-buttons-button-border-hover-bg-color-2);
                border-bottom: 0.1rem solid var(--postprocessor-menu-buttons-button-border-hover-bg-color-2);
                outline: none;
                color: var(--postprocessor-menu-buttons-button-icon-content-color-1);
            }

            .magnitude::-webkit-outer-spin-button,
            .magnitude::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .magnitude[type=number] {
                -moz-appearance: textfield;
            }

            .magnitude:hover {
                box-shadow: 0rem 0.15rem 0rem var(--postprocessor-menu-buttons-button-border-hover-bg-color-2);
            }

            .magnitude:focus {
                box-shadow: 0rem 0.15rem 0rem var(--postprocessor-menu-buttons-button-border-hover-bg-color-2);
            }

            .apply-cancel-buttons {
                margin-top: 1rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
            }

            .apply-button {
                background: var(--postprocessor-menu-buttons-button-bg-color-2);
                border: 0.2rem solid var(--postprocessor-menu-buttons-button-border-color);
                border-radius: 0.3rem;
                color: var(--postprocessor-menu-buttons-button-icon-content-color-1);
                padding: 0rem;
                margin: 0rem;
                width: 4rem;
                height: 1.7rem;
            }

            .apply-button:hover {
                border: 0.2rem solid var(--postprocessor-menu-buttons-button-border-hover-bg-color-2);
            }

            .analysis-info {
                display: flex;
                margin: 0rem;
                padding: 0rem;
            }

            .analysis-info-message {
                margin-top: 1rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
                color: var(--postprocessor-menu-buttons-button-icon-content-color-1);
                font-size: 80%;
                width: 12rem;
            }

            .highlighted {
                box-shadow: 0rem 0.1rem 0rem var(--postprocessor-menu-buttons-button-icon-content-color-2);
            }
        </style>

        <div class=wrapper>
            <div class="magnitude-field-content">
                <p class="magnitude-caption">Magnitude</p>
                <input class="magnitude" type="number" step="1"/>
            </div>
            
            <div class="apply-cancel-buttons">
                <button class="apply-button">Apply</button>
            </div>

            <div class="analysis-info">
                <p class="analysis-info-message"></p>
            </div>
        </div>
        `;

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.plotDisplacement());

        this.shadowRoot.querySelector(".magnitude").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".magnitude");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });
    }

    set isRendererLoaded(value) {
        this.props.isRendererLoaded = value;
    }

    set plotError(error) {
        if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = error;
        }
    }

    connectedCallback() {
        Object.keys(this.props).forEach((propName) => {
            if (this.hasOwnProperty(propName)) {
                let value = this[propName];
                delete this[propName];
                this[propName] = value;
            }
        });
        const frame = () => {
            this.getRendererLoadingStatus();
            if (this.props.isRendererLoaded === true) {
                this.defineNewMagnitude();
                clearInterval(id);
            }
        }
        const id = setInterval(frame, 10);
    }

    disconnectedCallback() {
    }

    static get observedAttributes() {
        return ["job-name"];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "job-name") {
            if (newValue !== "null") {
                this.props.jobName = newValue;
            }
        }
    }

    adoptedCallback() {
    }

    getRendererLoadingStatus() {
        this.dispatchEvent(new CustomEvent(GET_RENDERER_LOADING_STATUS_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
        }));
    }

    defineNewMagnitude() {
        this.shadowRoot.querySelector(".magnitude").value = 1.0;
    }


    plotDisplacement() {
        const newMagnitudeField = this.shadowRoot.querySelector(".magnitude");
        if (newMagnitudeField.value === "") {
            if (newMagnitudeField.classList.contains("highlighted") === false) {
                newMagnitudeField.classList.add("highlighted");
            }
        }

        if (newMagnitudeField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        if (this.isNumeric(newMagnitudeField.value) === false) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values!";
                return;
            } else {
                return;
            }
        }

        const message = { "magnitude": newMagnitudeField.value, "job_name": this.props.jobName };

        this.dispatchEvent(new CustomEvent(PLOT_DISPLACEMENTS_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
    }

    dropHighlight(highlightedElement) {
        if (highlightedElement.classList.contains("highlighted") === true) {
            highlightedElement.classList.remove("highlighted");
        }
    }

    isNumeric(str) {
        if (typeof str != "string") {
            return false;
        }
        return !isNaN(str) && !isNaN(parseFloat(str));
      }
}

export default FeaContoursDisplacementMenu;
