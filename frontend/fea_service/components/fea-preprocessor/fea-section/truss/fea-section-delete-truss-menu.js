import Store from "../../../../store/fea_store.js";
import { DELETE_TRUSS_SECTION_MESSAGE_HEADER } from "../../../../consts/actions_router_consts.js";
import { 
    CLIENT_MESSAGE_HEADER, GET_ACTION_ID_MESSAGE_HEADER, GET_WASM_LOADING_STATUS_MESSAGE_HEADER,
} from "../../../../consts/fea_app_consts.js";


class FeaSectionDeleteTrussMenu extends HTMLElement {
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

            .truss-section-name-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin: 0rem;
                align-items: center;
            }

            .truss-section-name-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .truss-section-name-select-filter-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: column;
            }

            .truss-section-name-filter-label {
                position: relative;
            }
                
            .truss-section-name-filter-label:before {
                content: "";
                position: absolute;
                left: 0rem;
                top: 0rem;
                bottom: 0rem;
                width: 0.8rem;
                background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
            }

            .truss-section-name-filter {
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

            .truss-section-name-filter::placeholder {
                font-size: 85%;
            }

            .truss-section-name-filter:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .truss-section-name-filter:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .truss-section-name {
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

            .truss-section-name option {
                background-color: var(--preprocessor-menu-content-dropdown-menu-bg-color);
            }

            .truss-section-name:hover {
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

            <div class="truss-section-name-field-content">
                <p class="truss-section-name-caption">Section name</p>
                <div class="truss-section-name-select-filter-content">
                    <label class="truss-section-name-filter-label">
                        <input class="truss-section-name-filter" type="text" placeholder="Filter..."/>
                    </label>
                    <select class="truss-section-name"></select>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.deleteTrussSection());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelTrussSectionDelete());

        this.shadowRoot.querySelector(".truss-section-name-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".truss-section-name-filter").value,
                this.shadowRoot.querySelector(".truss-section-name"));
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
                this.defineTrussSectionNameOptions();
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
            this.defineTrussSectionNameOptions();
        }
    }

    adoptedCallback() {
    }

    getActionId() {
        this.dispatchEvent(new CustomEvent(GET_ACTION_ID_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
        }));
    }

    getWasmLoadingStatus() {
        this.dispatchEvent(new CustomEvent(GET_WASM_LOADING_STATUS_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
        }));
    }

    defineTrussSectionNameOptions() {
        const trussSectionDeleteNameSelect = this.shadowRoot.querySelector(".truss-section-name");
        for (let i = trussSectionDeleteNameSelect.length - 1; i >= 0; i--) {
            trussSectionDeleteNameSelect.options[i] = null;
        }
        for (let i = 0; i < this.props.store.truss_sections_shelf.length; i++) {
            let deleteOption = document.createElement("option");
            deleteOption.value = this.props.store.truss_sections_shelf[i].name.replace(/['"]+/g, "");
            deleteOption.innerHTML = this.props.store.truss_sections_shelf[i].name.replace(/['"]+/g, "");
            trussSectionDeleteNameSelect.appendChild(deleteOption);
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

    deleteTrussSection() {
        const selectedTrussSectionNameField = this.shadowRoot.querySelector(".truss-section-name");
        if (selectedTrussSectionNameField.value == "") {
            if (selectedTrussSectionNameField.classList.contains("highlighted") === false) {
                selectedTrussSectionNameField.classList.add("highlighted");
            }
        }

        if (selectedTrussSectionNameField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        const deletedTrussSectionData = this.props.store.truss_sections_shelf
            .find(trussSection => trussSection.name == `"${selectedTrussSectionNameField.value}"`);

        const message = { [DELETE_TRUSS_SECTION_MESSAGE_HEADER]: { 
            action_id: this.props.actionId, name: deletedTrussSectionData.name.replace(/['"]+/g, "") 
        }};
        this.dispatchEvent(new CustomEvent([CLIENT_MESSAGE_HEADER], {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));

        this.shadowRoot.querySelector(".truss-section-name-filter").value = null;
    }

    cancelTrussSectionDelete() {
        if (this.props.store.truss_sections_shelf.length > 0) {
            this.defineTrussSectionNameOptions();
        }
        this.shadowRoot.querySelector(".truss-section-name-filter").value = null;
        const selectedTrussSectionNameForDeleteField = this.shadowRoot.querySelector(".truss-section-name");
        this.dropHighlight(selectedTrussSectionNameForDeleteField);
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    dropHighlight(highlightedElement) {
        if (highlightedElement.classList.contains("highlighted") === true) {
            highlightedElement.classList.remove("highlighted");
        }
    }
}

export default FeaSectionDeleteTrussMenu;
