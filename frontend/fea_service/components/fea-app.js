import {
  ADD_POINT_EVENT_NAME,
  UPDATE_POINT_EVENT_NAME,
  DELETE_POINT_EVENT_NAME,
  ADD_LINE_EVENT_NAME,
  UPDATE_LINE_EVENT_NAME,
  DELETE_LINE_EVENT_NAME,
  ADD_SURFACE_EVENT_NAME,
  UPDATE_SURFACE_EVENT_NAME,
  DELETE_SURFACE_EVENT_NAME,
  ADD_MATERIAL_EVENT_NAME,
  UPDATE_MATERIAL_EVENT_NAME,
  DELETE_MATERIAL_EVENT_NAME,
  ADD_TRUSS_SECTION_EVENT_NAME,
  UPDATE_TRUSS_SECTION_EVENT_NAME,
  DELETE_TRUSS_SECTION_EVENT_NAME,
  ADD_BEAM_SECTION_EVENT_NAME,
  UPDATE_BEAM_SECTION_EVENT_NAME,
  DELETE_BEAM_SECTION_EVENT_NAME,
  ADD_PLATE_SECTION_EVENT_NAME,
  UPDATE_PLATE_SECTION_EVENT_NAME,
  DELETE_PLATE_SECTION_EVENT_NAME,
  ADD_PROPERTIES_EVENT_NAME,
  UPDATE_PROPERTIES_EVENT_NAME,
  DELETE_PROPERTIES_EVENT_NAME,
  ADD_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_EVENT_NAME,
  DELETE_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_EVENT_NAME,
  ADD_CONCENTRATED_LOAD_EVENT_NAME,
  UPDATE_CONCENTRATED_LOAD_EVENT_NAME,
  DELETE_CONCENTRATED_LOAD_EVENT_NAME,
  ADD_POINT_BOUNDARY_CONDITION_EVENT_NAME,
  UPDATE_POINT_BOUNDARY_CONDITION_EVENT_NAME,
  DELETE_POINT_BOUNDARY_CONDITION_EVENT_NAME,
  INCREASE_ACTION_ID_EVENT_NAME,
  UPDATE_GLOBAL_MESH_SEED_EVENT_NAME,
} from "../consts/preprocessor_consts.js";

import {
  UPDATE_POINT_VISIBILITY_MESSAGE_HEADER,
  UPDATE_LINE_VISIBILITY_MESSAGE_HEADER,
  UPDATE_SURFACE_VISIBILITY_MESSAGE_HEADER,
  UPDATE_SURFACE_EDGES_1_3_VISIBILITY_MESSAGE_HEADER,
  UPDATE_SURFACE_EDGES_2_4_VISIBILITY_MESSAGE_HEADER,
  UPDATE_SURFACE_NORMAL_VISIBILITY_MESSAGE_HEADER,
  UPDATE_BEAM_SECTION_ORIENTATION_VISIBILITY_MESSAGE_HEADER,
  UPDATE_LOAD_VISIBILITY_MESSAGE_HEADER,
  UPDATE_BOUNDARY_CONDITION_VISIBILITY_MESSAGE_HEADER,
  UPDATE_MESH_SEED_VISIBILITY_MESSAGE_HEADER,
  AUTO_FIT_MESSAGE_HEADER,
  CHANGE_VIEW_MESSAGE_HEADER,
  SAVE_PREPROCESSOR_DATA_MESSAGE_HEADER,
  SAVE_POSTPROCESSOR_DATA_MESSAGE_HEADER,
  UPDATE_NODE_VISIBILITY_MESSAGE_HEADER,
  UPDATE_TRUSS_ELEMENT_VISIBILITY_MESSAGE_HEADER,
  UPDATE_BEAM_ELEMENT_VISIBILITY_MESSAGE_HEADER,
  UPDATE_PLATE_ELEMENT_VISIBILITY_MESSAGE_HEADER,
  UPDATE_LOCAL_AXES_VISIBILITY_MESSAGE_HEADER,
  LOAD_PREPROCESSOR_DATA_MESSAGE_HEADER,
  RESET_PREPROCESSOR_DATA_MESSAGE_HEADER,
} from "../consts/tool_bar_consts.js";

import { SELECTED_OBJECTS_EVENT_NAME } from "../consts/renderer_consts.js";

import {
  PREVIEW_SELECTED_LINE_NUMBERS_MESSAGE_HEADER,
  PREVIEW_SELECTED_SURFACE_NUMBERS_MESSAGE_HEADER,
  DECREASE_ACTION_ID_MESSAGE_HEADER,
  ENABLE_LINES_SELECTION_MODE_MESSAGE_HEADER,
  ENABLE_SURFACES_SELECTION_MODE_MESSAGE_HEADER,
  DISABLE_LINES_SELECTION_MODE_MESSAGE_HEADER,
  DISABLE_SURFACES_SELECTION_MODE_MESSAGE_HEADER,
  CLIENT_MESSAGE_HEADER,
  SUBMIT_JOB_MESSAGE_HEADER,
  DELETE_JOB_MESSAGE_HEADER,
  SHOW_JOB_ANALYSIS_RESULT_MESSAGE_HEADER,
  WASM_LOADED_MESSAGE_HEADER,
  RENDERER_LOADED_MESSAGE_HEADER,
  RESIZE_MESSAGE_HEADER,
  ACTIVATE_PREPROCESSOR_MENU_MESSAGE_HEADER,
  GET_ACTION_ID_MESSAGE_HEADER,
  GET_WASM_LOADING_STATUS_MESSAGE_HEADER,
  GET_RENDERER_LOADING_STATUS_MESSAGE_HEADER,
  PLOT_DISPLACEMENTS_MESSAGE_HEADER,
  PLOT_GLOBAL_FORCES_MESSAGE_HEADER,
  PLOT_GLOBAL_MOMENTS_MESSAGE_HEADER,
  PLOT_ELEMENTS_LOADS_MESSAGE_HEADER,
} from "../consts/fea_app_consts.js";

import {
  SOLVER_MESSAGE_EVENT_HEADER,
  COMPLETED_STATUS,
  ERROR_STATUS,
  CREATE_FEM_MESSAGE_HEADER,
  PERFORM_GLOBAL_ANALYSIS_DIRECT_MESSAGE_HEADER,
  PERFORM_GLOBAL_ANALYSIS_ITERATIVE_PCG_JACOBI_MESSAGE_HEADER,
  PERFORM_GLOBAL_ANALYSIS_ITERATIVE_PCG_BLOCK_JACOBI_MESSAGE_HEADER,
  PERFORM_GLOBAL_ANALYSIS_ITERATIVE_PCG_BLOCK_JACOBI_GPU_MESSAGE_HEADER,
  EXTRACT_GLOBAL_ANALYSIS_RESULT_MESSAGE_HEADER,
  EXTRACT_ELEMENTS_ANALYSIS_RESULT_MESSAGE_HEADER,
} from "../consts/solver_consts.js";

import { Communicator } from "../fea_communicator.js";

import Store from "../store/fea_store.js";

class FeaApp extends HTMLElement {
  constructor() {
    super();

    (this.props = {
      username: null,
      authLoginUrl: null,
      authSignupUrl: null,
      logoutUrl: null,
      accoutDetailsUrl: null,
      toolsUrl: null,
    }),
      (this.state = {
        actionId: 1,
        activeMenuName: null,
        communicator: new Communicator(),
        isWasmLoaded: false,
        isRendererLoaded: false,
        isLinesSelectionModeEnabled: false,
        isSurfacesSelectionModeEnabled: false,
        isProcessing: false,
        store: new Store(),

        linesSelectionDependentMenus: [
          "fea-properties-assign-properties-to-lines-menu",
          "fea-properties-beam-section-orientation-menu",
          "fea-mesh-lines-mesh-seed-menu",
        ],

        surfacesSelectionDependentMenus: [
          "fea-properties-assign-properties-to-surfaces-menu",
          "fea-mesh-surfaces-mesh-seed-menu",
        ],

        activeJobName: null,
      });

    this.attachShadow({ mode: "open" });

    this.shadowRoot.innerHTML =
      /*html*/
      `
        <style>
            :host {
                display: flex;
            }

            .main-window {
                padding: 0rem;
                margin: 0rem;
                display: block;
            }

            .wrapper {
                display: flex;
                align-items: start;
                flex-direction: row-reverse;
                box-sizing: content-box;
            }
        </style>
        <div class="main-window">
            <custom-app-menu-bar 
                username="${this.getAttribute("username")}"
                auth-login-url="${this.getAttribute("auth-login-url")}"
                auth-signup-url="${this.getAttribute("auth-signup-url")}"
                logout-url="${this.getAttribute("logout-url")}"
                auth-account-details-url="${this.getAttribute(
                  "auth-account-details-url"
                )}"
                tools-url="${this.getAttribute("tools-url")}"
                is-logged-in="${this.getAttribute("is-logged-in")}"
            >
            </custom-app-menu-bar>
            <fea-app-tool-bar action-id="${
              this.state.actionId
            }"></fea-app-tool-bar>
            <div class="wrapper">
                <fea-renderer></fea-renderer>
                <slot></slot>
            </div>
        </div>
        `;

    this.addEventListener(WASM_LOADED_MESSAGE_HEADER, (event) => {
      this.state.isWasmLoaded = true;
      event.stopPropagation();
    });

    this.addEventListener(RENDERER_LOADED_MESSAGE_HEADER, (event) => {
      this.state.isRendererLoaded = true;
      event.stopPropagation();
    });

    window.addEventListener(RESIZE_MESSAGE_HEADER, () =>
      this.updateCanvasSize()
    );

    this.addEventListener(ACTIVATE_PREPROCESSOR_MENU_MESSAGE_HEADER, () =>
      this.activatePreprocessorMenu()
    );

    this.addEventListener(GET_ACTION_ID_MESSAGE_HEADER, (event) =>
      this.getActionId(event)
    );

    this.addEventListener(GET_WASM_LOADING_STATUS_MESSAGE_HEADER, (event) =>
      this.getWasmLoadingStatus(event)
    );
    this.addEventListener(GET_RENDERER_LOADING_STATUS_MESSAGE_HEADER, (event) =>
      this.getRendererLoadingStatus(event)
    );

    this.addEventListener(SELECTED_OBJECTS_EVENT_NAME, (event) =>
      this.handleSelectedObjectsMessage(event)
    );

    this.addEventListener(UPDATE_POINT_VISIBILITY_MESSAGE_HEADER, (event) =>
      this.handleUpdatePointVisibilityMessage(event)
    );
    this.addEventListener(UPDATE_LINE_VISIBILITY_MESSAGE_HEADER, (event) =>
      this.handleUpdateLineVisibilityMessage(event)
    );
    this.addEventListener(UPDATE_SURFACE_VISIBILITY_MESSAGE_HEADER, (event) =>
      this.handleUpdateSurfaceVisibilityMessage(event)
    );
    this.addEventListener(
      UPDATE_SURFACE_EDGES_1_3_VISIBILITY_MESSAGE_HEADER,
      (event) => this.handleUpdateSurfaceEdges13VisibilityMessage(event)
    );
    this.addEventListener(
      UPDATE_SURFACE_EDGES_2_4_VISIBILITY_MESSAGE_HEADER,
      (event) => this.handleUpdateSurfaceEdges24VisibilityMessage(event)
    );
    this.addEventListener(
      UPDATE_SURFACE_NORMAL_VISIBILITY_MESSAGE_HEADER,
      (event) => this.handleUpdateSurfaceNormalVisibilityMessage(event)
    );
    this.addEventListener(
      UPDATE_BEAM_SECTION_ORIENTATION_VISIBILITY_MESSAGE_HEADER,
      (event) => this.handleUpdateBeamSectionOrientationVisibilityMessage(event)
    );
    this.addEventListener(UPDATE_LOAD_VISIBILITY_MESSAGE_HEADER, (event) =>
      this.handleUpdateLoadVisibilityMessage(event)
    );
    this.addEventListener(
      UPDATE_BOUNDARY_CONDITION_VISIBILITY_MESSAGE_HEADER,
      (event) => this.handleUpdateBoundaryConditionVisibilityMessage(event)
    );
    this.addEventListener(UPDATE_MESH_SEED_VISIBILITY_MESSAGE_HEADER, (event) =>
      this.handleUpdateMeshSeedVisibilityMessage(event)
    );
    this.addEventListener(CHANGE_VIEW_MESSAGE_HEADER, (event) =>
      this.handleChangeViewMessage(event)
    );
    this.addEventListener(AUTO_FIT_MESSAGE_HEADER, (event) =>
      this.handleAutoFitMessage(event)
    );

    this.addEventListener(UPDATE_NODE_VISIBILITY_MESSAGE_HEADER, (event) =>
      this.handleUpdateNodeVisibilityMessage(event)
    );
    this.addEventListener(
      UPDATE_TRUSS_ELEMENT_VISIBILITY_MESSAGE_HEADER,
      (event) => this.handleUpdateTrussElementVisibilityMessage(event)
    );
    this.addEventListener(
      UPDATE_BEAM_ELEMENT_VISIBILITY_MESSAGE_HEADER,
      (event) => this.handleUpdateBeamElementVisibilityMessage(event)
    );
    this.addEventListener(
      UPDATE_PLATE_ELEMENT_VISIBILITY_MESSAGE_HEADER,
      (event) => this.handleUpdatePlateElementVisibilityMessage(event)
    );
    this.addEventListener(
      UPDATE_LOCAL_AXES_VISIBILITY_MESSAGE_HEADER,
      (event) => this.handleUpdateLocalAxesVisibilityMessage(event)
    );

    this.addEventListener(SAVE_PREPROCESSOR_DATA_MESSAGE_HEADER, (event) =>
      this.handleSavePreprocessorDataMessage(event)
    );
    this.addEventListener(LOAD_PREPROCESSOR_DATA_MESSAGE_HEADER, (event) =>
      this.handleLoadPreprocessorDataMessage(event)
    );
    this.addEventListener(RESET_PREPROCESSOR_DATA_MESSAGE_HEADER, (_event) =>
      this.handleResetPreprocessorDataMessage()
    );

    this.addEventListener(SAVE_POSTPROCESSOR_DATA_MESSAGE_HEADER, (event) =>
      this.handleSavePostprocessorDataMessage(event)
    );

    this.addEventListener(
      PREVIEW_SELECTED_LINE_NUMBERS_MESSAGE_HEADER,
      (event) => this.handlePreviewSelectedLineNumbersMessage(event)
    );
    this.addEventListener(
      PREVIEW_SELECTED_SURFACE_NUMBERS_MESSAGE_HEADER,
      (event) => this.handlePreviewSelectedSurfaceNumbersMessage(event)
    );

    this.addEventListener(CLIENT_MESSAGE_HEADER, (event) =>
      this.handleClientMessage(event)
    );

    this.addEventListener(SUBMIT_JOB_MESSAGE_HEADER, (event) =>
      this.handleSubmitJobMessage(event)
    );
    this.addEventListener(SHOW_JOB_ANALYSIS_RESULT_MESSAGE_HEADER, (event) =>
      this.handleShowJobAnalysisResultMessage(event)
    );
    this.addEventListener(DELETE_JOB_MESSAGE_HEADER, (event) =>
      this.handleDeleteJobMessage(event)
    );

    this.addEventListener(SOLVER_MESSAGE_EVENT_HEADER, (event) =>
      this.handleSolverMessage(event)
    );

    this.addEventListener(PLOT_DISPLACEMENTS_MESSAGE_HEADER, (event) =>
      this.handlePlotDisplacementsMessage(event)
    );
    this.addEventListener(PLOT_GLOBAL_FORCES_MESSAGE_HEADER, (event) =>
      this.handlePlotGlobalForcesMessage(event)
    );
    this.addEventListener(PLOT_GLOBAL_MOMENTS_MESSAGE_HEADER, (event) =>
      this.handlePlotGlobalMomentsMessage(event)
    );

    this.addEventListener(PLOT_ELEMENTS_LOADS_MESSAGE_HEADER, (event) =>
      this.handleplotElementsLoadsMessage(event)
    );

    this.addEventListener(ADD_POINT_EVENT_NAME, (event) =>
      this.handleAddPointServerMessage(event)
    );
    this.addEventListener(UPDATE_POINT_EVENT_NAME, (event) =>
      this.handleUpdatePointServerMessage(event)
    );
    this.addEventListener(DELETE_POINT_EVENT_NAME, (event) =>
      this.handleDeletePointServerMessage(event)
    );

    this.addEventListener(ADD_LINE_EVENT_NAME, (event) =>
      this.handleAddLineServerMessage(event)
    );
    this.addEventListener(UPDATE_LINE_EVENT_NAME, (event) =>
      this.handleUpdateLineServerMessage(event)
    );
    this.addEventListener(DELETE_LINE_EVENT_NAME, (event) =>
      this.handleDeleteLineServerMessage(event)
    );

    this.addEventListener(ADD_SURFACE_EVENT_NAME, (event) =>
      this.handleAddSurfaceServerMessage(event)
    );
    this.addEventListener(UPDATE_SURFACE_EVENT_NAME, (event) =>
      this.handleUpdateSurfaceServerMessage(event)
    );
    this.addEventListener(DELETE_SURFACE_EVENT_NAME, (event) =>
      this.handleDeleteSurfaceServerMessage(event)
    );

    this.addEventListener(ADD_MATERIAL_EVENT_NAME, (event) =>
      this.handleAddMaterialServerMessage(event)
    );
    this.addEventListener(UPDATE_MATERIAL_EVENT_NAME, (event) =>
      this.handleUpdateMaterialServerMessage(event)
    );
    this.addEventListener(DELETE_MATERIAL_EVENT_NAME, (event) =>
      this.handleDeleteMaterialServerMessage(event)
    );

    this.addEventListener(ADD_TRUSS_SECTION_EVENT_NAME, (event) =>
      this.handleAddTrussSectionServerMessage(event)
    );
    this.addEventListener(UPDATE_TRUSS_SECTION_EVENT_NAME, (event) =>
      this.handleUpdateTrussSectionServerMessage(event)
    );
    this.addEventListener(DELETE_TRUSS_SECTION_EVENT_NAME, (event) =>
      this.handleDeleteTrussSectionServerMessage(event)
    );

    this.addEventListener(ADD_BEAM_SECTION_EVENT_NAME, (event) =>
      this.handleAddBeamSectionServerMessage(event)
    );
    this.addEventListener(UPDATE_BEAM_SECTION_EVENT_NAME, (event) =>
      this.handleUpdateBeamSectionServerMessage(event)
    );
    this.addEventListener(DELETE_BEAM_SECTION_EVENT_NAME, (event) =>
      this.handleDeleteBeamSectionServerMessage(event)
    );

    this.addEventListener(ADD_PLATE_SECTION_EVENT_NAME, (event) =>
      this.handleAddPlateSectionServerMessage(event)
    );
    this.addEventListener(UPDATE_PLATE_SECTION_EVENT_NAME, (event) =>
      this.handleUpdatePlateSectionServerMessage(event)
    );
    this.addEventListener(DELETE_PLATE_SECTION_EVENT_NAME, (event) =>
      this.handleDeletePlateSectionServerMessage(event)
    );

    this.addEventListener(ADD_PROPERTIES_EVENT_NAME, (event) =>
      this.handleAddPropertiesServerMessage(event)
    );
    this.addEventListener(UPDATE_PROPERTIES_EVENT_NAME, (event) =>
      this.handleUpdatePropertiesServerMessage(event)
    );
    this.addEventListener(DELETE_PROPERTIES_EVENT_NAME, (event) =>
      this.handleDeletePropertiesServerMessage(event)
    );

    this.addEventListener(
      ADD_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_EVENT_NAME,
      (event) =>
        this.handleAddBeamSectionLocalAxis1DirectionServerMessage(event)
    );
    this.addEventListener(
      DELETE_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_EVENT_NAME,
      (event) =>
        this.handleDeleteBeamSectionLocalAxis1DirectionServerMessage(event)
    );

    this.addEventListener(ADD_CONCENTRATED_LOAD_EVENT_NAME, (event) =>
      this.handleAddConcentratedLoadServerMessage(event)
    );
    this.addEventListener(UPDATE_CONCENTRATED_LOAD_EVENT_NAME, (event) =>
      this.handleUpdateConcentratedLoadServerMessage(event)
    );
    this.addEventListener(DELETE_CONCENTRATED_LOAD_EVENT_NAME, (event) =>
      this.handleDeleteConcentratedLoadServerMessage(event)
    );

    this.addEventListener(ADD_POINT_BOUNDARY_CONDITION_EVENT_NAME, (event) =>
      this.handleAddPointBoundaryConditionServerMessage(event)
    );
    this.addEventListener(UPDATE_POINT_BOUNDARY_CONDITION_EVENT_NAME, (event) =>
      this.handleUpdatePointBoundaryConditionServerMessage(event)
    );
    this.addEventListener(DELETE_POINT_BOUNDARY_CONDITION_EVENT_NAME, (event) =>
      this.handleDeletePointBoundaryConditionServerMessage(event)
    );

    this.addEventListener(UPDATE_GLOBAL_MESH_SEED_EVENT_NAME, (event) =>
      this.handleUpdateGlobalMeshSeedServerMessage(event)
    );

    this.addEventListener(INCREASE_ACTION_ID_EVENT_NAME, (_event) =>
      this.handleIncreaseActionIdMessage()
    );
    this.addEventListener(DECREASE_ACTION_ID_MESSAGE_HEADER, (_event) =>
      this.handleDecreaseActionIdMessage()
    );

    this.addEventListener(ENABLE_LINES_SELECTION_MODE_MESSAGE_HEADER, (event) =>
      this.handleEnableLinesSelectionModeMessage(event)
    );

    this.addEventListener(
      ENABLE_SURFACES_SELECTION_MODE_MESSAGE_HEADER,
      (event) => this.handleEnableSurfacesSelectionModeMessage(event)
    );

    this.addEventListener(
      DISABLE_LINES_SELECTION_MODE_MESSAGE_HEADER,
      (event) => this.handleDisableLinesSelectionModeMessage(event)
    );

    this.addEventListener(
      DISABLE_SURFACES_SELECTION_MODE_MESSAGE_HEADER,
      (event) => this.handleDisableSurfacesSelectionModeMessage(event)
    );
  }

  set feModelError(error) {
    throw error;
  }

  set actionIdSetter(delta) {
    this.state.actionId += delta;
    this.shadowRoot
      .querySelector("fea-app-tool-bar")
      .setAttribute("action-id", this.state.actionId);
    if (this.state.activeMenuName !== null) {
      this.querySelector(this.state.activeMenuName).setAttribute(
        "action-id",
        this.state.actionId
      );
    }
  }

  async connectedCallback() {
    await this.state.communicator.initWasmModules();
    this.activatePreprocessorMenu();
  }

  disconnectedCallback() {}

  static get observedAttributes() {
    return [
      "username",
      "auth-login-url",
      "auth-signup-url",
      "logout-url",
      "auth-account-details-url",
      "tools-url",
      "is-logged-in",
    ];
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (name === "username") {
      this.props.username = newValue;
      if (this.shadowRoot.querySelector("custom-app-menu-bar") !== null) {
        this.shadowRoot
          .querySelector("custom-app-menu-bar")
          .setAttribute("username", newValue);
      }
    }

    if (name === "auth-login-url") {
      this.props.authLoginUrl = newValue;
      if (this.shadowRoot.querySelector("custom-app-menu-bar") !== null) {
        this.shadowRoot
          .querySelector("custom-app-menu-bar")
          .setAttribute("auth-login-url", newValue);
      }
    }

    if (name === "auth-signup-url") {
      this.props.authSignupUrl = newValue;
      if (this.shadowRoot.querySelector("custom-app-menu-bar") !== null) {
        this.shadowRoot
          .querySelector("custom-app-menu-bar")
          .setAttribute("auth-signup-url", newValue);
      }
    }

    if (name === "logout-url") {
      this.props.logoutUrl = newValue;
      if (this.shadowRoot.querySelector("custom-app-menu-bar") !== null) {
        this.shadowRoot
          .querySelector("custom-app-menu-bar")
          .setAttribute("logout-url", newValue);
      }
    }

    if (name === "auth-account-details-url") {
      this.props.accoutDetailsUrl = newValue;
      if (this.shadowRoot.querySelector("custom-app-menu-bar") !== null) {
        this.shadowRoot
          .querySelector("custom-app-menu-bar")
          .setAttribute("auth-account-details-url", newValue);
      }
    }

    if (name === "tools-url") {
      this.props.toolsUrl = newValue;
      if (this.shadowRoot.querySelector("custom-app-menu-bar") !== null) {
        this.shadowRoot
          .querySelector("custom-app-menu-bar")
          .setAttribute("tools-url", newValue);
      }
    }

    if (name === "is-logged-in") {
      this.props.isLoggedIn = newValue;
      if (this.shadowRoot.querySelector("custom-app-menu-bar") !== null) {
        this.shadowRoot
          .querySelector("custom-app-menu-bar")
          .setAttribute("is-logged-in", newValue);
      }
    }
  }

  adoptedCallback() {}

  activatePreprocessorMenu() {
    if (this.querySelector("fea-postprocessor-menu") !== null) {
      this.querySelector("fea-postprocessor-menu").remove();
    }
    const feaPreprocessorMenu = document.createElement("fea-preprocessor-menu");
    this.append(feaPreprocessorMenu);
    this.state.activeMenuName = "fea-preprocessor-menu";
    feaPreprocessorMenu.setAttribute("action-id", this.state.actionId);
    feaPreprocessorMenu.setAttribute("is-processing", this.state.isProcessing);

    const frame = () => {
      if (this.state.isRendererLoaded === true) {
        this.shadowRoot
          .querySelector("fea-app-tool-bar")
          .setAttribute("is-postprocessor-active", false);
        this.shadowRoot
          .querySelector("fea-app-tool-bar")
          .setAttribute("is-preprocessor-active", true);
        this.shadowRoot
          .querySelector("fea-app-tool-bar")
          .setAttribute("job-name", null);
        this.shadowRoot
          .querySelector("fea-renderer")
          .activatePreprocessorState();
        this.updateCanvasSize();
        if (this.state.activeJobName) {
          this.state.activeJobName = null;
        }
        clearInterval(id);
      }
    };
    const id = setInterval(frame, 10);
  }

  activatePostprocessorMenu(jobName, job) {
    const frame = () => {
      if (this.state.isRendererLoaded === true) {
        if (this.querySelector("fea-preprocessor-menu") !== null) {
          this.querySelector("fea-preprocessor-menu").remove();
        }
        const feaPostprocessorMenu = document.createElement(
          "fea-postprocessor-menu"
        );
        this.append(feaPostprocessorMenu);
        this.state.activeMenuName = "fea-postprocessor-menu";
        feaPostprocessorMenu.setAttribute("job-name", jobName);
        this.shadowRoot
          .querySelector("fea-app-tool-bar")
          .setAttribute("is-preprocessor-active", false);
        this.shadowRoot
          .querySelector("fea-app-tool-bar")
          .setAttribute("is-postprocessor-active", true);
        this.shadowRoot
          .querySelector("fea-app-tool-bar")
          .setAttribute("job-name", jobName);
        this.shadowRoot.querySelector("fea-renderer").objectInfo = "";
        this.shadowRoot
          .querySelector("fea-renderer")
          .activatePostprocessorState(job);
        this.updateCanvasSize();
        this.state.activeJobName = jobName;
        clearInterval(id);
      }
    };
    const id = setInterval(frame, 10);
  }

  getActionId(event) {
    this.querySelector(event.target.tagName.toLowerCase()).actionId =
      this.state.actionId;
    event.stopPropagation();
  }

  getWasmLoadingStatus(event) {
    this.querySelector(event.target.tagName.toLowerCase()).isWasmLoaded =
      this.state.isWasmLoaded;
    event.stopPropagation();
  }

  getRendererLoadingStatus(event) {
    this.querySelector(event.target.tagName.toLowerCase()).isRendererLoaded =
      this.state.isRendererLoaded;
    event.stopPropagation();
  }

  // showObjectInfoHandler(objectInfo) {
  //     if ("point_data" in objectInfo) {
  //         const pointNumber = objectInfo.point_data.number;
  //         const composedObjectInfo = `Point:
  //             number: ${pointNumber},
  //             x: ${objectInfo.point_data.x},
  //             y: ${objectInfo.point_data.y},
  //             z: ${objectInfo.point_data.z}`;
  //         this.shadowRoot.querySelector("fea-renderer").objectInfo = composedObjectInfo;
  //         if (this.querySelector("fea-preprocessor-menu") !== null) {
  //             this.querySelector("fea-preprocessor-menu").selectPointInClient = pointNumber;
  //         }
  //     } else {
  //         throw "Fea-app: Unknown object!";
  //     }
  // }

  selectLinesInClientForDataAssign(selectedLineNumbers) {
    for (let i = 0; i < this.state.linesSelectionDependentMenus.length; i++) {
      if (
        this.querySelector(this.state.linesSelectionDependentMenus[i]) !== null
      ) {
        if (Array.isArray(selectedLineNumbers)) {
          for (let j = 0; j < selectedLineNumbers.length; j++) {
            this.querySelector(
              this.state.linesSelectionDependentMenus[i]
            ).selectLineInClientForDataAssign = selectedLineNumbers[j];
          }
        } else if (Number.isInteger(selectedLineNumbers)) {
          this.querySelector(
            this.state.linesSelectionDependentMenus[i]
          ).selectLineInClientForDataAssign = selectedLineNumbers;
        } else {
          throw "Fea-app: Unknown object!";
        }
      }
    }
  }

  selectSurfacesInClientForDataAssign(selectedSurfacesNumbers) {
    for (
      let i = 0;
      i < this.state.surfacesSelectionDependentMenus.length;
      i++
    ) {
      if (
        this.querySelector(this.state.surfacesSelectionDependentMenus[i]) !==
        null
      ) {
        if (Array.isArray(selectedSurfacesNumbers)) {
          for (let j = 0; j < selectedSurfacesNumbers.length; j++) {
            this.querySelector(
              this.state.surfacesSelectionDependentMenus[i]
            ).selectSurfaceInClientForDataAssign = selectedSurfacesNumbers[j];
          }
        } else if (Number.isInteger(selectedSurfacesNumbers)) {
          this.querySelector(
            this.state.surfacesSelectionDependentMenus[i]
          ).selectSurfaceInClientForDataAssign = selectedSurfacesNumbers;
        } else {
          throw "Fea-app: Unknown object!";
        }
      }
    }
  }

  composeSelectedPoints(uids) {
    let selectedPointsNumbers = Array();
    let selectedPoints = Array();
    this.state.store.points_shelf.forEach((point, number) => {
      if (uids.includes(point.uid)) {
        selectedPointsNumbers.push(number);
        selectedPoints.push(
          `Point: number: ${number}, x: ${point.x}, y: ${point.y}, z: ${point.z}`
        );
      }
    });
    return [selectedPointsNumbers, selectedPoints];
  }

  composeSelectedLines(uids) {
    let selectedLineNumbers = Array();
    let selectedLines = Array();
    this.state.store.lines_shelf.forEach((line, number) => {
      if (uids.includes(line.uid)) {
        selectedLineNumbers.push(number);
        if (line.optional_property) {
          selectedLines.push(`Line: number: ${number}, start point number: ${
            line.point_1_number
          },
                        end point number: ${line.point_2_number}, 
                        assigned property: ${line.optional_property.name.replace(
                          /['"]+/g,
                          ""
                        )},
                        cross section type: ${
                          Object.entries(
                            line.optional_property.property.cross_section
                          )[0][0]
                        },
                        cross section name: ${Object.entries(
                          line.optional_property.property.cross_section
                        )[0][1].replace(/['"]+/g, "")},
                        material: ${line.optional_property.property.material_name.replace(
                          /['"]+/g,
                          ""
                        )}`);
        } else {
          selectedLines.push(`Line: number: ${number}, start point number: ${line.point_1_number},
                        end point number: ${line.point_2_number}`);
        }
      }
    });
    return [selectedLineNumbers, selectedLines];
  }

  composeSelectedSurfaces(uids) {
    let selectedSurfacesNumbers = Array();
    let selectedSurfaces = Array();
    this.state.store.surfaces_shelf.forEach((surface, number) => {
      if (uids.includes(surface.uid)) {
        selectedSurfacesNumbers.push(number);
        if (surface.optional_property) {
          selectedSurfaces.push(`Surface: number: ${number}, point 1 number: ${
            surface.point_1_number
          },
                        point 2 number: ${
                          surface.point_2_number
                        }, point 3 number: ${surface.point_3_number},
                        point 4 number: ${surface.point_4_number}
                        assigned property: ${surface.optional_property.name.replace(
                          /['"]+/g,
                          ""
                        )},
                        cross section type: ${
                          Object.entries(
                            surface.optional_property.property.cross_section
                          )[0][0]
                        },
                        cross section name: ${Object.entries(
                          surface.optional_property.property.cross_section
                        )[0][1].replace(/['"]+/g, "")},
                        material: ${surface.optional_property.property.material_name.replace(
                          /['"]+/g,
                          ""
                        )}`);
        } else {
          selectedSurfaces.push(`Surface: number: ${number}, point 1 number: ${surface.point_1_number},
                        point 2 number: ${surface.point_2_number}, point 3 number: ${surface.point_3_number},
                        point 4 number: ${surface.point_4_number}`);
        }
      }
    });
    return [selectedSurfacesNumbers, selectedSurfaces];
  }

  composeSelectedConcentratedLoads(uids) {
    let selectedPointNumbers = Array();
    let selectedConcentratedLoads = Array();
    this.state.store.concentrated_loads_shelf.forEach(
      (concentrated_load, number) => {
        if (uids.includes(concentrated_load.uid)) {
          selectedPointNumbers.push(number);
          selectedConcentratedLoads.push(`Concentrated load: point number: ${number}, 
                    Fx: ${concentrated_load.fx}, Fy: ${concentrated_load.fy}, Fz: ${concentrated_load.fz}, 
                    Mx: ${concentrated_load.mx}, My: ${concentrated_load.my}, Mz: ${concentrated_load.mz}`);
        }
      }
    );
    return [selectedPointNumbers, selectedConcentratedLoads];
  }

  composeSelectedUniformlyDistributedLineLoads(uids) {
    let selectedLineNumbers = Array();
    let selectedUniformlyDistributedLineLoads = Array();
    this.state.store.lines_shelf.forEach((line, number) => {
      if (line.optional_uniformly_distributed_line_load) {
        if (uids.includes(line.optional_uniformly_distributed_line_load[0])) {
          selectedLineNumbers.push(number);
          selectedUniformlyDistributedLineLoads.push(`
                        Uniformly distributed line load: line number: ${number}, 
                        Qx: ${line.optional_uniformly_distributed_line_load[1]}, 
                        Qy: ${line.optional_uniformly_distributed_line_load[2]}, 
                        Qz: ${line.optional_uniformly_distributed_line_load[3]}`);
        }
      }
    });
    return [selectedLineNumbers, selectedUniformlyDistributedLineLoads];
  }

  composeSelectedUniformlyDistributedSurfaceLoads(uids) {
    let selectedSurfaceNumbers = Array();
    let selectedUniformlyDistributedSurfaceLoads = Array();
    this.state.store.surfaces_shelf.forEach((surface, number) => {
      if (surface.optional_uniformly_distributed_surface_load) {
        if (
          uids.includes(surface.optional_uniformly_distributed_surface_load[0])
        ) {
          selectedSurfaceNumbers.push(number);
          selectedUniformlyDistributedSurfaceLoads.push(`
                        Uniformly distributed surface load: surface number: ${number}, 
                        Px: ${surface.optional_uniformly_distributed_surface_load[1]}, 
                        Py: ${surface.optional_uniformly_distributed_surface_load[2]}, 
                        Pz: ${surface.optional_uniformly_distributed_surface_load[3]}`);
        }
      }
    });
    return [selectedSurfaceNumbers, selectedUniformlyDistributedSurfaceLoads];
  }

  composeSelectedPointBoundaryConditions(uids) {
    let selectedPointNumbers = Array();
    let selectedPointBoundaryConditions = Array();
    this.state.store.point_boundary_conditions_shelf.forEach(
      (point_boundary_condition, number) => {
        if (uids.includes(point_boundary_condition.uid)) {
          selectedPointNumbers.push(number);
          const uX =
            point_boundary_condition.optional_ux !== null
              ? point_boundary_condition.optional_ux
              : "Free";
          const uY =
            point_boundary_condition.optional_uy !== null
              ? point_boundary_condition.optional_uy
              : "Free";
          const uZ =
            point_boundary_condition.optional_uz !== null
              ? point_boundary_condition.optional_uz
              : "Free";
          const rX =
            point_boundary_condition.optional_rx !== null
              ? point_boundary_condition.optional_rx
              : "Free";
          const rY =
            point_boundary_condition.optional_ry !== null
              ? point_boundary_condition.optional_ry
              : "Free";
          const rZ =
            point_boundary_condition.optional_rz !== null
              ? point_boundary_condition.optional_rz
              : "Free";
          selectedPointBoundaryConditions.push(`Point boundary condition: point number: ${number}, 
                    Ux: ${uX}, Uy: ${uY}, Uz: ${uZ}, Rx: ${rX}, Ry: ${rY}, Rz: ${rZ}`);
        }
      }
    );
    return [selectedPointNumbers, selectedPointBoundaryConditions];
  }

  composeSelectedNodes(uids) {
    let selectedNodesNumbers = Array();
    let selectedNodes = Array();
    if (this.state.activeJobName) {
      const job = this.state.store.jobs_shelf.get(this.state.activeJobName);
      if (job) {
        for (const [number, node] of Object.entries(job.mesh.nodes)) {
          if (uids.includes(node[0])) {
            selectedNodesNumbers.push(number);
            let displacements = Array();
            let loads = Array();
            for (const [key, data] of Object.entries(
              job.global_analysis_result
            )) {
              if (key === number) {
                displacements.push(
                  `Ux: ${data[0]}, Uy: ${data[1]}, Uz: ${data[2]}, 
                                    Rx: ${data[3]}, Ry: ${data[4]}, Rz: ${data[5]}`
                );
                loads.push(
                  `Fx: ${data[6]}, Fy: ${data[7]}, Fz: ${data[8]}, 
                                    Mx: ${data[9]}, My: ${data[10]}, Mz: ${data[11]}`
                );
              }
            }
            selectedNodes.push(`Node: number ${number}, x: ${node[1][0]}, y: ${node[1][1]}, 
                            z: ${node[1][2]}, displacements: ${displacements}, loads: ${loads}`);
          }
        }
      }
    }
    return [selectedNodesNumbers, selectedNodes];
  }

  composeSelectedTrussElements(uids) {
    let selectedTrussElementsNumbers = Array();
    let selectedTrussElements = Array();
    if (this.state.activeJobName) {
      const job = this.state.store.jobs_shelf.get(this.state.activeJobName);
      if (job) {
        for (const [number, element] of Object.entries(
          job.mesh.truss_elements
        )) {
          if (uids.includes(element[0])) {
            selectedTrussElementsNumbers.push(number);
            let elementLoads = Array();
            for (const [key, data] of Object.entries(
              job.elements_analysis_result.truss_elements
            )) {
              if (key === number) {
                elementLoads.push(`Force_R: ${data[9]}`);
              }
            }
            selectedTrussElements.push(
              `Truss element: number ${number}, element forces: ${elementLoads}`
            );
          }
        }
      }
    }
    return [selectedTrussElementsNumbers, selectedTrussElements];
  }

  composeSelectedBeamElements(uids) {
    let selectedBeamElementsNumbers = Array();
    let selectedBeamElements = Array();
    if (this.state.activeJobName) {
      const job = this.state.store.jobs_shelf.get(this.state.activeJobName);
      if (job) {
        for (const [number, element] of Object.entries(
          job.mesh.beam_elements
        )) {
          if (uids.includes(element[0])) {
            selectedBeamElementsNumbers.push(number);
            let elementLoads = Array();
            for (const [key, data] of Object.entries(
              job.elements_analysis_result.beam_elements
            )) {
              if (key === number) {
                elementLoads.push(
                  `
                                    Force_R: ${data[9]},
                                    Force_S: ${data[10]},
                                    Force_T: ${data[11]},
                                    Moment_R: ${data[12]},
                                    Moment_S_node_1: ${data[13]},
                                    Moment_S_m: ${data[14]},
                                    Moment_S_node_2: ${data[15]},
                                    Moment_T_node_1: ${data[16]},
                                    Moment_T_m: ${data[17]},
                                    Moment_T_node_2: ${data[18]},
                                    `
                );
              }
            }
            selectedBeamElements.push(
              `Beam element: number ${number}, element forces: ${elementLoads}`
            );
          }
        }
      }
    }
    return [selectedBeamElementsNumbers, selectedBeamElements];
  }

  composeSelectedPlateElements(uids) {
    let selectedPlateElementsNumbers = Array();
    let selectedPlateElements = Array();
    if (this.state.activeJobName) {
      const job = this.state.store.jobs_shelf.get(this.state.activeJobName);
      if (job) {
        for (const [number, element] of Object.entries(
          job.mesh.plate_elements
        )) {
          if (uids.includes(element[0])) {
            selectedPlateElementsNumbers.push(number);
            let elementLoads = Array();
            for (const [key, data] of Object.entries(
              job.elements_analysis_result.plate_elements
            )) {
              if (key === number) {
                elementLoads.push(
                  `
                                    Membrane_Force_R: ${data[9]},
                                    Membrane_Force_S: ${data[10]},
                                    Membrane_Force_R_S: ${data[11]},
                                    Bending_Moment_R: ${data[12]},
                                    Bending_Moment_S: ${data[13]},
                                    Bending_Moment_R_S: ${data[14]},
                                    Shear_Force_R_T: ${data[15]},
                                    Shear_Force_S_T: ${data[16]},
                                    `
                );
              }
            }
            selectedPlateElements.push(
              `Plate element: number ${number}, element forces: ${elementLoads}`
            );
          }
        }
      }
    }
    return [selectedPlateElementsNumbers, selectedPlateElements];
  }

  async handleSelectedObjectsMessage(event) {
    const uids = event.detail.selected_objects;
    const [_selectedPointsNumbers, selectedPoints] =
      this.composeSelectedPoints(uids);
    if (selectedPoints.length !== 0) {
      this.shadowRoot.querySelector("fea-renderer").objectInfo =
        selectedPoints[selectedPoints.length - 1];
    }
    const [selectedLineNumbers, selectedLines] =
      this.composeSelectedLines(uids);
    if (selectedLines.length !== 0) {
      if (this.state.isLinesSelectionModeEnabled === false) {
        this.shadowRoot.querySelector("fea-renderer").objectInfo =
          selectedLines[selectedLines.length - 1];
      } else {
        this.selectLinesInClientForDataAssign(selectedLineNumbers);
      }
    }
    const [selectedSurfacesNumbers, selectedSurfaces] =
      this.composeSelectedSurfaces(uids);
    if (selectedSurfaces.length !== 0) {
      if (this.state.isSurfacesSelectionModeEnabled === false) {
        this.shadowRoot.querySelector("fea-renderer").objectInfo =
          selectedSurfaces[selectedSurfaces.length - 1];
      } else {
        this.selectSurfacesInClientForDataAssign(selectedSurfacesNumbers);
      }
    }
    const [_selectedPointNumbersWithConcLoads, selectedConcentratedLoads] =
      this.composeSelectedConcentratedLoads(uids);
    if (selectedConcentratedLoads.length !== 0) {
      this.shadowRoot.querySelector("fea-renderer").objectInfo =
        selectedConcentratedLoads[selectedConcentratedLoads.length - 1];
    }
    const [_selectedLineNumbers, selectedUniformlyDistributedLineLoads] =
      this.composeSelectedUniformlyDistributedLineLoads(uids);
    if (selectedUniformlyDistributedLineLoads.length !== 0) {
      this.shadowRoot.querySelector("fea-renderer").objectInfo =
        selectedUniformlyDistributedLineLoads[
          selectedUniformlyDistributedLineLoads.length - 1
        ];
    }
    const [_selectedSurfaceNumbers, selectedUniformlyDistributedSurfaceLoads] =
      this.composeSelectedUniformlyDistributedSurfaceLoads(uids);
    if (selectedUniformlyDistributedSurfaceLoads.length !== 0) {
      this.shadowRoot.querySelector("fea-renderer").objectInfo =
        selectedUniformlyDistributedSurfaceLoads[
          selectedUniformlyDistributedSurfaceLoads.length - 1
        ];
    }
    const [_selectedPointNumbersWithBC, selectedPointBoundaryConditions] =
      this.composeSelectedPointBoundaryConditions(uids);
    if (selectedPointBoundaryConditions.length !== 0) {
      this.shadowRoot.querySelector("fea-renderer").objectInfo =
        selectedPointBoundaryConditions[
          selectedPointBoundaryConditions.length - 1
        ];
    }
    const [_selectedNodesNumbers, selectedNodes] =
      this.composeSelectedNodes(uids);
    if (selectedNodes.length !== 0) {
      this.shadowRoot.querySelector("fea-renderer").objectInfo =
        selectedNodes[selectedNodes.length - 1];
    }
    const [_selectedTrussElementsNumbers, selectedTrussElements] =
      this.composeSelectedTrussElements(uids);
    if (selectedTrussElements.length !== 0) {
      this.shadowRoot.querySelector("fea-renderer").objectInfo =
        selectedTrussElements[selectedTrussElements.length - 1];
    }
    const [_selectedBeamElementsNumbers, selectedBeamElements] =
      this.composeSelectedBeamElements(uids);
    if (selectedBeamElements.length !== 0) {
      this.shadowRoot.querySelector("fea-renderer").objectInfo =
        selectedBeamElements[selectedBeamElements.length - 1];
    }
    const [_selectedPlateElementsNumbers, selectedPlateElements] =
      this.composeSelectedPlateElements(uids);
    if (selectedPlateElements.length !== 0) {
      this.shadowRoot.querySelector("fea-renderer").objectInfo =
        selectedPlateElements[selectedPlateElements.length - 1];
    }

    event.stopPropagation();
  }

  handleUpdatePointVisibilityMessage(event) {
    const isPointVisible = event.detail.is_point_visible;
    this.shadowRoot.querySelector("fea-renderer").updatePointVisibility =
      isPointVisible;
    event.stopPropagation();
  }

  handleUpdateLineVisibilityMessage(event) {
    const isLineVisible = event.detail.is_line_visible;
    this.shadowRoot.querySelector("fea-renderer").updateLineVisibility =
      isLineVisible;
    event.stopPropagation();
  }

  handleUpdateSurfaceVisibilityMessage(event) {
    const isSurfaceVisible = event.detail.is_surface_visible;
    this.shadowRoot.querySelector("fea-renderer").updateSurfaceVisibility =
      isSurfaceVisible;
    event.stopPropagation();
  }

  handleUpdateSurfaceEdges13VisibilityMessage(event) {
    const isSurfaceEdges13Visible = event.detail.is_surface_edges_1_3_visible;
    this.shadowRoot.querySelector(
      "fea-renderer"
    ).updateSurfaceEdges13Visibility = isSurfaceEdges13Visible;
    event.stopPropagation();
  }

  handleUpdateSurfaceEdges24VisibilityMessage(event) {
    const isSurfaceEdges24Visible = event.detail.is_surface_edges_2_4_visible;
    this.shadowRoot.querySelector(
      "fea-renderer"
    ).updateSurfaceEdges24Visibility = isSurfaceEdges24Visible;
    event.stopPropagation();
  }

  handleUpdateSurfaceNormalVisibilityMessage(event) {
    const isSurfaceNormalVisible = event.detail.is_surface_normal_visible;
    this.shadowRoot.querySelector(
      "fea-renderer"
    ).updateSurfaceNormalVisibility = isSurfaceNormalVisible;
    event.stopPropagation();
  }

  handleUpdateBeamSectionOrientationVisibilityMessage(event) {
    const isBeamSectionOrientationVisible =
      event.detail.is_beam_section_orientation_visible;
    this.shadowRoot.querySelector(
      "fea-renderer"
    ).updateBeamSectionOrientationVisibility = isBeamSectionOrientationVisible;
    event.stopPropagation();
  }

  handleUpdateLoadVisibilityMessage(event) {
    const isLoadVisible = event.detail.is_load_visible;
    this.shadowRoot.querySelector("fea-renderer").updateLoadVisibility =
      isLoadVisible;
    event.stopPropagation();
  }

  handleUpdateBoundaryConditionVisibilityMessage(event) {
    const isBoundaryConditionVisible =
      event.detail.is_boundary_condition_visible;
    this.shadowRoot.querySelector(
      "fea-renderer"
    ).updateBoundaryConditionVisibility = isBoundaryConditionVisible;
    event.stopPropagation();
  }

  handleUpdateMeshSeedVisibilityMessage(event) {
    const isMeshSeedVisible = event.detail.is_mesh_seed_visible;
    this.shadowRoot.querySelector("fea-renderer").updateMeshSeedVisibility =
      isMeshSeedVisible;
    event.stopPropagation();
  }

  handleUpdateNodeVisibilityMessage(event) {
    const isNodeVisible = event.detail.is_node_visible;
    this.shadowRoot.querySelector("fea-renderer").updateNodeVisibility =
      isNodeVisible;
    event.stopPropagation();
  }

  handleUpdateTrussElementVisibilityMessage(event) {
    const isTrussElementVisible = event.detail.is_truss_element_visible;
    this.shadowRoot.querySelector("fea-renderer").updateTrussElementVisibility =
      isTrussElementVisible;
    event.stopPropagation();
  }

  handleUpdateBeamElementVisibilityMessage(event) {
    const isBeamElementVisible = event.detail.is_beam_element_visible;
    this.shadowRoot.querySelector("fea-renderer").updateBeamElementVisibility =
      isBeamElementVisible;
    event.stopPropagation();
  }

  handleUpdatePlateElementVisibilityMessage(event) {
    const isPlateElementVisible = event.detail.is_plate_element_visible;
    this.shadowRoot.querySelector("fea-renderer").updatePlateElementVisibility =
      isPlateElementVisible;
    event.stopPropagation();
  }

  handleUpdateLocalAxesVisibilityMessage(event) {
    const isLocalAxesVisible = event.detail.is_local_axes_visible;
    this.shadowRoot.querySelector("fea-renderer").updateLocalAxesVisibility =
      isLocalAxesVisible;
    event.stopPropagation();
  }

  handleChangeViewMessage(event) {
    const selectedView = event.detail.selectedView;
    this.shadowRoot.querySelector("fea-renderer").selectedView = selectedView;
    event.stopPropagation();
  }

  handleAutoFitMessage(event) {
    this.shadowRoot.querySelector("fea-renderer").autoFit();
    event.stopPropagation();
  }

  handleSavePreprocessorDataMessage(event) {
    const fileName = event.detail.file_name;
    this.state.communicator.savePreprocessorData(fileName);
  }

  handleSavePostprocessorDataMessage(event) {
    const fileName = event.detail.file_name;
    const jobName = event.detail.job_name;
    const job = this.state.store.jobs_shelf.get(jobName);
    if (job) {
      this.state.communicator.savePostprocessorData(fileName, job);
    }
  }

  handleLoadPreprocessorDataMessage(event) {
    this.state.actionId = 1;
    this.state.store.reset();
    this.state.communicator.resetPreprocessor();
    this.shadowRoot.querySelector("fea-renderer").resetScene();
    const activeActions = event.detail.activeActions;

    const calls = async () => {
      for (let i = 0; i < activeActions.length - 1; i++) {
        const currentMessage = JSON.parse(activeActions[i]);
        await this.state.communicator.handleMessage(currentMessage);
      }
    };
    calls();
  }

  handleResetPreprocessorDataMessage() {
    this.state.actionId = 1;
    const delta = 0;
    this.actionIdSetter = delta;
    this.state.store.reset();
    this.state.communicator.resetPreprocessor();
    this.shadowRoot.querySelector("fea-renderer").resetScene();
  }

  handlePreviewSelectedLineNumbersMessage(event) {
    const selectedLineNumbers = event.detail.line_numbers;
    const lineUids = selectedLineNumbers.map((lineNumber) =>
      this.state.store.getLineUidByNumber(lineNumber)
    );
    try {
      this.shadowRoot.querySelector("fea-renderer").previewSelectedObjects =
        lineUids;
    } catch (error) {
      this.querySelector(event.target.tagName.toLowerCase()).rendererError =
        error;
      throw error;
    }
    event.stopPropagation();
  }

  handlePreviewSelectedSurfaceNumbersMessage(event) {
    const selectedSurfaceNumbers = event.detail.surface_numbers;
    const surfaceUids = selectedSurfaceNumbers.map((surfaceNumber) =>
      this.state.store.getSurfaceUidByNumber(surfaceNumber)
    );
    try {
      this.shadowRoot.querySelector("fea-renderer").previewSelectedObjects =
        surfaceUids;
    } catch (error) {
      this.querySelector(event.target.tagName.toLowerCase()).rendererError =
        error;
      throw error;
    }
    event.stopPropagation();
  }

  handleEnableLinesSelectionModeMessage(event) {
    this.state.isLinesSelectionModeEnabled = true;
    event.stopPropagation();
  }

  handleEnableSurfacesSelectionModeMessage(event) {
    this.state.isSurfacesSelectionModeEnabled = true;
    event.stopPropagation();
  }

  handleDisableLinesSelectionModeMessage(event) {
    this.state.isLinesSelectionModeEnabled = false;
    event.stopPropagation();
  }

  handleDisableSurfacesSelectionModeMessage(event) {
    this.state.isSurfacesSelectionModeEnabled = false;
    event.stopPropagation();
  }

  async handleClientMessage(event) {
    const message = event.detail.message;
    try {
      await this.state.communicator.handleMessage(message);
    } catch (error) {
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          error
        );
      }
      event.stopPropagation();
      throw error;
    }
    event.stopPropagation();
  }

  async handleSubmitJobMessage(event) {
    const jobName = event.detail.message.job_name;
    const solver = event.detail.message.solver;
    const maxIter = event.detail.message.max_iter;
    try {
      await this.state.communicator.submitJob(jobName, solver, maxIter);
    } catch (error) {
      this.querySelector(event.target.tagName.toLowerCase()).submitJobError =
        error;
      event.stopPropagation();
      throw error;
    }
    this.state.isProcessing = true;
    if (this.state.activeMenuName !== null) {
      this.querySelector(this.state.activeMenuName).setAttribute(
        "is-processing",
        this.state.isProcessing
      );
    }
    event.stopPropagation();
  }

  handleSolverMessage(event) {
    const header = event.detail.header;
    const jobName = event.detail.job_name;
    const status = event.detail.status;
    const message = event.detail.message;

    if (status === COMPLETED_STATUS) {
      this.shadowRoot.querySelector("fea-renderer").objectInfo = message;
    }

    if (status === ERROR_STATUS) {
      this.state.isProcessing = false;
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "is-processing",
          this.state.isProcessing
        );
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          message
        );
        this.shadowRoot.querySelector("fea-renderer").objectInfo = message;
        const job = this.state.store.jobs_shelf.get(jobName);
        if (job) {
          this.state.store.deleteJob(jobName);
        }
      }
      return;
    }

    if (header === CREATE_FEM_MESSAGE_HEADER) {
      const mesh = event.detail.mesh;
      const solver = event.detail.solver;
      const maxIter = event.detail.max_iter;
      this.state.store.addJob(jobName, mesh);
      this.state.communicator.performGlobalAnalysis(jobName, solver, maxIter);
    }

    if (header === PERFORM_GLOBAL_ANALYSIS_DIRECT_MESSAGE_HEADER) {
      this.state.communicator.extractGlobalAnalysisResult(jobName);
    }

    if (
      header === PERFORM_GLOBAL_ANALYSIS_ITERATIVE_PCG_JACOBI_MESSAGE_HEADER
    ) {
      const iterations = event.detail.iterations;
      console.log(`iterations: ${iterations}`);
      this.state.communicator.extractGlobalAnalysisResult(jobName);
    }

    if (
      header ===
      PERFORM_GLOBAL_ANALYSIS_ITERATIVE_PCG_BLOCK_JACOBI_MESSAGE_HEADER
    ) {
      const iterations = event.detail.iterations;
      console.log(`iterations: ${iterations}`);
      this.state.communicator.extractGlobalAnalysisResult(jobName);
    }

    if (
      header ===
      PERFORM_GLOBAL_ANALYSIS_ITERATIVE_PCG_BLOCK_JACOBI_GPU_MESSAGE_HEADER
    ) {
      const iterations = event.detail.iterations;
      console.log(`iterations: ${iterations}`);
      this.state.communicator.extractGlobalAnalysisResult(jobName);
    }

    if (header === EXTRACT_GLOBAL_ANALYSIS_RESULT_MESSAGE_HEADER) {
      const globalAnalysisResult = event.detail.global_analysis_result;
      const job = this.state.store.jobs_shelf.get(jobName);
      if (job) {
        job.global_analysis_result =
          globalAnalysisResult.global_analysis_result;
        job.extreme_global_loads = globalAnalysisResult.extreme_global_loads;
        job.extreme_global_displacements =
          globalAnalysisResult.extreme_global_displacements;
        this.state.communicator.extractElementsAnalysisResult(jobName);
      } else {
        this.state.isProcessing = false;
        if (this.state.activeMenuName !== null) {
          this.querySelector(this.state.activeMenuName).setAttribute(
            "is-processing",
            this.state.isProcessing
          );
          this.querySelector(this.state.activeMenuName).setAttribute(
            "error-message",
            `Job ${jobName} does not exist!`
          );
          this.shadowRoot.querySelector("fea-renderer").objectInfo = message;
        }
      }
    }

    if (header === EXTRACT_ELEMENTS_ANALYSIS_RESULT_MESSAGE_HEADER) {
      const elementsAnalysisResult = event.detail.elements_analysis_result;
      const job = this.state.store.jobs_shelf.get(jobName);
      if (job) {
        job.elements_analysis_result =
          elementsAnalysisResult.elements_analysis_result;
        job.extreme_elements_loads =
          elementsAnalysisResult.extreme_elements_loads;
        this.state.isProcessing = false;
        if (this.state.activeMenuName !== null) {
          this.querySelector(this.state.activeMenuName).setAttribute(
            "is-processing",
            this.state.isProcessing
          );
          const delta = 0;
          this.actionIdSetter = delta;
        }
      } else {
        this.state.isProcessing = false;
        if (this.state.activeMenuName !== null) {
          this.querySelector(this.state.activeMenuName).setAttribute(
            "is-processing",
            this.state.isProcessing
          );
          this.querySelector(this.state.activeMenuName).setAttribute(
            "error-message",
            `Job ${jobName} does not exist!`
          );
          this.shadowRoot.querySelector("fea-renderer").objectInfo = message;
        }
      }
    }
    event.stopPropagation();
  }

  handleShowJobAnalysisResultMessage(event) {
    const jobName = event.detail.message.job_name;
    const job = this.state.store.jobs_shelf.get(jobName);
    if (!job) {
      this.querySelector(this.state.activeMenuName).setAttribute(
        "error-message",
        `Job ${jobName} does not exist!`
      );
    } else {
      this.activatePostprocessorMenu(jobName, job);
    }
    event.stopPropagation();
  }

  handleDeleteJobMessage(event) {
    const jobName = event.detail.message.job_name;
    this.state.store.deleteJob(jobName);

    const delta = 0;
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handlePlotDisplacementsMessage(event) {
    const plotDisplacementsData = event.detail.message;
    try {
      this.shadowRoot.querySelector("fea-renderer").plotDisplacements =
        plotDisplacementsData;
    } catch (error) {
      this.querySelector(event.target.tagName.toLowerCase()).plotError = error;
      event.stopPropagation();
      throw error;
    }
    event.stopPropagation();
  }

  handlePlotGlobalForcesMessage(event) {
    const jobName = event.detail.message.job_name;
    try {
      this.shadowRoot.querySelector("fea-renderer").plotGlobalForces();
    } catch (error) {
      this.querySelector(event.target.tagName.toLowerCase()).plotError = error;
      event.stopPropagation();
      throw error;
    }
    event.stopPropagation();
  }

  handlePlotGlobalMomentsMessage(event) {
    const jobName = event.detail.message.job_name;
    try {
      this.shadowRoot.querySelector("fea-renderer").plotGlobalMoments();
    } catch (error) {
      this.querySelector(event.target.tagName.toLowerCase()).plotError = error;
      event.stopPropagation();
      throw error;
    }
    event.stopPropagation();
  }

  handleplotElementsLoadsMessage(event) {
    const selectedElementForceComponentData = event.detail.message;
    try {
      this.shadowRoot.querySelector("fea-renderer").plotElementsLoads =
        selectedElementForceComponentData;
    } catch (error) {
      this.querySelector(event.target.tagName.toLowerCase()).plotError = error;
      event.stopPropagation();
      throw error;
    }
    event.stopPropagation();
  }

  handleAddPointServerMessage(event) {
    const point = {
      number: event.detail.point_data.number,
      uid: event.detail.point_data.uid,
      x: event.detail.point_data.x,
      y: event.detail.point_data.y,
      z: event.detail.point_data.z,
    };
    this.state.store.addPoint(point);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    try {
      this.shadowRoot.querySelector("fea-renderer").addPointToRenderer = point;
    } catch (error) {
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          error
        );
      }
      throw error;
    }
    event.stopPropagation();
  }

  handleUpdatePointServerMessage(event) {
    const point = {
      number: event.detail.point_data.number,
      uid: event.detail.point_data.uid,
      x: event.detail.point_data.x,
      y: event.detail.point_data.y,
      z: event.detail.point_data.z,
    };
    this.state.store.updatePoint(point);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    try {
      this.shadowRoot.querySelector("fea-renderer").updatePointInRenderer =
        point;
    } catch (error) {
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          error
        );
      }
      throw error;
    }
    event.stopPropagation();
  }

  handleDeletePointServerMessage(event) {
    const point = {
      number: event.detail.point_data.number,
      uid: event.detail.point_data.uid,
    };
    this.state.store.deletePoint(point);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    try {
      this.shadowRoot.querySelector("fea-renderer").deletePointFromRenderer =
        point;
    } catch (error) {
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          error
        );
      }
      throw error;
    }
    event.stopPropagation();
  }

  handleAddLineServerMessage(event) {
    const line = {
      number: event.detail.line_data.number,
      uid: event.detail.line_data.uid,
      point_1_number: event.detail.line_data.point_1_number,
      point_2_number: event.detail.line_data.point_2_number,
      optional_property: event.detail.line_data.optional_property,
      optional_local_axis_1_direction:
        event.detail.line_data.optional_local_axis_1_direction,
      optional_transformed_local_axis_1_direction:
        event.detail.line_data.optional_transformed_local_axis_1_direction,
      optional_mesh_seed: event.detail.line_data.optional_mesh_seed,
      optional_uniformly_distributed_line_load:
        event.detail.line_data.optional_uniformly_distributed_line_load,
    };
    this.state.store.addLine(line);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    try {
      this.shadowRoot.querySelector("fea-renderer").addLineToRenderer = line;
    } catch (error) {
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          error
        );
      }
      throw error;
    }
    event.stopPropagation();
  }

  handleUpdateLineServerMessage(event) {
    const line = {
      number: event.detail.line_data.number,
      uid: event.detail.line_data.uid,
      point_1_number: event.detail.line_data.point_1_number,
      point_2_number: event.detail.line_data.point_2_number,
      optional_property: event.detail.line_data.optional_property,
      optional_local_axis_1_direction:
        event.detail.line_data.optional_local_axis_1_direction,
      optional_transformed_local_axis_1_direction:
        event.detail.line_data.optional_transformed_local_axis_1_direction,
      optional_mesh_seed: event.detail.line_data.optional_mesh_seed,
      optional_uniformly_distributed_line_load:
        event.detail.line_data.optional_uniformly_distributed_line_load,
    };
    this.state.store.updateLine(line);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    try {
      this.shadowRoot.querySelector("fea-renderer").updateLineInRenderer = line;
    } catch (error) {
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          error
        );
      }
      throw error;
    }
    event.stopPropagation();
  }

  handleDeleteLineServerMessage(event) {
    const line = {
      number: event.detail.line_data.number,
      uid: event.detail.line_data.uid,
    };
    this.state.store.deleteLine(line);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    try {
      this.shadowRoot.querySelector("fea-renderer").deleteLineFromRenderer =
        line;
    } catch (error) {
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          error
        );
      }
      throw error;
    }
    event.stopPropagation();
  }

  handleAddSurfaceServerMessage(event) {
    const surface = {
      number: event.detail.surface_data.number,
      uid: event.detail.surface_data.uid,
      point_1_number: event.detail.surface_data.point_1_number,
      point_2_number: event.detail.surface_data.point_2_number,
      point_3_number: event.detail.surface_data.point_3_number,
      point_4_number: event.detail.surface_data.point_4_number,
      normal: event.detail.surface_data.normal,
      optional_property: event.detail.surface_data.optional_property,
      optional_mesh_seed: event.detail.surface_data.optional_mesh_seed,
      optional_uniformly_distributed_surface_load:
        event.detail.surface_data.optional_uniformly_distributed_surface_load,
    };
    this.state.store.addSurface(surface);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    try {
      this.shadowRoot.querySelector("fea-renderer").addSurfaceToRenderer =
        surface;
    } catch (error) {
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          error
        );
      }
      throw error;
    }
    event.stopPropagation();
  }

  handleUpdateSurfaceServerMessage(event) {
    const surface = {
      number: event.detail.surface_data.number,
      uid: event.detail.surface_data.uid,
      point_1_number: event.detail.surface_data.point_1_number,
      point_2_number: event.detail.surface_data.point_2_number,
      point_3_number: event.detail.surface_data.point_3_number,
      point_4_number: event.detail.surface_data.point_4_number,
      normal: event.detail.surface_data.normal,
      optional_property: event.detail.surface_data.optional_property,
      optional_mesh_seed: event.detail.surface_data.optional_mesh_seed,
      optional_uniformly_distributed_surface_load:
        event.detail.surface_data.optional_uniformly_distributed_surface_load,
    };
    this.state.store.updateSurface(surface);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    try {
      this.shadowRoot.querySelector("fea-renderer").updateSurfaceInRenderer =
        surface;
    } catch (error) {
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          error
        );
      }
      throw error;
    }
    event.stopPropagation();
  }

  handleDeleteSurfaceServerMessage(event) {
    const surface = {
      number: event.detail.surface_data.number,
      uid: event.detail.surface_data.uid,
    };
    this.state.store.deleteSurface(surface);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    try {
      this.shadowRoot.querySelector("fea-renderer").deleteSurfaceFromRenderer =
        surface;
    } catch (error) {
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          error
        );
      }
      throw error;
    }
    event.stopPropagation();
  }

  handleAddMaterialServerMessage(event) {
    const material = {
      name: event.detail.material_data.name,
      young_modulus: event.detail.material_data.young_modulus,
      poisson_ratio: event.detail.material_data.poisson_ratio,
    };
    this.state.store.addMaterial(material);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleUpdateMaterialServerMessage(event) {
    const material = {
      name: event.detail.material_data.name,
      young_modulus: event.detail.material_data.young_modulus,
      poisson_ratio: event.detail.material_data.poisson_ratio,
    };
    this.state.store.updateMaterial(material);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleDeleteMaterialServerMessage(event) {
    const material = { number: event.detail.material_data.name };
    this.state.store.deleteMaterial(material);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleAddTrussSectionServerMessage(event) {
    const trussSection = {
      name: event.detail.truss_section_data.name,
      area: event.detail.truss_section_data.area,
      area2: event.detail.truss_section_data.area2,
    };
    this.state.store.addTrussSection(trussSection);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleUpdateTrussSectionServerMessage(event) {
    const trussSection = {
      name: event.detail.truss_section_data.name,
      area: event.detail.truss_section_data.area,
      area2: event.detail.truss_section_data.area2,
    };
    this.state.store.updateTrussSection(trussSection);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleDeleteTrussSectionServerMessage(event) {
    const trussSection = { name: event.detail.truss_section_data.name };
    this.state.store.deleteTrussSection(trussSection);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleAddBeamSectionServerMessage(event) {
    const beamSection = {
      name: event.detail.beam_section_data.name,
      area: event.detail.beam_section_data.area,
      i11: event.detail.beam_section_data.i11,
      i22: event.detail.beam_section_data.i22,
      i12: event.detail.beam_section_data.i12,
      it: event.detail.beam_section_data.it,
      shear_factor: event.detail.beam_section_data.shear_factor,
    };
    this.state.store.addBeamSection(beamSection);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleUpdateBeamSectionServerMessage(event) {
    const beamSection = {
      name: event.detail.beam_section_data.name,
      area: event.detail.beam_section_data.area,
      i11: event.detail.beam_section_data.i11,
      i22: event.detail.beam_section_data.i22,
      i12: event.detail.beam_section_data.i12,
      it: event.detail.beam_section_data.it,
      shear_factor: event.detail.beam_section_data.shear_factor,
    };
    this.state.store.updateBeamSection(beamSection);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleDeleteBeamSectionServerMessage(event) {
    const beamSection = { name: event.detail.beam_section_data.name };
    this.state.store.deleteBeamSection(beamSection);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleAddPlateSectionServerMessage(event) {
    const plateSection = {
      name: event.detail.plate_section_data.name,
      thickness: event.detail.plate_section_data.thickness,
      shear_factor: event.detail.plate_section_data.shear_factor,
    };
    this.state.store.addPlateSection(plateSection);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleUpdatePlateSectionServerMessage(event) {
    const plateSection = {
      name: event.detail.plate_section_data.name,
      thickness: event.detail.plate_section_data.thickness,
      shear_factor: event.detail.plate_section_data.shear_factor,
    };
    this.state.store.updatePlateSection(plateSection);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleDeletePlateSectionServerMessage(event) {
    const plateSection = { name: event.detail.plate_section_data.name };
    this.state.store.deletePlateSection(plateSection);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleAddPropertiesServerMessage(event) {
    const properties = {
      name: event.detail.properties_data.name,
      material_name: event.detail.properties_data.material_name,
      cross_section_name: event.detail.properties_data.cross_section_name,
      cross_section_type: event.detail.properties_data.cross_section_type,
    };
    this.state.store.addProperties(properties);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleUpdatePropertiesServerMessage(event) {
    const properties = {
      name: event.detail.properties_data.name,
      material_name: event.detail.properties_data.material_name,
      cross_section_name: event.detail.properties_data.cross_section_name,
      cross_section_type: event.detail.properties_data.cross_section_type,
    };
    this.state.store.updateProperties(properties);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleDeletePropertiesServerMessage(event) {
    const properties = { name: event.detail.properties_data.name };
    this.state.store.deleteProperties(properties);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleAddBeamSectionLocalAxis1DirectionServerMessage(event) {
    const beamSectionLocalAxis1DirectionData =
      event.detail.local_axis_1_direction_data.local_axis_1_direction;
    this.state.store.addBeamSectionLocalAxis1Direction(
      beamSectionLocalAxis1DirectionData
    );

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleDeleteBeamSectionLocalAxis1DirectionServerMessage(event) {
    const beamSectionLocalAxis1DirectionData =
      event.detail.local_axis_1_direction_data.local_axis_1_direction;
    this.state.store.deleteBeamSectionLocalAxis1Direction(
      beamSectionLocalAxis1DirectionData
    );

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    event.stopPropagation();
  }

  handleAddConcentratedLoadServerMessage(event) {
    const concentratedLoad = {
      point_number: event.detail.concentrated_load_data.point_number,
      uid: event.detail.concentrated_load_data.uid,
      fx: event.detail.concentrated_load_data.fx,
      fy: event.detail.concentrated_load_data.fy,
      fz: event.detail.concentrated_load_data.fz,
      mx: event.detail.concentrated_load_data.mx,
      my: event.detail.concentrated_load_data.my,
      mz: event.detail.concentrated_load_data.mz,
    };
    this.state.store.addConcentratedLoad(concentratedLoad);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    try {
      this.shadowRoot.querySelector(
        "fea-renderer"
      ).addConcentratedLoadToRenderer = concentratedLoad;
    } catch (error) {
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          error
        );
      }
      throw error;
    }
    event.stopPropagation();
  }

  handleUpdateConcentratedLoadServerMessage(event) {
    const concentratedLoad = {
      point_number: event.detail.concentrated_load_data.point_number,
      uid: event.detail.concentrated_load_data.uid,
      fx: event.detail.concentrated_load_data.fx,
      fy: event.detail.concentrated_load_data.fy,
      fz: event.detail.concentrated_load_data.fz,
      mx: event.detail.concentrated_load_data.mx,
      my: event.detail.concentrated_load_data.my,
      mz: event.detail.concentrated_load_data.mz,
    };
    this.state.store.updateConcentratedLoad(concentratedLoad);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    try {
      this.shadowRoot.querySelector(
        "fea-renderer"
      ).updateConcentratedLoadInRenderer = concentratedLoad;
    } catch (error) {
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          error
        );
      }
      throw error;
    }
    event.stopPropagation();
  }

  handleDeleteConcentratedLoadServerMessage(event) {
    const concentratedLoad = {
      point_number: event.detail.concentrated_load_data.point_number,
      uid: event.detail.concentrated_load_data.uid,
    };
    this.state.store.deleteConcentratedLoad(concentratedLoad);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    try {
      this.shadowRoot.querySelector(
        "fea-renderer"
      ).deleteConcentratedLoadFromRenderer = concentratedLoad;
    } catch (error) {
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          error
        );
      }
      throw error;
    }
    event.stopPropagation();
  }

  handleAddPointBoundaryConditionServerMessage(event) {
    const pointBoundaryCondition = {
      point_number: event.detail.point_boundary_condition_data.point_number,
      uid: event.detail.point_boundary_condition_data.uid,
      optional_ux: event.detail.point_boundary_condition_data.optional_ux,
      optional_uy: event.detail.point_boundary_condition_data.optional_uy,
      optional_uz: event.detail.point_boundary_condition_data.optional_uz,
      optional_rx: event.detail.point_boundary_condition_data.optional_rx,
      optional_ry: event.detail.point_boundary_condition_data.optional_ry,
      optional_rz: event.detail.point_boundary_condition_data.optional_rz,
    };
    this.state.store.addPointBoundaryCondition(pointBoundaryCondition);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    try {
      this.shadowRoot.querySelector(
        "fea-renderer"
      ).addPointBoundaryConditionToRenderer = pointBoundaryCondition;
    } catch (error) {
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          error
        );
      }
      throw error;
    }
    event.stopPropagation();
  }

  handleUpdatePointBoundaryConditionServerMessage(event) {
    const pointBoundaryCondition = {
      point_number: event.detail.point_boundary_condition_data.point_number,
      uid: event.detail.point_boundary_condition_data.uid,
      optional_ux: event.detail.point_boundary_condition_data.optional_ux,
      optional_uy: event.detail.point_boundary_condition_data.optional_uy,
      optional_uz: event.detail.point_boundary_condition_data.optional_uz,
      optional_rx: event.detail.point_boundary_condition_data.optional_rx,
      optional_ry: event.detail.point_boundary_condition_data.optional_ry,
      optional_rz: event.detail.point_boundary_condition_data.optional_rz,
    };
    this.state.store.updatePointBoundaryCondition(pointBoundaryCondition);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    try {
      this.shadowRoot.querySelector(
        "fea-renderer"
      ).updatePointBoundaryConditionInRenderer = pointBoundaryCondition;
    } catch (error) {
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          error
        );
      }
      throw error;
    }
    event.stopPropagation();
  }

  handleDeletePointBoundaryConditionServerMessage(event) {
    const pointBoundaryCondition = {
      point_number: event.detail.point_boundary_condition_data.point_number,
      uid: event.detail.point_boundary_condition_data.uid,
    };
    this.state.store.deletePointBoundaryCondition(pointBoundaryCondition);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;

    try {
      this.shadowRoot.querySelector(
        "fea-renderer"
      ).deletePointBoundaryConditionFromRenderer = pointBoundaryCondition;
    } catch (error) {
      if (this.state.activeMenuName !== null) {
        this.querySelector(this.state.activeMenuName).setAttribute(
          "error-message",
          error
        );
      }
      throw error;
    }
    event.stopPropagation();
  }

  handleUpdateGlobalMeshSeedServerMessage(event) {
    const globalMeshSeedValue =
      event.detail.global_mesh_seed_data.global_mesh_seed_value;
    this.state.store.updateGlobalMeshSeedValue(globalMeshSeedValue);

    let delta;
    if (event.detail.is_action_id_should_be_increased === true) {
      delta = 1;
    } else {
      delta = 0;
    }
    this.actionIdSetter = delta;
    event.stopPropagation();
  }

  delay(t, v) {
    return new Promise(function (resolve) {
      setTimeout(resolve.bind(null, v), t);
    });
  }

  handleIncreaseActionIdMessage() {
    const delta = 1;
    this.actionIdSetter = delta;
  }

  handleDecreaseActionIdMessage() {
    const delta = -1;
    this.actionIdSetter = delta;
  }

  updateCanvasSize() {
    if (this.querySelector("fea-postprocessor-menu") !== null) {
      const canvasWidth =
        window.innerWidth -
        this.querySelector("fea-postprocessor-menu").offsetWidth -
        15;
      const canvasHeight =
        window.innerHeight -
        this.shadowRoot.querySelector("custom-app-menu-bar").offsetHeight -
        this.shadowRoot.querySelector("fea-app-tool-bar").offsetHeight -
        65;
      this.shadowRoot.querySelector("fea-renderer").canvasSize = {
        width: canvasWidth,
        height: canvasHeight,
      };
      this.querySelector("fea-postprocessor-menu").setAttribute(
        "menu-height",
        canvasHeight + 49
      );
    } else if (this.querySelector("fea-preprocessor-menu") !== null) {
      const canvasWidth =
        window.innerWidth -
        this.querySelector("fea-preprocessor-menu").offsetWidth -
        15;
      const canvasHeight =
        window.innerHeight -
        this.shadowRoot.querySelector("custom-app-menu-bar").offsetHeight -
        this.shadowRoot.querySelector("fea-app-tool-bar").offsetHeight -
        65;
      this.shadowRoot.querySelector("fea-renderer").canvasSize = {
        width: canvasWidth,
        height: canvasHeight,
      };
      this.querySelector("fea-preprocessor-menu").setAttribute(
        "menu-height",
        canvasHeight + 49
      );
    } else {
      this.shadowRoot.querySelector("fea-renderer").canvasSize = {
        width: window.innerWidth,
        height: window.innerHeight,
      };
    }
  }
}

export default FeaApp;
