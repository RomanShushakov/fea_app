// import Store from "../../../store/fea_store.js";
import { ADD_MATERIAL_MESSAGE_HEADER } from "../../../consts/actions_router_consts.js";
import { CLIENT_MESSAGE_HEADER, GET_WASM_LOADING_STATUS_MESSAGE_HEADER } from "../../../consts/fea_app_consts.js";


class FeaMaterialAddMaterialMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,             // u32;
            isWasmLoaded: false,     // load status of wasm modules;
            // store: new Store(),
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
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
                align-items: center;
            }

            .material-name-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin: 0rem;
            }

            .material-name-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .material-name {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                width: 5rem;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-content-caption-border-color);
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                outline: none;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .material-name:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .material-name:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .young-modulus-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .young-modulus-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .young-modulus {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                width: 5rem;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-content-caption-border-color);
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                outline: none;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .young-modulus[type=number]::-webkit-outer-spin-button,
            .young-modulus[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .young-modulus[type=number] {
                -moz-appearance: textfield;
            }

            .young-modulus:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .young-modulus:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .poisson-ratio-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .poisson-ratio-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .poisson-ratio {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                width: 5rem;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-content-caption-border-color);
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                outline: none;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .poisson-ratio[type=number]::-webkit-outer-spin-button,
            .poisson-ratio[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .poisson-ratio[type=number] {
                -moz-appearance: textfield;
            }

            .poisson-ratio:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .poisson-ratio:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .apply-cancel-buttons {
                margin-top: 1rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
            }

            .apply-button {
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 4rem;
                height: 1.7rem;
            }

            .apply-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .cancel-button {
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 4rem;
                height: 1.7rem;
            }

            .cancel-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
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
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 80%;
                width: 12rem;
            }

            .highlighted {
                box-shadow: 0rem 0.1rem 0rem var(--preprocessor-menu-content-active-buttons-hover-color);
            }
        </style>

        <div class=wrapper>
            <div class="material-name-field-content">
                <p class="material-name-caption">Material name</p>
                <input class="material-name" type="text"/>
            </div>

            <div class="young-modulus-field-content">
                <p class="young-modulus-caption">Young's modulus</p>
                <input class="young-modulus" type="number"/>
            </div>

            <div class="poisson-ratio-field-content">
                <p class="poisson-ratio-caption">Poisson's ratio</p>
                <input class="poisson-ratio" type="number"/>
            </div>
            
            <div class="apply-cancel-buttons">
                <button class="apply-button">Apply</button>
                <button class="cancel-button">Cancel</button>
            </div>

            <div class="analysis-info">
                <p class="analysis-info-message"></p>
            </div>
        </div>
        `;

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.addMaterial());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelMaterialAddition());

        this.shadowRoot.querySelector(".material-name").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".material-name");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".young-modulus").addEventListener("click", () => {
            const inputtedYoungModulus = this.shadowRoot.querySelector(".young-modulus");
            this.dropHighlight(inputtedYoungModulus);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".poisson-ratio").addEventListener("click", () => {
            const inputtedPoissonRatio = this.shadowRoot.querySelector(".poisson-ratio");
            this.dropHighlight(inputtedPoissonRatio);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });
    }

    set isWasmLoaded(value) {
        this.props.isWasmLoaded = value;
    }

    set feModelError(error) {
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
            this.getWasmLoadingStatus();
            if (this.props.isWasmLoaded === true) {
                clearInterval(id);
                this.defineNewMaterialName();
            }
        }
        const id = setInterval(frame, 10);
    }

    disconnectedCallback() {
    }

    static get observedAttributes() {
        return ["action-id"];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "action-id") {
            this.props.actionId = parseInt(newValue);
            this.defineNewMaterialName();
        }
    }

    adoptedCallback() {
    }

    getWasmLoadingStatus() {
        this.dispatchEvent(new CustomEvent(GET_WASM_LOADING_STATUS_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
        }));
    }

    defineNewMaterialName() {
        const newMaterialName = "mat1";
        this.shadowRoot.querySelector(".material-name").value = newMaterialName;
        this.shadowRoot.querySelector(".young-modulus").value = 10000000;
        this.shadowRoot.querySelector(".poisson-ratio").value = 0.3;
    }

    addMaterial() {
        const newMaterialNameField = this.shadowRoot.querySelector(".material-name");
        if (newMaterialNameField.value === "") {
            if (newMaterialNameField.classList.contains("highlighted") === false) {
                newMaterialNameField.classList.add("highlighted");
            }
        }
        const youngModulusField = this.shadowRoot.querySelector(".young-modulus");
        if (youngModulusField.value === "") {
            if (youngModulusField.classList.contains("highlighted") === false) {
                youngModulusField.classList.add("highlighted");
            }
        }
        const poissonRatioField = this.shadowRoot.querySelector(".poisson-ratio");
        if (poissonRatioField.value === "") {
            if (poissonRatioField.classList.contains("highlighted") === false) {
                poissonRatioField.classList.add("highlighted");
            }
        }

        if (newMaterialNameField.value === "" || youngModulusField.value === "" || 
            poissonRatioField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        if (this.isNumeric(youngModulusField.value) === false || this.isNumeric(poissonRatioField.value) === false) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers should be used as input values for Young's modulus and Poisson's ratio!";
                return;
            } else {
                return;
            }
        }

        const message = { [ADD_MATERIAL_MESSAGE_HEADER]: {
            action_id: this.props.actionId,
            name: newMaterialNameField.value, 
            young_modulus:  youngModulusField.value, poisson_ratio:  poissonRatioField.value,
        }};

        this.dispatchEvent(new CustomEvent([CLIENT_MESSAGE_HEADER], {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
    }

    cancelMaterialAddition() {
        this.defineNewMaterialName();
        const newMaterialNameField = this.shadowRoot.querySelector(".material-name");
        this.dropHighlight(newMaterialNameField);
        const inputtedYoungModulusField = this.shadowRoot.querySelector(".young-modulus");
        this.dropHighlight(inputtedYoungModulusField);
        const inputtedPoissonRatioField = this.shadowRoot.querySelector(".poisson-ratio");
        this.dropHighlight(inputtedPoissonRatioField);
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
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

export default FeaMaterialAddMaterialMenu;
