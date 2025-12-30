import Store from "../../../../../store/fea_store.js";
import { UPDATE_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_MESSAGE_HEADER } from "../../../../../consts/actions_router_consts.js";
import { CLIENT_MESSAGE_HEADER, GET_WASM_LOADING_STATUS_MESSAGE_HEADER } from "../../../../../consts/fea_app_consts.js";


class FeaLoadUpdateUniformlyDistributedSurfaceLoadMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,                     // u32;
            isWasmLoaded: false,             // load status of wasm modules;
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

            .surface-number-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin: 0rem;
                align-items: center;
            }

            .surface-number-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .surface-number-select-filter-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: column;
            }

            .surface-number-filter-label {
                position: relative;
            }
                
            .surface-number-filter-label:before {
                content: "";
                position: absolute;
                left: 0rem;
                top: 0rem;
                bottom: 0rem;
                width: 0.8rem;
                background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
            }

            .surface-number-filter {
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

            .surface-number-filter::placeholder {
                font-size: 85%;
            }

            .surface-number-filter::-webkit-outer-spin-button,
            .surface-number-filter::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .surface-number-filter[type=number] {
                -moz-appearance: textfield;
            }

            .surface-number-filter:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .surface-number-filter:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .surface-number {
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

            .surface-number option {
                background-color: var(--preprocessor-menu-content-dropdown-menu-bg-color);
            }

            .surface-number:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .px-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .px-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .px {
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

            .px[type=number]::-webkit-outer-spin-button,
            .px[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .px[type=number] {
                -moz-appearance: textfield;
            }

            .px:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .px:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .py-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .py-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .py {
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

            .py[type=number]::-webkit-outer-spin-button,
            .py[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .py[type=number] {
                -moz-appearance: textfield;
            }

            .py:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .py:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .pz-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .pz-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .pz {
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

            .pz[type=number]::-webkit-outer-spin-button,
            .pz[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .pz[type=number] {
                -moz-appearance: textfield;
            }

            .pz:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .pz:focus {
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

            <div class="surface-number-field-content">
                <p class="surface-number-caption">Surface number</p>
                <div class="surface-number-select-filter-content">
                    <label class="surface-number-filter-label">
                        <input class="surface-number-filter" type="number" placeholder="Filter..."/>
                    </label>
                    <select class="surface-number"></select>
                </div>
            </div>

            <div class="px-field-content">
                <p class="px-caption">Px</p>
                <input class="px" type="number"/>
            </div>

            <div class="py-field-content">
                <p class="py-caption">Py</p>
                <input class="py" type="number"/>
            </div>

            <div class="pz-field-content">
                <p class="pz-caption">Pz</p>
                <input class="pz" type="number"/>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", 
            () => this.updateDistributedSurfaceLoad());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", 
            () => this.cancelDistributedSurfaceLoadUpdate());

        this.shadowRoot.querySelector(".surface-number").addEventListener("change", 
            () => this.updateDistributedSurfaceLoadValues());

        this.shadowRoot.querySelector(".surface-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".surface-number-filter").value,
                this.shadowRoot.querySelector(".surface-number"));
        });

        this.shadowRoot.querySelector(".px").addEventListener("click", () => {
            const inputtedPXField = this.shadowRoot.querySelector(".px");
            this.dropHighlight(inputtedPXField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".py").addEventListener("click", () => {
            const inputtedPYField = this.shadowRoot.querySelector(".py");
            this.dropHighlight(inputtedPYField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".pz").addEventListener("click", () => {
            const inputtedPZField = this.shadowRoot.querySelector(".pz");
            this.dropHighlight(inputtedPZField);
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
                this.defineDistributedSurfaceLoadOptions();
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
            this.defineDistributedSurfaceLoadOptions();
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

    defineDistributedSurfaceLoadOptions() {
        const surfaceNumberSelect = this.shadowRoot.querySelector(".surface-number");
        for (let i = surfaceNumberSelect.length - 1; i >= 0; i--) {
            surfaceNumberSelect.options[i] = null;
        }

        let surfaceNumbers = Array();
        this.props.store.surfaces_shelf.forEach((surface, number) => {
            if (surface.optional_uniformly_distributed_surface_load) {
                surfaceNumbers.push(parseInt(number));
            }
        });

        if (surfaceNumbers.length > 0) {
            const sortedSurfaceNumbers = Array.from(surfaceNumbers).sort((a, b) => a - b);
            for (let i = 0; i < sortedSurfaceNumbers.length; i++) {
                let surfaceNumberOption = document.createElement("option");
                surfaceNumberOption.value = sortedSurfaceNumbers[i];
                surfaceNumberOption.innerHTML = sortedSurfaceNumbers[i];
                surfaceNumberSelect.appendChild(surfaceNumberOption);
            }
            this.shadowRoot.querySelector(".px").value = 
                this.props.store.surfaces_shelf.get(sortedSurfaceNumbers[0]).optional_uniformly_distributed_surface_load[1];
            this.shadowRoot.querySelector(".py").value = 
                this.props.store.surfaces_shelf.get(sortedSurfaceNumbers[0]).optional_uniformly_distributed_surface_load[2];
            this.shadowRoot.querySelector(".pz").value = 
                this.props.store.surfaces_shelf.get(sortedSurfaceNumbers[0]).optional_uniformly_distributed_surface_load[3];
        } else {
            this.shadowRoot.querySelector(".px").value = "";
            this.shadowRoot.querySelector(".py").value = "";
            this.shadowRoot.querySelector(".pz").value = "";
        }
    }

    updateDistributedSurfaceLoadValues() {
        const selectedSurfaceNumber = this.shadowRoot.querySelector(".surface-number").value;
        this.shadowRoot.querySelector(".px").value = 
            this.props.store.surfaces_shelf.get(parseInt(selectedSurfaceNumber)).optional_uniformly_distributed_surface_load[1];
        this.dropHighlight(this.shadowRoot.querySelector(".px"));
        this.shadowRoot.querySelector(".py").value = 
            this.props.store.surfaces_shelf.get(parseInt(selectedSurfaceNumber)).optional_uniformly_distributed_surface_load[2];
        this.dropHighlight(this.shadowRoot.querySelector(".py"));
        this.shadowRoot.querySelector(".pz").value = 
            this.props.store.surfaces_shelf.get(parseInt(selectedSurfaceNumber)).optional_uniformly_distributed_surface_load[3];
        this.dropHighlight(this.shadowRoot.querySelector(".pz"));
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

    updateDistributedSurfaceLoad() {
        const selectedSurfaceNumberField = this.shadowRoot.querySelector(".surface-number");
        if (selectedSurfaceNumberField.value == "") {
            if (selectedSurfaceNumberField.classList.contains("highlighted") === false) {
                selectedSurfaceNumberField.classList.add("highlighted");
            }
        }

        const inputtedPXField = this.shadowRoot.querySelector(".px");
        if (inputtedPXField.value == "") {
            if (inputtedPXField.classList.contains("highlighted") === false) {
                inputtedPXField.classList.add("highlighted");
            }
        }
        const inputtedPYField = this.shadowRoot.querySelector(".py");
        if (inputtedPYField.value == "") {
            if (inputtedPYField.classList.contains("highlighted") === false) {
                inputtedPYField.classList.add("highlighted");
            }
        }

        const inputtedPZField = this.shadowRoot.querySelector(".pz");
        if (inputtedPZField.value == "") {
            if (inputtedPZField.classList.contains("highlighted") === false) {
                inputtedPZField.classList.add("highlighted");
            }
        }

        if (selectedSurfaceNumberField.value === "" || 
            inputtedPXField.value === "" || inputtedPYField.value === "" || inputtedPZField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        if (this.isNumeric(selectedSurfaceNumberField.value) === false || 
            this.isNumeric(inputtedPXField.value) === false || 
            this.isNumeric(inputtedPYField.value) === false || 
            this.isNumeric(inputtedPZField.value) === false) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values!";
                return;
            } else {
                return;
            }
        }

        const oldDistributedSurfaceLoadValues = this.props.store.surfaces_shelf.get(
            parseInt(selectedSurfaceNumberField.value)).optional_uniformly_distributed_surface_load;

        const message = { [UPDATE_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_MESSAGE_HEADER]: {
            action_id: this.props.actionId,
            surface_number: selectedSurfaceNumberField.value, 
            old_uniformly_distributed_surface_load_values: 
                { 
                    px: oldDistributedSurfaceLoadValues[1], py: oldDistributedSurfaceLoadValues[2],
                    pz: oldDistributedSurfaceLoadValues[3],   
                },
            new_uniformly_distributed_surface_load_values: 
                { 
                    px: inputtedPXField.value, py: inputtedPYField.value,
                    pz: inputtedPZField.value,
                }
        }};

        this.dispatchEvent(new CustomEvent([CLIENT_MESSAGE_HEADER], {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));

        this.shadowRoot.querySelector(".surface-number-filter").value = null;
    }

    cancelDistributedSurfaceLoadUpdate() {
        let surfaceNumbers = Array();
        this.props.store.surfaces_shelf.forEach((surface, number) => {
            if (surface.optional_uniformly_distributed_surface_load) {
                surfaceNumbers.push(parseInt(number));
            }
        });
        if (surfaceNumbers.length > 0) {
            this.defineDistributedSurfaceLoadOptions();
        }
        this.shadowRoot.querySelector(".surface-number-filter").value = null;
        const selectedSurfaceNumberForUpdateField = this.shadowRoot.querySelector(".surface-number");
        this.dropHighlight(selectedSurfaceNumberForUpdateField);
        const inputtedPXField = this.shadowRoot.querySelector(".px");
        this.dropHighlight(inputtedPXField);
        const inputtedPYField = this.shadowRoot.querySelector(".py");
        this.dropHighlight(inputtedPYField);
        const inputtedPZField = this.shadowRoot.querySelector(".pz");
        this.dropHighlight(inputtedPZField);
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

export default FeaLoadUpdateUniformlyDistributedSurfaceLoadMenu;
