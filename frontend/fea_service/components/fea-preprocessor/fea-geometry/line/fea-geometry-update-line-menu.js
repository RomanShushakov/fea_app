import Store from "../../../../store/fea_store.js";
import { UPDATE_LINE_MESSAGE_HEADER } from "../../../../consts/actions_router_consts.js";
import { CLIENT_MESSAGE_HEADER, GET_WASM_LOADING_STATUS_MESSAGE_HEADER } from "../../../../consts/fea_app_consts.js";


class FeaGeometryUpdateLineMenu extends HTMLElement {
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

            .line-number-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin: 0rem;
                align-items: center;
            }

            .line-number-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .line-number-select-filter-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: column;
            }

            .line-number-filter-label {
                position: relative;
            }
                
            .line-number-filter-label:before {
                content: "";
                position: absolute;
                left: 0rem;
                top: 0rem;
                bottom: 0rem;
                width: 0.8rem;
                background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
            }

            .line-number-filter {
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

            .line-number-filter::placeholder {
                font-size: 85%;
            }

            .line-number-filter::-webkit-outer-spin-button,
            .line-number-filter::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .line-number-filter[type=number] {
                -moz-appearance: textfield;
            }

            .line-number-filter:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .line-number-filter:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .line-number {
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

            .line-number option {
                background-color: var(--preprocessor-menu-content-dropdown-menu-bg-color);
            }

            .line-number:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .point-1-number-field-content {
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

            .point-1-number-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .point-1-number-select-filter-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: column;
            }

            .point-1-number-filter-label {
                position: relative;
            }
                
            .point-1-number-filter-label:before {
                content: "";
                position: absolute;
                left: 0rem;
                top: 0rem;
                bottom: 0rem;
                width: 0.8rem;
                background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
            }

            .point-1-number-filter {
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

            .point-1-number-filter::placeholder {
                font-size: 85%;
            }

            .point-1-number-filter::-webkit-outer-spin-button,
            .point-1-number-filter::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .point-1-number-filter[type=number] {
                -moz-appearance: textfield;
            }

            .point-1-number-filter:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .point-1-number-filter:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .point-1-number {
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

            .point-1-number option {
                background-color: var(--preprocessor-menu-content-dropdown-menu-bg-color);
            }

            .point-1-number:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .point-2-number-field-content {
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

            .point-2-number-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .point-2-number-select-filter-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: column;
            }

            .point-2-number-filter-label {
                position: relative;
            }
                
            .point-2-number-filter-label:before {
                content: "";
                position: absolute;
                left: 0rem;
                top: 0rem;
                bottom: 0rem;
                width: 0.8rem;
                background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
            }

            .point-2-number-filter {
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

            .point-2-number-filter::placeholder {
                font-size: 85%;
            }

            .point-2-number-filter::-webkit-outer-spin-button,
            .point-2-number-filter::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .point-2-number-filter[type=number] {
                -moz-appearance: textfield;
            }

            .point-2-number-filter:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .point-2-number-filter:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .point-2-number {
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

            .point-2-number option {
                background-color: var(--preprocessor-menu-content-dropdown-menu-bg-color);
            }

            .point-2-number:hover {
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
            <div class="line-number-field-content">
                <p class="line-number-caption">Line number</p>
                <div class="line-number-select-filter-content">
                    <label class="line-number-filter-label">
                        <input class="line-number-filter" type="number" placeholder="Filter..."/>
                    </label>
                    <select class="line-number"></select>
                </div>
            </div>

            <div class="point-1-number-field-content">
                <p class="point-1-number-caption">Start point number</p>
                <div class="point-1-number-select-filter-content">
                    <label class="point-1-number-filter-label">
                        <input class="point-1-number-filter" type="number" placeholder="Filter..."/>
                    </label>
                    <select class="point-1-number"></select>
                </div>
            </div>

            <div class="point-2-number-field-content">
                <p class="point-2-number-caption">End point number</p>
                <div class="point-2-number-select-filter-content">
                    <label class="point-2-number-filter-label">
                        <input class="point-2-number-filter" type="number" placeholder="Filter..."/>
                    </label>
                    <select class="point-2-number"></select>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.updateLine());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelLineUpdate());

        this.shadowRoot.querySelector(".line-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".line-number-filter").value,
                this.shadowRoot.querySelector(".line-number"));
        });

        this.shadowRoot.querySelector(".line-number").addEventListener("change", 
            (event) => this.updateSelectedLinePoint1Andpoint2Numbers(parseInt(event.target.value)));

        this.shadowRoot.querySelector(".point-1-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".point-1-number-filter").value,
                this.shadowRoot.querySelector(".point-1-number"));
        });

        this.shadowRoot.querySelector(".point-1-number").addEventListener("change", 
            () => this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "");

        this.shadowRoot.querySelector(".point-2-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".point-2-number-filter").value,
                this.shadowRoot.querySelector(".point-2-number"));
        });

        this.shadowRoot.querySelector(".point-2-number").addEventListener("change", 
            () => this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "");
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
                this.defineLineNumberOptions();
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
            this.defineLineNumberOptions();
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

    defineLineNumberOptions() {
        const lineUpdateNumberSelect = this.shadowRoot.querySelector(".line-number");
        for (let i = lineUpdateNumberSelect.length - 1; i >= 0; i--) {
            lineUpdateNumberSelect.options[i] = null;
        }
        this.definePoint1NumberOptions();
        this.definepoint2NumberOptions();
        if (this.props.store.lines_shelf.size > 0) {
            const linesNumbers = Array.from(this.props.store.lines_shelf.keys()).sort((a, b) => a - b);
            for (let i = 0; i < linesNumbers.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = linesNumbers[i];
                updateOption.innerHTML = linesNumbers[i];
                lineUpdateNumberSelect.appendChild(updateOption);
            }
            const selectedLinePoint1Number = this.props.store.lines_shelf.get(linesNumbers[0]).point_1_number;
            const selectedLinepoint2Number = this.props.store.lines_shelf.get(linesNumbers[0]).point_2_number;
            const point1NumberSelect = this.shadowRoot.querySelector(".point-1-number");
            const point1NumberOptions = point1NumberSelect.options;
            for (let option, i = 0; option = point1NumberOptions[i]; i++) {
                if (option.value == selectedLinePoint1Number) {
                    point1NumberSelect.selectedIndex = i;
                    break;
                }
            }
            const point2NumberSelect = this.shadowRoot.querySelector(".point-2-number");
            const point2NumberOptions = point2NumberSelect.options;
            for (let option, i = 0; option = point2NumberOptions[i]; i++) {
                if (option.value == selectedLinepoint2Number) {
                    point2NumberSelect.selectedIndex = i;
                    break;
                }
            }
        }
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

    definePoint1NumberOptions() {
        const point1NumberSelect = this.shadowRoot.querySelector(".point-1-number");
        for (let i = point1NumberSelect.length - 1; i >= 0; i--) {
            point1NumberSelect.options[i] = null;
        }
        if (this.props.store.points_shelf.size > 0) {
            const pointsNumbers = Array.from(this.props.store.points_shelf.keys()).sort((a, b) => a - b);
            for (let i = 0; i < pointsNumbers.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = pointsNumbers[i];
                updateOption.innerHTML = pointsNumbers[i];
                point1NumberSelect.appendChild(updateOption);
            }
        }
    }

    definepoint2NumberOptions() {
        const point2NumberSelect = this.shadowRoot.querySelector(".point-2-number");
        for (let i = point2NumberSelect.length - 1; i >= 0; i--) {
            point2NumberSelect.options[i] = null;
        }
        if (this.props.store.points_shelf.size > 0) {
            const pointsNumbers = Array.from(this.props.store.points_shelf.keys()).sort((a, b) => a - b);
            for (let i = 0; i < pointsNumbers.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = pointsNumbers[i];
                updateOption.innerHTML = pointsNumbers[i];
                point2NumberSelect.appendChild(updateOption);
            }
        }
    }

    updateSelectedLinePoint1Andpoint2Numbers(selectedLineNumber) {
        const selectedLinePoint1Number = this.props.store.lines_shelf.get(selectedLineNumber).point_1_number;
        const selectedLinepoint2Number = this.props.store.lines_shelf.get(selectedLineNumber).point_2_number;
        const point1NumberSelect = this.shadowRoot.querySelector(".point-1-number");
        const point1NumberOptions =  point1NumberSelect.options;
        for (let option, i = 0; option = point1NumberOptions[i]; i++) {
            if (option.value == selectedLinePoint1Number) {
                point1NumberSelect.selectedIndex = i;
                break;
            }
        }
        const point2NumberSelect =  this.shadowRoot.querySelector(".point-2-number");
        const point2NumberOptions =  point2NumberSelect.options;
        for (let option, i = 0; option = point2NumberOptions[i]; i++) {
            if (option.value == selectedLinepoint2Number) {
                point2NumberSelect.selectedIndex = i;
                break;
            }
        }
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    updateLine() {
        const selectedLineNumberField = this.shadowRoot.querySelector(".line-number");

        if (selectedLineNumberField.value == "") {
            if (selectedLineNumberField.classList.contains("highlighted") === false) {
                selectedLineNumberField.classList.add("highlighted");
            }
        }

        const point1Field = this.shadowRoot.querySelector(".point-1-number");
        if (point1Field.value == "") {
            if (point1Field.classList.contains("highlighted") === false) {
                point1Field.classList.add("highlighted");
            }
        }

        const point2Field = this.shadowRoot.querySelector(".point-2-number");
        if (point2Field.value == "") {
            if (point2Field.classList.contains("highlighted") === false) {
                point2Field.classList.add("highlighted");
            }
        }

        if (selectedLineNumberField.value == "" || point1Field.value == "" || point2Field.value == "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        const oldLineValues = this.props.store.lines_shelf.get(parseInt(selectedLineNumberField.value));

        const message = { [UPDATE_LINE_MESSAGE_HEADER]: {
            action_id: this.props.actionId,
            number: selectedLineNumberField.value, 
            old_line_values: { point_1_number:  oldLineValues.point_1_number, point_2_number: oldLineValues.point_2_number },
            new_line_values: { point_1_number:  point1Field.value, point_2_number: point2Field.value }
        }};

        this.dispatchEvent(new CustomEvent([CLIENT_MESSAGE_HEADER], {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));

        this.shadowRoot.querySelector(".line-number-filter").value = null;
        this.shadowRoot.querySelector(".point-1-number-filter").value = null;
        this.shadowRoot.querySelector(".point-2-number-filter").value = null;
    }

    cancelLineUpdate() {
        this.defineLineNumberOptions();
        this.shadowRoot.querySelector(".line-number-filter").value = null;
        this.shadowRoot.querySelector(".point-1-number-filter").value = null;
        this.shadowRoot.querySelector(".point-2-number-filter").value = null;
        const selectedLineNumberField = this.shadowRoot.querySelector(".line-number");
        this.dropHighlight(selectedLineNumberField);
        const point1Field = this.shadowRoot.querySelector(".point-1-number");
        this.dropHighlight(point1Field);
        const point2Field = this.shadowRoot.querySelector(".point-2-number");
        this.dropHighlight(point2Field);
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    dropHighlight(highlightedElement) {
        if (highlightedElement.classList.contains("highlighted") === true) {
            highlightedElement.classList.remove("highlighted");
        }
    }
}

export default FeaGeometryUpdateLineMenu;
