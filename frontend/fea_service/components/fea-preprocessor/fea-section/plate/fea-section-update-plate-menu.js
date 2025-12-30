import Store from "../../../../store/fea_store.js";
import { UPDATE_PLATE_SECTION_MESSAGE_HEADER } from "../../../../consts/actions_router_consts.js";
import { CLIENT_MESSAGE_HEADER, GET_WASM_LOADING_STATUS_MESSAGE_HEADER } from "../../../../consts/fea_app_consts.js";


class FeaSectionUpdatePlateMenu extends HTMLElement {
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

            .plate-section-name-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin: 0rem;
                align-items: center;
            }

            .plate-section-name-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .plate-section-name-select-filter-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: column;
            }

            .plate-section-name-filter-label {
                position: relative;
            }
                
            .plate-section-name-filter-label:before {
                content: "";
                position: absolute;
                left: 0rem;
                top: 0rem;
                bottom: 0rem;
                width: 0.8rem;
                background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
            }

            .plate-section-name-filter {
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

            .plate-section-name-filter::placeholder {
                font-size: 85%;
            }

            .plate-section-name-filter:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .plate-section-name-filter:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .plate-section-name {
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

            .plate-section-name option {
                background-color: var(--preprocessor-menu-content-dropdown-menu-bg-color);
            }

            .plate-section-name:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .thickness-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .thickness-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .thickness {
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

            .thickness[type=number]::-webkit-outer-spin-button,
            .thickness[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .thickness[type=number] {
                -moz-appearance: textfield;
            }

            .thickness:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .thickness:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .shear-factor-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .shear-factor-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .shear-factor {
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

            .shear-factor[type=number]::-webkit-outer-spin-button,
            .shear-factor[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .shear-factor[type=number] {
                -moz-appearance: textfield;
            }

            .shear-factor:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .shear-factor:focus {
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
                box-shadow: 0rem 0.1rem 0rem var(--preprocessor-menu-content-active-buttons-color);
            }
        </style>

        <div class=wrapper>

            <div class="plate-section-name-field-content">
                <p class="plate-section-name-caption">Section name</p>
                <div class="plate-section-name-select-filter-content">
                    <label class="plate-section-name-filter-label">
                        <input class="plate-section-name-filter" type="text" placeholder="Filter..."/>
                    </label>
                    <select class="plate-section-name"></select>
                </div>
            </div>

            <div class="thickness-field-content">
                <p class="thickness-caption">Thickness</p>
                <input class="thickness" type="number"/>
            </div>

            <div class="shear-factor-field-content">
                <p class="shear-factor-caption">Shear factor</p>
                <input class="shear-factor" type="number"/>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.updatePlateSection());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelPlateSectionUpdate());

        this.shadowRoot.querySelector(".plate-section-name").addEventListener("change", () => this.updatePlateSectionData());

        this.shadowRoot.querySelector(".plate-section-name-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".plate-section-name-filter").value,
                this.shadowRoot.querySelector(".plate-section-name"));
        });

        this.shadowRoot.querySelector(".thickness").addEventListener("click", () => {
            const inputtedArea = this.shadowRoot.querySelector(".thickness");
            this.dropHighlight(inputtedArea);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".shear-factor").addEventListener("click", () => {
            const inputtedShearFactor = this.shadowRoot.querySelector(".shear-factor");
            this.dropHighlight(inputtedShearFactor);
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
                this.definePlateSectionNameOptions();
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
            this.definePlateSectionNameOptions();
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

    definePlateSectionNameOptions() {
        const updatePlateSectionNameSelect = this.shadowRoot.querySelector(".plate-section-name");
        for (let i = updatePlateSectionNameSelect.length - 1; i >= 0; i--) {
            updatePlateSectionNameSelect.options[i] = null;
        }
        if (this.props.store.plate_sections_shelf.length > 0) {
            for (let i = 0; i < this.props.store.plate_sections_shelf.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = this.props.store.plate_sections_shelf[i].name.replace(/['"]+/g, "");
                updateOption.innerHTML = this.props.store.plate_sections_shelf[i].name.replace(/['"]+/g, "");
                updatePlateSectionNameSelect.appendChild(updateOption);
            }
            this.shadowRoot.querySelector(".thickness").value = this.props.store.plate_sections_shelf[0].thickness;
            this.shadowRoot.querySelector(".shear-factor").value = this.props.store.plate_sections_shelf[0].shear_factor;
        } else {
            this.shadowRoot.querySelector(".thickness").value = "";
            this.shadowRoot.querySelector(".shear-factor").value = "";
        }
    }

    updatePlateSectionData() {
        const selectedPlateSectionName = this.shadowRoot.querySelector(".plate-section-name").value;
        const plateSectionInProps = this.props.store.plate_sections_shelf
            .find(plateSection => plateSection.name == `"${selectedPlateSectionName}"`);
        this.shadowRoot.querySelector(".thickness").value = plateSectionInProps.thickness;
        this.dropHighlight(this.shadowRoot.querySelector(".thickness"));
        this.shadowRoot.querySelector(".shear-factor").value = plateSectionInProps.shear_factor;
        this.dropHighlight(this.shadowRoot.querySelector(".shear-factor"));
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

    updatePlateSection() {
        const selectedPlateSectionNameField = this.shadowRoot.querySelector(".plate-section-name");
        if (selectedPlateSectionNameField.value == "") {
            if (selectedPlateSectionNameField.classList.contains("highlighted") === false) {
                selectedPlateSectionNameField.classList.add("highlighted");
            }
        }

        const inputtedThicknessField = this.shadowRoot.querySelector(".thickness");
        if (inputtedThicknessField.value == "") {
            if (inputtedThicknessField.classList.contains("highlighted") === false) {
                inputtedThicknessField.classList.add("highlighted");
            }
        }

        const inputtedShearFactorField = this.shadowRoot.querySelector(".shear-factor");
        if (inputtedShearFactorField.value == "") {
            if (inputtedShearFactorField.classList.contains("highlighted") === false) {
                inputtedShearFactorField.classList.add("highlighted");
            }
        }

        if (selectedPlateSectionNameField.value === "" || inputtedThicknessField.value === "" ||
            inputtedShearFactorField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        if (this.isNumeric(inputtedThicknessField.value) === false || 
            this.isNumeric(inputtedShearFactorField.value) === false) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values for Thickness and Shear factor!";
                return;
            } else {
                return;
            }
        }

        const oldPlateSectionValues = this.props.store.plate_sections_shelf
            .find(plateSection => plateSection.name == `"${selectedPlateSectionNameField.value}"`);
        
        const message = { [UPDATE_PLATE_SECTION_MESSAGE_HEADER]: {
            action_id: this.props.actionId,
            name: selectedPlateSectionNameField.value, 
            old_plate_section_values: { 
                thickness: oldPlateSectionValues.thickness,
                shear_factor: oldPlateSectionValues.shear_factor 
            },
            new_plate_section_values: { 
                thickness: inputtedThicknessField.value,
                shear_factor: inputtedShearFactorField.value 
            }
        }};
        this.dispatchEvent(new CustomEvent([CLIENT_MESSAGE_HEADER], {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));

        this.shadowRoot.querySelector(".plate-section-name-filter").value = null;
    }

    cancelPlateSectionUpdate() {
        if (this.props.store.plate_sections_shelf.length > 0) {
            this.definePlateSectionNameOptions();
        }
        this.shadowRoot.querySelector(".plate-section-name-filter").value = null;
        const selectedPlateSectionNameForUpdateField = this.shadowRoot.querySelector(".plate-section-name");
        this.dropHighlight(selectedPlateSectionNameForUpdateField);
        const inputtedAreaField = this.shadowRoot.querySelector(".thickness");
        this.dropHighlight(inputtedAreaField);
        const inputtedShearFactorField = this.shadowRoot.querySelector(".shear-factor");
        this.dropHighlight(inputtedShearFactorField);
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

export default FeaSectionUpdatePlateMenu;
