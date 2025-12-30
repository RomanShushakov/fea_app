import Store from "../../../../../store/fea_store.js";
import { DELETE_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_MESSAGE_HEADER } from "../../../../../consts/actions_router_consts.js";
import { CLIENT_MESSAGE_HEADER, GET_WASM_LOADING_STATUS_MESSAGE_HEADER } from "../../../../../consts/fea_app_consts.js";


class FeaLoadDeleteDistributedSurfaceLoadMenu extends HTMLElement {
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
            () => this.deleteDistributedSurfaceLoad());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", 
            () => this.cancelDistributedSurfaceLoadDelete());

        this.shadowRoot.querySelector(".surface-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".surface-number-filter").value,
                this.shadowRoot.querySelector(".surface-number"));
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

        const sortedSurfaceNumbers = Array.from(surfaceNumbers).sort((a, b) => a - b);
        for (let i = 0; i < sortedSurfaceNumbers.length; i++) {
            let surfaceNumberOption = document.createElement("option");
            surfaceNumberOption.value = sortedSurfaceNumbers[i];
            surfaceNumberOption.innerHTML = sortedSurfaceNumbers[i];
            surfaceNumberSelect.appendChild(surfaceNumberOption);
        }
    }

    filter(keywordField, selectField) {
        for (let i = 0; i < selectField.length; i++) {
            let txt = selectField.options[i].value;
            if (txt.substring(0, keywordField.length).toLowerCase() !== keywordField.toLowerCase() && keywordField.trim() !== "") {
                selectField.options[i].style.display = "none";
            } else {
                selectField.options[i].style.display = "list-item";
            }
        }
    }

    deleteDistributedSurfaceLoad() {
        const selectedSurfaceNumberField = this.shadowRoot.querySelector(".surface-number");
        if (selectedSurfaceNumberField.value == "") {
            if (selectedSurfaceNumberField.classList.contains("highlighted") === false) {
                selectedSurfaceNumberField.classList.add("highlighted");
            }
        }

        if (selectedSurfaceNumberField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }
        
        const message = { [DELETE_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_MESSAGE_HEADER]: { 
            action_id: this.props.actionId, surface_number: parseInt(selectedSurfaceNumberField.value) 
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

    cancelDistributedSurfaceLoadDelete() {
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
        const selectedSurfaceNumberForDeleteField = this.shadowRoot.querySelector(".surface-number");
        this.dropHighlight(selectedSurfaceNumberForDeleteField);
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    dropHighlight(highlightedElement) {
        if (highlightedElement.classList.contains("highlighted") === true) {
            highlightedElement.classList.remove("highlighted");
        }
    }
}

export default FeaLoadDeleteDistributedSurfaceLoadMenu;
