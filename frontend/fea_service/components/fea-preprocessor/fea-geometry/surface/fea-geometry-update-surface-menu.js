import Store from "../../../../store/fea_store.js";
import { 
    UPDATE_SURFACE_MESSAGE_HEADER, ROTATE_SURFACE_VERTICES_CLOCKWISE_MESSAGE_HEADER,
    ROTATE_SURFACE_VERTICES_COUNTER_CLOCKWISE_MESSAGE_HEADER, FLIP_SURFACE_NORMAL_AXIS_MESSAGE_HEADER, 
} from "../../../../consts/actions_router_consts.js";
import { CLIENT_MESSAGE_HEADER, GET_WASM_LOADING_STATUS_MESSAGE_HEADER } from "../../../../consts/fea_app_consts.js";


class FeaGeometryUpdateSurfaceMenu extends HTMLElement {
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

            .point-3-number-field-content {
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

            .point-3-number-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .point-3-number-select-filter-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: column;
            }

            .point-3-number-filter-label {
                position: relative;
            }
                
            .point-3-number-filter-label:before {
                content: "";
                position: absolute;
                left: 0rem;
                top: 0rem;
                bottom: 0rem;
                width: 0.8rem;
                background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
            }

            .point-3-number-filter {
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

            .point-3-number-filter::placeholder {
                font-size: 85%;
            }

            .point-3-number-filter::-webkit-outer-spin-button,
            .point-3-number-filter::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .point-3-number-filter[type=number] {
                -moz-appearance: textfield;
            }

            .point-3-number-filter:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .point-3-number-filter:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .point-3-number {
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

            .point-3-number option {
                background-color: var(--preprocessor-menu-content-dropdown-menu-bg-color);
            }

            .point-3-number:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .point-4-number-field-content {
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

            .point-4-number-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .point-4-number-select-filter-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: column;
            }

            .point-4-number-filter-label {
                position: relative;
            }
                
            .point-4-number-filter-label:before {
                content: "";
                position: absolute;
                left: 0rem;
                top: 0rem;
                bottom: 0rem;
                width: 0.8rem;
                background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
            }

            .point-4-number-filter {
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

            .point-4-number-filter::placeholder {
                font-size: 85%;
            }

            .point-4-number-filter::-webkit-outer-spin-button,
            .point-4-number-filter::-webkit-inner-spin-button {
                -webkit-appearance: none;
                margin: 0;
            }

            .point-4-number-filter[type=number] {
                -moz-appearance: textfield;
            }

            .point-4-number-filter:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .point-4-number-filter:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .point-4-number {
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

            .point-4-number option {
                background-color: var(--preprocessor-menu-content-dropdown-menu-bg-color);
            }

            .point-4-number:hover {
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

            .rotate-surface-vertices-menu-buttons-caption {
                margin-top: 1rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding-top: 0rem;
                padding-bottom: 0.3rem;
                padding-left: 0rem;
                padding-right: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                font-size: 85%;
                width: 12rem;
            }

            .rotate-surface-menu-buttons {
                margin-top: 0.5rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: row;
            }

            .rotate-clockwise-button, .rotate-counter-clockwise-button {
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 5.5rem;
                height: 2.5rem;
                font-size: 70%;
            }

            .rotate-clockwise-button:hover, .rotate-counter-clockwise-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .flip-normal-axis-menu-buttons-caption {
                margin-top: 1rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding-top: 0rem;
                padding-bottom: 0.3rem;
                padding-left: 0rem;
                padding-right: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                font-size: 85%;
                width: 12rem;
            }

            .flip-normal-axis-menu-buttons {
                margin-top: 0.5rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: row;
            }

            .flip-button {
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 4.0rem;
                height: 1.7rem;
                font-size: 70%;
            }

            .flip-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
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

            <div class="point-1-number-field-content">
                <p class="point-1-number-caption">Point 1 number</p>
                <div class="point-1-number-select-filter-content">
                    <label class="point-1-number-filter-label">
                        <input class="point-1-number-filter" type="number" placeholder="Filter..."/>
                    </label>
                    <select class="point-1-number"></select>
                </div>
            </div>

            <div class="point-2-number-field-content">
                <p class="point-2-number-caption">Point 2 number</p>
                <div class="point-2-number-select-filter-content">
                    <label class="point-2-number-filter-label">
                        <input class="point-2-number-filter" type="number" placeholder="Filter..."/>
                    </label>
                    <select class="point-2-number"></select>
                </div>
            </div>

            <div class="point-3-number-field-content">
                <p class="point-3-number-caption">Point 3 number</p>
                <div class="point-3-number-select-filter-content">
                    <label class="point-3-number-filter-label">
                        <input class="point-3-number-filter" type="number" placeholder="Filter..."/>
                    </label>
                    <select class="point-3-number"></select>
                </div>
            </div>

            <div class="point-4-number-field-content">
                <p class="point-4-number-caption">Point 4 number</p>
                <div class="point-4-number-select-filter-content">
                    <label class="point-4-number-filter-label">
                        <input class="point-4-number-filter" type="number" placeholder="Filter..."/>
                    </label>
                    <select class="point-4-number"></select>
                </div>
            </div>
            
            <div class="apply-cancel-buttons">
                <button class="apply-button">Apply</button>
                <button class="cancel-button">Cancel</button>
            </div>

            <div class="analysis-info">
                <p class="analysis-info-message"></p>
            </div>

            <p class="rotate-surface-vertices-menu-buttons-caption">Rotate vertices around normal axis</p>

            <div class="rotate-surface-menu-buttons">
                <button class="rotate-clockwise-button">Clockwise</button>
                <button class="rotate-counter-clockwise-button">Counter clockwise</button>
            </div>

            <p class="flip-normal-axis-menu-buttons-caption">Flip normal axis</p>
            <div class="flip-normal-axis-menu-buttons">
                <button class="flip-button">Flip</button>
            </div>
        </div>
        `;

        this.shadowRoot.querySelector(".apply-button").addEventListener("click", () => this.updateSurface());

        this.shadowRoot.querySelector(".cancel-button").addEventListener("click", () => this.cancelSurfaceUpdate());

        this.shadowRoot.querySelector(".surface-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".surface-number-filter").value,
                this.shadowRoot.querySelector(".surface-number"));
        });

        this.shadowRoot.querySelector(".surface-number").addEventListener("change", 
            (event) => this.updateSelectedSurfacePointsNumbers(parseInt(event.target.value)));

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

        this.shadowRoot.querySelector(".point-3-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".point-3-number-filter").value,
                this.shadowRoot.querySelector(".point-3-number"));
        });

        this.shadowRoot.querySelector(".point-3-number").addEventListener("change", 
            () => this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "");

        this.shadowRoot.querySelector(".point-4-number-filter").addEventListener("keyup", () => {
            this.filter(
                this.shadowRoot.querySelector(".point-4-number-filter").value,
                this.shadowRoot.querySelector(".point-4-number"));
        });

        this.shadowRoot.querySelector(".point-4-number").addEventListener("change", 
            () => this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "");

        this.shadowRoot.querySelector(".rotate-clockwise-button").addEventListener("click", 
            () => this.rotateSurfaceAroundNormalAxis("clockwise"));

        this.shadowRoot.querySelector(".rotate-counter-clockwise-button").addEventListener("click", 
            () => this.rotateSurfaceAroundNormalAxis("counterClockwise"));

        this.shadowRoot.querySelector(".flip-button").addEventListener("click", () => this.flipSurfaceNormalAxis());
    }

    set isWasmLoaded(value) {
        this.props.isWasmLoaded = value;
    }

    set feModelError(error) {
        if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
            this.shadowRoot.querySelector(".analysis-info-message").innerHTML = error;
        }
    }

    set selectSurfaceInClient(surfaceNumber) {
        const frame = () => {
            if (this.props.isWasmLoaded === true) {
                clearInterval(id);
                const surfaceNumberSelect =  this.shadowRoot.querySelector(".surface-number");
                const surfaceNumberOptions =  surfaceNumberSelect.options;
                for (let option, i = 0; option = surfaceNumberOptions[i]; i++) {
                    if (option.value == surfaceNumber) {
                        surfaceNumberSelect.selectedIndex = i;
                        break;
                    }
                }
                this.updateSelectedSurfacePointsNumbers(surfaceNumber);
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
                this.defineSurfaceNumberOptions();
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
            this.defineSurfaceNumberOptions();
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

    defineSurfaceNumberOptions() {
        const surfaceUpdateNumberSelect = this.shadowRoot.querySelector(".surface-number");
        for (let i = surfaceUpdateNumberSelect.length - 1; i >= 0; i--) {
            surfaceUpdateNumberSelect.options[i] = null;
        }
        this.definePoint1NumberOptions();
        this.definePoint2NumberOptions();
        this.definePoint3NumberOptions();
        this.definePoint4NumberOptions();
        if (this.props.store.surfaces_shelf.size > 0) {
            const surfacesNumbers = Array.from(this.props.store.surfaces_shelf.keys()).sort((a, b) => a - b);
            for (let i = 0; i < surfacesNumbers.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = surfacesNumbers[i];
                updateOption.innerHTML = surfacesNumbers[i];
                surfaceUpdateNumberSelect.appendChild(updateOption);
            }
            const selectedSurfacePoint1Number = this.props.store.surfaces_shelf.get(surfacesNumbers[0]).point_1_number;
            const selectedSurfacePoint2Number = this.props.store.surfaces_shelf.get(surfacesNumbers[0]).point_2_number;
            const selectedSurfacePoint3Number = this.props.store.surfaces_shelf.get(surfacesNumbers[0]).point_3_number;
            const selectedSurfacePoint4Number = this.props.store.surfaces_shelf.get(surfacesNumbers[0]).point_4_number;

            const point1NumberSelect = this.shadowRoot.querySelector(".point-1-number");
            const point1NumberOptions = point1NumberSelect.options;
            for (let option, i = 0; option = point1NumberOptions[i]; i++) {
                if (option.value == selectedSurfacePoint1Number) {
                    point1NumberSelect.selectedIndex = i;
                    break;
                }
            }
            const point2NumberSelect = this.shadowRoot.querySelector(".point-2-number");
            const point2NumberOptions = point2NumberSelect.options;
            for (let option, i = 0; option = point2NumberOptions[i]; i++) {
                if (option.value == selectedSurfacePoint2Number) {
                    point2NumberSelect.selectedIndex = i;
                    break;
                }
            }
            const point3NumberSelect = this.shadowRoot.querySelector(".point-3-number");
            const point3NumberOptions = point3NumberSelect.options;
            for (let option, i = 0; option = point3NumberOptions[i]; i++) {
                if (option.value == selectedSurfacePoint3Number) {
                    point3NumberSelect.selectedIndex = i;
                    break;
                }
            }
            const point4NumberSelect = this.shadowRoot.querySelector(".point-4-number");
            const point4NumberOptions = point4NumberSelect.options;
            for (let option, i = 0; option = point4NumberOptions[i]; i++) {
                if (option.value == selectedSurfacePoint4Number) {
                    point4NumberSelect.selectedIndex = i;
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

    definePoint2NumberOptions() {
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

    definePoint3NumberOptions() {
        const point3NumberSelect = this.shadowRoot.querySelector(".point-3-number");
        for (let i = point3NumberSelect.length - 1; i >= 0; i--) {
            point3NumberSelect.options[i] = null;
        }
        if (this.props.store.points_shelf.size > 0) {
            const pointsNumbers = Array.from(this.props.store.points_shelf.keys()).sort((a, b) => a - b);
            for (let i = 0; i < pointsNumbers.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = pointsNumbers[i];
                updateOption.innerHTML = pointsNumbers[i];
                point3NumberSelect.appendChild(updateOption);
            }
        }
    }

    definePoint4NumberOptions() {
        const point4NumberSelect = this.shadowRoot.querySelector(".point-4-number");
        for (let i = point4NumberSelect.length - 1; i >= 0; i--) {
            point4NumberSelect.options[i] = null;
        }
        if (this.props.store.points_shelf.size > 0) {
            const pointsNumbers = Array.from(this.props.store.points_shelf.keys()).sort((a, b) => a - b);
            for (let i = 0; i < pointsNumbers.length; i++) {
                let updateOption = document.createElement("option");
                updateOption.value = pointsNumbers[i];
                updateOption.innerHTML = pointsNumbers[i];
                point4NumberSelect.appendChild(updateOption);
            }
        }
    }

    updateSelectedSurfacePointsNumbers(selectedSurfaceNumber) {
        const selectedSurfacePoint1Number = this.props.store.surfaces_shelf.get(selectedSurfaceNumber).point_1_number;
        const selectedSurfacePoint2Number = this.props.store.surfaces_shelf.get(selectedSurfaceNumber).point_2_number;
        const selectedSurfacePoint3Number = this.props.store.surfaces_shelf.get(selectedSurfaceNumber).point_3_number;
        const selectedSurfacePoint4Number = this.props.store.surfaces_shelf.get(selectedSurfaceNumber).point_4_number;

        const point1NumberSelect = this.shadowRoot.querySelector(".point-1-number");
        const point1NumberOptions =  point1NumberSelect.options;
        for (let option, i = 0; option = point1NumberOptions[i]; i++) {
            if (option.value == selectedSurfacePoint1Number) {
                point1NumberSelect.selectedIndex = i;
                break;
            }
        }
        const point2NumberSelect =  this.shadowRoot.querySelector(".point-2-number");
        const point2NumberOptions =  point2NumberSelect.options;
        for (let option, i = 0; option = point2NumberOptions[i]; i++) {
            if (option.value == selectedSurfacePoint2Number) {
                point2NumberSelect.selectedIndex = i;
                break;
            }
        }
        const point3NumberSelect =  this.shadowRoot.querySelector(".point-3-number");
        const point3NumberOptions =  point3NumberSelect.options;
        for (let option, i = 0; option = point3NumberOptions[i]; i++) {
            if (option.value == selectedSurfacePoint3Number) {
                point3NumberSelect.selectedIndex = i;
                break;
            }
        }
        const point4NumberSelect =  this.shadowRoot.querySelector(".point-4-number");
        const point4NumberOptions =  point4NumberSelect.options;
        for (let option, i = 0; option = point4NumberOptions[i]; i++) {
            if (option.value == selectedSurfacePoint4Number) {
                point4NumberSelect.selectedIndex = i;
                break;
            }
        }
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    updateSurface() {
        const selectedSurfaceNumberField = this.shadowRoot.querySelector(".surface-number");

        if (selectedSurfaceNumberField.value == "") {
            if (selectedSurfaceNumberField.classList.contains("highlighted") === false) {
                selectedSurfaceNumberField.classList.add("highlighted");
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

        const point3Field = this.shadowRoot.querySelector(".point-3-number");
        if (point3Field.value == "") {
            if (point3Field.classList.contains("highlighted") === false) {
                point3Field.classList.add("highlighted");
            }
        }

        const point4Field = this.shadowRoot.querySelector(".point-4-number");
        if (point4Field.value == "") {
            if (point4Field.classList.contains("highlighted") === false) {
                point4Field.classList.add("highlighted");
            }
        }

        if (selectedSurfaceNumberField.value == "" || point1Field.value == "" || point2Field.value == "" ||
            point3Field.value == "" || point4Field.value == "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        const oldSurfaceValues = this.props.store.surfaces_shelf.get(parseInt(selectedSurfaceNumberField.value));

        const message = { [UPDATE_SURFACE_MESSAGE_HEADER]: {
            action_id: this.props.actionId,
            number: selectedSurfaceNumberField.value, 
            old_surface_values: { 
                point_1_number:  oldSurfaceValues.point_1_number, point_2_number: oldSurfaceValues.point_2_number,
                point_3_number:  oldSurfaceValues.point_3_number, point_4_number: oldSurfaceValues.point_4_number, 
            },
            new_surface_values: { 
                point_1_number:  point1Field.value, point_2_number: point2Field.value, 
                point_3_number:  point3Field.value, point_4_number: point4Field.value, 
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
        this.shadowRoot.querySelector(".point-1-number-filter").value = null;
        this.shadowRoot.querySelector(".point-2-number-filter").value = null;
        this.shadowRoot.querySelector(".point-3-number-filter").value = null;
        this.shadowRoot.querySelector(".point-4-number-filter").value = null;
    }

    cancelSurfaceUpdate() {
        this.defineSurfaceNumberOptions();
        this.shadowRoot.querySelector(".surface-number-filter").value = null;
        this.shadowRoot.querySelector(".point-1-number-filter").value = null;
        this.shadowRoot.querySelector(".point-2-number-filter").value = null;
        this.shadowRoot.querySelector(".point-3-number-filter").value = null;
        this.shadowRoot.querySelector(".point-4-number-filter").value = null;
        const selectedSurfaceNumberField = this.shadowRoot.querySelector(".surface-number");
        this.dropHighlight(selectedSurfaceNumberField);
        const point1Field = this.shadowRoot.querySelector(".point-1-number");
        this.dropHighlight(point1Field);
        const point2Field = this.shadowRoot.querySelector(".point-2-number");
        this.dropHighlight(point2Field);
        const point3Field = this.shadowRoot.querySelector(".point-3-number");
        this.dropHighlight(point3Field);
        const point4Field = this.shadowRoot.querySelector(".point-4-number");
        this.dropHighlight(point4Field);
        this.shadowRoot.querySelector(".analysis-info-message").innerHTML = "";
    }

    rotateSurfaceAroundNormalAxis(direction) {
        const selectedSurfaceNumberField = this.shadowRoot.querySelector(".surface-number");

        if (selectedSurfaceNumberField.value == "") {
            if (selectedSurfaceNumberField.classList.contains("highlighted") === false) {
                selectedSurfaceNumberField.classList.add("highlighted");
            }
        }

        if (selectedSurfaceNumberField.value == "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        let messageHeader;
        switch (direction) {
            case "clockwise":
                messageHeader = ROTATE_SURFACE_VERTICES_CLOCKWISE_MESSAGE_HEADER;
                break;
            case "counterClockwise":
                messageHeader = ROTATE_SURFACE_VERTICES_COUNTER_CLOCKWISE_MESSAGE_HEADER;
                break;
        }

        const message = { [messageHeader]: {
            action_id: this.props.actionId,
            number: selectedSurfaceNumberField.value, 
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

    flipSurfaceNormalAxis() {
        const selectedSurfaceNumberField = this.shadowRoot.querySelector(".surface-number");

        if (selectedSurfaceNumberField.value == "") {
            if (selectedSurfaceNumberField.classList.contains("highlighted") === false) {
                selectedSurfaceNumberField.classList.add("highlighted");
            }
        }

        if (selectedSurfaceNumberField.value == "") {
            if (this.shadowRoot.querySelector(".analysis-info-message").innerHTML === "") {
                this.shadowRoot.querySelector(".analysis-info-message").innerHTML = 
                    "Note: The highlighted fields should be filled!";
                return;
            } else {
                return;
            }
        }

        const message = { [FLIP_SURFACE_NORMAL_AXIS_MESSAGE_HEADER]: {
            action_id: this.props.actionId,
            number: selectedSurfaceNumberField.value, 
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

    dropHighlight(highlightedElement) {
        if (highlightedElement.classList.contains("highlighted") === true) {
            highlightedElement.classList.remove("highlighted");
        }
    }
}

export default FeaGeometryUpdateSurfaceMenu;
