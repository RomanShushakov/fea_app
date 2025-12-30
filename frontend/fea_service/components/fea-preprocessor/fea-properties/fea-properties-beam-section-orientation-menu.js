import Store from "../../../store/fea_store.js";
import { 
    ADD_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER, DELETE_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER,
    ASSIGN_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER,
} from "../../../consts/actions_router_consts.js";
import { 
    PREVIEW_SELECTED_LINE_NUMBERS_MESSAGE_HEADER, ENABLE_LINES_SELECTION_MODE_MESSAGE_HEADER, 
    DISABLE_LINES_SELECTION_MODE_MESSAGE_HEADER, GET_WASM_LOADING_STATUS_MESSAGE_HEADER, CLIENT_MESSAGE_HEADER,
} from "../../../consts/fea_app_consts.js";

class FeaPropertiesBeamSectionOrientationMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,                         // u32;
            isWasmLoaded: false,                 // load status of wasm modules;
            store: new Store(),
        };

        this.state = {
            selectedLines: new Set(),
            assignToLines: new Set(),
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

            .local-axis-1-direction-input-field-content {
                display: flex;
                flex-direction: column;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
            }

            .local-axis-1-direction-input-caption-content {
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

            .local-axis-1-direction-input-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 12rem;
            }

            .local-axis-1-direction-input {
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

            .local-axis-1-direction-input:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .local-axis-1-direction-input:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .local-axis-1-direction-input-buttons {
                margin-top: 0.3rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
            }

            .add-inputted-button {
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 5rem;
                height: 1.5rem;
                font-size: 70%;
            }

            .add-inputted-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .remove-inputted-button {
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

            .remove-inputted-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .local-axis-1-direction-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                align-items: center;
            }

            .local-axis-1-direction-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 4rem;
            }

            .local-axis-1-direction-select-filter-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: column;
            }

            .local-axis-1-direction-filter-label {
                position: relative;
            }
                
            .local-axis-1-direction-filter-label:before {
                content: "";
                position: absolute;
                left: 0rem;
                top: 0rem;
                bottom: 0rem;
                width: 0.8rem;
                background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
            }

            .local-axis-1-direction-filter {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding-left: 1.3rem;
                width: 5.5rem;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                border: var(--preprocessor-menu-content-caption-border-color);
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                outline: none;
                color: var(--preprocessor-menu-content-caption-color);
            }

            .local-axis-1-direction-filter::placeholder {
                font-size: 85%;
            }

            .local-axis-1-direction-filter:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .local-axis-1-direction-filter:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .local-axis-1-direction {
                width: 7rem;
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

            .local-axis-1-direction option {
                background-color: var(--preprocessor-menu-content-dropdown-menu-bg-color);
            }

            .local-axis-1-direction:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .selected-lines-field-content {
                display: flex;
                flex-direction: column;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
            }

            .selected-lines-caption-content {
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

            .selected-lines-caption {
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

            .selected-lines {
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

            .selected-lines:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .selected-lines:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .selected-lines-field-buttons {
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

            .assign-to-lines-field-content {
                display: flex;
                flex-direction: column;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
            }

            .assign-to-lines-caption-content {
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

            .assign-to-lines-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 7rem;
            }

            .assign-to-lines-field-buttons {
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

            .assign-to-lines {
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

            /* width */
            ::-webkit-scrollbar {
                width: 0.5rem;
            }

            /* Track */
            ::-webkit-scrollbar-track {
                background: var(--preprocessor-menu-content-scrollbar-track-color);
            }

            /* Handle */
            ::-webkit-scrollbar-thumb {
                background: var(--preprocessor-menu-content-scrollbar-thumb-color);
            }

            /* Handle on hover */
            ::-webkit-scrollbar-thumb:hover {
                background: var(--preprocessor-menu-content-scrollbar-thumb-hover-color);
            }

            .assign-to-lines:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .assign-to-lines:focus {
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

            .local-axis-1-direction-input-info {
                display: flex;
                margin: 0rem;
                padding: 0rem;
            }

            .local-axis-1-direction-input-info-message {
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

            <div class="local-axis-1-direction-input-field-content">
                <div class="local-axis-1-direction-input-caption-content">
                    <p class="local-axis-1-direction-input-caption">Input local axis 1 direction:</p>
                </div>
                <input class="local-axis-1-direction-input" type="text" placeholder="ex 1.0, 0.0, 0.0"/>
            </div>

            <div class="local-axis-1-direction-input-buttons">
                <button class="add-inputted-button">Add inputted</button>
                <button class="remove-inputted-button">Remove inputted</button>
            </div>

            <div class="local-axis-1-direction-input-info">
                <p class="local-axis-1-direction-input-info-message"></p>
            </div>

            <div class="local-axis-1-direction-field-content">
                <p class="local-axis-1-direction-caption">Local axis 1 direction</p>
                <div class="local-axis-1-direction-select-filter-content">
                    <label class="local-axis-1-direction-filter-label">
                        <input class="local-axis-1-direction-filter" type="text" placeholder="Filter..."/>
                    </label>
                    <select class="local-axis-1-direction"></select>
                </div>
            </div>

            <div class="selected-lines-field-content">
                <div class="selected-lines-caption-content">
                    <p class="selected-lines-caption">Selected lines:</p>
                    <div class="selected-lines-field-buttons">
                        <button class="clear-button">Clear</button>
                    </div>
                </div>
                <input class="selected-lines" type="text" placeholder="ex 1, 2, ..., etc."/>
            </div>

            <div class="selected-lines-field-buttons">
                <button class="add-button">Add to list</button>
                <button class="remove-button">Remove from list</button>
            </div>

            <div class="assign-to-lines-field-content">
                <div class="assign-to-lines-caption-content">
                    <p class="assign-to-lines-caption">Assign to lines:</p>
                    <div class="assign-to-lines-field-buttons">
                        <button class="preview-button">Preview</button>
                    </div>
                </div>
                <textarea class="assign-to-lines" type="text" placeholder="ex 1, 2, ..., etc."/></textarea>
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

        this.shadowRoot.querySelector(".add-inputted-button").addEventListener("click", () => 
            this.addBeamSectionLocalAxis1Direction());

        this.shadowRoot.querySelector(".remove-inputted-button").addEventListener("click", () => 
            this.removeBeamSectionLocalAxis1Direction());

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => 
            this.updateBeamSectionOrientationData());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => 
            this.cancelBeamSectionOrientationDataUpdate());

        this.shadowRoot.querySelector(".local-axis-1-direction").addEventListener("change", (event) => 
            this.updateSelectedBeamSectionOrientationData(event.target.value)
        );

        this.shadowRoot.querySelector(".local-axis-1-direction-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".local-axis-1-direction-filter").value,
                this.shadowRoot.querySelector(".local-axis-1-direction"));
        });

        this.shadowRoot.querySelector(".local-axis-1-direction-input").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".local-axis-1-direction-input");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".local-axis-1-direction-input-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".selected-lines").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".selected-lines");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".add-button").addEventListener("click", () => {
            this.addToAssignToLines();
        });

        this.shadowRoot.querySelector(".remove-button").addEventListener("click", () => {
            this.removeFromAssignToLines();
        });

        this.shadowRoot.querySelector(".clear-button").addEventListener("click", () => {
            this.state.selectedLines.clear();
            this.updateSelectedLinesField();
        });

        this.shadowRoot.querySelector(".preview-button").addEventListener("click", () => 
            this.previewBeamSectionOrientationOnSelectedLines());

        this.shadowRoot.querySelector(".assign-to-lines").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".assign-to-lines");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });
    }

    set isWasmLoaded(value) {
        this.props.isWasmLoaded = value;
    }

    set selectLineInClientForDataAssign(lineNumber) {
        this.addToSelectedLines(lineNumber);
        this.updateSelectedLinesField();
    }

    set rendererError(error) {
        const assignToLinesField = this.shadowRoot.querySelector(".assign-to-lines");
        if (assignToLinesField.classList.contains("highlighted") === false) {
            assignToLinesField.classList.add("highlighted");
        }
        if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = error;
        }
    }

    set feModelError(errorData) {
        if (errorData.dst === "local_axis_1_direction_input_info") {
            const localAxis1DirectionInputField = this.shadowRoot.querySelector(".local-axis-1-direction-input");
            if (localAxis1DirectionInputField.classList.contains("highlighted") === false) {
                localAxis1DirectionInputField.classList.add("highlighted");
            }
            if (this.shadowRoot.querySelector(".local-axis-1-direction-input-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".local-axis-1-direction-input-info-message").innerHTML = errorData.error;
            }
        } else if (errorData.dst === "analysis_info") {
            const assignToLinesField = this.shadowRoot.querySelector(".assign-to-lines");
            if (assignToLinesField.classList.contains("highlighted") === false) {
                assignToLinesField.classList.add("highlighted");
            }
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = errorData.error;
            }
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
        document.querySelector("fea-app").dispatchEvent(new CustomEvent(ENABLE_LINES_SELECTION_MODE_MESSAGE_HEADER));
        const frame = () => {
            this.getWasmLoadingStatus();
            if (this.props.isWasmLoaded === true) {
                clearInterval(id);
                this.defineLocalAxis1DirectionOptions();
            }
        }
        const id = setInterval(frame, 10);
    }

    disconnectedCallback() {
        document.querySelector("fea-app").dispatchEvent(new CustomEvent(DISABLE_LINES_SELECTION_MODE_MESSAGE_HEADER));
    }

    static get observedAttributes() {
        return ["action-id"];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "action-id") {
            this.props.actionId = parseInt(newValue);
            this.defineLocalAxis1DirectionOptions();
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

    addBeamSectionLocalAxis1Direction() {
        const localAxis1DirectionInputField = this.shadowRoot.querySelector(".local-axis-1-direction-input");
        let localAxis1Direction = localAxis1DirectionInputField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < localAxis1Direction.length; i++) {
            if (this.isNumeric(localAxis1Direction[i]) === false) {
                if (localAxis1DirectionInputField.classList.contains("highlighted") === false) {
                    localAxis1DirectionInputField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".local-axis-1-direction-input-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".local-axis-1-direction-input-info-message").innerHTML = 
                        "Note: Only numbers could be used as local axis 1 direction values!";
                }
                return;
            }
        }
        localAxis1Direction = localAxis1Direction.map((item) => parseFloat(item));
        const message = { 
            [ADD_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER]: { 
                action_id: this.props.actionId,
                local_axis_1_direction: localAxis1Direction,
            } 
        };
        this.dispatchEvent(new CustomEvent([CLIENT_MESSAGE_HEADER], {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
        localAxis1DirectionInputField.value = null;
    }

    removeBeamSectionLocalAxis1Direction() {
        const localAxis1DirectionInputField = this.shadowRoot.querySelector(".local-axis-1-direction-input");
        let localAxis1Direction = localAxis1DirectionInputField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < localAxis1Direction.length; i++) {
            if (this.isNumeric(localAxis1Direction[i]) === false) {
                if (localAxis1DirectionInputField.classList.contains("highlighted") === false) {
                    localAxis1DirectionInputField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".local-axis-1-direction-input-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".local-axis-1-direction-input-info-message").innerHTML = 
                        "Note: Only numbers could be used as local axis 1 direction values!";
                }
                return;
            }
        }
        localAxis1Direction = localAxis1Direction.map((item) => parseFloat(item));
        const message = { 
            [DELETE_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER]: { 
                action_id: this.props.actionId,
                local_axis_1_direction: localAxis1Direction,
            } 
        };
        this.dispatchEvent(new CustomEvent([CLIENT_MESSAGE_HEADER], {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
        localAxis1DirectionInputField.value = null;
    }

    addToSelectedLines(lineNumber) {
        const selectedLinesField = this.shadowRoot.querySelector(".selected-lines");
        let selectedLines = selectedLinesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < selectedLines.length; i++) {
            if (this.isNumeric(selectedLines[i]) === false || this.isInt(selectedLines[i]) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only integer numbers could be used as selected lines values!";
                }
                return;
            }
            if (this.props.store.lines_shelf.has(parseInt(selectedLines[i])) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed lines numbers could be used as selected lines values!";
                }
                return;
            }

        }
        selectedLines = selectedLines.map((item) => parseInt(item));
        this.state.selectedLines = new Set(selectedLines);
        this.state.selectedLines.add(lineNumber);
    }

    addToAssignToLines() {
        const selectedLinesField = this.shadowRoot.querySelector(".selected-lines");
        let selectedLines = selectedLinesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < selectedLines.length; i++) {
            if (this.isNumeric(selectedLines[i]) === false || this.isInt(selectedLines[i]) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only integer numbers could be used as selected lines values!";
                }
                return;
            }
            if (this.props.store.lines_shelf.has(parseInt(selectedLines[i])) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed lines numbers could be used as selected lines values!";
                }
                return;
            }

        }
        selectedLines = selectedLines.map((item) => parseInt(item));
        const union = new Set([...selectedLines, ...this.state.assignToLines]);
        this.state.assignToLines = union;
        this.state.selectedLines.clear();
        this.updateSelectedLinesField();
        this.updateAssignToLinesField();
    }

    removeFromAssignToLines() {
        const selectedLinesField = this.shadowRoot.querySelector(".selected-lines");
        let selectedLines = selectedLinesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < selectedLines.length; i++) {
            if (this.isNumeric(selectedLines[i]) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only numbers could be used as selected lines values!";
                }
                return;
            }
            if (this.props.store.lines_shelf.has(parseInt(selectedLines[i])) === false) {
                if (selectedLinesField.classList.contains("highlighted") === false) {
                    selectedLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed lines numbers could be used as selected lines values!";
                }
                return;
            }
        }
        selectedLines = selectedLines.map((item) => parseInt(item));
        const selectedLinesSet = new Set(selectedLines);
        let difference = new Set([...this.state.assignToLines].filter(
            (lineNumber) => !selectedLinesSet.has(lineNumber)));
        this.state.assignToLines = difference;
        this.state.selectedLines.clear();
        this.updateSelectedLinesField();
        this.updateAssignToLinesField();
    }

    updateSelectedLinesField() {
        let selectedLinesFieldValue = "";
        for (let item of this.state.selectedLines) {
            selectedLinesFieldValue += `${item}, `
        }
        this.shadowRoot.querySelector(".selected-lines").value = selectedLinesFieldValue;
    }

    updateAssignToLinesField() {
        let assignToLinesFieldValue = "";
        for (let item of this.state.assignToLines) {
            assignToLinesFieldValue += `${item}, `
        }
        this.shadowRoot.querySelector(".assign-to-lines").value = assignToLinesFieldValue;
    }

    defineLocalAxis1DirectionOptions() {
        const localAxis1DirectionSelect = this.shadowRoot.querySelector(".local-axis-1-direction");
        for (let i = localAxis1DirectionSelect.length - 1; i >= 0; i--) {
            localAxis1DirectionSelect.options[i] = null;
        }
        for (let i = 0; i < this.props.store.beam_sections_local_axis1_directions_shelf.length; i++) {
            let localAxis1DirectionOption = document.createElement("option");
            localAxis1DirectionOption.value = this.props.store.beam_sections_local_axis1_directions_shelf[i];
            localAxis1DirectionOption.innerHTML = this.props.store.beam_sections_local_axis1_directions_shelf[i];
            localAxis1DirectionSelect.appendChild(localAxis1DirectionOption);
        }
        if (localAxis1DirectionSelect.value !== "") {
            this.updateSelectedBeamSectionOrientationData(localAxis1DirectionSelect.value);
        } else {
            this.state.assignToLines.clear();
            this.shadowRoot.querySelector(".assign-to-lines").value = "";
        }
    }

    updateSelectedBeamSectionOrientationData(selectedLocalAxis1Direction) {
        const localAxis1Direction = selectedLocalAxis1Direction
            .split(",")
            .map((item) => parseFloat(item))
            .filter((item) => item !== "");
        let assignedToLines =  this.props.store.getLineNumbersWithLocalAxis1Direction(localAxis1Direction);
        assignedToLines.sort();
        this.state.assignToLines = new Set(assignedToLines);
        let assignToLinesFieldValue = "";
        for (let k = 0; k < assignedToLines.length; k++) {
            assignToLinesFieldValue += `${assignedToLines[k]}, `
        }
        this.shadowRoot.querySelector(".assign-to-lines").value = assignToLinesFieldValue;
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        const highlightedElement = this.shadowRoot.querySelector(".local-axis-1-direction");
        this.dropHighlight(highlightedElement);
    }

    previewBeamSectionOrientationOnSelectedLines() {
        const localAxis1DirectionSelect = this.shadowRoot.querySelector(".local-axis-1-direction");
        if (localAxis1DirectionSelect.value == "") {
            return;
        }
        const assignToLinesField = this.shadowRoot.querySelector(".assign-to-lines");
        const assignToLines = assignToLinesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");
        for (let i = 0; i < assignToLines.length; i++) {
            if (this.isNumeric(assignToLines[i]) === false) {
                if (assignToLinesField.classList.contains("highlighted") === false) {
                    assignToLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only numbers could be used as assign to lines values!";
                }
                return;
            }
            if (this.props.store.lines_shelf.has(parseInt(assignToLines[i])) === false) {
                if (assignToLinesField.classList.contains("highlighted") === false) {
                    assignToLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed lines numbers could be used as assign to lines values!";
                }
                return;
            }
            assignToLines[i] = Number.parseInt(assignToLines[i]);
        }
        if (assignToLines.length > 0) {
            this.dispatchEvent(new CustomEvent(PREVIEW_SELECTED_LINE_NUMBERS_MESSAGE_HEADER, {
                bubbles: true,
                composed: true,
                detail: { "line_numbers": assignToLines },
            }));
        }
    }

    filter(keywordField, selectField) {
        for (let i = 0; i < selectField.length; i++) {
            let txt = selectField.options[i].value;
            if (txt.substring(0, keywordField.length).toLowerCase() !== keywordField.toLowerCase() && 
                keywordField.trim() !== "") {
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

    updateBeamSectionOrientationData() {
        const selectedLocalAxis1DirectionField = this.shadowRoot.querySelector(".local-axis-1-direction");
        if (selectedLocalAxis1DirectionField.value === "") {
            if (selectedLocalAxis1DirectionField.classList.contains("highlighted") === false) {
                selectedLocalAxis1DirectionField.classList.add("highlighted");
            }
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        const selectedLocalAxis1Direction = selectedLocalAxis1DirectionField.value
            .split(",")
            .map((item) => parseFloat(item))
            .filter((item) => item !== "");

        const assignToLinesField = this.shadowRoot.querySelector(".assign-to-lines");
        const assignToLines = assignToLinesField.value
            .split(",")
            .map((item) => item.replace(/\s/g,'', ""))
            .filter((item) => item !== "");

        for (let i = 0; i < assignToLines.length; i++) {
            if (this.isNumeric(assignToLines[i]) === false || this.isInt(assignToLines[i]) === false) {
                if (assignToLinesField.classList.contains("highlighted") === false) {
                    assignToLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only integer numbers could be used as assign to lines values!";
                }
                return;
            }
            
            if (this.props.store.lines_shelf.has(parseInt(assignToLines[i])) === false) {
                if (assignToLinesField.classList.contains("highlighted") === false) {
                    assignToLinesField.classList.add("highlighted");
                }
                if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                    this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                        "Note: Only existed lines numbers could be used as assign to lines values!";
                }
                return;
            }
        }

        const oldLineNumbers = this.props.store.getLineNumbersWithLocalAxis1Direction(selectedLocalAxis1Direction);

        const lineNumbers = assignToLines.map((item) => parseInt(item));

        const message = { 
            [ASSIGN_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER]: { 
                action_id: this.props.actionId,
                local_axis_1_direction: selectedLocalAxis1Direction,
                old_beam_section_orientation_values: {
                    line_numbers: oldLineNumbers,
                },
                new_beam_section_orientation_values: {
                    line_numbers: lineNumbers,
                },
            } 
        };
        this.dispatchEvent(new CustomEvent([CLIENT_MESSAGE_HEADER], {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
        this.shadowRoot.querySelector(".local-axis-1-direction-filter").value = null;
    }

    cancelBeamSectionOrientationDataUpdate() {
        this.shadowRoot.querySelector(".local-axis-1-direction-filter").value = null;
        if (this.props.store.beam_sections_local_axis1_directions_shelf.length > 0) {
            this.defineLocalAxis1DirectionOptions();
        }
        const selectedLocalAxis1DirectionForAssignField = this.shadowRoot.querySelector(".local-axis-1-direction");
        this.dropHighlight(selectedLocalAxis1DirectionForAssignField);
        const assignToLinesField = this.shadowRoot.querySelector(".assign-to-lines");
        this.dropHighlight(assignToLinesField);
        this.state.selectedLines.clear();
        this.updateSelectedLinesField();
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

export default FeaPropertiesBeamSectionOrientationMenu;
