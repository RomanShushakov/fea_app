import Store from "../../../store/fea_store.js";
import { UPDATE_MATERIAL_MESSAGE_HEADER } from "../../../consts/actions_router_consts.js";
import { CLIENT_MESSAGE_HEADER, GET_WASM_LOADING_STATUS_MESSAGE_HEADER } from "../../../consts/fea_app_consts.js";


class FeaMaterialUpdateMaterialMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,             // u32;
            isWasmLoaded: false,     // load status of wasm modules;
            store: new Store(),
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
                align-items: center;
            }

            .material-name-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .material-name-select-filter-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: column;
            }

            .material-name-filter-label {
                position: relative;
            }
                
            .material-name-filter-label:before {
                content: "";
                position: absolute;
                left: 0rem;
                top: 0rem;
                bottom: 0rem;
                width: 0.8rem;
                background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
            }

            .material-name-filter {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding-left: 1.3rem;
                width: 3.5rem;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-content-caption-border-color);
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                outline: none;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .material-name-filter::placeholder {
                font-size: 85%;
            }

            .material-name-filter:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .material-name-filter:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .material-name {
                width: 5rem;
                margin-top: 0.5rem;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-content-caption-border-color);
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                outline: none;
                color: var(--preprocessor-menu-content-caption-color);
                -webkit-appearance: none;
                -moz-appearance: none;
                background: url('data:image/svg+xml,<svg width="4" height="4" viewBox="0 0 4 4" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M1 1L2 2L3 1" stroke="rgb(112, 112, 114)" stroke-width="0.5"/></svg>') right / contain no-repeat;
            }

            .material-name option {
                background-color: var(--preprocessor-menu-content-dropdown-menu-bg-color);
            }

            .material-name:hover {
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
                <div class="material-name-select-filter-content">
                    <label class="material-name-filter-label">
                        <input class="material-name-filter" type="text" placeholder="Filter..."/>
                    </label>
                    <select class="material-name"></select>
                </div>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.updateMaterial());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelMaterialUpdate());

        this.shadowRoot.querySelector(".material-name").addEventListener("change", () => this.updateMaterialData());

        this.shadowRoot.querySelector(".material-name-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".material-name-filter").value,
                this.shadowRoot.querySelector(".material-name"));
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
                this.defineMaterialNameOptions();
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
            this.defineMaterialNameOptions();
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

    defineMaterialNameOptions() {
        const updateMaterialNameSelect = this.shadowRoot.querySelector(".material-name");
        for (let i = updateMaterialNameSelect.length - 1; i >= 0; i--) {
            updateMaterialNameSelect.options[i] = null;
        }
        if (this.props.store.materials_shelf.length > 0) {
            for (let i = 0; i < this.props.store.materials_shelf.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = this.props.store.materials_shelf[i].name.replace(/['"]+/g, "");
                updateOption.innerHTML = this.props.store.materials_shelf[i].name.replace(/['"]+/g, "");
                updateMaterialNameSelect.appendChild(updateOption);
            }
            this.shadowRoot.querySelector(".young-modulus").value = this.props.store.materials_shelf[0].young_modulus;
            this.shadowRoot.querySelector(".poisson-ratio").value = this.props.store.materials_shelf[0].poisson_ratio;
        } else {
            this.shadowRoot.querySelector(".young-modulus").value = "";
            this.shadowRoot.querySelector(".poisson-ratio").value = "";
        }
    }

    updateMaterialData() {
        const selectedMaterialName = this.shadowRoot.querySelector(".material-name").value;
        const materialInProps = this.props.store.materials_shelf.find(material => material.name == `"${selectedMaterialName}"`);
        this.shadowRoot.querySelector(".young-modulus").value = materialInProps.young_modulus;
        this.dropHighlight(this.shadowRoot.querySelector(".young-modulus"));
        this.shadowRoot.querySelector(".poisson-ratio").value = materialInProps.poisson_ratio;
        this.dropHighlight(this.shadowRoot.querySelector(".poisson-ratio"));
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    filter(keywordField, selectField) {
        for (let i = 0; i < selectField.length; i++) {
            let txt = selectField.options[i].value;
            if (txt.substring(0, keywordField.length).toLowerCase() !== keywordField.toLowerCase() && 
                keywordField.trim() !== "") {
                selectField.options[i].style.display = "none";
            } else {
                selectField.options[i].style.display = "list-item";
            }
        }
    }

    updateMaterial() {
        const selectedMaterialNameField = this.shadowRoot.querySelector(".material-name");
        if (selectedMaterialNameField.value == "") {
            if (selectedMaterialNameField.classList.contains("highlighted") === false) {
                selectedMaterialNameField.classList.add("highlighted");
            }
        }

        const inputtedYoungModulusField = this.shadowRoot.querySelector(".young-modulus");
        if (inputtedYoungModulusField.value == "") {
            if (inputtedYoungModulusField.classList.contains("highlighted") === false) {
                inputtedYoungModulusField.classList.add("highlighted");
            }
        }
        const inputtedPoissonRatio = this.shadowRoot.querySelector(".poisson-ratio");
        if (inputtedPoissonRatio.value == "") {
            if (inputtedPoissonRatio.classList.contains("highlighted") === false) {
                inputtedPoissonRatio.classList.add("highlighted");
            }
        }

        if (selectedMaterialNameField.value === "" || inputtedYoungModulusField.value === "" || 
            inputtedPoissonRatio.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        if (this.isNumeric(inputtedYoungModulusField.value) === false || this.isNumeric(inputtedPoissonRatio.value) === false) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values for Young's modulus and Poisson's ratio!";
                return;
            } else {
                return;
            }
        }

        const oldMaterialValues = this.props.store.materials_shelf.find(
            material => material.name == `"${selectedMaterialNameField.value}"`);

        // this.getActionId();

        const message = { [UPDATE_MATERIAL_MESSAGE_HEADER]: {
            action_id: this.props.actionId,
            name: selectedMaterialNameField.value, 
            old_material_values: { 
                young_modulus:  oldMaterialValues.young_modulus,
                poisson_ratio: oldMaterialValues.poisson_ratio },
            new_material_values: { 
                young_modulus: inputtedYoungModulusField.value,
                poisson_ratio: inputtedPoissonRatio.value }
        }};
        this.dispatchEvent(new CustomEvent([CLIENT_MESSAGE_HEADER], {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));

        this.shadowRoot.querySelector(".material-name-filter").value = null;
    }

    cancelMaterialUpdate() {
        if (this.props.store.materials_shelf.length > 0) {
            this.defineMaterialNameOptions();
        }
        this.shadowRoot.querySelector(".material-name-filter").value = null;
        const selectedMaterialNameForUpdateField = this.shadowRoot.querySelector(".material-name");
        this.dropHighlight(selectedMaterialNameForUpdateField);
        const inputtedYongModulusField = this.shadowRoot.querySelector(".young-modulus");
        this.dropHighlight(inputtedYongModulusField);
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

export default FeaMaterialUpdateMaterialMenu;
