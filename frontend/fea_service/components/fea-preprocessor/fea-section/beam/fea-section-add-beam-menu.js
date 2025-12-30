// import Store from "../../../../store/fea_store.js";
import { ADD_BEAM_SECTION_MESSAGE_HEADER } from "../../../../consts/actions_router_consts.js";
import { CLIENT_MESSAGE_HEADER, GET_WASM_LOADING_STATUS_MESSAGE_HEADER } from "../../../../consts/fea_app_consts.js";


class FeaSectionAddBeamMenu extends HTMLElement {
    constructor() {
        super();

        this.props = {
            actionId: null,             // u32;
            isWasmLoaded: false,     // load status of wasm modules;
            // store: new Store(),
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

            .beam-section-name-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin: 0rem;
            }

            .beam-section-name-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .beam-section-name {
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

            .beam-section-name:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .beam-section-name:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .area-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .area-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .area {
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

            .area[type=number]::-webkit-outer-spin-button,
            .area[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .area[type=number] {
                -moz-appearance: textfield;
            }

            .area:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .area:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .I11-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .I11-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .I11 {
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

            .I11[type=number]::-webkit-outer-spin-button,
            .I11[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .I11[type=number] {
                -moz-appearance: textfield;
            }

            .I11:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .I11:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .I22-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .I22-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .I22 {
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

            .I22[type=number]::-webkit-outer-spin-button,
            .I22[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .I22[type=number] {
                -moz-appearance: textfield;
            }

            .I22:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .I22:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .I12-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .I12-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .I12 {
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

            .I12[type=number]::-webkit-outer-spin-button,
            .I12[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .I12[type=number] {
                -moz-appearance: textfield;
            }

            .I12:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .I12:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .It-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .It-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .It {
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

            .It[type=number]::-webkit-outer-spin-button,
            .It[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .It[type=number] {
                -moz-appearance: textfield;
            }

            .It:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .It:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .Shear-factor-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
            }

            .Shear-factor-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .Shear-factor {
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

            .Shear-factor[type=number]::-webkit-outer-spin-button,
            .Shear-factor[type=number]::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .Shear-factor[type=number] {
                -moz-appearance: textfield;
            }

            .Shear-factor:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .Shear-factor:focus {
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
            <div class="beam-section-name-field-content">
                <p class="beam-section-name-caption">Section name</p>
                <input class="beam-section-name" type="text"/>
            </div>

            <div class="area-field-content">
                <p class="area-caption">Area</p>
                <input class="area" type="number"/>
            </div>

            <div class="I11-field-content">
                <p class="I11-caption">I11</p>
                <input class="I11" type="number"/>
            </div>

            <div class="I22-field-content">
                <p class="I22-caption">I22</p>
                <input class="I22" type="number"/>
            </div>

            <div class="I12-field-content">
                <p class="I12-caption">I12</p>
                <input class="I12" type="number"/>
            </div>

            <div class="It-field-content">
                <p class="It-caption">It</p>
                <input class="It" type="number"/>
            </div>

            <div class="Shear-factor-field-content">
                <p class="Shear-factor-caption">Shear factor</p>
                <input class="Shear-factor" type="number"/>
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

         this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.addBeamSection());

         this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelBeamSectionAddition());

         this.shadowRoot.querySelector(".beam-section-name").addEventListener("click", () => {
            const highlightedElement = this.shadowRoot.querySelector(".beam-section-name");
            this.dropHighlight(highlightedElement);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".area").addEventListener("click", () => {
            const inputtedArea = this.shadowRoot.querySelector(".area");
            this.dropHighlight(inputtedArea);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".I11").addEventListener("click", () => {
            const inputtedI11 = this.shadowRoot.querySelector(".I11");
            this.dropHighlight(inputtedI11);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".I22").addEventListener("click", () => {
            const inputtedI22 = this.shadowRoot.querySelector(".I22");
            this.dropHighlight(inputtedI22);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".I12").addEventListener("click", () => {
            const inputtedI12 = this.shadowRoot.querySelector(".I12");
            this.dropHighlight(inputtedI12);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".It").addEventListener("click", () => {
            const inputtedIt = this.shadowRoot.querySelector(".It");
            this.dropHighlight(inputtedIt);
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
        });

        this.shadowRoot.querySelector(".Shear-factor").addEventListener("click", () => {
            const inputtedShearFactor = this.shadowRoot.querySelector(".Shear-factor");
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
                this.defineNewBeamSectionName();
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
            this.defineNewBeamSectionName();
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

    defineNewBeamSectionName() {
        const newBeamSectionName = "beam1";
        this.shadowRoot.querySelector(".beam-section-name").value = newBeamSectionName;
        this.shadowRoot.querySelector(".area").value = 1;
        this.shadowRoot.querySelector(".I11").value = 1;
        this.shadowRoot.querySelector(".I22").value = 1;
        this.shadowRoot.querySelector(".I12").value = 0;
        this.shadowRoot.querySelector(".It").value = 1;
        this.shadowRoot.querySelector(".Shear-factor").value = 1;
    }


    addBeamSection() {
        const newBeamSectionNameField = this.shadowRoot.querySelector(".beam-section-name");
        if (newBeamSectionNameField.value === "") {
            if (newBeamSectionNameField.classList.contains("highlighted") === false) {
                newBeamSectionNameField.classList.add("highlighted");
            }
        }

        const areaField = this.shadowRoot.querySelector(".area");
        if (areaField.value === "") {
            if (areaField.classList.contains("highlighted") === false) {
                areaField.classList.add("highlighted");
            }
        }

        const I11Field = this.shadowRoot.querySelector(".I11");
        if (I11Field.value === "") {
            if (I11Field.classList.contains("highlighted") === false) {
                I11Field.classList.add("highlighted");
            }
        }

        const I22Field = this.shadowRoot.querySelector(".I22");
        if (I22Field.value === "") {
            if (I22Field.classList.contains("highlighted") === false) {
                I22Field.classList.add("highlighted");
            }
        }

        const I12Field = this.shadowRoot.querySelector(".I12");
        if (I12Field.value === "") {
            if (I12Field.classList.contains("highlighted") === false) {
                I12Field.classList.add("highlighted");
            }
        }

        const ItField = this.shadowRoot.querySelector(".It");
        if (ItField.value === "") {
            if (ItField.classList.contains("highlighted") === false) {
                ItField.classList.add("highlighted");
            }
        }

        const ShearFactorField = this.shadowRoot.querySelector(".Shear-factor");
        if (ShearFactorField.value === "") {
            if (ShearFactorField.classList.contains("highlighted") === false) {
                ShearFactorField.classList.add("highlighted");
            }
        }

        if (newBeamSectionNameField.value === "" || areaField.value === "" || I11Field.value === "" ||
            I22Field.value === "" || I12Field.value === "" || ItField.value === "" || 
            ShearFactorField.value === "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        if (this.isNumeric(areaField.value) === false || this.isNumeric(I11Field.value) === false ||
            this.isNumeric(I22Field.value) === false || this.isNumeric(I12Field.value) === false ||
            this.isNumeric(ItField.value) === false || this.isNumeric(ShearFactorField.value) === false) {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: Only numbers could be used as input values for Area, I11, I22, I12, It and Shear factor!";
                return;
            } else {
                return;
            }
        }

        const message = { [ADD_BEAM_SECTION_MESSAGE_HEADER]: {
            action_id: this.props.actionId,
            name: newBeamSectionNameField.value, 
            area: areaField.value, i11: I11Field.value, i22: I22Field.value, 
            i12: I12Field.value, it: ItField.value, shear_factor: ShearFactorField.value,
        }};

        this.dispatchEvent(new CustomEvent([CLIENT_MESSAGE_HEADER], {
            bubbles: true,
            composed: true,
            detail: {
                message: message,
            },
        }));
    }

    cancelBeamSectionAddition() {
        this.defineNewBeamSectionName();
        const newBeamSectionNameField = this.shadowRoot.querySelector(".beam-section-name");
        this.dropHighlight(newBeamSectionNameField);
        const inputtedAreaField = this.shadowRoot.querySelector(".area");
        this.dropHighlight(inputtedAreaField);
        const inputtedI11Field = this.shadowRoot.querySelector(".I11");
        this.dropHighlight(inputtedI11Field);
        const inputtedI22Field = this.shadowRoot.querySelector(".I22");
        this.dropHighlight(inputtedI22Field);
        const inputtedI12Field = this.shadowRoot.querySelector(".I12");
        this.dropHighlight(inputtedI12Field);
        const inputtedItField = this.shadowRoot.querySelector(".It");
        this.dropHighlight(inputtedItField);
        const inputtedShearFactorField = this.shadowRoot.querySelector(".Shear-factor");
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

export default FeaSectionAddBeamMenu;
