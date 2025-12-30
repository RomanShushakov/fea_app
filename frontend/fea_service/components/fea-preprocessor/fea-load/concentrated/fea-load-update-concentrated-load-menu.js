import Store from "../../../../store/fea_store.js";
import { UPDATE_CONCENTRATED_LOAD_MESSAGE_HEADER } from "../../../../consts/actions_router_consts.js";
import { CLIENT_MESSAGE_HEADER, GET_WASM_LOADING_STATUS_MESSAGE_HEADER } from "../../../../consts/fea_app_consts.js";


class FeaLoadUpdateConcentratedLoadMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,                 // u32;
            isWasmLoaded: false,         // load status of wasm modules;
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

            .point-number-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin: 0rem;
                align-items: center;
            }

            .point-number-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .point-number-select-filter-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: column;
            }

            .point-number-filter-label {
                position: relative;
            }
                
            .point-number-filter-label:before {
                content: "";
                position: absolute;
                left: 0rem;
                top: 0rem;
                bottom: 0rem;
                width: 0.8rem;
                background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
            }

            .point-number-filter {
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

            .point-number-filter::placeholder {
                font-size: 85%;
            }

            .point-number-filter::-webkit-outer-spin-button,
            .point-number-filter::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .point-number-filter[type=number] {
                -moz-appearance: textfield;
            }

            .point-number-filter:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .point-number-filter:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .point-number {
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

            .point-number option {
                background-color: var(--preprocessor-menu-content-dropdown-menu-bg-color);
            }

            .point-number:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .fx-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .fx-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .fx {
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

            .fx[type=number]::-webkit-outer-spin-button,
            .fx[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .fx[type=number] {
                -moz-appearance: textfield;
            }

            .fx:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .fx:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .fy-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .fy-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .fy {
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

            .fy[type=number]::-webkit-outer-spin-button,
            .fy[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .fy[type=number] {
                -moz-appearance: textfield;
            }

            .fy:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .fy:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .fz-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .fz-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .fz {
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

            .fz[type=number]::-webkit-outer-spin-button,
            .fz[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .fz[type=number] {
                -moz-appearance: textfield;
            }

            .fz:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .fz:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .mx-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .mx-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .mx {
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

            .mx[type=number]::-webkit-outer-spin-button,
            .mx[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .mx[type=number] {
                -moz-appearance: textfield;
            }

            .mx:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .mx:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .my-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .my-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .my {
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

            .my[type=number]::-webkit-outer-spin-button,
            .my[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .my[type=number] {
                -moz-appearance: textfield;
            }

            .my:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .my:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .mz-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .mz-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .mz {
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

            .mz[type=number]::-webkit-outer-spin-button,
            .mz[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .mz[type=number] {
                -moz-appearance: textfield;
            }

            .mz:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .mz:focus {
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

            <div class="point-number-field-content">
                <p class="point-number-caption">Point number</p>
                <div class="point-number-select-filter-content">
                    <label class="point-number-filter-label">
                        <input class="point-number-filter" type="number" placeholder="Filter..."/>
                    </label>
                    <select class="point-number"></select>
                </div>
            </div>

            <div class="fx-field-content">
                <p class="fx-caption">Fx</p>
                <input class="fx" type="number"/>
            </div>

            <div class="fy-field-content">
                <p class="fy-caption">Fy</p>
                <input class="fy" type="number"/>
            </div>

            <div class="fz-field-content">
                <p class="fz-caption">Fz</p>
                <input class="fz" type="number"/>
            </div>

            <div class="mx-field-content">
                <p class="mx-caption">Mx</p>
                <input class="mx" type="number"/>
            </div>

            <div class="my-field-content">
                <p class="my-caption">My</p>
                <input class="my" type="number"/>
            </div>

            <div class="mz-field-content">
                <p class="mz-caption">Mz</p>
                <input class="mz" type="number"/>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.updateConcentratedLoad());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelConcentratedLoadUpdate());

        this.shadowRoot.querySelector(".point-number").addEventListener("change", () => this.updateConcentratedLoadValues());

        this.shadowRoot.querySelector(".point-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".point-number-filter").value,
                this.shadowRoot.querySelector(".point-number"));
        });

        this.shadowRoot.querySelector(".fx").addEventListener("click", () => {
            const inputtedFXField = this.shadowRoot.querySelector(".fx");
            this.dropHighlight(inputtedFXField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".fy").addEventListener("click", () => {
            const inputtedFYField = this.shadowRoot.querySelector(".fy");
            this.dropHighlight(inputtedFYField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".fz").addEventListener("click", () => {
            const inputtedFZField = this.shadowRoot.querySelector(".fz");
            this.dropHighlight(inputtedFZField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".mx").addEventListener("click", () => {
            const inputtedMXField = this.shadowRoot.querySelector(".mx");
            this.dropHighlight(inputtedMXField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".my").addEventListener("click", () => {
            const inputtedMYField = this.shadowRoot.querySelector(".my");
            this.dropHighlight(inputtedMYField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".mz").addEventListener("click", () => {
            const inputtedMZField = this.shadowRoot.querySelector(".mz");
            this.dropHighlight(inputtedMZField);
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

    set selectConcentratedLoadInClient(pointNumber) {
        const frame = () => {
            if (this.props.isWasmLoaded === true) {
                clearInterval(id);
                const pointNumberSelect = this.shadowRoot.querySelector(".point-number");
                const pointNumberOptions = pointNumberSelect.options;
                for (let option, i = 0; option = pointNumberOptions[i]; i++) {
                    if (option.value == pointNumber) {
                        pointNumberSelect.selectedIndex = i;
                        break;
                    }
                }
                this.updateConcentratedLoadValues();
            }
        }
        const id = setInterval(frame, 10);
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
                this.defineConcentratedLoadOptions();
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
            this.defineConcentratedLoadOptions();
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

    defineConcentratedLoadOptions() {
        const pointNumberSelect = this.shadowRoot.querySelector(".point-number");
        for (let i = pointNumberSelect.length - 1; i >= 0; i--) {
            pointNumberSelect.options[i] = null;
        }
        if (this.props.store.concentrated_loads_shelf.size > 0) {
            const pointsNumbers = Array.from(this.props.store.concentrated_loads_shelf.keys()).sort((a, b) => a - b);
            for (let i = 0; i < pointsNumbers.length; i++) {
                let pointNumberOption = document.createElement("option");
                pointNumberOption.value = pointsNumbers[i];
                pointNumberOption.innerHTML = pointsNumbers[i];
                pointNumberSelect.appendChild(pointNumberOption);
            }

            if (this.props.store.concentrated_loads_shelf.get(pointsNumbers[0]) !== undefined) {
                this.shadowRoot.querySelector(".fx").value = this.props.store.concentrated_loads_shelf.get(pointsNumbers[0]).fx;
                this.shadowRoot.querySelector(".fy").value = this.props.store.concentrated_loads_shelf.get(pointsNumbers[0]).fy;
                this.shadowRoot.querySelector(".fz").value = this.props.store.concentrated_loads_shelf.get(pointsNumbers[0]).fz;
                this.shadowRoot.querySelector(".mx").value = this.props.store.concentrated_loads_shelf.get(pointsNumbers[0]).mx;
                this.shadowRoot.querySelector(".my").value = this.props.store.concentrated_loads_shelf.get(pointsNumbers[0]).my;
                this.shadowRoot.querySelector(".mz").value = this.props.store.concentrated_loads_shelf.get(pointsNumbers[0]).mz;
            } else {
                this.shadowRoot.querySelector(".fx").value = "";
                this.shadowRoot.querySelector(".fy").value = "";
                this.shadowRoot.querySelector(".fz").value = "";
                this.shadowRoot.querySelector(".mx").value = "";
                this.shadowRoot.querySelector(".my").value = "";
                this.shadowRoot.querySelector(".mz").value = "";
            }

        } else {
            this.shadowRoot.querySelector(".fx").value = "";
            this.shadowRoot.querySelector(".fy").value = "";
            this.shadowRoot.querySelector(".fz").value = "";
            this.shadowRoot.querySelector(".mx").value = "";
            this.shadowRoot.querySelector(".my").value = "";
            this.shadowRoot.querySelector(".mz").value = "";
        }
    }

    updateConcentratedLoadValues() {
        const selectedPointNumber = this.shadowRoot.querySelector(".point-number").value;
        this.shadowRoot.querySelector(".fx").value = this.props.store.concentrated_loads_shelf.get(parseInt(selectedPointNumber)).fx;
        this.dropHighlight(this.shadowRoot.querySelector(".fx"));
        this.shadowRoot.querySelector(".fy").value = this.props.store.concentrated_loads_shelf.get(parseInt(selectedPointNumber)).fy;
        this.dropHighlight(this.shadowRoot.querySelector(".fy"));
        this.shadowRoot.querySelector(".fz").value = this.props.store.concentrated_loads_shelf.get(parseInt(selectedPointNumber)).fz;
        this.dropHighlight(this.shadowRoot.querySelector(".fz"));
        this.shadowRoot.querySelector(".mx").value = this.props.store.concentrated_loads_shelf.get(parseInt(selectedPointNumber)).mx;
        this.dropHighlight(this.shadowRoot.querySelector(".mx"));
        this.shadowRoot.querySelector(".my").value = this.props.store.concentrated_loads_shelf.get(parseInt(selectedPointNumber)).my;
        this.dropHighlight(this.shadowRoot.querySelector(".my"));
        this.shadowRoot.querySelector(".mz").value = this.props.store.concentrated_loads_shelf.get(parseInt(selectedPointNumber)).mz;
        this.dropHighlight(this.shadowRoot.querySelector(".mz"));
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

    updateConcentratedLoad() {
        const selectedPointNumberField = this.shadowRoot.querySelector(".point-number");
        if (selectedPointNumberField.value == "") {
            if (selectedPointNumberField.classList.contains("highlighted") === false) {
                selectedPointNumberField.classList.add("highlighted");
            }
        }

        const inputtedFXField = this.shadowRoot.querySelector(".fx");
        if (inputtedFXField.value == "") {
            if (inputtedFXField.classList.contains("highlighted") === false) {
                inputtedFXField.classList.add("highlighted");
            }
        }
        const inputtedFYField = this.shadowRoot.querySelector(".fy");
        if (inputtedFYField.value == "") {
            if (inputtedFYField.classList.contains("highlighted") === false) {
                inputtedFYField.classList.add("highlighted");
            }
        }

        const inputtedFZField = this.shadowRoot.querySelector(".fz");
        if (inputtedFZField.value == "") {
            if (inputtedFZField.classList.contains("highlighted") === false) {
                inputtedFZField.classList.add("highlighted");
            }
        }

        const inputtedMXField = this.shadowRoot.querySelector(".mx");
        if (inputtedMXField.value == "") {
            if (inputtedMXField.classList.contains("highlighted") === false) {
                inputtedMXField.classList.add("highlighted");
            }
        }
        const inputtedMYField = this.shadowRoot.querySelector(".my");
        if (inputtedMYField.value == "") {
            if (inputtedMYField.classList.contains("highlighted") === false) {
                inputtedMYField.classList.add("highlighted");
            }
        }

        const inputtedMZField = this.shadowRoot.querySelector(".mz");
        if (inputtedMZField.value == "") {
            if (inputtedMZField.classList.contains("highlighted") === false) {
                inputtedMZField.classList.add("highlighted");
            }
        }

        if (selectedPointNumberField.value === "" || 
            inputtedFXField.value === "" || inputtedFYField.value === "" || inputtedFZField.value === "" ||
            inputtedMXField.value === "" || inputtedMYField.value === "" || inputtedMZField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        if (this.isNumeric(selectedPointNumberField.value) === false || 
            this.isNumeric(inputtedFXField.value) === false || 
            this.isNumeric(inputtedFYField.value) === false || 
            this.isNumeric(inputtedFZField.value) === false ||
            this.isNumeric(inputtedMXField.value) === false || 
            this.isNumeric(inputtedMYField.value) === false || 
            this.isNumeric(inputtedMZField.value) === false) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values!";
                return;
            } else {
                return;
            }
        }

        const oldConcentratedLoadValues = this.props.store.concentrated_loads_shelf.get(parseInt(selectedPointNumberField.value));

        const message = { [UPDATE_CONCENTRATED_LOAD_MESSAGE_HEADER]: {
            action_id: this.props.actionId,
            point_number: selectedPointNumberField.value, 
            old_concentrated_load_values: 
                { 
                    fx: oldConcentratedLoadValues.fx, fy: oldConcentratedLoadValues.fy,
                    fz: oldConcentratedLoadValues.fz, mx: oldConcentratedLoadValues.mx,
                    my: oldConcentratedLoadValues.my, mz: oldConcentratedLoadValues.mz,   
                },
            new_concentrated_load_values: 
                { 
                    fx: inputtedFXField.value, fy: inputtedFYField.value,
                    fz: inputtedFZField.value, mx: inputtedMXField.value,
                    my: inputtedMYField.value, mz: inputtedMZField.value,
                }
        }};

        this.dispatchEvent(new CustomEvent([CLIENT_MESSAGE_HEADER], {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));

        this.shadowRoot.querySelector(".point-number-filter").value = null;
    }

    cancelConcentratedLoadUpdate() {
        if (this.props.store.concentrated_loads_shelf.size > 0) {
            this.defineConcentratedLoadOptions();
        }
        this.shadowRoot.querySelector(".point-number-filter").value = null;
        const selectedPointNumberForUpdateField = this.shadowRoot.querySelector(".point-number");
        this.dropHighlight(selectedPointNumberForUpdateField);
        const inputtedFXField = this.shadowRoot.querySelector(".fx");
        this.dropHighlight(inputtedFXField);
        const inputtedFYField = this.shadowRoot.querySelector(".fy");
        this.dropHighlight(inputtedFYField);
        const inputtedFZField = this.shadowRoot.querySelector(".fz");
        this.dropHighlight(inputtedFZField);
        const inputtedMXField = this.shadowRoot.querySelector(".mx");
        this.dropHighlight(inputtedMXField);
        const inputtedMYField = this.shadowRoot.querySelector(".my");
        this.dropHighlight(inputtedMYField);
        const inputtedMZField = this.shadowRoot.querySelector(".mz");
        this.dropHighlight(inputtedMZField);
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

export default FeaLoadUpdateConcentratedLoadMenu;
