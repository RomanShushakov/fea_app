import Store from "../../../store/fea_store.js";
import {
  SUBMIT_JOB_MESSAGE_HEADER,
  DELETE_JOB_MESSAGE_HEADER,
  SHOW_JOB_ANALYSIS_RESULT_MESSAGE_HEADER,
  GET_WASM_LOADING_STATUS_MESSAGE_HEADER,
} from "../../../consts/fea_app_consts.js";

class FeaAnalysisMenu extends HTMLElement {
  constructor() {
    super();

    this.props = {
      actionId: null, // u32;
      isWasmLoaded: false, // load status of wasm modules;
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
                padding: 1rem;
                overflow-y: auto;
                overflow-x: hidden;
                scrollbar-color: var(--renderer-menu-content-scrollbar-thumb-color) var(--preprocessor-menu-buttons-active-button-bg-color);
                scrollbar-width: thin;
            }

            ::-webkit-scrollbar {
                width: 0.5rem;
            }

            ::-webkit-scrollbar-track {
                background: var(--preprocessor-menu-buttons-active-button-bg-color);
            }

            ::-webkit-scrollbar-thumb {
                background: var(--renderer-menu-content-scrollbar-thumb-color);
            }

            ::-webkit-scrollbar-thumb:hover {
                background: var(--renderer-menu-content-scrollbar-thumb-hover-color);
            }

            .analysis-menu-caption {
                margin: 0rem;
                padding-top: 0rem;
                padding-bottom: 0.3rem;
                padding-left: 0rem;
                padding-right: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
                font-size: 85%;
            }

            .new-job-name-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 1rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
            }

            .new-job-name-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .new-job-name {
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

            .new-job-name:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .new-job-name:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .new-job-buttons {
                margin-top: 1rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
                align-self: center;
            }

            .check-button {
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 5rem;
                height: 1.7rem;
            }

            .check-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .submit-button {
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 5.5rem;
                height: 1.7rem;
            }

            .processing {
                background: var(--preprocessor-menu-content-processing-buttons-color);
            }

            .submit-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .new-job-info {
                display: flex;
                margin: 0rem;
                padding: 0rem;
                flex-direction: column;
                align-items: center;
            }

            .new-job-info-message {
                margin-top: 1rem;
                margin-bottom: 0.5rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 80%;
                width: 12rem;
            }

            .new-job-hide-message-button {
                background: var(--preprocessor-menu-content-aux-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 6rem;
                height: 1.7rem;
                font-size: 70%;
            }

            .new-job-hide-message-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .new-job-yes-button {
                background: var(--preprocessor-menu-content-aux-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 4rem;
                height: 1.7rem;
                font-size: 70%;
            }

            .new-job-yes-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .new-job-no-button {
                background: var(--preprocessor-menu-content-aux-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 4rem;
                height: 1.7rem;
                font-size: 70%;
            }

            .new-job-no-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .job-name-field-content {
                display: flex;
                flex-direction: row;
                background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
                padding: 0rem;
                margin-top: 5rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                align-items: center;
            }

            .job-name-caption {
                margin: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 85%;
                width: 6rem;
            }

            .job-name-select-filter-content {
                margin-top: 0rem;
                margin-bottom: 0rem;
                margin-left: 1rem;
                margin-right: 0rem;
                padding: 0rem;
                display: flex;
                flex-direction: column;
            }

            .job-name-filter-label {
                position: relative;
            }
                
            .job-name-filter-label:before {
                content: "";
                position: absolute;
                left: 0rem;
                top: 0rem;
                bottom: 0rem;
                width: 0.8rem;
                background: url('data:image/svg+xml,<svg width="19" height="17" viewBox="0 0 19 17" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12.1182 13.15L7.48598 16L7.48598 6.25L2 0.999999L17 1L12.1182 6.25L12.1182 13.15Z" fill="rgb(112, 112, 114)" stroke="rgb(112, 112, 114)"/></svg>') center / contain no-repeat;
            }

            .job-name-filter {
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

            .job-name-filter::placeholder {
                font-size: 85%;
            }

            .job-name-filter:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .job-name-filter:focus {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .job-name {
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

            .job-name option {
                background-color: var(--preprocessor-menu-content-dropdown-menu-bg-color);
            }

            .job-name:hover {
                box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .solver-settings {
              display: flex;
              flex-direction: column;
              padding-bottom: 1.2rem;
              padding-top: 0.6rem;
              border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .solver-caption {
              margin: 0rem;
              padding: 0rem;
              color: var(--preprocessor-menu-content-caption-color);
              font-size: 85%;
            }

            .solver-option {
              display: flex;
              align-items: center;
              margin-top: 0.6rem;
              color: var(--preprocessor-menu-content-caption-color);
              font-size: 80%;
              gap: 0.5rem;
              user-select: none;
            }

            .solver-option input[type="radio"] {
              accent-color: var(--preprocessor-menu-content-apply-cancel-buttons-color);
            }

            .solver-extra {
              display: flex;
              align-items: center;
              margin-left: 1.3rem; /* indent under radio */
              margin-top: 0.4rem;
              gap: 0.7rem;
            }

            .solver-extra-caption {
              margin: 0rem;
              padding: 0rem;
              color: var(--preprocessor-menu-content-caption-color);
              font-size: 75%;
            }

            .solver-max-iter {
              width: 5rem;
              background-color: var(--preprocessor-menu-buttons-active-button-bg-color);
              border: var(--preprocessor-menu-content-caption-border-color);
              border-bottom: 0.1rem solid var(--preprocessor-menu-content-caption-border-color);
              outline: none;
              color: var(--preprocessor-menu-content-caption-color);
            }

            .solver-max-iter:hover,
            .solver-max-iter:focus {
              box-shadow: 0rem 0.15rem 0rem var(--preprocessor-menu-content-caption-border-color);
            }

            .solver-max-iter::-webkit-outer-spin-button,
            .solver-max-iter::-webkit-inner-spin-button {
              -webkit-appearance: none;
              margin: 0;
            }

            .solver-max-iter {
              -moz-appearance: textfield;
              appearance: textfield;
            }

            .hidden {
              display: none;
            }

            .job-buttons {
                margin-top: 1rem;
                margin-bottom: 0rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
                align-self: center;
            }

            .show-result-button {
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 5.5rem;
                height: 1.7rem;
            }

            .show-result-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .delete-job-button {
                background: var(--preprocessor-menu-content-apply-cancel-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 5.5rem;
                height: 1.7rem;
            }

            .delete-job-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .job-info {
                display: flex;
                margin: 0rem;
                padding: 0rem;
                flex-direction: column;
                align-items: center;
            }

            .job-info-message {
                margin-top: 1rem;
                margin-bottom: 0.5rem;
                margin-left: 0rem;
                margin-right: 0rem;
                padding: 0rem;
                color: var(--preprocessor-menu-content-caption-color);
                font-size: 80%;
                width: 12rem;
            }

            .job-hide-message-button {
                background: var(--preprocessor-menu-content-aux-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 6rem;
                height: 1.7rem;
                font-size: 70%;
            }

            .job-hide-message-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .job-yes-button {
                background: var(--preprocessor-menu-content-aux-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 4rem;
                height: 1.7rem;
                font-size: 70%;
            }

            .job-yes-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .job-no-button {
                background: var(--preprocessor-menu-content-aux-buttons-color);
                border: 0.2rem solid var(--preprocessor-menu-buttons-active-button-bg-color);
                border-radius: 0.3rem;
                color: var(--preprocessor-menu-content-caption-color);
                padding: 0rem;
                margin: 0rem;
                width: 4rem;
                height: 1.7rem;
                font-size: 70%;
            }

            .job-no-button:hover {
                border: 0.2rem solid var(--preprocessor-menu-content-caption-border-color);
            }

            .highlighted {
                box-shadow: 0rem 0.1rem 0rem var(--preprocessor-menu-content-active-buttons-color);
            }
        </style>
        <div class=wrapper>
            <p class="analysis-menu-caption">Analysis</p>

            <div class="new-job-name-field-content">
                <p class="new-job-name-caption">New job name</p>
                <input class="new-job-name" type="text"/>
            </div>

            <div class="solver-settings">
              <p class="solver-caption">Solver</p>

              <label class="solver-option">
                <input type="radio" name="solver" value="direct" checked />
                <span>Direct</span>
              </label>

              <label class="solver-option">
                <input type="radio" name="solver" value="pcg_jacobi_cpu" />
                <span>Iterative (Jacobi, CPU)</span>
              </label>

              <div class="solver-extra solver-extra-jacobi hidden">
                <p class="solver-extra-caption">Max iterations</p>
                <input class="solver-max-iter solver-max-iter-jacobi" type="number" min="1" step="1" value="2000" />
              </div>

              <label class="solver-option">
                <input type="radio" name="solver" value="pcg_block_jacobi_cpu" />
                <span>Iterative (Block Jacobi, CPU)</span>
              </label>

              <div class="solver-extra solver-extra-block hidden">
                <p class="solver-extra-caption">Max iterations</p>
                <input class="solver-max-iter solver-max-iter-block" type="number" min="1" step="1" value="2000" />
              </div>

              <label class="solver-option">
                <input type="radio" name="solver" value="pcg_block_jacobi_gpu" />
                <span>Iterative (Block Jacobi, GPU)</span>
              </label>

              <div class="solver-extra solver-extra-block-gpu hidden">
                <p class="solver-extra-caption">Max iterations</p>
                <input class="solver-max-iter solver-max-iter-block-gpu" type="number" min="1" step="1" value="2000" />
              </div>
            </div>

            <div class="new-job-buttons">
                <button class="submit-button">Submit</button>
            </div>

            <div class="new-job-info">
                <p class="new-job-info-message"></p>
            </div>

            <div class="job-name-field-content">
                <p class="job-name-caption">Job name</p>
                <div class="job-name-select-filter-content">
                    <label class="job-name-filter-label">
                        <input class="job-name-filter" type="text" placeholder="Filter..."/>
                    </label>
                    <select class="job-name"></select>
                </div>
            </div>

            <div class="job-buttons">
                <button class="show-result-button">Show result</button>
                <button class="delete-job-button">Delete job</button>
            </div>

            <div class="job-info">
                <p class="job-info-message"></p>
            </div>
        </div>
        `;

    this.shadowRoot
      .querySelector(".new-job-name")
      .addEventListener("click", () => {
        const highlightedElement =
          this.shadowRoot.querySelector(".new-job-name");
        this.dropHighlight(highlightedElement);
        this.shadowRoot.querySelector(".new-job-info-message").innerHTML = "";
        if (
          this.shadowRoot.querySelector(".new-job-hide-message-button") !=
          undefined
        ) {
          this.shadowRoot
            .querySelector(".new-job-hide-message-button")
            .remove();
        }
        if (this.shadowRoot.querySelector(".new-job-yes-button") != undefined) {
          this.shadowRoot.querySelector(".new-job-yes-button").remove();
        }
        if (this.shadowRoot.querySelector(".new-job-no-button") != undefined) {
          this.shadowRoot.querySelector(".new-job-no-button").remove();
        }
      });

    this.shadowRoot
      .querySelector(".submit-button")
      .addEventListener("click", () => this.submitJob());

    this.shadowRoot
      .querySelector(".job-name-filter")
      .addEventListener("keyup", () => {
        this.filter(
          this.shadowRoot.querySelector(".job-name-filter").value,
          this.shadowRoot.querySelector(".job-name")
        );
      });

    this.shadowRoot
      .querySelector(".show-result-button")
      .addEventListener("click", () => this.showJobAnalysisResult());

    this.shadowRoot
      .querySelector(".delete-job-button")
      .addEventListener("click", () => this.deleteJob());

    this.shadowRoot
      .querySelectorAll('input[name="solver"]')
      .forEach((el) =>
        el.addEventListener("change", () => this.onSolverChanged())
      );
  }

  set isWasmLoaded(value) {
    this.props.isWasmLoaded = value;
  }

  set submitJobError(error) {
    this.shadowRoot.querySelector(".new-job-info-message").innerHTML = error;
    if (
      this.shadowRoot.querySelector(".new-job-hide-message-button") == undefined
    ) {
      const newJobHideMessageButton = document.createElement("button");
      newJobHideMessageButton.className = "new-job-hide-message-button";
      newJobHideMessageButton.innerHTML = "Hide message";
      newJobHideMessageButton.addEventListener("click", () => {
        this.shadowRoot.querySelector(".new-job-info-message").innerHTML = "";
        if (
          this.shadowRoot.querySelector(".new-job-hide-message-button") !=
          undefined
        ) {
          this.shadowRoot
            .querySelector(".new-job-hide-message-button")
            .remove();
        }
        if (this.shadowRoot.querySelector(".new-job-yes-button") != undefined) {
          this.shadowRoot.querySelector(".new-job-yes-button").remove();
        }
        if (this.shadowRoot.querySelector(".new-job-no-button") != undefined) {
          this.shadowRoot.querySelector(".new-job-no-button").remove();
        }
        this.dropHighlight(this.shadowRoot.querySelector(".new-job-name"));
      });
      this.shadowRoot
        .querySelector(".new-job-info")
        .append(newJobHideMessageButton);
    }
  }

  set submitJobSuccess(message) {
    this.shadowRoot.querySelector(".new-job-info-message").innerHTML = message;
    if (
      this.shadowRoot.querySelector(".new-job-hide-message-button") == undefined
    ) {
      const newJobHideMessageButton = document.createElement("button");
      newJobHideMessageButton.className = "new-job-hide-message-button";
      newJobHideMessageButton.innerHTML = "Hide message";
      newJobHideMessageButton.addEventListener("click", () => {
        this.shadowRoot.querySelector(".new-job-info-message").innerHTML = "";
        if (
          this.shadowRoot.querySelector(".new-job-hide-message-button") !=
          undefined
        ) {
          this.shadowRoot
            .querySelector(".new-job-hide-message-button")
            .remove();
        }
        if (this.shadowRoot.querySelector(".new-job-yes-button") != undefined) {
          this.shadowRoot.querySelector(".new-job-yes-button").remove();
        }
        if (this.shadowRoot.querySelector(".new-job-no-button") != undefined) {
          this.shadowRoot.querySelector(".new-job-no-button").remove();
        }
        this.dropHighlight(this.shadowRoot.querySelector(".new-job-name"));
      });
      this.shadowRoot
        .querySelector(".new-job-info")
        .append(newJobHideMessageButton);
    }
  }

  set jobError(error) {
    this.shadowRoot.querySelector(".job-info-message").innerHTML = error;
    if (
      this.shadowRoot.querySelector(".job-hide-message-button") == undefined
    ) {
      const jobHideMessageButton = document.createElement("button");
      jobHideMessageButton.className = "job-hide-message-button";
      jobHideMessageButton.innerHTML = "Hide message";
      jobHideMessageButton.addEventListener("click", () => {
        this.shadowRoot.querySelector(".job-info-message").innerHTML = "";
        if (
          this.shadowRoot.querySelector(".job-hide-message-button") != undefined
        ) {
          this.shadowRoot.querySelector(".job-hide-message-button").remove();
        }
        this.dropHighlight(this.shadowRoot.querySelector(".job-name"));
      });
      this.shadowRoot.querySelector(".job-info").append(jobHideMessageButton);
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
        this.defineJobNameOptions();
        clearInterval(id);
      }
    };
    const id = setInterval(frame, 10);
  }

  disconnectedCallback() {}

  static get observedAttributes() {
    return ["action-id", "is-processing", "error-message"];
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (name === "action-id") {
      this.props.actionId = parseInt(newValue);
      this.defineJobNameOptions();
    }

    if (name === "is-processing") {
      this.shadowRoot.querySelector(".submit-button").disabled =
        newValue === "true";
      if (newValue === "true") {
        this.shadowRoot
          .querySelector(".submit-button")
          .classList.add("processing");
        this.shadowRoot.querySelector(".submit-button").innerHTML =
          "Processing..";
      }
      if (newValue === "false") {
        this.shadowRoot
          .querySelector(".submit-button")
          .classList.remove("processing");
        this.shadowRoot.querySelector(".submit-button").innerHTML = "Submit";
      }
    }

    if (name === "error-message") {
      this.submitJobError = newValue;
    }
  }

  adoptedCallback() {}

  getWasmLoadingStatus() {
    this.dispatchEvent(
      new CustomEvent(GET_WASM_LOADING_STATUS_MESSAGE_HEADER, {
        bubbles: true,
        composed: true,
      })
    );
  }

  defineJobNameOptions() {
    this.shadowRoot.querySelector(".new-job-name").value = "";
    const jobNameSelect = this.shadowRoot.querySelector(".job-name");
    for (let i = jobNameSelect.length - 1; i >= 0; i--) {
      jobNameSelect.options[i] = null;
    }
    let jobNames = Array.from(this.props.store.jobs_shelf.keys());
    jobNames.sort();
    for (let i = 0; i < jobNames.length; i++) {
      let option = document.createElement("option");
      option.value = jobNames[i].replace(/['"]+/g, "");
      option.innerHTML = jobNames[i].replace(/['"]+/g, "");
      jobNameSelect.appendChild(option);
    }
  }

  getSelectedSolver() {
    const el = this.shadowRoot.querySelector('input[name="solver"]:checked');
    return el ? el.value : "direct";
  }

  getSelectedMaxIter(solver) {
    if (solver === "pcg_jacobi_cpu") {
      const v = parseInt(
        this.shadowRoot.querySelector(".solver-max-iter-jacobi").value,
        10
      );
      return Number.isFinite(v) && v > 0 ? v : 2000;
    }
    if (solver === "pcg_block_jacobi_cpu") {
      const v = parseInt(
        this.shadowRoot.querySelector(".solver-max-iter-block").value,
        10
      );
      return Number.isFinite(v) && v > 0 ? v : 2000;
    }
    if (solver === "pcg_block_jacobi_gpu") {
      const v = parseInt(
        this.shadowRoot.querySelector(".solver-max-iter-block-gpu").value,
        10
      );
      return Number.isFinite(v) && v > 0 ? v : 2000;
    }
    return null; // direct solver doesn't use max_iter
  }

  buildSubmitPayload(jobName) {
    const solver = this.getSelectedSolver();
    const max_iter = this.getSelectedMaxIter(solver);

    return {
      job_name: jobName,
      solver: solver, // "direct" | "pcg_jacobi_cpu" | "pcg_block_jacobi_cpu | pcg_block_jacobi_gpu"
      max_iter: max_iter, // number or null
    };
  }

  submitJob() {
    const newJobNameField = this.shadowRoot.querySelector(".new-job-name");
    if (newJobNameField.value === "") {
      if (newJobNameField.classList.contains("highlighted") === false) {
        newJobNameField.classList.add("highlighted");
      }
    }

    if (newJobNameField.value === "") {
      if (
        this.shadowRoot.querySelector(".new-job-info-message").innerHTML === ""
      ) {
        this.shadowRoot.querySelector(".new-job-info-message").innerHTML =
          "Note: The highlighted fields should be filled!";
        if (
          this.shadowRoot.querySelector(".new-job-hide-message-button") ==
          undefined
        ) {
          const newJobHideMessageButton = document.createElement("button");
          newJobHideMessageButton.className = "new-job-hide-message-button";
          newJobHideMessageButton.innerHTML = "Hide message";
          newJobHideMessageButton.addEventListener("click", () => {
            this.shadowRoot.querySelector(".new-job-info-message").innerHTML =
              "";
            if (
              this.shadowRoot.querySelector(".new-job-hide-message-button") !=
              undefined
            ) {
              this.shadowRoot
                .querySelector(".new-job-hide-message-button")
                .remove();
            }
            if (
              this.shadowRoot.querySelector(".new-job-yes-button") != undefined
            ) {
              this.shadowRoot.querySelector(".new-job-yes-button").remove();
            }
            if (
              this.shadowRoot.querySelector(".new-job-no-button") != undefined
            ) {
              this.shadowRoot.querySelector(".new-job-no-button").remove();
            }
            this.dropHighlight(this.shadowRoot.querySelector(".new-job-name"));
          });
          this.shadowRoot
            .querySelector(".new-job-info")
            .append(newJobHideMessageButton);
        }
        return;
      } else {
        return;
      }
    }

    const newJobName = newJobNameField.value;
    const payload = this.buildSubmitPayload(newJobName);

    if (this.props.store.jobs_shelf.has(newJobName)) {
      if (newJobNameField.classList.contains("highlighted") === false) {
        newJobNameField.classList.add("highlighted");
      }
      if (
        this.shadowRoot.querySelector(".new-job-hide-message-button") !=
        undefined
      ) {
        this.shadowRoot.querySelector(".new-job-hide-message-button").remove();
      }
      if (this.shadowRoot.querySelector(".new-job-yes-button") != undefined) {
        this.shadowRoot.querySelector(".new-job-yes-button").remove();
      }
      if (this.shadowRoot.querySelector(".new-job-no-button") != undefined) {
        this.shadowRoot.querySelector(".new-job-no-button").remove();
      }
      this.shadowRoot.querySelector(
        ".new-job-info-message"
      ).innerHTML = `Note: The ${newJobName} does already exist! Do you want to overwrite ${newJobName}?`;
      const newJobYesButton = document.createElement("button");
      newJobYesButton.className = "new-job-yes-button";
      newJobYesButton.innerHTML = "Yes";
      newJobYesButton.addEventListener("click", () => {
        this.dropHighlight(this.shadowRoot.querySelector(".new-job-name"));
        this.shadowRoot.querySelector(".new-job-info-message").innerHTML = "";
        if (this.shadowRoot.querySelector(".new-job-yes-button") != undefined) {
          this.shadowRoot.querySelector(".new-job-yes-button").remove();
        }
        if (this.shadowRoot.querySelector(".new-job-no-button") != undefined) {
          this.shadowRoot.querySelector(".new-job-no-button").remove();
        }

        this.dispatchEvent(
          new CustomEvent(SUBMIT_JOB_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: {
              message: payload,
            },
          })
        );
      });
      const newJobNoButton = document.createElement("button");
      newJobNoButton.className = "new-job-no-button";
      newJobNoButton.innerHTML = "No";
      newJobNoButton.addEventListener("click", () => {
        this.dropHighlight(this.shadowRoot.querySelector(".new-job-name"));
        this.shadowRoot.querySelector(".new-job-info-message").innerHTML = "";
        if (this.shadowRoot.querySelector(".new-job-yes-button") != undefined) {
          this.shadowRoot.querySelector(".new-job-yes-button").remove();
        }
        if (this.shadowRoot.querySelector(".new-job-no-button") != undefined) {
          this.shadowRoot.querySelector(".new-job-no-button").remove();
        }
      });
      this.shadowRoot.querySelector(".new-job-info").append(newJobYesButton);
      this.shadowRoot.querySelector(".new-job-info").append(newJobNoButton);
      return;
    }

    this.dispatchEvent(
      new CustomEvent(SUBMIT_JOB_MESSAGE_HEADER, {
        bubbles: true,
        composed: true,
        detail: {
          message: payload,
        },
      })
    );
  }

  showJobAnalysisResult() {
    const selectedJobNameField = this.shadowRoot.querySelector(".job-name");
    if (selectedJobNameField.value == "") {
      if (selectedJobNameField.classList.contains("highlighted") === false) {
        selectedJobNameField.classList.add("highlighted");
      }
    }

    if (selectedJobNameField.value === "") {
      if (this.shadowRoot.querySelector(".job-info-message").innerHTML === "") {
        this.shadowRoot.querySelector(".job-info-message").innerHTML =
          "Note: The highlighted fields should be filled!";
        if (
          this.shadowRoot.querySelector(".job-hide-message-button") == undefined
        ) {
          const jobHideMessageButton = document.createElement("button");
          jobHideMessageButton.className = "job-hide-message-button";
          jobHideMessageButton.innerHTML = "Hide message";
          jobHideMessageButton.addEventListener("click", () => {
            this.shadowRoot.querySelector(".job-info-message").innerHTML = "";
            if (
              this.shadowRoot.querySelector(".job-hide-message-button") !=
              undefined
            ) {
              this.shadowRoot
                .querySelector(".job-hide-message-button")
                .remove();
            }
            this.dropHighlight(this.shadowRoot.querySelector(".job-name"));
          });
          this.shadowRoot
            .querySelector(".job-info")
            .append(jobHideMessageButton);
        }
        return;
      } else {
        return;
      }
    }

    const jobName = selectedJobNameField.value;
    const message = { job_name: jobName };

    this.dispatchEvent(
      new CustomEvent(SHOW_JOB_ANALYSIS_RESULT_MESSAGE_HEADER, {
        bubbles: true,
        composed: true,
        detail: {
          message: message,
        },
      })
    );
    this.shadowRoot.querySelector(".job-name-filter").value = null;
  }

  deleteJob() {
    const selectedJobNameField = this.shadowRoot.querySelector(".job-name");
    if (selectedJobNameField.value == "") {
      if (selectedJobNameField.classList.contains("highlighted") === false) {
        selectedJobNameField.classList.add("highlighted");
      }
    }

    if (selectedJobNameField.value === "") {
      if (this.shadowRoot.querySelector(".job-info-message").innerHTML === "") {
        this.shadowRoot.querySelector(".job-info-message").innerHTML =
          "Note: The highlighted fields should be filled!";
        if (
          this.shadowRoot.querySelector(".job-hide-message-button") == undefined
        ) {
          const jobHideMessageButton = document.createElement("button");
          jobHideMessageButton.className = "job-hide-message-button";
          jobHideMessageButton.innerHTML = "Hide message";
          jobHideMessageButton.addEventListener("click", () => {
            this.shadowRoot.querySelector(".job-info-message").innerHTML = "";
            if (
              this.shadowRoot.querySelector(".job-hide-message-button") !=
              undefined
            ) {
              this.shadowRoot
                .querySelector(".job-hide-message-button")
                .remove();
            }
            this.dropHighlight(this.shadowRoot.querySelector(".job-name"));
          });
          this.shadowRoot
            .querySelector(".job-info")
            .append(jobHideMessageButton);
        }
        return;
      } else {
        return;
      }
    }

    if (this.shadowRoot.querySelector(".job-info-message").innerHTML === "") {
      this.shadowRoot.querySelector(
        ".job-info-message"
      ).innerHTML = `Note: Do you really want to delete analysis result for ${selectedJobNameField.value}?`;
      const jobYesButton = document.createElement("button");
      jobYesButton.className = "job-yes-button";
      jobYesButton.innerHTML = "Yes";
      jobYesButton.addEventListener("click", () => {
        this.shadowRoot.querySelector(".job-info-message").innerHTML = "";
        if (this.shadowRoot.querySelector(".job-yes-button") != undefined) {
          this.shadowRoot.querySelector(".job-yes-button").remove();
        }
        if (this.shadowRoot.querySelector(".job-no-button") != undefined) {
          this.shadowRoot.querySelector(".job-no-button").remove();
        }

        const jobName = selectedJobNameField.value;
        const message = { job_name: jobName };

        this.dispatchEvent(
          new CustomEvent(DELETE_JOB_MESSAGE_HEADER, {
            bubbles: true,
            composed: true,
            detail: {
              message: message,
            },
          })
        );
      });
      const jobNoButton = document.createElement("button");
      jobNoButton.className = "job-no-button";
      jobNoButton.innerHTML = "No";
      jobNoButton.addEventListener("click", () => {
        this.shadowRoot.querySelector(".job-info-message").innerHTML = "";
        if (this.shadowRoot.querySelector(".job-yes-button") != undefined) {
          this.shadowRoot.querySelector(".job-yes-button").remove();
        }
        if (this.shadowRoot.querySelector(".job-no-button") != undefined) {
          this.shadowRoot.querySelector(".job-no-button").remove();
        }
      });
      this.shadowRoot.querySelector(".job-info").append(jobYesButton);
      this.shadowRoot.querySelector(".job-info").append(jobNoButton);

      this.shadowRoot.querySelector(".job-name-filter").value = null;
    }
  }

  filter(keywordField, selectField) {
    for (let i = 0; i < selectField.length; i++) {
      let txt = selectField.options[i].value;
      if (
        txt.substring(0, keywordField.length).toLowerCase() !==
          keywordField.toLowerCase() &&
        keywordField.trim() !== ""
      ) {
        selectField.options[i].style.display = "none";
      } else {
        selectField.options[i].style.display = "list-item";
      }
    }
  }

  dropHighlight(highlightedElement) {
    if (highlightedElement.classList.contains("highlighted") === true) {
      highlightedElement.classList.remove("highlighted");
    }
  }

  onSolverChanged() {
    const solver = this.getSelectedSolver();

    const jacobiExtra = this.shadowRoot.querySelector(".solver-extra-jacobi");
    const blockExtra = this.shadowRoot.querySelector(".solver-extra-block");
    const blockGpuExtra = this.shadowRoot.querySelector(
      ".solver-extra-block-gpu"
    );

    jacobiExtra.classList.add("hidden");
    blockExtra.classList.add("hidden");
    blockGpuExtra.classList.add("hidden");

    if (solver === "pcg_jacobi_cpu") {
      jacobiExtra.classList.remove("hidden");
    } else if (solver === "pcg_block_jacobi_cpu") {
      blockExtra.classList.remove("hidden");
    } else if (solver === "pcg_block_jacobi_gpu") {
      blockGpuExtra.classList.remove("hidden");
    }
  }
}

export default FeaAnalysisMenu;
