import { initializeActionsRouter } from "./wasm_modules_initialization/actions_router_initialization.js";
import {
  ADD_POINT_MESSAGE_HEADER,
  UPDATE_POINT_MESSAGE_HEADER,
  DELETE_POINT_MESSAGE_HEADER,
  RESTORE_POINT_MESSAGE_HEADER,
  ADD_LINE_MESSAGE_HEADER,
  UPDATE_LINE_MESSAGE_HEADER,
  DELETE_LINE_MESSAGE_HEADER,
  RESTORE_LINE_MESSAGE_HEADER,
  ADD_SURFACE_MESSAGE_HEADER,
  UPDATE_SURFACE_MESSAGE_HEADER,
  ROTATE_SURFACE_VERTICES_CLOCKWISE_MESSAGE_HEADER,
  ROTATE_SURFACE_VERTICES_COUNTER_CLOCKWISE_MESSAGE_HEADER,
  FLIP_SURFACE_NORMAL_AXIS_MESSAGE_HEADER,
  DELETE_SURFACE_MESSAGE_HEADER,
  RESTORE_SURFACE_MESSAGE_HEADER,
  ADD_MATERIAL_MESSAGE_HEADER,
  UPDATE_MATERIAL_MESSAGE_HEADER,
  DELETE_MATERIAL_MESSAGE_HEADER,
  RESTORE_MATERIAL_MESSAGE_HEADER,
  ADD_TRUSS_SECTION_MESSAGE_HEADER,
  UPDATE_TRUSS_SECTION_MESSAGE_HEADER,
  DELETE_TRUSS_SECTION_MESSAGE_HEADER,
  RESTORE_TRUSS_SECTION_MESSAGE_HEADER,
  ADD_BEAM_SECTION_MESSAGE_HEADER,
  UPDATE_BEAM_SECTION_MESSAGE_HEADER,
  DELETE_BEAM_SECTION_MESSAGE_HEADER,
  RESTORE_BEAM_SECTION_MESSAGE_HEADER,
  ADD_PLATE_SECTION_MESSAGE_HEADER,
  UPDATE_PLATE_SECTION_MESSAGE_HEADER,
  DELETE_PLATE_SECTION_MESSAGE_HEADER,
  RESTORE_PLATE_SECTION_MESSAGE_HEADER,
  ADD_PROPERTIES_MESSAGE_HEADER,
  UPDATE_PROPERTIES_MESSAGE_HEADER,
  DELETE_PROPERTIES_MESSAGE_HEADER,
  RESTORE_PROPERTIES_MESSAGE_HEADER,
  ASSIGN_PROPERTIES_TO_LINES_MESSAGE_HEADER,
  ADD_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER,
  DELETE_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER,
  ASSIGN_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER,
  RESTORE_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER,
  ASSIGN_PROPERTIES_TO_SURFACES_MESSAGE_HEADER,
  ADD_CONCENTRATED_LOAD_MESSAGE_HEADER,
  UPDATE_CONCENTRATED_LOAD_MESSAGE_HEADER,
  DELETE_CONCENTRATED_LOAD_MESSAGE_HEADER,
  RESTORE_CONCENTRATED_LOAD_MESSAGE_HEADER,
  ADD_UNIFORMLY_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER,
  UPDATE_UNIFORMLY_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER,
  DELETE_UNIFORMLY_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER,
  RESTORE_UNIFORMLY_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER,
  ADD_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_MESSAGE_HEADER,
  UPDATE_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_MESSAGE_HEADER,
  DELETE_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_MESSAGE_HEADER,
  RESTORE_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_MESSAGE_HEADER,
  ADD_POINT_BOUNDARY_CONDITION_MESSAGE_HEADER,
  UPDATE_POINT_BOUNDARY_CONDITION_MESSAGE_HEADER,
  DELETE_POINT_BOUNDARY_CONDITION_MESSAGE_HEADER,
  RESTORE_POINT_BOUNDARY_CONDITION_MESSAGE_HEADER,
  UPDATE_GLOBAL_MESH_SEED_MESSAGE_HEADER,
  UPDATE_LINES_MESH_SEED_MESSAGE_HEADER,
  UNDO_LINES_MESH_SEED_UPDATE_MESSAGE_HEADER,
  UPDATE_SURFACES_MESH_SEED_MESSAGE_HEADER,
  UNDO_SURFACES_MESH_SEED_UPDATE_MESSAGE_HEADER,
} from "./consts/actions_router_consts.js";
import { EVENT_TARGET } from "./consts/common_consts.js";
import {
  DATA_FOR_MESHER_EXTRACTED_MESSAGE_HEADER,
  MESH_FOR_SOLVER_EXTRACTED_MESSAGE_HEADER,
  COMPLETED_STATUS,
  ERROR_STATUS,
  SOLVER_MESSAGE_EVENT_HEADER,
} from "./consts/solver_consts.js";

import { WASM_LOADED_MESSAGE_HEADER } from "./consts/fea_app_consts.js";

import { initializePreprocessor } from "./wasm_modules_initialization/preprocessor_initialization.js";
import { initializeMesher } from "./wasm_modules_initialization/mesher_initialization.js";

import {
  createFEM,
  performGlobalAnalysis,
  extractGlobalAnalysisResult,
  extractElementsAnalysisResult,
} from "./workers/solver_worker.js";

export class Communicator {
  constructor() {
    this.state = {
      actionsRouter: null, // wasm_module "actions_router"
      preprocessor: null, // wasm_module "preprocessor"
      mesher: null, // wasm_module "mesher"
    };
  }

  async initWasmModules() {
    this.state.actionsRouter = await initializeActionsRouter();
    this.state.preprocessor = await initializePreprocessor();
    this.state.mesher = await initializeMesher();

    document.querySelector(EVENT_TARGET).dispatchEvent(
      new CustomEvent(WASM_LOADED_MESSAGE_HEADER, {
        bubbles: true,
        composed: true,
      })
    );
  }

  async handleMessage(message) {
    try {
      const messageHandlingResult =
        this.state.actionsRouter.handle_message(message);
      const actionId = messageHandlingResult["action_id"];
      const isActionIdShouldBeIncreased =
        messageHandlingResult["is_action_id_should_be_increased"];
      const uid = messageHandlingResult["uid"];
      if (messageHandlingResult.hasOwnProperty(ADD_POINT_MESSAGE_HEADER)) {
        const pointData = messageHandlingResult[ADD_POINT_MESSAGE_HEADER];
        try {
          this.state.preprocessor.add_point(
            actionId,
            pointData.number,
            pointData.x,
            pointData.y,
            pointData.z,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (messageHandlingResult.hasOwnProperty(UPDATE_POINT_MESSAGE_HEADER)) {
        const pointData = messageHandlingResult[UPDATE_POINT_MESSAGE_HEADER];
        try {
          this.state.preprocessor.update_point(
            actionId,
            pointData.number,
            pointData.x,
            pointData.y,
            pointData.z,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (messageHandlingResult.hasOwnProperty(DELETE_POINT_MESSAGE_HEADER)) {
        const pointData = messageHandlingResult[DELETE_POINT_MESSAGE_HEADER];
        try {
          this.state.preprocessor.delete_point(
            actionId,
            pointData.number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (messageHandlingResult.hasOwnProperty(RESTORE_POINT_MESSAGE_HEADER)) {
        const pointData = messageHandlingResult[RESTORE_POINT_MESSAGE_HEADER];
        try {
          this.state.preprocessor.restore_point(
            actionId,
            pointData.number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (messageHandlingResult.hasOwnProperty(ADD_LINE_MESSAGE_HEADER)) {
        const lineData = messageHandlingResult[ADD_LINE_MESSAGE_HEADER];
        try {
          this.state.preprocessor.add_line(
            actionId,
            lineData.number,
            lineData.point_1_number,
            lineData.point_2_number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (messageHandlingResult.hasOwnProperty(UPDATE_LINE_MESSAGE_HEADER)) {
        const lineData = messageHandlingResult[UPDATE_LINE_MESSAGE_HEADER];
        try {
          this.state.preprocessor.update_line(
            actionId,
            lineData.number,
            lineData.point_1_number,
            lineData.point_2_number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (messageHandlingResult.hasOwnProperty(DELETE_LINE_MESSAGE_HEADER)) {
        const lineData = messageHandlingResult[DELETE_LINE_MESSAGE_HEADER];
        try {
          this.state.preprocessor.delete_line(
            actionId,
            lineData.number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (messageHandlingResult.hasOwnProperty(RESTORE_LINE_MESSAGE_HEADER)) {
        const lineData = messageHandlingResult[RESTORE_LINE_MESSAGE_HEADER];
        try {
          this.state.preprocessor.restore_line(
            actionId,
            lineData.number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (messageHandlingResult.hasOwnProperty(ADD_SURFACE_MESSAGE_HEADER)) {
        const surfaceData = messageHandlingResult[ADD_SURFACE_MESSAGE_HEADER];
        try {
          this.state.preprocessor.add_surface(
            actionId,
            surfaceData.number,
            surfaceData.point_1_number,
            surfaceData.point_2_number,
            surfaceData.point_3_number,
            surfaceData.point_4_number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (messageHandlingResult.hasOwnProperty(UPDATE_SURFACE_MESSAGE_HEADER)) {
        const surfaceData =
          messageHandlingResult[UPDATE_SURFACE_MESSAGE_HEADER];
        try {
          this.state.preprocessor.update_surface(
            actionId,
            surfaceData.number,
            surfaceData.point_1_number,
            surfaceData.point_2_number,
            surfaceData.point_3_number,
            surfaceData.point_4_number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          ROTATE_SURFACE_VERTICES_CLOCKWISE_MESSAGE_HEADER
        )
      ) {
        const surfaceData =
          messageHandlingResult[
            ROTATE_SURFACE_VERTICES_CLOCKWISE_MESSAGE_HEADER
          ];
        try {
          this.state.preprocessor.rotate_surface_vertices_clockwise(
            actionId,
            surfaceData.number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          ROTATE_SURFACE_VERTICES_COUNTER_CLOCKWISE_MESSAGE_HEADER
        )
      ) {
        const surfaceData =
          messageHandlingResult[
            ROTATE_SURFACE_VERTICES_COUNTER_CLOCKWISE_MESSAGE_HEADER
          ];
        try {
          this.state.preprocessor.rotate_surface_vertices_counter_clockwise(
            actionId,
            surfaceData.number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          FLIP_SURFACE_NORMAL_AXIS_MESSAGE_HEADER
        )
      ) {
        const surfaceData =
          messageHandlingResult[FLIP_SURFACE_NORMAL_AXIS_MESSAGE_HEADER];
        try {
          this.state.preprocessor.flip_surface_normal_axis(
            actionId,
            surfaceData.number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (messageHandlingResult.hasOwnProperty(DELETE_SURFACE_MESSAGE_HEADER)) {
        const surfaceData =
          messageHandlingResult[DELETE_SURFACE_MESSAGE_HEADER];
        try {
          this.state.preprocessor.delete_surface(
            actionId,
            surfaceData.number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(RESTORE_SURFACE_MESSAGE_HEADER)
      ) {
        const surfaceData =
          messageHandlingResult[RESTORE_SURFACE_MESSAGE_HEADER];
        try {
          this.state.preprocessor.restore_surface(
            actionId,
            surfaceData.number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (messageHandlingResult.hasOwnProperty(ADD_MATERIAL_MESSAGE_HEADER)) {
        const materialData = messageHandlingResult[ADD_MATERIAL_MESSAGE_HEADER];
        try {
          this.state.preprocessor.add_material(
            actionId,
            materialData.name,
            materialData.young_modulus,
            materialData.poisson_ratio,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(UPDATE_MATERIAL_MESSAGE_HEADER)
      ) {
        const materialData =
          messageHandlingResult[UPDATE_MATERIAL_MESSAGE_HEADER];
        try {
          this.state.preprocessor.update_material(
            actionId,
            materialData.name,
            materialData.young_modulus,
            materialData.poisson_ratio,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(DELETE_MATERIAL_MESSAGE_HEADER)
      ) {
        const materialData =
          messageHandlingResult[DELETE_MATERIAL_MESSAGE_HEADER];
        try {
          this.state.preprocessor.delete_material(
            actionId,
            materialData.name,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(RESTORE_MATERIAL_MESSAGE_HEADER)
      ) {
        const materialData =
          messageHandlingResult[RESTORE_MATERIAL_MESSAGE_HEADER];
        try {
          this.state.preprocessor.restore_material(
            actionId,
            materialData.name,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(ADD_TRUSS_SECTION_MESSAGE_HEADER)
      ) {
        const trussSectionData =
          messageHandlingResult[ADD_TRUSS_SECTION_MESSAGE_HEADER];
        try {
          this.state.preprocessor.add_truss_section(
            actionId,
            trussSectionData.name,
            trussSectionData.area,
            trussSectionData.area2,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          UPDATE_TRUSS_SECTION_MESSAGE_HEADER
        )
      ) {
        const trussSectionData =
          messageHandlingResult[UPDATE_TRUSS_SECTION_MESSAGE_HEADER];
        try {
          this.state.preprocessor.update_truss_section(
            actionId,
            trussSectionData.name,
            trussSectionData.area,
            trussSectionData.area2,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          DELETE_TRUSS_SECTION_MESSAGE_HEADER
        )
      ) {
        const trussSectionData =
          messageHandlingResult[DELETE_TRUSS_SECTION_MESSAGE_HEADER];
        try {
          this.state.preprocessor.delete_truss_section(
            actionId,
            trussSectionData.name,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          RESTORE_TRUSS_SECTION_MESSAGE_HEADER
        )
      ) {
        const trussSectionData =
          messageHandlingResult[RESTORE_TRUSS_SECTION_MESSAGE_HEADER];
        try {
          this.state.preprocessor.restore_truss_section(
            actionId,
            trussSectionData.name,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(ADD_BEAM_SECTION_MESSAGE_HEADER)
      ) {
        const beamSectionData =
          messageHandlingResult[ADD_BEAM_SECTION_MESSAGE_HEADER];
        try {
          this.state.preprocessor.add_beam_section(
            actionId,
            beamSectionData.name,
            beamSectionData.area,
            beamSectionData.i11,
            beamSectionData.i22,
            beamSectionData.i12,
            beamSectionData.it,
            beamSectionData.shear_factor,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(UPDATE_BEAM_SECTION_MESSAGE_HEADER)
      ) {
        const beamSectionData =
          messageHandlingResult[UPDATE_BEAM_SECTION_MESSAGE_HEADER];
        try {
          this.state.preprocessor.update_beam_section(
            actionId,
            beamSectionData.name,
            beamSectionData.area,
            beamSectionData.i11,
            beamSectionData.i22,
            beamSectionData.i12,
            beamSectionData.it,
            beamSectionData.shear_factor,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(DELETE_BEAM_SECTION_MESSAGE_HEADER)
      ) {
        const beamSectionData =
          messageHandlingResult[DELETE_BEAM_SECTION_MESSAGE_HEADER];
        try {
          this.state.preprocessor.delete_beam_section(
            actionId,
            beamSectionData.name,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          RESTORE_BEAM_SECTION_MESSAGE_HEADER
        )
      ) {
        const beamSectionData =
          messageHandlingResult[RESTORE_BEAM_SECTION_MESSAGE_HEADER];
        try {
          this.state.preprocessor.restore_beam_section(
            actionId,
            beamSectionData.name,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(ADD_PLATE_SECTION_MESSAGE_HEADER)
      ) {
        const plateSectionData =
          messageHandlingResult[ADD_PLATE_SECTION_MESSAGE_HEADER];
        console.log(plateSectionData);
        try {
          this.state.preprocessor.add_plate_section(
            actionId,
            plateSectionData.name,
            plateSectionData.thickness,
            plateSectionData.shear_factor,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          UPDATE_PLATE_SECTION_MESSAGE_HEADER
        )
      ) {
        const plateSectionData =
          messageHandlingResult[UPDATE_PLATE_SECTION_MESSAGE_HEADER];
        try {
          this.state.preprocessor.update_plate_section(
            actionId,
            plateSectionData.name,
            plateSectionData.thickness,
            plateSectionData.shear_factor,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          DELETE_PLATE_SECTION_MESSAGE_HEADER
        )
      ) {
        const plateSectionData =
          messageHandlingResult[DELETE_PLATE_SECTION_MESSAGE_HEADER];
        try {
          this.state.preprocessor.delete_plate_section(
            actionId,
            plateSectionData.name,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          RESTORE_PLATE_SECTION_MESSAGE_HEADER
        )
      ) {
        const plateSectionData =
          messageHandlingResult[RESTORE_PLATE_SECTION_MESSAGE_HEADER];
        try {
          this.state.preprocessor.restore_plate_section(
            actionId,
            plateSectionData.name,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (messageHandlingResult.hasOwnProperty(ADD_PROPERTIES_MESSAGE_HEADER)) {
        const propertiesData =
          messageHandlingResult[ADD_PROPERTIES_MESSAGE_HEADER];
        try {
          this.state.preprocessor.add_properties(
            actionId,
            propertiesData.name,
            propertiesData.material_name,
            propertiesData.cross_section_name,
            propertiesData.cross_section_type,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(UPDATE_PROPERTIES_MESSAGE_HEADER)
      ) {
        const propertiesData =
          messageHandlingResult[UPDATE_PROPERTIES_MESSAGE_HEADER];
        try {
          this.state.preprocessor.update_properties(
            actionId,
            propertiesData.name,
            propertiesData.material_name,
            propertiesData.cross_section_name,
            propertiesData.cross_section_type,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(DELETE_PROPERTIES_MESSAGE_HEADER)
      ) {
        const propertiesData =
          messageHandlingResult[DELETE_PROPERTIES_MESSAGE_HEADER];
        try {
          this.state.preprocessor.delete_properties(
            actionId,
            propertiesData.name,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(RESTORE_PROPERTIES_MESSAGE_HEADER)
      ) {
        const propertiesData =
          messageHandlingResult[RESTORE_PROPERTIES_MESSAGE_HEADER];
        try {
          this.state.preprocessor.restore_properties(
            actionId,
            propertiesData.name,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          ASSIGN_PROPERTIES_TO_LINES_MESSAGE_HEADER
        )
      ) {
        const assignPropertiesToLinesData =
          messageHandlingResult[ASSIGN_PROPERTIES_TO_LINES_MESSAGE_HEADER];
        try {
          this.state.preprocessor.assign_properties_to_lines(
            actionId,
            assignPropertiesToLinesData.name,
            assignPropertiesToLinesData.line_numbers,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          ADD_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER
        )
      ) {
        const beamSectionLocalAxis1DirectionData =
          messageHandlingResult[
            ADD_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER
          ];
        try {
          this.state.preprocessor.add_beam_section_local_axis_1_direction(
            actionId,
            beamSectionLocalAxis1DirectionData.local_axis_1_direction,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          DELETE_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER
        )
      ) {
        const beamSectionLocalAxis1DirectionData =
          messageHandlingResult[
            DELETE_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER
          ];
        try {
          this.state.preprocessor.delete_beam_section_local_axis_1_direction(
            actionId,
            beamSectionLocalAxis1DirectionData.local_axis_1_direction,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          RESTORE_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER
        )
      ) {
        const beamSectionLocalAxis1DirectionData =
          messageHandlingResult[
            RESTORE_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER
          ];
        try {
          this.state.preprocessor.restore_beam_section_local_axis_1_direction(
            actionId,
            beamSectionLocalAxis1DirectionData.local_axis_1_direction,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          ASSIGN_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER
        )
      ) {
        const beamSectionOrientationData =
          messageHandlingResult[
            ASSIGN_BEAM_SECTION_LOCAL_AXIS_1_DIRECTION_MESSAGE_HEADER
          ];
        try {
          this.state.preprocessor.assign_beam_section_local_axis_1_direction(
            actionId,
            beamSectionOrientationData.local_axis_1_direction,
            beamSectionOrientationData.line_numbers,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          ASSIGN_PROPERTIES_TO_SURFACES_MESSAGE_HEADER
        )
      ) {
        const assignPropertiesToSurfacesData =
          messageHandlingResult[ASSIGN_PROPERTIES_TO_SURFACES_MESSAGE_HEADER];
        try {
          this.state.preprocessor.assign_properties_to_surfaces(
            actionId,
            assignPropertiesToSurfacesData.name,
            assignPropertiesToSurfacesData.surface_numbers,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          ADD_CONCENTRATED_LOAD_MESSAGE_HEADER
        )
      ) {
        const concentratedLoadData =
          messageHandlingResult[ADD_CONCENTRATED_LOAD_MESSAGE_HEADER];
        try {
          this.state.preprocessor.add_concentrated_load(
            actionId,
            concentratedLoadData.point_number,
            concentratedLoadData.fx,
            concentratedLoadData.fy,
            concentratedLoadData.fz,
            concentratedLoadData.mx,
            concentratedLoadData.my,
            concentratedLoadData.mz,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          UPDATE_CONCENTRATED_LOAD_MESSAGE_HEADER
        )
      ) {
        const concentratedLoadData =
          messageHandlingResult[UPDATE_CONCENTRATED_LOAD_MESSAGE_HEADER];
        try {
          this.state.preprocessor.update_concentrated_load(
            actionId,
            concentratedLoadData.point_number,
            concentratedLoadData.fx,
            concentratedLoadData.fy,
            concentratedLoadData.fz,
            concentratedLoadData.mx,
            concentratedLoadData.my,
            concentratedLoadData.mz,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          DELETE_CONCENTRATED_LOAD_MESSAGE_HEADER
        )
      ) {
        const concentratedLoadData =
          messageHandlingResult[DELETE_CONCENTRATED_LOAD_MESSAGE_HEADER];
        try {
          this.state.preprocessor.delete_concentrated_load(
            actionId,
            concentratedLoadData.point_number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          RESTORE_CONCENTRATED_LOAD_MESSAGE_HEADER
        )
      ) {
        const concentratedLoadData =
          messageHandlingResult[RESTORE_CONCENTRATED_LOAD_MESSAGE_HEADER];
        try {
          this.state.preprocessor.restore_concentrated_load(
            actionId,
            concentratedLoadData.point_number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          ADD_UNIFORMLY_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER
        )
      ) {
        const uniformlyDistributedLineLoadData =
          messageHandlingResult[
            ADD_UNIFORMLY_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER
          ];
        try {
          this.state.preprocessor.add_uniformly_distributed_line_load(
            actionId,
            uniformlyDistributedLineLoadData.line_number,
            uniformlyDistributedLineLoadData.qx,
            uniformlyDistributedLineLoadData.qy,
            uniformlyDistributedLineLoadData.qz,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          UPDATE_UNIFORMLY_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER
        )
      ) {
        const uniformlyDistributedLineLoadData =
          messageHandlingResult[
            UPDATE_UNIFORMLY_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER
          ];
        try {
          this.state.preprocessor.update_uniformly_distributed_line_load(
            actionId,
            uniformlyDistributedLineLoadData.line_number,
            uniformlyDistributedLineLoadData.qx,
            uniformlyDistributedLineLoadData.qy,
            uniformlyDistributedLineLoadData.qz,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          DELETE_UNIFORMLY_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER
        )
      ) {
        const uniformlyDistributedLineLoadData =
          messageHandlingResult[
            DELETE_UNIFORMLY_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER
          ];
        try {
          this.state.preprocessor.delete_uniformly_distributed_line_load(
            actionId,
            uniformlyDistributedLineLoadData.line_number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          RESTORE_UNIFORMLY_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER
        )
      ) {
        const uniformlyDistributedLineLoadData =
          messageHandlingResult[
            RESTORE_UNIFORMLY_DISTRIBUTED_LINE_LOAD_MESSAGE_HEADER
          ];
        try {
          this.state.preprocessor.restore_uniformly_distributed_line_load(
            actionId,
            uniformlyDistributedLineLoadData.line_number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          ADD_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_MESSAGE_HEADER
        )
      ) {
        const uniformlyDistributedSurfaceLoadData =
          messageHandlingResult[
            ADD_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_MESSAGE_HEADER
          ];
        try {
          this.state.preprocessor.add_uniformly_distributed_surface_load(
            actionId,
            uniformlyDistributedSurfaceLoadData.surface_number,
            uniformlyDistributedSurfaceLoadData.px,
            uniformlyDistributedSurfaceLoadData.py,
            uniformlyDistributedSurfaceLoadData.pz,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          UPDATE_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_MESSAGE_HEADER
        )
      ) {
        const uniformlyDistributedSurfaceLoadData =
          messageHandlingResult[
            UPDATE_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_MESSAGE_HEADER
          ];
        try {
          this.state.preprocessor.update_uniformly_distributed_surface_load(
            actionId,
            uniformlyDistributedSurfaceLoadData.surface_number,
            uniformlyDistributedSurfaceLoadData.px,
            uniformlyDistributedSurfaceLoadData.py,
            uniformlyDistributedSurfaceLoadData.pz,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          DELETE_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_MESSAGE_HEADER
        )
      ) {
        const uniformlyDistributedSurfaceLoadData =
          messageHandlingResult[
            DELETE_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_MESSAGE_HEADER
          ];
        try {
          this.state.preprocessor.delete_uniformly_distributed_surface_load(
            actionId,
            uniformlyDistributedSurfaceLoadData.surface_number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          RESTORE_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_MESSAGE_HEADER
        )
      ) {
        const uniformlyDistributedSurfaceLoadData =
          messageHandlingResult[
            RESTORE_UNIFORMLY_DISTRIBUTED_SURFACE_LOAD_MESSAGE_HEADER
          ];
        try {
          this.state.preprocessor.restore_uniformly_distributed_surface_load(
            actionId,
            uniformlyDistributedSurfaceLoadData.surface_number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          ADD_POINT_BOUNDARY_CONDITION_MESSAGE_HEADER
        )
      ) {
        const pointBoundaryConditionData =
          messageHandlingResult[ADD_POINT_BOUNDARY_CONDITION_MESSAGE_HEADER];
        try {
          this.state.preprocessor.add_point_boundary_condition(
            actionId,
            pointBoundaryConditionData.point_number,
            pointBoundaryConditionData.optional_ux,
            pointBoundaryConditionData.optional_uy,
            pointBoundaryConditionData.optional_uz,
            pointBoundaryConditionData.optional_rx,
            pointBoundaryConditionData.optional_ry,
            pointBoundaryConditionData.optional_rz,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          UPDATE_POINT_BOUNDARY_CONDITION_MESSAGE_HEADER
        )
      ) {
        const pointBoundaryConditionData =
          messageHandlingResult[UPDATE_POINT_BOUNDARY_CONDITION_MESSAGE_HEADER];
        try {
          this.state.preprocessor.update_point_boundary_condition(
            actionId,
            pointBoundaryConditionData.point_number,
            pointBoundaryConditionData.optional_ux,
            pointBoundaryConditionData.optional_uy,
            pointBoundaryConditionData.optional_uz,
            pointBoundaryConditionData.optional_rx,
            pointBoundaryConditionData.optional_ry,
            pointBoundaryConditionData.optional_rz,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          DELETE_POINT_BOUNDARY_CONDITION_MESSAGE_HEADER
        )
      ) {
        const pointBoundaryConditionData =
          messageHandlingResult[DELETE_POINT_BOUNDARY_CONDITION_MESSAGE_HEADER];
        try {
          this.state.preprocessor.delete_point_boundary_condition(
            actionId,
            pointBoundaryConditionData.point_number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          RESTORE_POINT_BOUNDARY_CONDITION_MESSAGE_HEADER
        )
      ) {
        const pointBoundaryConditionData =
          messageHandlingResult[
            RESTORE_POINT_BOUNDARY_CONDITION_MESSAGE_HEADER
          ];
        try {
          this.state.preprocessor.restore_point_boundary_condition(
            actionId,
            pointBoundaryConditionData.point_number,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          UPDATE_GLOBAL_MESH_SEED_MESSAGE_HEADER
        )
      ) {
        const globalMeshSeedData =
          messageHandlingResult[UPDATE_GLOBAL_MESH_SEED_MESSAGE_HEADER];
        try {
          this.state.preprocessor.update_global_mesh_seed_value(
            actionId,
            globalMeshSeedData.global_mesh_seed_value,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          UPDATE_LINES_MESH_SEED_MESSAGE_HEADER
        )
      ) {
        const linesMeshSeedData =
          messageHandlingResult[UPDATE_LINES_MESH_SEED_MESSAGE_HEADER];
        try {
          this.state.preprocessor.update_lines_mesh_seed(
            actionId,
            linesMeshSeedData.lines_mesh_seed_value,
            linesMeshSeedData.line_numbers,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          UNDO_LINES_MESH_SEED_UPDATE_MESSAGE_HEADER
        )
      ) {
        const linesMeshSeedData =
          messageHandlingResult[UNDO_LINES_MESH_SEED_UPDATE_MESSAGE_HEADER];
        try {
          this.state.preprocessor.undo_lines_mesh_seed_update(
            actionId,
            linesMeshSeedData.line_numbers,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          UPDATE_SURFACES_MESH_SEED_MESSAGE_HEADER
        )
      ) {
        const surfacesMeshSeedData =
          messageHandlingResult[UPDATE_SURFACES_MESH_SEED_MESSAGE_HEADER];
        try {
          this.state.preprocessor.update_surfaces_mesh_seed(
            actionId,
            surfacesMeshSeedData.surfaces_edges_1_3_mesh_seed_value,
            surfacesMeshSeedData.surfaces_edges_2_4_mesh_seed_value,
            surfacesMeshSeedData.surface_numbers,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
      if (
        messageHandlingResult.hasOwnProperty(
          UNDO_SURFACES_MESH_SEED_UPDATE_MESSAGE_HEADER
        )
      ) {
        const surfacesMeshSeedData =
          messageHandlingResult[UNDO_SURFACES_MESH_SEED_UPDATE_MESSAGE_HEADER];
        try {
          this.state.preprocessor.undo_surfaces_mesh_seed_update(
            actionId,
            surfacesMeshSeedData.surface_numbers,
            isActionIdShouldBeIncreased
          );
          await this.state.actionsRouter.approve_action(uid);
        } catch (error) {
          this.state.actionsRouter.discard_action(uid);
          throw error;
        }
      }
    } catch (error) {
      throw error;
    }
  }

  savePreprocessorData(fileName) {
    const activeActions = this.state.actionsRouter.extract_active_actions();
    let fileContent = new String();
    for (let i = 0; i < activeActions.length; i++) {
      const currentAction = JSON.stringify(activeActions[i])
        .replace(/\\/g, "")
        .replaceAll('""', '"');
      fileContent += currentAction + "\n";
    }
    const a = document.createElement("a");
    a.href = window.URL.createObjectURL(
      new Blob([fileContent], { type: "text/plain" })
    );
    a.download = `${fileName}.prep.txt`;
    a.click();
  }

  resetPreprocessor() {
    this.state.actionsRouter.reset();
    this.state.preprocessor.reset();
  }

  savePostprocessorData(fileName, job) {
    let jobData = "";

    const nodesData = Array();
    for (const [number, node] of Object.entries(job.mesh.nodes)) {
      nodesData.push(
        `${number}, ${node[1][0]}, ${node[1][1]}, ${node[1][2]}\n`
      );
    }
    if (nodesData.length !== 0) {
      jobData += "*NODES\n** number, x, y, z\n";
      jobData += nodesData.join("");
      jobData += "\n";
    }

    const trussElementsData = Array();
    for (const [number, trussElement] of Object.entries(
      job.mesh.truss_elements
    )) {
      trussElementsData.push(
        [
          number,
          trussElement[1][0],
          trussElement[1][1],
          trussElement[2][0],
          trussElement[2][1],
        ]
          .join(", ")
          .replace(/,\s*$/, "") + "\n"
      );
    }
    if (trussElementsData.length !== 0) {
      jobData += "*TRUSS_ELEMENTS\n** number, node_1, node_2, e, area\n";
      jobData += trussElementsData.join("");
      jobData += "\n";
    }

    const beamElementsData = Array();
    for (const [number, beamElement] of Object.entries(
      job.mesh.beam_elements
    )) {
      beamElementsData.push(
        [
          number,
          beamElement[1][0],
          beamElement[1][1],
          beamElement[2][0],
          beamElement[2][1],
          beamElement[2][2],
          beamElement[2][3],
          beamElement[2][4],
          beamElement[2][5],
          beamElement[2][6],
          beamElement[2][7],
        ]
          .join(", ")
          .replace(/,\s*$/, "") + "\n"
      );
    }
    if (beamElementsData.length !== 0) {
      jobData += "*BEAM_ELEMENTS\n";
      jobData +=
        [
          "** number",
          "node_1",
          "node_2",
          "e",
          "nu",
          "area",
          "i11",
          "i22",
          "i12",
          "it",
          "shear_factor",
        ]
          .join(", ")
          .replace(/,\s*$/, "") + "\n";
      jobData += beamElementsData.join("");
      jobData += "\n";
    }

    const plateElementsData = Array();
    for (const [number, plateElement] of Object.entries(
      job.mesh.plate_elements
    )) {
      plateElementsData.push(
        [
          number,
          plateElement[1][0],
          plateElement[1][1],
          plateElement[1][2],
          plateElement[1][3],
          plateElement[2][0],
          plateElement[2][1],
          plateElement[2][2],
          plateElement[2][3],
        ]
          .join(", ")
          .replace(/,\s*$/, "") + "\n"
      );
    }
    if (plateElementsData.length !== 0) {
      jobData += "*PLATE_ELEMENTS\n";
      jobData +=
        [
          "** number",
          "node_1",
          "node_2",
          "node_3",
          "node_4",
          "e",
          "nu",
          "thickness",
          "shear_factor",
        ]
          .join(", ")
          .replace(/,\s*$/, "") + "\n";
      jobData += plateElementsData.join("");
      jobData += "\n";
    }

    const displacementsData = Array();
    const globalLoadsData = Array();
    for (const [number, nodalData] of Object.entries(
      job.global_analysis_result
    )) {
      displacementsData.push(
        [
          number,
          nodalData[0],
          nodalData[1],
          nodalData[2],
          nodalData[3],
          nodalData[4],
          nodalData[5],
        ]
          .join(", ")
          .replace(/,\s*$/, "") + "\n"
      );
      globalLoadsData.push(
        [
          number,
          nodalData[6],
          nodalData[7],
          nodalData[8],
          nodalData[9],
          nodalData[10],
          nodalData[11],
        ]
          .join(", ")
          .replace(/,\s*$/, "") + "\n"
      );
    }
    if (displacementsData.length !== 0) {
      jobData += "*DISPLACEMENTS\n";
      jobData +=
        ["** node", "ux", "uy", "uz", "rx", "ry", "rz"]
          .join(", ")
          .replace(/,\s*$/, "") + "\n";
      jobData += displacementsData.join("");
      jobData += "\n";
    }
    if (globalLoadsData.length !== 0) {
      jobData += "*GLOBAL_LOADS\n";
      jobData +=
        ["** node", "fx", "fy", "fz", "mx", "my", "mz"]
          .join(", ")
          .replace(/,\s*$/, "") + "\n";
      jobData += globalLoadsData.join("");
      jobData += "\n";
    }

    const trussElementsAnalysisResultData = Array();
    for (const [number, trussElementData] of Object.entries(
      job.elements_analysis_result.truss_elements
    )) {
      trussElementsAnalysisResultData.push(
        [number, trussElementData[9]].join(", ").replace(/,\s*$/, "") + "\n"
      );
    }
    if (trussElementsAnalysisResultData.length !== 0) {
      jobData += "*TRUSS_ELEMENTS_ANALYSIS_RESULT\n";
      jobData +=
        ["** number", "force_r"].join(", ").replace(/,\s*$/, "") + "\n";
      jobData += trussElementsAnalysisResultData.join("");
      jobData += "\n";
    }

    const beamElementsAnalysisResultData = Array();
    for (const [number, beamElementData] of Object.entries(
      job.elements_analysis_result.beam_elements
    )) {
      beamElementsAnalysisResultData.push(
        [
          number,
          beamElementData[9],
          beamElementData[10],
          beamElementData[11],
          beamElementData[12],
          beamElementData[13],
          beamElementData[14],
          beamElementData[15],
          beamElementData[16],
          beamElementData[17],
          beamElementData[18],
        ]
          .join(", ")
          .replace(/,\s*$/, "") + "\n"
      );
    }
    if (beamElementsAnalysisResultData.length !== 0) {
      jobData += "*BEAM_ELEMENTS_ANALYSIS_RESULT\n";
      jobData +=
        [
          "** number",
          "force_r",
          "force_s",
          "force_t",
          "moment_r",
          "moment_s_at_node_1",
          "moment_s_at_mid",
          "moment_s_at_node_2",
          "moment_t_at_node_1",
          "moment_t_at_mid",
          "moment_t_at_node_2",
        ]
          .join(", ")
          .replace(/,\s*$/, "") + "\n";
      jobData += beamElementsAnalysisResultData.join("");
      jobData += "\n";
    }

    const plateElementsAnalysisResultData = Array();
    for (const [number, plateElementData] of Object.entries(
      job.elements_analysis_result.plate_elements
    )) {
      plateElementsAnalysisResultData.push(
        [
          number,
          plateElementData[9],
          plateElementData[10],
          plateElementData[11],
          plateElementData[12],
          plateElementData[13],
          plateElementData[14],
          plateElementData[15],
          plateElementData[16],
        ]
          .join(", ")
          .replace(/,\s*$/, "") + "\n"
      );
    }
    if (plateElementsAnalysisResultData.length !== 0) {
      jobData += "*PLATE_ELEMENTS_ANALYSIS_RESULT\n";
      jobData +=
        [
          "** number",
          "mem_force_r",
          "mem_force_s",
          "mem_force_r_s",
          "bend_moment_r",
          "bend_moment_s",
          "bend_moment_r_s",
          "shear_force_r_t",
          "shear_force_s_t",
        ]
          .join(", ")
          .replace(/,\s*$/, "") + "\n";
      jobData += plateElementsAnalysisResultData.join("");
      jobData += "\n";
    }

    const a = document.createElement("a");
    a.href = window.URL.createObjectURL(
      new Blob([jobData], { type: "text/plain" })
    );
    a.download = `${fileName}.post.txt`;
    a.click();
  }

  async submitJob(jobName, solver, maxIter) {
    try {
      const dataForMesher = this.state.preprocessor.extract_data_for_mesher();
      document.querySelector(EVENT_TARGET).dispatchEvent(
        new CustomEvent(SOLVER_MESSAGE_EVENT_HEADER, {
          bubbles: true,
          composed: true,
          detail: {
            header: DATA_FOR_MESHER_EXTRACTED_MESSAGE_HEADER,
            job_name: jobName,
            status: COMPLETED_STATUS,
            message: "Data for mesher has been successfully extracted",
          },
        })
      );
      const mesh = this.state.mesher.generate_mesh(dataForMesher);
      document.querySelector(EVENT_TARGET).dispatchEvent(
        new CustomEvent(SOLVER_MESSAGE_EVENT_HEADER, {
          bubbles: true,
          composed: true,
          detail: {
            header: MESH_FOR_SOLVER_EXTRACTED_MESSAGE_HEADER,
            job_name: jobName,
            status: COMPLETED_STATUS,
            message: "Mesh has been successfully generated",
          },
        })
      );
      createFEM(jobName, mesh, solver, maxIter);
    } catch (error) {
      throw error;
    }
  }

  performGlobalAnalysis(jobName, solver, maxIter) {
    performGlobalAnalysis(jobName, solver, maxIter);
  }

  extractGlobalAnalysisResult(jobName) {
    extractGlobalAnalysisResult(jobName);
  }

  extractElementsAnalysisResult(jobName) {
    extractElementsAnalysisResult(jobName);
  }
}
