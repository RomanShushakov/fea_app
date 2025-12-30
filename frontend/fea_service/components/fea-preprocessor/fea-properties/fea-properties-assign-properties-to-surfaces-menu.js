import Store from "../../../store/fea_store.js";
import { ASSIGN_PROPERTIES_TO_SURFACES_MESSAGE_HEADER } from "../../../consts/actions_router_consts.js";
import { PREVIEW_SELECTED_SURFACE_NUMBERS_MESSAGE_HEADER } from "../../../consts/fea_app_consts.js";
import { CLIENT_MESSAGE_HEADER, GET_WASM_LOADING_STATUS_MESSAGE_HEADER } from "../../../consts/fea_app_consts.js";


class FeaPropertiesAssignPropertiesToSurfacesMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,                 // u32;
            isWasmLoaded: false,         // load status of wasm modules;
            store: new Store(),
        };

        this.state = {
            selectedSurfaces: new Set(),
            assignToSurfaces: new Set(),
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
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
                align-items: center;
            }

            .properties-name-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin: 0rem;
                align-items: center;
            }

            .properties-name-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .properties-name-select-filter-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: column;
            }

            .properties-name-filter-label {
                position: relative;
            }
                
            .properties-name-filter-label:before {
                content: "";
                position: absolute;
                left: 0rem;
                top: 0rem;
                bottom: 0rem;
                width: 0.8rem;
                background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
            }

            .properties-name-filter {
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

            .properties-name-filter::placeholder {
                font-size: 85%;
            }

            .properties-name-filter:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .properties-name-filter:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .properties-name {
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

            .properties-name option {
                background-color: var(--preprocessor-menu-content-dropdown-menu-bg-color);
            }

            .properties-name:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .selected-surfaces-field-content {
                display: flex;
                flex-direction: column;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
            }

            .selected-surfaces-caption-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                align-items: center;
            }

            .selected-surfaces-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 7rem;
            }

            .clear-button {
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin-left: 0.8rem;
                width: 3.5rem;
                height: 1.5rem;
                font-size: 70%;
            }

            .clear-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .selected-surfaces {
                margin-top: 0.5rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding-left: 0.3rem;
                width: 11rem;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                outline: none;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .selected-surfaces:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .selected-surfaces:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .selected-surfaces-field-buttons {
                margin-top: 0.3rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
            }

            .add-button {
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 4.5rem;
                height: 1.5rem;
                font-size: 70%;
            }

            .add-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .remove-button {
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 6.5rem;
                height: 1.5rem;
                font-size: 70%;
            }

            .remove-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .assign-to-surfaces-field-content {
                display: flex;
                flex-direction: column;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
            }

            .assign-to-surfaces-caption-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                align-items: center;
            }

            .assign-to-surfaces-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 7rem;
            }

            .assign-to-surfaces-field-buttons {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
            }

            .preview-button {
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin-left: 0.8rem;
                width: 3.5rem;
                height: 1.5rem;
                font-size: 70%;
            }

            .preview-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .assign-to-surfaces {
                margin-top: 0.5rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding-left: 0.3rem;
                width: 11rem;
                height: 5rem;
                resize: none;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                outline: none;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 90%;
            }

            ::-webkit-scrollbar {
                width: 0.5rem;
            }

            ::-webkit-scrollbar-track {
                background: var(--preprocessor-menu-content-scrollbar-track-color);
            }

            ::-webkit-scrollbar-thumb {
                background: var(--preprocessor-menu-content-scrollbar-thumb-color);
            }

            ::-webkit-scrollbar-thumb:hover {
                background: var(--preprocessor-menu-content-scrollbar-thumb-hover-color);
            }

            .assign-to-surfaces:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .assign-to-surfaces:focus {
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

            <div class="properties-name-field-content">
                <p class="properties-name-caption">Properties name</p>
                <div class="properties-name-select-filter-content">
                    <label class="properties-name-filter-label">
                        <input class="properties-name-filter" type="text" placeholder="Filter..."/>
                    </label>
                    <select class="properties-name"></select>
                </div>
            </div>

            <div class="selected-surfaces-field-content">
                <div class="selected-surfaces-caption-content">
                    <p class="selected-surfaces-caption">Selected surfaces:</p>
                    <div class="selected-surfaces-field-buttons">
                        <button class="clear-button">Clear</button>
                    </div>
                </div>
                <input class="selected-surfaces" type="text" placeholder="ex 1, 2, ..., etc."/>
            </div>

            <div class="selected-surfaces-field-buttons">
                <button class="add-button">Add to list</button>
                <button class="remove-button">Remove from list</button>
            </div>

            <div class="assign-to-surfaces-field-content">
                <div class="assign-to-surfaces-caption-content">
                    <p class="assign-to-surfaces-caption">Assign to surfaces:</p>
                    <div class="assign-to-surfaces-field-buttons">
                        <button class="preview-button">Preview</button>
                    </div>
                </div>
                <textarea class="assign-to-surfaces" type="text" placeholder="ex 1, 2, ..., etc."/></textarea>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.assignProperties());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelPropertiesAssign());

        this.shadowRoot.querySelector(".properties-name").addEventListener("change",
            (event) => this.updateSelectedPropertiesData(event.target.value));

        this.shadowRoot.querySelector(".properties-name-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".properties-name-filter").value,
                this.shadowRoot.querySelector(".properties-name"));
        });

        this.shadowRoot.querySelector(".selected-surfaces").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".selected-surfaces");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".assign-to-surfaces").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".assign-to-surfaces");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".add-button").addEventListener("click", () => {
            this.addToAssignToSurfaces();
        });

        this.shadowRoot.querySelector(".remove-button").addEventListener("click", () => {
            this.removeFromAssignToSurfaces();
        });

        this.shadowRoot.querySelector(".clear-button").addEventListener("click", () => {
            this.state.selectedSurfaces.clear();
            this.updateSelectedSurfacesField();
        });

        this.shadowRoot.querySelector(".preview-button").addEventListener("click", () => this.previewSelectedSurfaces());
    }

    set isWasmLoaded(value) {
        this.props.isWasmLoaded = value;
    }

    set selectSurfaceInClientForDataAssign(surfaceNumber) {
        this.addToSelectedSurfaces(surfaceNumber);
        this.updateSelectedSurfacesField();
    }

    set rendererError(error) {
        const assignToSurfacesField = this.shadowRoot.querySelector(".assign-to-surfaces");
        if (assignToSurfacesField.classList.contains("highlighted") === false) {
            assignToSurfacesField.classList.add("highlighted");
        }
        if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = error;
        }
    }

    set feModelError(error) {
        const assignToSurfacesField = this.shadowRoot.querySelector(".assign-to-surfaces");
        if (assignToSurfacesField.classList.contains("highlighted") === false) {
            assignToSurfacesField.classList.add("highlighted");
        }
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
        document.querySelector("fea-app").dispatchEvent(new CustomEvent("enableSurfacesSelectionMode"));
        const frame = () => {
            this.getWasmLoadingStatus();
            if (this.props.isWasmLoaded === true) {
                clearInterval(id);
                this.definePropertiesNameOptions();
            }
        }
        const id = setInterval(frame, 10);
    }

    disconnectedCallback() {
        document.querySelector("fea-app").dispatchEvent(new CustomEvent("disableSurfacesSelectionMode"));
    }

    static get observedAttributes() {
        return ["action-id"];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "action-id") {
            this.props.actionId = parseInt(newValue);
            this.definePropertiesNameOptions();
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

    addToSelectedSurfaces(surfaceNumber) {
        const selectedSurfacesField = this.shadowRoot.querySelector(".selected-surfaces");
        let selectedSurfaces = selectedSurfacesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        selectedSurfaces = selectedSurfaces.map((item) => parseInt(item));
        this.state.selectedSurfaces = new Set(selectedSurfaces);
        this.state.selectedSurfaces.add(surfaceNumber);
    }

    addToAssignToSurfaces() {
        const selectedSurfacesField = this.shadowRoot.querySelector(".selected-surfaces");
        let selectedSurfaces = selectedSurfacesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < selectedSurfaces.length; i++) {
            if (this.isNumeric(selectedSurfaces[i]) === false || this.isInt(selectedSurfaces[i]) === false) {
                if (selectedSurfacesField.classList.contains("highlighted") === false) {
                    selectedSurfacesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only integer numbers could be used as selected surfaces values!";
                }
                return;
            }
            if (this.props.store.surfaces_shelf.has(parseInt(selectedSurfaces[i])) === false) {
                if (selectedSurfacesField.classList.contains("highlighted") === false) {
                    selectedSurfacesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed surfaces numbers could be used as selected surfaces values!";
                }
                return;
            }

        }
        selectedSurfaces = selectedSurfaces.map((item) => parseInt(item));
        const union = new Set([...selectedSurfaces, ...this.state.assignToSurfaces]);
        this.state.assignToSurfaces = union;
        this.state.selectedSurfaces.clear();
        this.updateSelectedSurfacesField();
        this.updateAssignToSurfacesField();
    }

    removeFromAssignToSurfaces() {
        const selectedSurfacesField = this.shadowRoot.querySelector(".selected-surfaces");
        let selectedSurfaces = selectedSurfacesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < selectedSurfaces.length; i++) {
            if (this.isNumeric(selectedSurfaces[i]) === false || this.isInt(selectedSurfaces[i]) === false) {
                if (selectedSurfacesField.classList.contains("highlighted") === false) {
                    selectedSurfacesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only integer numbers could be used as selected surfaces values!";
                }
                return;
            }
            if (this.props.store.surfaces_shelf.has(parseInt(selectedSurfaces[i])) === false) {
                if (selectedSurfacesField.classList.contains("highlighted") === false) {
                    selectedSurfacesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed surfaces numbers could be used as selected surfaces values!";
                }
                return;
            }
        }
        selectedSurfaces = selectedSurfaces.map((item) => parseInt(item));
        const selectedSurfacesSet = new Set(selectedSurfaces);
        let difference = new Set([...this.state.assignToSurfaces].filter(
            (surfaceNumber) => !selectedSurfacesSet.has(surfaceNumber)));
        this.state.assignToSurfaces = difference;
        this.state.selectedSurfaces.clear();
        this.updateSelectedSurfacesField();
        this.updateAssignToSurfacesField();
    }

    updateSelectedSurfacesField() {
        let selectedSurfacesFieldValue = "";
        for (let item of this.state.selectedSurfaces) {
            selectedSurfacesFieldValue += `${item}, `
        }
        this.shadowRoot.querySelector(".selected-surfaces").value = selectedSurfacesFieldValue;
    }

    updateAssignToSurfacesField() {
        let assignToSurfacesFieldValue = "";
        for (let item of this.state.assignToSurfaces) {
            assignToSurfacesFieldValue += `${item}, `
        }
        this.shadowRoot.querySelector(".assign-to-surfaces").value = assignToSurfacesFieldValue;
    }

    filter(keywordField, selectField) {
        for (let i = 0; i < selectField.length; i++) {
            let txt = selectField.options[i].value;
            if (txt.substring(0, keywordField.length).toLowerCase() !== keywordField.toLowerCase() && keywordField.trim() !== "") {
                selectField.options[i].style.display = "none";
            } else {
                if (txt !== "") {
                    selectField.options[i].style.display = "list-item";
                } else {
                    selectField.options[i].style.display = "none";
                }
                
            }
        }
    }

    definePropertiesNameOptions() {
        const propertiesAssignNameSelect = this.shadowRoot.querySelector(".properties-name");
        for (let i = propertiesAssignNameSelect.length - 1; i >= 0; i--) {
            propertiesAssignNameSelect.options[i] = null;
        }
        for (let i = 0; i < this.props.store.properties_shelf.length; i++) {
            if (this.props.store.properties_shelf[i].cross_section_type === "plate") {
                let assignOption = document.createElement("option");
                assignOption.value = this.props.store.properties_shelf[i].name.replace(/['"]+/g, "");
                assignOption.innerHTML = this.props.store.properties_shelf[i].name.replace(/['"]+/g, "");
                propertiesAssignNameSelect.appendChild(assignOption);
            }
        }
        this.updateSelectedPropertiesData(propertiesAssignNameSelect.value);
    }

    updateSelectedPropertiesData(selectedPropertiesName) {
        let assignToSurfacesFieldValue = "";
        let surfaceNumbersWithProperty = this.props.store.getSurfaceNumbersWithProperty(`"${selectedPropertiesName}"`);
        surfaceNumbersWithProperty.sort();
        this.state.assignToSurfaces = new Set(surfaceNumbersWithProperty);
        for (let i = 0; i < surfaceNumbersWithProperty.length; i++) {
            assignToSurfacesFieldValue += `${surfaceNumbersWithProperty[i]}, `
        }
        this.shadowRoot.querySelector(".assign-to-surfaces").value = assignToSurfacesFieldValue;
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    previewSelectedSurfaces() {
        const assignToSurfacesField = this.shadowRoot.querySelector(".assign-to-surfaces");
        const assignToSurfaces = assignToSurfacesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < assignToSurfaces.length; i++) {
            if (this.isNumeric(assignToSurfaces[i]) === false || this.isInt(assignToSurfaces[i]) === false) {
                if (assignToSurfacesField.classList.contains("highlighted") === false) {
                    assignToSurfacesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only integer numbers could be used as assign to surfaces values!";
                }
                return;
            }

            if (this.props.store.surfaces_shelf.has(parseInt(assignToSurfaces[i])) === false) {
                if (assignToSurfacesField.classList.contains("highlighted") === false) {
                    assignToSurfacesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed surfaces numbers could be used as assign to surfaces values!";
                }
                return;
            }

            assignToSurfaces[i] = Number.parseInt(assignToSurfaces[i]);
        }
        if (assignToSurfaces.length > 0) {
            this.dispatchEvent(new CustomEvent(PREVIEW_SELECTED_SURFACE_NUMBERS_MESSAGE_HEADER, {
                bubbles: true,
                composed: true,
                detail: { "surface_numbers": assignToSurfaces },
            }));
        }
    }

    assignProperties() {
        const equals = (a, b) => a.length === b.length && a.every((v, i) => v === b[i]);
        const selectedPropertiesNameField = this.shadowRoot.querySelector(".properties-name");
        if (selectedPropertiesNameField.value === "") {
            if (selectedPropertiesNameField.classList.contains("highlighted") === false) {
                selectedPropertiesNameField.classList.add("highlighted");
            }
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        const assignToSurfacesField = this.shadowRoot.querySelector(".assign-to-surfaces");
        const assignToSurfaces = assignToSurfacesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < assignToSurfaces.length; i++) {
            if (this.isNumeric(assignToSurfaces[i]) === false || this.isInt(assignToSurfaces[i]) === false) {
                if (assignToSurfacesField.classList.contains("highlighted") === false) {
                    assignToSurfacesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only integer numbers could be used as assign to surfaces values!";
                }
                return;
            }

            if (this.props.store.surfaces_shelf.has(parseInt(assignToSurfaces[i])) === false) {
                if (assignToSurfacesField.classList.contains("highlighted") === false) {
                    assignToSurfacesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed surfaces numbers could be used as assign to surfaces values!";
                }
                return;
            }

            assignToSurfaces[i] = parseInt(assignToSurfaces[i]);
        }

        const propertiesData = this.props.store.properties_shelf.find(
            properties => properties.name == `"${selectedPropertiesNameField.value}"`);
        
        const oldSurfaceNumbers = this.props.store.getSurfaceNumbersWithProperty(propertiesData.name);

        const message = {
            [ASSIGN_PROPERTIES_TO_SURFACES_MESSAGE_HEADER]: {
                action_id: this.props.actionId,
                name: propertiesData.name.replace(/['"]+/g, ""),
                old_assigned_properties_to_surfaces_values: {
                    surface_numbers: oldSurfaceNumbers,
                },
                new_assigned_properties_to_surfaces_values: {
                    surface_numbers: assignToSurfaces,
                }
            }
        };
        this.dispatchEvent(new CustomEvent([CLIENT_MESSAGE_HEADER], {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
        this.state.selectedSurfaces.clear();
        this.updateSelectedSurfacesField();
    }

    cancelPropertiesAssign() {
        this.shadowRoot.querySelector(".properties-name-filter").value = null;
        if (this.props.store.properties_shelf.length > 0) {
            this.definePropertiesNameOptions();
        }
        const selectedPropertiesNameForAssignField = this.shadowRoot.querySelector(".properties-name");
        this.dropHighlight(selectedPropertiesNameForAssignField);
        const assignToSurfacesField = this.shadowRoot.querySelector(".assign-to-surfaces");
        this.dropHighlight(assignToSurfacesField);
        this.state.selectedSurfaces.clear();
        this.updateSelectedSurfacesField();
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

    isInt(str) {
        const n = parseFloat(str);
        return n % 1 === 0;
    }
}

export default FeaPropertiesAssignPropertiesToSurfacesMenu;

