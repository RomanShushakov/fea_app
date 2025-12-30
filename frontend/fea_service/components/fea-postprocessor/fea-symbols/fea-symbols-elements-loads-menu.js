import { 
    GET_RENDERER_LOADING_STATUS_MESSAGE_HEADER, PLOT_ELEMENTS_LOADS_MESSAGE_HEADER,
} from "../../../consts/fea_app_consts.js";

class FeaSymbolsElementsLoadsMenu extends HTMLElement {
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

            .elements-loads-type-radio-buttons {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: column;
            }

            .elements-loads-type-radio-button-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
                color: var(--postprocessor-menu-buttons-button-icon-content-color-1);
                font-size: 85%;
            }

            input[type="radio"] {
                -webkit-appearance: none;
                -moz-appearance: none;
                width: 1rem;
                height: 1rem;
                border: 1px solid var(--postprocessor-menu-input-checkbox-border-color);
                border-radius: 50%;
                outline: none;
                
            }
                
            input[type="radio"]:hover {
                box-shadow: 0 0 0.25rem 0 #acb1ab inset;
            }
            
            input[type="radio"]:before {
                content: "";
                display: block;
                width: 60%;
                height: 60%;
                margin: 20% auto;    
                border-radius: 50%;    
            }
            
            input[type="radio"]:checked:before {
                background: var(--postprocessor-menu-buttons-button-bg-color-2);
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

        <div class="wrapper">
            <div class="elements-loads-type-radio-buttons">

                <div class="elements-loads-type-radio-button-content">
                    <input type="radio" id="el-force-r" name="elements-loads-type" checked value="elforce_r">
                    <label for="el-force-r">ELFORCE_R</label>
                </div>

                <div class="elements-loads-type-radio-button-content">
                    <input type="radio" id="el-force-s" name="elements-loads-type" value="elforce_s">
                    <label for="el-force-s">ELFORCE_S</label>
                </div>

                <div class="elements-loads-type-radio-button-content">
                    <input type="radio" id="el-force-t" name="elements-loads-type" value="elforce_t">
                    <label for="el-force-t">ELFORCE_T</label>
                </div>

                <div class="elements-loads-type-radio-button-content">
                    <input type="radio" id="el-moment-r" name="elements-loads-type" value="elmoment_r">
                    <label for="el-moment-r">ELMOMENT_R</label>
                </div>

                <div class="elements-loads-type-radio-button-content">
                    <input type="radio" id="el-moment-s" name="elements-loads-type" value="elmoment_s">
                    <label for="el-moment-s">ELMOMENT_S</label>
                </div>

                <div class="elements-loads-type-radio-button-content">
                    <input type="radio" id="el-moment-t" name="elements-loads-type" value="elmoment_t">
                    <label for="el-moment-t">ELMOMENT_T</label>
                </div>

                <div class="elements-loads-type-radio-button-content">
                    <input type="radio" id="el-mem-force-r" name="elements-loads-type" value="elmemforce_r">
                    <label for="el-mem-force-r">ELMEMFORCE_R</label>
                </div>

                <div class="elements-loads-type-radio-button-content">
                    <input type="radio" id="el-mem-force-s" name="elements-loads-type" value="elmemforce_s">
                    <label for="el-mem-force-s">ELMEMFORCE_S</label>
                </div>

                <div class="elements-loads-type-radio-button-content">
                    <input type="radio" id="el-mem-force-r-s" name="elements-loads-type" value="elmemforce_r_s">
                    <label for="el-mem-force-r-s">ELMEMFORCE_R_S</label>
                </div>

                <div class="elements-loads-type-radio-button-content">
                    <input type="radio" id="el-bend-moment-r" name="elements-loads-type" value="elbendmoment_r">
                    <label for="el-bend-moment-r">ELBENDMOMENT_R</label>
                </div>

                <div class="elements-loads-type-radio-button-content">
                    <input type="radio" id="el-bend-moment-s" name="elements-loads-type" value="elbendmoment_s">
                    <label for="el-bend-moment-s">ELBENDMOMENT_S</label>
                </div>

                <div class="elements-loads-type-radio-button-content">
                    <input type="radio" id="el-bend-moment-r-s" name="elements-loads-type" value="elbendmoment_r_s">
                    <label for="el-bend-moment-r-s">ELBENDMOMENT_R_S</label>
                </div>

                <div class="elements-loads-type-radio-button-content">
                    <input type="radio" id="el-shear-force-r-t" name="elements-loads-type" value="elshearforce_r_t">
                    <label for="el-shear-force-r-t">ELSHEARFORCE_R_T</label>
                </div>

                <div class="elements-loads-type-radio-button-content">
                    <input type="radio" id="el-shear-force-s-t" name="elements-loads-type" value="elshearforce_s_t">
                    <label for="el-shear-force-s-t">ELSHEARFORCE_S_T</label>
                </div>
            </div>

            <div class="apply-cancel-buttons">
                <button class="apply-button">Apply</button>
            </div>

            <div class="analysis-info">
                <p class="analysis-info-message"></p>
            </div>
        </div>
        `;

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.plotElementForces());
    }

    set isRendererLoaded(value) {
        this.props.isRendererLoaded = value;
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

    plotElementForces() {
        const elForceRRadioButton = this.shadowRoot.getElementById("el-force-r");

        const elForceSRadioButton = this.shadowRoot.getElementById("el-force-s");
        const elForceTRadioButton = this.shadowRoot.getElementById("el-force-t");
        const elMomentRRadioButton = this.shadowRoot.getElementById("el-moment-r");
        const elMomentSRadioButton = this.shadowRoot.getElementById("el-moment-s");
        const elMomentTRadioButton = this.shadowRoot.getElementById("el-moment-t");

        const elMemForceRRadioButton = this.shadowRoot.getElementById("el-mem-force-r");
        const elMemForceSRadioButton = this.shadowRoot.getElementById("el-mem-force-s");
        const elMemForceRSRadioButton = this.shadowRoot.getElementById("el-mem-force-r-s");
        const elShearForceRTRadioButton = this.shadowRoot.getElementById("el-shear-force-r-t");
        const elShearForceSTRadioButton = this.shadowRoot.getElementById("el-shear-force-s-t");
        const elBendMomentRRadioButton = this.shadowRoot.getElementById("el-bend-moment-r");
        const elBendMomentSRadioButton = this.shadowRoot.getElementById("el-bend-moment-s");
        const elBendMomentRSRadioButton = this.shadowRoot.getElementById("el-bend-moment-r-s");

        let selectedElementLoadComponent = elForceRRadioButton.value;

        if (elForceSRadioButton.checked === true) {
            selectedElementLoadComponent = elForceSRadioButton.value;
        } else if (elForceTRadioButton.checked === true) {
            selectedElementLoadComponent = elForceTRadioButton.value;
        } else if (elMomentRRadioButton.checked === true) {
            selectedElementLoadComponent = elMomentRRadioButton.value;
        } else if (elMomentSRadioButton.checked === true) {
            selectedElementLoadComponent = elMomentSRadioButton.value;
        } else if (elMomentTRadioButton.checked === true) {
            selectedElementLoadComponent = elMomentTRadioButton.value;
        } else if (elMemForceRRadioButton.checked === true) {
            selectedElementLoadComponent = elMemForceRRadioButton.value;
        } else if (elMemForceSRadioButton.checked === true) {
            selectedElementLoadComponent = elMemForceSRadioButton.value;
        } else if (elMemForceRSRadioButton.checked === true) {
            selectedElementLoadComponent = elMemForceRSRadioButton.value;
        } else if (elShearForceRTRadioButton.checked === true) {
            selectedElementLoadComponent = elShearForceRTRadioButton.value;
        } else if (elShearForceSTRadioButton.checked === true) {
            selectedElementLoadComponent = elShearForceSTRadioButton.value;
        } else if (elBendMomentRRadioButton.checked === true) {
            selectedElementLoadComponent = elBendMomentRRadioButton.value;
        } else if (elBendMomentSRadioButton.checked === true) {
            selectedElementLoadComponent = elBendMomentSRadioButton.value;
        } else if (elBendMomentRSRadioButton.checked === true) {
            selectedElementLoadComponent = elBendMomentRSRadioButton.value;
        }

        const message = {
            "selected_element_load_component": selectedElementLoadComponent,
            "job_name": this.props.jobName,
        };

        this.dispatchEvent(new CustomEvent(PLOT_ELEMENTS_LOADS_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
    }
}

export default FeaSymbolsElementsLoadsMenu;
