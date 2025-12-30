import { 
    GET_RENDERER_LOADING_STATUS_MESSAGE_HEADER, PLOT_GLOBAL_FORCES_MESSAGE_HEADER, PLOT_GLOBAL_MOMENTS_MESSAGE_HEADER,
} from "../../../consts/fea_app_consts.js";

class FeaSymbolsGlobalLoadsMenu extends HTMLElement {
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

            .global-loads-type-radio-buttons {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: row;
            }

            .global-loads-type-force-radio-button-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
                color: var(--postprocessor-menu-buttons-button-icon-content-color-1);
                font-size: 85%;
                width: 5.5rem;
            }

            .global-loads-type-moment-radio-button-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
                color: var(--postprocessor-menu-buttons-button-icon-content-color-1);
                font-size: 85%;
                width: 5.5rem;
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
            <div class="global-loads-type-radio-buttons">
                <div class="global-loads-type-force-radio-button-content">
                    <input type="radio" id="force-global-force-type-radio" name="global-loads-type" value="force">
                    <label for="force-global-force-type-radio">Force</label>
                </div>

                <div class="global-loads-type-moment-radio-button-content">
                    <input type="radio" id="moment-global-force-type-radio" name="global-loads-type" value="moment">
                    <label for="moment-global-force-type-radio">Moment</label>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.plotGlobalForces());
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
                this.shadowRoot.getElementById("force-global-force-type-radio").checked = true;
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
                this.props.jobName = parseInt(newValue);
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

    plotGlobalForces() {
        if (this.shadowRoot.getElementById("force-global-force-type-radio").checked) {
            const message = { "job_name": this.props.jobName };

            this.dispatchEvent(new CustomEvent(PLOT_GLOBAL_FORCES_MESSAGE_HEADER, {
                bubbles: true,
                composed: true,
                detail: {
                    message: message,
                },
            }));
            return;
        }

        if (this.shadowRoot.getElementById("moment-global-force-type-radio").checked) {
            const message = { "job_name": this.props.jobName };

            this.dispatchEvent(new CustomEvent(PLOT_GLOBAL_MOMENTS_MESSAGE_HEADER, {
                bubbles: true,
                composed: true,
                detail: {
                    message: message,
                },
            }));
            return;
        }
    }
}

export default FeaSymbolsGlobalLoadsMenu;
