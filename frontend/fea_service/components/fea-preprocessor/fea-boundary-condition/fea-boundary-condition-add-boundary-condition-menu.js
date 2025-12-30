import Store from "../../../store/fea_store.js";
import { ADD_POINT_BOUNDARY_CONDITION_MESSAGE_HEADER } from "../../../consts/actions_router_consts.js";
import { CLIENT_MESSAGE_HEADER, GET_WASM_LOADING_STATUS_MESSAGE_HEADER } from "../../../consts/fea_app_consts.js";


class FeaBoundaryConditionAddBoundaryConditionMenu extends HTMLElement {
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

            .optional-ux-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .optional-ux-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .optional-ux {
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

            .optional-ux[type=number]::-webkit-outer-spin-button,
            .optional-ux[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .optional-ux[type=number] {
                -moz-appearance: textfield;
            }

            .optional-ux:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .optional-ux:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .optional-uy-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .optional-uy-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .optional-uy {
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

            .optional-uy[type=number]::-webkit-outer-spin-button,
            .optional-uy[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .optional-uy[type=number] {
                -moz-appearance: textfield;
            }

            .optional-uy:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .optional-uy:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .optional-uz-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .optional-uz-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .optional-uz {
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

            .optional-uz[type=number]::-webkit-outer-spin-button,
            .optional-uz[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .optional-uz[type=number] {
                -moz-appearance: textfield;
            }

            .optional-uz:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .optional-uz:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .optional-rx-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .optional-rx-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .optional-rx {
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

            .optional-rx[type=number]::-webkit-outer-spin-button,
            .optional-rx[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .optional-rx[type=number] {
                -moz-appearance: textfield;
            }

            .optional-rx:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .optional-rx:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .optional-ry-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .optional-ry-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .optional-ry {
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

            .optional-ry[type=number]::-webkit-outer-spin-button,
            .optional-ry[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .optional-ry[type=number] {
                -moz-appearance: textfield;
            }

            .optional-ry:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .optional-ry:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .optional-rz-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .optional-rz-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .optional-rz {
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

            .optional-rz[type=number]::-webkit-outer-spin-button,
            .optional-rz[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .optional-rz[type=number] {
                -moz-appearance: textfield;
            }

            .optional-rz:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .optional-rz:focus {
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

            <div class="point-number-field-content">
                <p class="point-number-caption">Point number</p>
                <div class="point-number-select-filter-content">
                    <label class="point-number-filter-label">
                        <input class="point-number-filter" type="number" placeholder="Filter..."/>
                    </label>
                    <select class="point-number"></select>
                </div>
            </div>

            <div class="optional-ux-field-content">
                <p class="optional-ux-caption">Ux</p>
                <input class="optional-ux" type="number"/>
            </div>

            <div class="optional-uy-field-content">
                <p class="optional-uy-caption">Uy</p>
                <input class="optional-uy" type="number"/>
            </div>

            <div class="optional-uz-field-content">
                <p class="optional-uz-caption">Uz</p>
                <input class="optional-uz" type="number"/>
            </div>

            <div class="optional-rx-field-content">
                <p class="optional-rx-caption">Rx</p>
                <input class="optional-rx" type="number"/>
            </div>

            <div class="optional-ry-field-content">
                <p class="optional-ry-caption">Ry</p>
                <input class="optional-ry" type="number"/>
            </div>

            <div class="optional-rz-field-content">
                <p class="optional-rz-caption">Rz</p>
                <input class="optional-rz" type="number"/>
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

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.addBoundaryCondition());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelBoundaryConditionAddition());

        this.shadowRoot.querySelector(".point-number").addEventListener("change", () => this.updateBoundaryConditionValues());

        this.shadowRoot.querySelector(".point-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".point-number-filter").value,
                this.shadowRoot.querySelector(".point-number"));
        });

        this.shadowRoot.querySelector(".optional-ux").addEventListener("click", () => {
            const inputtedUXField = this.shadowRoot.querySelector(".optional-ux");
            this.dropHighlight(inputtedUXField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".optional-uy").addEventListener("click", () => {
            const inputtedUYField = this.shadowRoot.querySelector(".optional-uy");
            this.dropHighlight(inputtedUYField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".optional-uz").addEventListener("click", () => {
            const inputtedUZField = this.shadowRoot.querySelector(".optional-uz");
            this.dropHighlight(inputtedUZField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".optional-rx").addEventListener("click", () => {
            const inputtedRXField = this.shadowRoot.querySelector(".optional-rx");
            this.dropHighlight(inputtedRXField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".optional-ry").addEventListener("click", () => {
            const inputtedRYField = this.shadowRoot.querySelector(".optional-ry");
            this.dropHighlight(inputtedRYField);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".optional-rz").addEventListener("click", () => {
            const inputtedRZField = this.shadowRoot.querySelector(".optional-rz");
            this.dropHighlight(inputtedRZField);
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
                this.defineBoundaryConditionOptions();
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
            this.defineBoundaryConditionOptions();
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

    defineBoundaryConditionOptions() {
        const pointNumberSelect = this.shadowRoot.querySelector(".point-number");
        for (let i = pointNumberSelect.length - 1; i >= 0; i--) {
            pointNumberSelect.options[i] = null;
        }
        if (this.props.store.points_shelf.size > 0) {
            const pointsNumbers = Array.from(this.props.store.points_shelf.keys()).sort((a, b) => a - b);
            for (let i = 0; i < pointsNumbers.length; i++) {
                let pointNumberOption = document.createElement("option");
                pointNumberOption.value = pointsNumbers[i];
                pointNumberOption.innerHTML = pointsNumbers[i];
                pointNumberSelect.appendChild(pointNumberOption);
            }

            if (this.props.store.point_boundary_conditions_shelf.get(pointsNumbers[0]) !== undefined) {
                this.shadowRoot.querySelector(".optional-ux").value = 
                    this.props.store.point_boundary_conditions_shelf.get(pointsNumbers[0]).optional_ux;
                this.shadowRoot.querySelector(".optional-uy").value = 
                    this.props.store.point_boundary_conditions_shelf.get(pointsNumbers[0]).optional_uy;
                this.shadowRoot.querySelector(".optional-uz").value = 
                    this.props.store.point_boundary_conditions_shelf.get(pointsNumbers[0]).optional_uz;
                this.shadowRoot.querySelector(".optional-rx").value = 
                    this.props.store.point_boundary_conditions_shelf.get(pointsNumbers[0]).optional_rx;
                this.shadowRoot.querySelector(".optional-ry").value = 
                    this.props.store.point_boundary_conditions_shelf.get(pointsNumbers[0]).optional_ry;
                this.shadowRoot.querySelector(".optional-rz").value = 
                    this.props.store.point_boundary_conditions_shelf.get(pointsNumbers[0]).optional_rz;
            } else {
                this.shadowRoot.querySelector(".optional-ux").value = "";
                this.shadowRoot.querySelector(".optional-uy").value = "";
                this.shadowRoot.querySelector(".optional-uz").value = "";
                this.shadowRoot.querySelector(".optional-rx").value = "";
                this.shadowRoot.querySelector(".optional-ry").value = "";
                this.shadowRoot.querySelector(".optional-rz").value = "";
            }

        } else {
            this.shadowRoot.querySelector(".optional-ux").value = "";
            this.shadowRoot.querySelector(".optional-uy").value = "";
            this.shadowRoot.querySelector(".optional-uz").value = "";
            this.shadowRoot.querySelector(".optional-rx").value = "";
            this.shadowRoot.querySelector(".optional-ry").value = "";
            this.shadowRoot.querySelector(".optional-rz").value = "";
        }
    }

    updateBoundaryConditionValues() {
        const selectedPointNumber = this.shadowRoot.querySelector(".point-number").value;
        if (this.props.store.point_boundary_conditions_shelf.get(parseInt(selectedPointNumber)) !== undefined) {
            this.shadowRoot.querySelector(".optional-ux").value = 
                this.props.store.point_boundary_conditions_shelf.get(parseInt(selectedPointNumber)).optional_ux;
            this.dropHighlight(this.shadowRoot.querySelector(".optional-ux"));
            this.shadowRoot.querySelector(".optional-uy").value = 
                this.props.store.point_boundary_conditions_shelf.get(parseInt(selectedPointNumber)).optional_uy;
            this.dropHighlight(this.shadowRoot.querySelector(".optional-uy"));
            this.shadowRoot.querySelector(".optional-uz").value = 
                this.props.store.point_boundary_conditions_shelf.get(parseInt(selectedPointNumber)).optional_uz;
            this.dropHighlight(this.shadowRoot.querySelector(".optional-uz"));
            this.shadowRoot.querySelector(".optional-rx").value = 
                this.props.store.point_boundary_conditions_shelf.get(parseInt(selectedPointNumber)).optional_rx;
            this.dropHighlight(this.shadowRoot.querySelector(".optional-rx"));
            this.shadowRoot.querySelector(".optional-ry").value = 
                this.props.store.point_boundary_conditions_shelf.get(parseInt(selectedPointNumber)).optional_ry;
            this.dropHighlight(this.shadowRoot.querySelector(".optional-ry"));
            this.shadowRoot.querySelector(".optional-rz").value = 
                this.props.store.point_boundary_conditions_shelf.get(parseInt(selectedPointNumber)).optional_rz;
            this.dropHighlight(this.shadowRoot.querySelector(".optional-rz"));
        } else {
            this.shadowRoot.querySelector(".optional-ux").value = "";
            this.dropHighlight(this.shadowRoot.querySelector(".optional-ux"));
            this.shadowRoot.querySelector(".optional-uy").value = "";
            this.dropHighlight(this.shadowRoot.querySelector(".optional-uy"));
            this.shadowRoot.querySelector(".optional-uz").value = "";
            this.dropHighlight(this.shadowRoot.querySelector(".optional-uz"));
            this.shadowRoot.querySelector(".optional-rx").value = "";
            this.dropHighlight(this.shadowRoot.querySelector(".optional-rx"));
            this.shadowRoot.querySelector(".optional-ry").value = "";
            this.dropHighlight(this.shadowRoot.querySelector(".optional-ry"));
            this.shadowRoot.querySelector(".optional-rz").value = "";
            this.dropHighlight(this.shadowRoot.querySelector(".optional-rz"));
        }
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

    addBoundaryCondition() {
        const selectedPointNumberField = this.shadowRoot.querySelector(".point-number");
        if (selectedPointNumberField.value == "") {
            if (selectedPointNumberField.classList.contains("highlighted") === false) {
                selectedPointNumberField.classList.add("highlighted");
            }
        }

        const inputtedUXField = this.shadowRoot.querySelector(".optional-ux");
        const inputtedUYField = this.shadowRoot.querySelector(".optional-uy");
        const inputtedUZField = this.shadowRoot.querySelector(".optional-uz");
        const inputtedRXField = this.shadowRoot.querySelector(".optional-rx");
        const inputtedRYField = this.shadowRoot.querySelector(".optional-ry");
        const inputtedRZField = this.shadowRoot.querySelector(".optional-rz");

        if (selectedPointNumberField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The point number field should be filled!";
                return;
            } else {
                return;
            }
        }

        if (inputtedUXField.value === "" && inputtedUYField.value === "" && inputtedUZField.value === "" &&
            inputtedRXField.value === "" && inputtedRYField.value === "" && inputtedRZField.value === "") {
            if (inputtedUXField.classList.contains("highlighted") === false) {
                inputtedUXField.classList.add("highlighted");
            }
            if (inputtedUYField.classList.contains("highlighted") === false) {
                inputtedUYField.classList.add("highlighted");
            }
            if (inputtedUZField.classList.contains("highlighted") === false) {
                inputtedUZField.classList.add("highlighted");
            }
            if (inputtedRXField.classList.contains("highlighted") === false) {
                inputtedRXField.classList.add("highlighted");
            }
            if (inputtedRYField.classList.contains("highlighted") === false) {
                inputtedRYField.classList.add("highlighted");
            }
            if (inputtedRZField.classList.contains("highlighted") === false) {
                inputtedRZField.classList.add("highlighted");
            }
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: At least one of the highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        if ((selectedPointNumberField.value != "" && this.isNumeric(selectedPointNumberField.value) === false) || 
            (inputtedUXField.value != "" && this.isNumeric(inputtedUXField.value) === false) || 
            (inputtedUYField.value != "" && this.isNumeric(inputtedUYField.value) === false) || 
            (inputtedUZField.value != "" && this.isNumeric(inputtedUZField.value) === false) ||
            (inputtedRXField.value != "" && this.isNumeric(inputtedRXField.value) === false) || 
            (inputtedRYField.value != "" && this.isNumeric(inputtedRYField.value) === false) || 
            (inputtedRZField.value != "" && this.isNumeric(inputtedRZField.value) === false)) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values!";
                return;
            } else {
                return;
            }
        }

        const message = { [ADD_POINT_BOUNDARY_CONDITION_MESSAGE_HEADER]: {
            action_id: this.props.actionId,
            point_number: selectedPointNumberField.value, 
            optional_ux: inputtedUXField.value != "" ? inputtedUXField.value : null,
            optional_uy: inputtedUYField.value != "" ? inputtedUYField.value : null,
            optional_uz: inputtedUZField.value != "" ? inputtedUZField.value : null,
            optional_rx: inputtedRXField.value != "" ? inputtedRXField.value : null,
            optional_ry: inputtedRYField.value != "" ? inputtedRYField.value : null,
            optional_rz: inputtedRZField.value != "" ? inputtedRZField.value : null,
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

    cancelBoundaryConditionAddition() {
        if (this.props.store.points_shelf.size > 0) {
            this.defineBoundaryConditionOptions();
        }
        this.shadowRoot.querySelector(".point-number-filter").value = null;
        const selectedPointNumberField = this.shadowRoot.querySelector(".point-number");
        this.dropHighlight(selectedPointNumberField);
        const inputtedUXField = this.shadowRoot.querySelector(".optional-ux");
        this.dropHighlight(inputtedUXField);
        const inputtedUYField = this.shadowRoot.querySelector(".optional-uy");
        this.dropHighlight(inputtedUYField);
        const inputtedUZField = this.shadowRoot.querySelector(".optional-uz");
        this.dropHighlight(inputtedUZField);
        const inputtedRXField = this.shadowRoot.querySelector(".optional-rx");
        this.dropHighlight(inputtedRXField);
        const inputtedRYField = this.shadowRoot.querySelector(".optional-ry");
        this.dropHighlight(inputtedRYField);
        const inputtedRZField = this.shadowRoot.querySelector(".optional-rz");
        this.dropHighlight(inputtedRZField);
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

export default FeaBoundaryConditionAddBoundaryConditionMenu;
