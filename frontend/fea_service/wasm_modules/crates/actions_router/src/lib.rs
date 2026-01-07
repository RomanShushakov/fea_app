use std::collections::HashMap;
use serde_json::Value;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use wasm_bindgen_futures::future_to_promise;
use serde_json::json;
use serde::Serialize;
use serde_wasm_bindgen::Serializer;

use js_sys::{Array, Promise};

mod functions;
use functions::{log, generate_uid};

mod action;
use action::
{
    Action, GeometryActionType, ActionType, PropertiesActionType, LoadsActionType, Coordinates,
    ConcentratedLoad, UniformlyDistributedLineLoad, UniformlyDistributedSurfaceLoad, PointBoundaryCondition, 
    BoundaryConditionsActionType, MeshSeedActionType, ActionInPool,
};

mod props;
use props::Props;

mod types;

mod methods_for_geometry_type_actions_handle;

mod methods_for_properties_type_actions_handle;

mod methods_for_loads_type_actions_handle;

mod methods_for_boundary_condition_type_actions_handle;

mod methods_for_mesh_seed_type_actions_handle;


#[wasm_bindgen]
pub struct ActionsRouter
{
    props: Props,

    // ( action, is_action_id_should_be_added_to_active_actions )
    current_action: Option<(Action, bool)>,
    // HashMap<uid, ( action, is_action_id_should_be_added_to_active_actions, is_action_add_to_cache )>  
    action_pool: HashMap<u32, ActionInPool>,
    active_actions: Vec<Action>,
    undo_actions: Vec<Action>,
}


#[wasm_bindgen]
impl ActionsRouter
{
    pub fn create(
        add_point_message_header: String,
        update_point_message_header: String,
        delete_point_message_header: String,
        restore_point_message_header: String,

        add_line_message_header: String,
        update_line_message_header: String,
        delete_line_message_header: String,
        restore_line_message_header: String,

        add_surface_message_header: String,
        update_surface_message_header: String,
        rotate_surface_vertices_clockwise_message_header: String,
        rotate_surface_vertices_counter_clockwise_message_header: String,
        flip_surface_normal_axis_message_header: String,
        delete_surface_message_header: String,
        restore_surface_message_header: String,

        add_material_message_header: String,
        update_material_message_header: String,
        delete_material_message_header: String,
        restore_material_message_header: String,

        add_truss_section_message_header: String,
        update_truss_section_message_header: String,
        delete_truss_section_message_header: String,
        restore_truss_section_message_header: String,

        add_beam_section_message_header: String,
        update_beam_section_message_header: String,
        delete_beam_section_message_header: String,
        restore_beam_section_message_header: String,

        add_plate_section_message_header: String,
        update_plate_section_message_header: String,
        delete_plate_section_message_header: String,
        restore_plate_section_message_header: String,

        add_properties_message_header: String,
        update_properties_message_header: String,
        delete_properties_message_header: String,
        restore_properties_message_header: String,

        assign_properties_to_lines_message_header: String,

        add_beam_section_local_axis_1_direction_message_header: String,
        delete_beam_section_local_axis_1_direction_message_header: String,
        assign_beam_section_local_axis_1_direction_message_header: String,
        restore_beam_section_local_axis_1_direction_message_header: String,

        assign_properties_to_surfaces_message_header: String,

        add_concentrated_load_message_header: String,
        update_concentrated_load_message_header: String,
        delete_concentrated_load_message_header: String,
        restore_concentrated_load_message_header: String,

        add_uniformly_distributed_line_load_message_header: String,
        update_uniformly_distributed_line_load_message_header: String,
        delete_uniformly_distributed_line_load_message_header: String,
        restore_uniformly_distributed_line_load_message_header: String,

        add_uniformly_distributed_surface_load_message_header: String,
        update_uniformly_distributed_surface_load_message_header: String,
        delete_uniformly_distributed_surface_load_message_header: String,
        restore_uniformly_distributed_surface_load_message_header: String,

        add_point_boundary_condition_message_header: String,
        update_point_boundary_condition_message_header: String,
        delete_point_boundary_condition_message_header: String,
        restore_point_boundary_condition_message_header: String,

        update_global_mesh_seed_message_header: String,

        update_lines_mesh_seed_message_header: String,
        undo_lines_mesh_seed_update_message_header: String,

        update_surfaces_mesh_seed_message_header: String,
        undo_surfaces_mesh_seed_update_message_header: String,

        undo_message_header: String,
        redo_message_header: String,

        local_axis_1_direction_input_info_message_header: String,

        max_point_number: u32,
        max_line_number: u32,
        max_surface_number: u32,
    ) 
        -> ActionsRouter
    {
        let props = Props::create(
            add_point_message_header,
            update_point_message_header,
            delete_point_message_header,
            restore_point_message_header,

            add_line_message_header,
            update_line_message_header,
            delete_line_message_header,
            restore_line_message_header,

            add_surface_message_header,
            update_surface_message_header,
            rotate_surface_vertices_clockwise_message_header,
            rotate_surface_vertices_counter_clockwise_message_header,
            flip_surface_normal_axis_message_header,
            delete_surface_message_header,
            restore_surface_message_header,

            add_material_message_header,
            update_material_message_header,
            delete_material_message_header,
            restore_material_message_header,

            add_truss_section_message_header,
            update_truss_section_message_header,
            delete_truss_section_message_header,
            restore_truss_section_message_header,

            add_beam_section_message_header,
            update_beam_section_message_header,
            delete_beam_section_message_header,
            restore_beam_section_message_header,

            add_plate_section_message_header,
            update_plate_section_message_header,
            delete_plate_section_message_header,
            restore_plate_section_message_header,

            add_properties_message_header,
            update_properties_message_header,
            delete_properties_message_header,
            restore_properties_message_header,

            assign_properties_to_lines_message_header,

            add_beam_section_local_axis_1_direction_message_header,
            delete_beam_section_local_axis_1_direction_message_header,
            assign_beam_section_local_axis_1_direction_message_header,
            restore_beam_section_local_axis_1_direction_message_header,

            assign_properties_to_surfaces_message_header,

            add_concentrated_load_message_header,
            update_concentrated_load_message_header,
            delete_concentrated_load_message_header,
            restore_concentrated_load_message_header,

            add_uniformly_distributed_line_load_message_header,
            update_uniformly_distributed_line_load_message_header,
            delete_uniformly_distributed_line_load_message_header,
            restore_uniformly_distributed_line_load_message_header,

            add_uniformly_distributed_surface_load_message_header,
            update_uniformly_distributed_surface_load_message_header,
            delete_uniformly_distributed_surface_load_message_header,
            restore_uniformly_distributed_surface_load_message_header,

            add_point_boundary_condition_message_header,
            update_point_boundary_condition_message_header,
            delete_point_boundary_condition_message_header,
            restore_point_boundary_condition_message_header,

            update_global_mesh_seed_message_header,

            update_lines_mesh_seed_message_header,
            undo_lines_mesh_seed_update_message_header,

            update_surfaces_mesh_seed_message_header,
            undo_surfaces_mesh_seed_update_message_header,

            undo_message_header,
            redo_message_header,

            local_axis_1_direction_input_info_message_header,

            max_point_number,
            max_line_number,
            max_surface_number,
        );
        let current_action = None;
        let action_pool = HashMap::new();
        let active_actions = Vec::new();
        let undo_actions = Vec::new();
        
        ActionsRouter
        {
            props,
            current_action,
            action_pool,
            active_actions,
            undo_actions,
        }
    }


    fn handle_undo_message(&mut self, undo_data: &Value) -> Result<(), JsValue>
    {
        let action_id = undo_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Redo action: \
                Action id could not be converted to u32!")))?;
        if let Some(position) = self.active_actions.iter().rposition(|action|
            action.is_action_id_same(action_id))
        {
            let undo_action = self.active_actions.remove(position);
            match &undo_action.ref_action_type()
            {
                ActionType::GeometryActionType(geometry_action_type) =>
                {
                    match geometry_action_type
                    {
                        GeometryActionType::AddPoint(
                            point_number,
                            _coordinates,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                GeometryActionType::DeletePoint(
                                    *point_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        GeometryActionType::UpdatePoint(
                            point_number,
                            old_coordinates,
                            new_coordinates,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                GeometryActionType::UpdatePoint(
                                    *point_number,
                                    new_coordinates.to_owned(),
                                    old_coordinates.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        GeometryActionType::DeletePoint(
                            point_number,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                GeometryActionType::RestorePoint(
                                    *point_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        GeometryActionType::RestorePoint(_, _) => unreachable!(),
                        GeometryActionType::AddLine(
                            line_number,
                            _point_1_number,
                            _point_2_number,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                GeometryActionType::DeleteLine(
                                    *line_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        GeometryActionType::UpdateLine(
                            line_number,
                            old_point_1_number,
                            old_point_2_number,
                            new_point_1_number,
                            new_point_2_number,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                GeometryActionType::UpdateLine(
                                    *line_number,
                                    *new_point_1_number,
                                    *new_point_2_number,
                                    *old_point_1_number,
                                    *old_point_2_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        GeometryActionType::DeleteLine(
                            line_number,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                GeometryActionType::RestoreLine(
                                    *line_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        GeometryActionType::RestoreLine(_, _) => unreachable!(),
                        GeometryActionType::AddSurface(
                            surface_number,
                            _point_1_number,
                            _point_2_number,
                            _point_3_number,
                            _point_4_number,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                GeometryActionType::DeleteSurface(
                                    *surface_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        GeometryActionType::UpdateSurface(
                            surface_number,
                            old_point_1_number,
                            old_point_2_number,
                            old_point_3_number,
                            old_point_4_number,
                            new_point_1_number,
                            new_point_2_number,
                            new_point_3_number,
                            new_point_4_number,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                GeometryActionType::UpdateSurface(
                                    *surface_number,
                                    *new_point_1_number,
                                    *new_point_2_number,
                                    *new_point_3_number,
                                    *new_point_4_number,
                                    *old_point_1_number,
                                    *old_point_2_number,
                                    *old_point_3_number,
                                    *old_point_4_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        GeometryActionType::RotateSurfaceVerticesClockwise(
                            surface_number,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                GeometryActionType::RotateSurfaceVerticesCounterClockwise(
                                    *surface_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        GeometryActionType::RotateSurfaceVerticesCounterClockwise(
                            surface_number,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                GeometryActionType::RotateSurfaceVerticesClockwise(
                                    *surface_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        GeometryActionType::FlipSurfaceNormalAxis(
                            surface_number,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                GeometryActionType::FlipSurfaceNormalAxis(
                                    *surface_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        GeometryActionType::DeleteSurface(
                            surface_number,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                GeometryActionType::RestoreSurface(
                                    *surface_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        GeometryActionType::RestoreSurface(_, _) => unreachable!(),
                    }
                },
                ActionType::PropertiesActionType(properties_action_type) =>
                {
                    match properties_action_type
                    {
                        PropertiesActionType::AddMaterial(
                            material_name,
                            _young_modulus,
                            _poisson_ratio,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::DeleteMaterial(
                                    material_name.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::UpdateMaterial(
                            material_name,
                            old_young_modulus,
                            old_poisson_ratio,
                            new_young_modulus,
                            new_poisson_ratio,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::UpdateMaterial(
                                    material_name.to_owned(),
                                    *new_young_modulus, *new_poisson_ratio,
                                    *old_young_modulus, *old_poisson_ratio,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::DeleteMaterial(
                            material_name,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::RestoreMaterial(
                                    material_name.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::RestoreMaterial(_, _) => unreachable!(),
                        PropertiesActionType::AddTrussSection(
                            truss_section_name,
                            _area,
                            _area2,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::DeleteTrussSection(
                                    truss_section_name.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::UpdateTrussSection(
                            truss_section_name,
                            old_area,
                            old_area2,
                            new_area,
                            new_area2,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::UpdateTrussSection(
                                    truss_section_name.to_owned(),
                                    *new_area, *new_area2,
                                    *old_area, *old_area2,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::DeleteTrussSection(
                            truss_section_name,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::RestoreTrussSection(
                                    truss_section_name.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::RestoreTrussSection(_, _) => unreachable!(),
                        PropertiesActionType::AddBeamSection(
                            beam_section_name,
                            _area,
                            _i11,
                            _i22,
                            _i12,
                            _it,
                            _shear_factor,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::DeleteBeamSection(
                                    beam_section_name.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::UpdateBeamSection(
                            beam_section_name,
                            old_area,
                            old_i11,
                            old_i22,
                            old_i12,
                            old_it,
                            old_shear_factor,
                            new_area,
                            new_i11,
                            new_i22,
                            new_i12,
                            new_it,
                            new_shear_factor,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::UpdateBeamSection(
                                    beam_section_name.to_owned(),
                                    *new_area, *new_i11,
                                    *new_i22, *new_i12,
                                    *new_it, *new_shear_factor,
                                    *old_area, *old_i11,
                                    *old_i22, *old_i12,
                                    *old_it, *old_shear_factor,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::DeleteBeamSection(
                            beam_section_name,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::RestoreBeamSection(
                                    beam_section_name.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::RestoreBeamSection(_, _) => unreachable!(),
                        PropertiesActionType::AddPlateSection(
                            plate_section_name,
                            _thickness,
                            _shear_factor,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::DeletePlateSection(
                                    plate_section_name.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::UpdatePlateSection(
                            plate_section_name,
                            old_thickness,
                            old_shear_factor,
                            new_thickness,
                            new_shear_factor,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::UpdatePlateSection(
                                    plate_section_name.to_owned(),
                                    *new_thickness, *new_shear_factor,
                                    *old_thickness, *old_shear_factor,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::DeletePlateSection(
                            plate_section_name,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::RestorePlateSection(
                                    plate_section_name.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::RestorePlateSection(_, _) => unreachable!(),
                        PropertiesActionType::AddProperties(
                            properties_name,
                            _material_name,
                            _cross_section_name,
                            _cross_section_type,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::DeleteProperties(
                                    properties_name.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::UpdateProperties(
                            properties_name,
                            old_material_name,
                            old_cross_section_name,
                            old_cross_section_type,
                            new_material_name,
                            new_cross_section_name,
                            new_cross_section_type,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::UpdateProperties(
                                    properties_name.to_owned(),
                                    new_material_name.to_owned(),
                                    new_cross_section_name.to_owned(),
                                    new_cross_section_type.to_owned(),
                                    old_material_name.to_owned(),
                                    old_cross_section_name.to_owned(),
                                    old_cross_section_type.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::DeleteProperties(
                            properties_name,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::RestoreProperties(
                                    properties_name.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::RestoreProperties(_, _) => unreachable!(),
                        PropertiesActionType::AssignPropertiesToLines(
                            assigned_properties_name,
                            old_line_numbers,
                            new_line_numbers,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::AssignPropertiesToLines(
                                    assigned_properties_name.to_owned(),
                                    new_line_numbers.to_owned(),
                                    old_line_numbers.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::AddBeamSectionLocalAxis1Direction(
                            local_axis_1_direction,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::DeleteBeamSectionLocalAxis1Direction(
                                    local_axis_1_direction.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::DeleteBeamSectionLocalAxis1Direction(
                            local_axis_1_direction,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::RestoreBeamSectionLocalAxis1Direction(
                                    local_axis_1_direction.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::RestoreBeamSectionLocalAxis1Direction(_, _) => unreachable!(),
                        PropertiesActionType::AssignBeamSectionLocalAxis1Direction(
                            local_axis_1_direction,
                            old_line_numbers,
                            new_line_numbers,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::AssignBeamSectionLocalAxis1Direction(
                                    local_axis_1_direction.to_owned(),
                                    new_line_numbers.to_owned(),
                                    old_line_numbers.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        PropertiesActionType::AssignPropertiesToSurfaces(
                            assigned_properties_name,
                            old_surface_numbers,
                            new_surface_numbers,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                PropertiesActionType::AssignPropertiesToSurfaces(
                                    assigned_properties_name.to_owned(),
                                    new_surface_numbers.to_owned(),
                                    old_surface_numbers.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                    }
                },
                ActionType::LoadsActionType(loads_action_type) =>
                {
                    match loads_action_type
                    {
                        LoadsActionType::AddConcentratedLoad(
                            point_number,
                            _concentrated_load,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                LoadsActionType::DeleteConcentratedLoad(
                                    *point_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        LoadsActionType::UpdateConcentratedLoad(
                            point_number,
                            old_concentrated_load,
                            new_concentrated_load,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                LoadsActionType::UpdateConcentratedLoad(
                                    *point_number,
                                    new_concentrated_load.to_owned(),
                                    old_concentrated_load.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        LoadsActionType::DeleteConcentratedLoad(
                            point_number,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                LoadsActionType::RestoreConcentratedLoad(
                                    *point_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        LoadsActionType::RestoreConcentratedLoad(_, _) => unreachable!(),
                        LoadsActionType::AddUniformlyDistributedLineLoad(
                            line_number,
                            _uniformly_distributed_line_load,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                LoadsActionType::DeleteUniformlyDistributedLineLoad(
                                    *line_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        LoadsActionType::UpdateUniformlyDistributedLineLoad(
                            line_number,
                            old_uniformly_distributed_line_load,
                            new_uniformly_distributed_line_load,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                LoadsActionType::UpdateUniformlyDistributedLineLoad(
                                    *line_number,
                                    new_uniformly_distributed_line_load.to_owned(),
                                    old_uniformly_distributed_line_load.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        LoadsActionType::DeleteUniformlyDistributedLineLoad(
                            line_number,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                LoadsActionType::RestoreUniformlyDistributedLineLoad(
                                    *line_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        LoadsActionType::RestoreUniformlyDistributedLineLoad(_, _) => unreachable!(),
                        LoadsActionType::AddUniformlyDistributedSurfaceLoad(
                            surface_number,
                            _uniformly_distributed_surface_load,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                LoadsActionType::DeleteUniformlyDistributedSurfaceLoad(
                                    *surface_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        LoadsActionType::UpdateUniformlyDistributedSurfaceLoad(
                            surface_number,
                            old_uniformly_distributed_surface_load,
                            new_uniformly_distributed_surface_load,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                LoadsActionType::UpdateUniformlyDistributedSurfaceLoad(
                                    *surface_number,
                                    new_uniformly_distributed_surface_load.to_owned(),
                                    old_uniformly_distributed_surface_load.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        LoadsActionType::DeleteUniformlyDistributedSurfaceLoad(
                            surface_number,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                LoadsActionType::RestoreUniformlyDistributedSurfaceLoad(
                                    *surface_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        LoadsActionType::RestoreUniformlyDistributedSurfaceLoad(_, _) => unreachable!(),
                    }
                },
                ActionType::BoundaryConditionsActionType(boundary_conditions_action_type) =>
                {
                    match boundary_conditions_action_type
                    {
                        BoundaryConditionsActionType::AddPointBoundaryCondition(
                            point_number,
                            _boundary_condition,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                BoundaryConditionsActionType::DeletePointBoundaryCondition(
                                    *point_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        BoundaryConditionsActionType::UpdatePointBoundaryCondition(
                            point_number,
                            old_boundary_condition,
                            new_boundary_condition,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                BoundaryConditionsActionType::UpdatePointBoundaryCondition(
                                    *point_number,
                                    new_boundary_condition.to_owned(),
                                    old_boundary_condition.to_owned(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        BoundaryConditionsActionType::DeletePointBoundaryCondition(
                            point_number,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                BoundaryConditionsActionType::RestorePointBoundaryCondition(
                                    *point_number,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        BoundaryConditionsActionType::RestorePointBoundaryCondition(_, _) => unreachable!(),
                    }
                }
                ActionType::MeshSeedActionType(mesh_seed_action_type) =>
                {
                    match mesh_seed_action_type
                    {
                        MeshSeedActionType::UpdateGlobalMeshSeed(
                            old_global_mesh_seed_value, 
                            new_global_mesh_seed_value,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                MeshSeedActionType::UpdateGlobalMeshSeed(
                                    *new_global_mesh_seed_value,
                                    *old_global_mesh_seed_value,
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        MeshSeedActionType::UpdateLinesMeshSeed(
                            _lines_mesh_seed_value,
                            line_numbers,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                MeshSeedActionType::UndoLinesMeshSeedUpdate(
                                    line_numbers.to_vec(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        MeshSeedActionType::UndoLinesMeshSeedUpdate(_, _) => unreachable!(),
                        MeshSeedActionType::UpdateSurfacesMeshSeed(
                            _edges_1_3_mesh_seed_value,
                            _edges_2_4_mesh_seed_value,
                            surface_numbers,
                            _is_action_id_should_be_increased) =>
                        {
                            let action_type = ActionType::from(
                                MeshSeedActionType::UndoSurfacesMeshSeedUpdate(
                                    surface_numbers.to_vec(),
                                    false));
                            let action = Action::create(action_id, action_type);
                            let add_to_active_actions = false;
                            self.current_action = Some((action, add_to_active_actions));
                        },
                        MeshSeedActionType::UndoSurfacesMeshSeedUpdate(_, _) => unreachable!(),
                    }
                }
            }
            self.undo_actions.push(undo_action);
        }
        Ok(())
    }


    fn handle_redo_message(&mut self, redo_data: &Value) -> Result<(), JsValue>
    {
        let action_id = redo_data["actionId"].to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Actions router: Redo action: \
                Action id could not be converted to u32!")))?;
        if let Some(position) = self.undo_actions.iter().position(|action|
            action.is_action_id_same(action_id))
        {
            let redo_action = self.undo_actions.remove(position);
            let add_to_active_actions = true;
            self.current_action = Some((redo_action, add_to_active_actions));
        }
        Ok(())
    }

    
    fn handle_current_action(&mut self, is_redo: bool) -> Result<JsValue, JsValue>
    {
        let mut uid_generator = |action: Action, add_to_active_actions: bool| 
        {
            let uid = generate_uid(&self.action_pool);
            let action_in_pool = ActionInPool::create(action.clone(), add_to_active_actions, is_redo);
            self.action_pool.insert(uid, action_in_pool);
            uid
        };

        let error_message = "Actions router: Handle current action: There are no action data!";
        let mut action_handling_result = JsValue::from(error_message);
        if let Some((action, add_to_active_actions)) = &self.current_action
        {
            let ref_action_id = action.ref_action_id();
            let ref_action_type = action.ref_action_type();
            let serializer = Serializer::json_compatible();
            match ref_action_type
            {
                ActionType::GeometryActionType(geometry_action_type) =>
                {
                    match geometry_action_type
                    {
                        GeometryActionType::AddPoint(
                            point_number,
                            coordinates,
                            is_action_id_should_be_increased) =>
                        {
                            let Coordinates { x, y, z } = coordinates;
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.add_point_message_header: 
                                { 
                                    "number": *point_number, "x": *x, "y": *y, "z": *z 
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Add point action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        GeometryActionType::UpdatePoint(
                            point_number,
                            _old_coordinates,
                            new_coordinates,
                            is_action_id_should_be_increased) =>
                        {
                            let Coordinates { x, y, z } =
                                new_coordinates;
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.update_point_message_header: 
                                { 
                                    "number": *point_number, "x": *x, "y": *y, "z": *z 
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update point action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        GeometryActionType::DeletePoint(
                            point_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.delete_point_message_header: 
                                { "number": *point_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Delete point action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        GeometryActionType::RestorePoint(
                            point_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.restore_point_message_header: 
                                { "number": *point_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Restore point action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        GeometryActionType::AddLine(
                            line_number,
                            point_1_number,
                            point_2_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.add_line_message_header: 
                                { 
                                    "number": *line_number, "point_1_number": *point_1_number,
                                    "point_2_number": *point_2_number
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Add line action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        GeometryActionType::UpdateLine(
                            line_number,
                            _old_point_1_number,
                            _old_point_2_number,
                            new_point_1_number,
                            new_point_2_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.update_line_message_header: 
                                { 
                                    "number": *line_number, "point_1_number": *new_point_1_number,
                                    "point_2_number": *new_point_2_number
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update line action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        GeometryActionType::DeleteLine(
                            line_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.delete_line_message_header: 
                                { "number": *line_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Delete line action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        GeometryActionType::RestoreLine(
                            line_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.restore_line_message_header: 
                                { "number": *line_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Restore line action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        GeometryActionType::AddSurface(
                            surface_number,
                            point_1_number,
                            point_2_number,
                            point_3_number,
                            point_4_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.add_surface_message_header: 
                                { 
                                    "number": *surface_number, "point_1_number": *point_1_number,
                                    "point_2_number": *point_2_number, "point_3_number": *point_3_number,
                                    "point_4_number": *point_4_number,
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Add surface action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        GeometryActionType::UpdateSurface(
                            line_number,
                            _old_point_1_number,
                            _old_point_2_number,
                            _old_point_3_number,
                            _old_point_4_number,
                            new_point_1_number,
                            new_point_2_number,
                            new_point_3_number,
                            new_point_4_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.update_surface_message_header: 
                                { 
                                    "number": *line_number, "point_1_number": *new_point_1_number,
                                    "point_2_number": *new_point_2_number, "point_3_number": *new_point_3_number,
                                    "point_4_number": *new_point_4_number,  
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update surface action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        GeometryActionType::RotateSurfaceVerticesClockwise(
                            surface_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.rotate_surface_vertices_clockwise_message_header: 
                                { "number": *surface_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Rotate surface vertices clockwise action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        GeometryActionType::RotateSurfaceVerticesCounterClockwise(
                            surface_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ 
                                &self.props.rotate_surface_vertices_counter_clockwise_message_header: 
                                { "number": *surface_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Rotate surface vertices counter clockwise \
                                    action: Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        GeometryActionType::FlipSurfaceNormalAxis(
                            surface_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ 
                                &self.props.flip_surface_normal_axis_message_header: 
                                { "number": *surface_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Flip surface normal axis \
                                    action: Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        GeometryActionType::DeleteSurface(
                            surface_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.delete_surface_message_header: 
                                { "number": *surface_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Delete surface action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        GeometryActionType::RestoreSurface(
                            surface_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.restore_surface_message_header: 
                                { "number": *surface_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Restore surface action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                    }
                },
                ActionType::PropertiesActionType(properties_action_type) =>
                {
                    match properties_action_type
                    {
                        PropertiesActionType::AddMaterial(
                            material_name,
                            young_modulus,
                            poisson_ratio,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.add_material_message_header: 
                                { 
                                    "name": material_name,
                                    "young_modulus": *young_modulus, "poisson_ratio": *poisson_ratio
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Add material action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::UpdateMaterial(
                            material_name,
                            _old_young_modulus,
                            _old_poisson_ratio,
                            new_young_modulus,
                            new_poisson_ratio,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.update_material_message_header: 
                                { 
                                    "name": material_name,
                                    "young_modulus": *new_young_modulus, "poisson_ratio": *new_poisson_ratio
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update material action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::DeleteMaterial(
                            material_name,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.delete_material_message_header: 
                                { "name": material_name },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Delete material action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::RestoreMaterial(
                            material_name,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.restore_material_message_header: 
                                { "name": material_name },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Restore material action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::AddTrussSection(
                            truss_section_name,
                            area,
                            area2,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.add_truss_section_message_header: 
                                { 
                                    "name": truss_section_name, "area": *area, "area2": *area2
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Add truss section action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::UpdateTrussSection(
                            truss_section_name,
                            _old_area,
                            _old_area2,
                            new_area,
                            new_area2,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.update_truss_section_message_header: 
                                { 
                                    "name": truss_section_name, "area": *new_area, "area2": *new_area2
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update truss section action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::DeleteTrussSection(
                            truss_section_name,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.delete_truss_section_message_header: 
                                { "name": truss_section_name },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Delete truss section action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::RestoreTrussSection(
                            truss_section_name,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.restore_truss_section_message_header: 
                                { "name": truss_section_name },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Restore truss section action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::AddBeamSection(
                            beam_section_name,
                            area,
                            i11,
                            i22,
                            i12,
                            it,
                            shear_factor,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.add_beam_section_message_header: 
                                { 
                                    "name": beam_section_name, "area": *area, "i11": *i11, "i22": *i22, 
                                    "i12": *i12, "it": *it, "shear_factor": *shear_factor
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Add beam section action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::UpdateBeamSection(
                            beam_section_name,
                            _old_area,
                            _old_i11,
                            _old_i22,
                            _old_i12,
                            _old_it,
                            _old_shear_factor,
                            new_area,
                            new_i11,
                            new_i22,
                            new_i12,
                            new_it,
                            new_shear_factor,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.update_beam_section_message_header: 
                                { 
                                    "name": beam_section_name, "area": *new_area, "i11": *new_i11, "i22": *new_i22,
                                    "i12": *new_i12, "it": *new_it, "shear_factor": *new_shear_factor
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update beam section action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::DeleteBeamSection(
                            beam_section_name,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.delete_beam_section_message_header: 
                                { "name": beam_section_name },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Delete beam section action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::RestoreBeamSection(
                            beam_section_name,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.restore_beam_section_message_header: 
                                { "name": beam_section_name },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Restore beam section action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::AddPlateSection(
                            plate_section_name,
                            thickness,
                            shear_factor,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.add_plate_section_message_header: 
                                { 
                                    "name": plate_section_name, "thickness": *thickness, "shear_factor": *shear_factor
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Add beam section action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::UpdatePlateSection(
                            plate_section_name,
                            _old_thickness,
                            _old_shear_factor,
                            new_thickness,
                            new_shear_factor,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.update_plate_section_message_header: 
                                { 
                                    "name": plate_section_name, "thickness": *new_thickness, 
                                    "shear_factor": *new_shear_factor
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update beam section action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::DeletePlateSection(
                            plate_section_name,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.delete_plate_section_message_header: 
                                { "name": plate_section_name },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Delete beam section action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::RestorePlateSection(
                            plate_section_name,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.restore_plate_section_message_header: 
                                { "name": plate_section_name },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Restore beam section action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::AddProperties(
                            properties_name,
                            material_name,
                            cross_section_name,
                            cross_section_type,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.add_properties_message_header: 
                                { 
                                    "name": properties_name, "material_name": material_name,
                                    "cross_section_name": cross_section_name, 
                                    "cross_section_type": cross_section_type,
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Add properties action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::UpdateProperties(
                            properties_name,
                            _old_material_name,
                            _old_cross_section_name,
                            _old_cross_section_type,
                            new_material_name,
                            new_cross_section_name,
                            new_cross_section_type,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.update_properties_message_header: 
                                { 
                                    "name": properties_name, "material_name": new_material_name,
                                    "cross_section_name": new_cross_section_name, 
                                    "cross_section_type": new_cross_section_type,
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update properties action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::DeleteProperties(
                            properties_name,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.delete_properties_message_header: 
                                { "name": properties_name },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Delete properties action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::RestoreProperties(
                            properties_name,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.restore_properties_message_header: 
                                { "name": properties_name },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Restore properties action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::AssignPropertiesToLines(
                            properties_name,
                            _old_line_numbers,
                            new_line_numbers,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.assign_properties_to_lines_message_header: 
                                { 
                                    "name": properties_name, "line_numbers": new_line_numbers
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update assigned properties to lines action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::AddBeamSectionLocalAxis1Direction(
                            local_axis_1_direction,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.add_beam_section_local_axis_1_direction_message_header: 
                                { "local_axis_1_direction": local_axis_1_direction },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Add beam section local axis 1 direction action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::DeleteBeamSectionLocalAxis1Direction(
                            local_axis_1_direction,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.delete_beam_section_local_axis_1_direction_message_header: 
                                { "local_axis_1_direction": local_axis_1_direction },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Delete beam section local axis 1 direction action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::RestoreBeamSectionLocalAxis1Direction(
                            local_axis_1_direction,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.restore_beam_section_local_axis_1_direction_message_header: 
                                { "local_axis_1_direction": local_axis_1_direction },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Restore beam section local axis 1 direction action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::AssignBeamSectionLocalAxis1Direction(
                            local_axis_1_direction,
                            _old_line_numbers,
                            new_line_numbers,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.assign_beam_section_local_axis_1_direction_message_header: 
                                { 
                                    "local_axis_1_direction": local_axis_1_direction,
                                    "line_numbers": new_line_numbers, 
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update beam section orientation data action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        PropertiesActionType::AssignPropertiesToSurfaces(
                            properties_name,
                            _old_surface_numbers,
                            new_surface_numbers,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.assign_properties_to_surfaces_message_header: 
                                { 
                                    "name": properties_name, "surface_numbers": new_surface_numbers
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update assigned properties to surfaces \
                                    action: Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                    }
                },
                ActionType::LoadsActionType(loads_action_type) =>
                {
                    match loads_action_type
                    {
                        LoadsActionType::AddConcentratedLoad(
                            point_number,
                            concentrated_load,
                            is_action_id_should_be_increased) =>
                        {
                            let ConcentratedLoad { fx, fy, fz,
                                mx, my, mz } = concentrated_load;
                            
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.add_concentrated_load_message_header: 
                                { 
                                    "point_number": *point_number,
                                    "fx": *fx, "fy": *fy, "fz": *fz, "mx": *mx, "my": *my, "mz": *mz,
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Add concentrated load action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        LoadsActionType::UpdateConcentratedLoad(
                            point_number,
                            _old_concentrated_load,
                            new_concentrated_load,
                            is_action_id_should_be_increased) =>
                        {
                            let ConcentratedLoad { fx, fy, fz,
                                mx, my, mz } =
                                    new_concentrated_load;

                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.update_concentrated_load_message_header: 
                                { 
                                    "point_number": *point_number,
                                    "fx": *fx, "fy": *fy, "fz": *fz, "mx": *mx, "my": *my, "mz": *mz,
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update concentrated load action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        LoadsActionType::DeleteConcentratedLoad(
                            point_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.delete_concentrated_load_message_header: 
                                { "point_number": *point_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Delete concentrated load action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        LoadsActionType::RestoreConcentratedLoad(
                            point_number,
                        is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.restore_concentrated_load_message_header: 
                                { "point_number": *point_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Restore concentrated load action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        LoadsActionType::AddUniformlyDistributedLineLoad(
                            line_number,
                            uniformly_distributed_line_load,
                            is_action_id_should_be_increased) =>
                        {
                            let UniformlyDistributedLineLoad { qx, qy,
                                qz } = uniformly_distributed_line_load;

                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.add_uniformly_distributed_line_load_message_header: 
                                {
                                    "line_number": *line_number, "qx": *qx, "qy": *qy, "qz": *qz,
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Add uniformly distributed line load action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        LoadsActionType::UpdateUniformlyDistributedLineLoad(
                            line_number,
                            _old_uniformly_distributed_line_load,
                            new_uniformly_distributed_line_load,
                            is_action_id_should_be_increased) =>
                        {
                            let UniformlyDistributedLineLoad { qx, qy,
                                qz } = new_uniformly_distributed_line_load;

                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.update_uniformly_distributed_line_load_message_header: 
                                { 
                                    "line_number": *line_number, "qx": *qx, "qy": *qy, "qz": *qz,
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update uniformly distributed line load action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        LoadsActionType::DeleteUniformlyDistributedLineLoad(
                            line_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.delete_uniformly_distributed_line_load_message_header: 
                                { "line_number": *line_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Delete uniformly distributed line load action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        LoadsActionType::RestoreUniformlyDistributedLineLoad(
                            line_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.restore_uniformly_distributed_line_load_message_header: 
                                { "line_number": *line_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Restore uniformly distributed line load action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        LoadsActionType::AddUniformlyDistributedSurfaceLoad(
                            surface_number,
                            uniformly_distributed_surface_load,
                            is_action_id_should_be_increased) =>
                        {
                            let UniformlyDistributedSurfaceLoad { px, py,
                                pz } = uniformly_distributed_surface_load;

                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.add_uniformly_distributed_surface_load_message_header: 
                                {
                                    "surface_number": *surface_number, "px": *px, "py": *py, "pz": *pz,
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Add uniformly distributed surface load action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        LoadsActionType::UpdateUniformlyDistributedSurfaceLoad(
                            surface_number,
                            _old_uniformly_distributed_surface_load,
                            new_uniformly_distributed_surface_load,
                            is_action_id_should_be_increased) =>
                        {
                            let UniformlyDistributedSurfaceLoad { px, py,
                                pz } = new_uniformly_distributed_surface_load;

                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.update_uniformly_distributed_surface_load_message_header: 
                                { 
                                    "surface_number": *surface_number, "px": *px, "py": *py, "pz": *pz,
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update uniformly distributed surface load \
                                    action: Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        LoadsActionType::DeleteUniformlyDistributedSurfaceLoad(
                            surface_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.delete_uniformly_distributed_surface_load_message_header: 
                                { "surface_number": *surface_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Delete uniformly distributed surface load \
                                    action: Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        LoadsActionType::RestoreUniformlyDistributedSurfaceLoad(
                            surface_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.restore_uniformly_distributed_surface_load_message_header: 
                                { "surface_number": *surface_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Restore uniformly distributed surface load \
                                    action: Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                    }
                },
                ActionType::BoundaryConditionsActionType(boundary_conditions_action_type) =>
                {
                    match boundary_conditions_action_type
                    {
                        BoundaryConditionsActionType::AddPointBoundaryCondition(
                            point_number,
                            boundary_condition,
                            is_action_id_should_be_increased) =>
                        {
                            let PointBoundaryCondition { optional_ux,
                                optional_uy, optional_uz,
                                optional_rx, optional_ry,
                                optional_rz } = boundary_condition;

                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.add_point_boundary_condition_message_header: 
                                { 
                                    "point_number": *point_number,
                                    "optional_ux": *optional_ux, "optional_uy": *optional_uy,
                                    "optional_uz": *optional_uz, "optional_rx": *optional_rx,
                                    "optional_ry": *optional_ry, "optional_rz": *optional_rz,
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Add point boundary condition action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        BoundaryConditionsActionType::UpdatePointBoundaryCondition(
                            point_number,
                            _old_boundary_condition,
                            new_boundary_condition,
                            is_action_id_should_be_increased) =>
                        {
                            let PointBoundaryCondition { optional_ux,
                                optional_uy, optional_uz,
                                optional_rx, optional_ry,
                                optional_rz } = new_boundary_condition;

                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.update_point_boundary_condition_message_header: 
                                { 
                                    "point_number": *point_number,
                                    "optional_ux": *optional_ux, "optional_uy": *optional_uy,
                                    "optional_uz": *optional_uz, "optional_rx": *optional_rx,
                                    "optional_ry": *optional_ry, "optional_rz": *optional_rz,
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update point boundary condition action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        BoundaryConditionsActionType::DeletePointBoundaryCondition(
                            point_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.delete_point_boundary_condition_message_header: 
                                { "point_number": *point_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Delete point boundary condition action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        BoundaryConditionsActionType::RestorePointBoundaryCondition(
                            point_number,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.restore_point_boundary_condition_message_header: 
                                { "point_number": *point_number },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Restore point boundary condition action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                    }
                },
                ActionType::MeshSeedActionType(mesh_seed_action_type) =>
                {
                    match mesh_seed_action_type
                    {
                        MeshSeedActionType::UpdateGlobalMeshSeed(_old_global_mesh_seed_value, new_global_mesh_seed_value,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.update_global_mesh_seed_message_header: 
                                { 
                                    "global_mesh_seed_value": *new_global_mesh_seed_value,
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update global mesh seed action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                                
                        },
                        MeshSeedActionType::UpdateLinesMeshSeed(lines_mesh_seed_value, line_numbers,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.update_lines_mesh_seed_message_header: 
                                { 
                                    "lines_mesh_seed_value": *lines_mesh_seed_value,
                                    "line_numbers": line_numbers,
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update lines mesh seed action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        MeshSeedActionType::UndoLinesMeshSeedUpdate(line_numbers, 
                            is_action_id_should_be_increased) => 
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.undo_lines_mesh_seed_update_message_header: 
                                { 
                                    "line_numbers": line_numbers,
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Undo lines mesh seed update action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        MeshSeedActionType::UpdateSurfacesMeshSeed(edges_1_3_mesh_seed_value, 
                            edges_2_4_mesh_seed_value, surface_numbers,
                            is_action_id_should_be_increased) =>
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.update_surfaces_mesh_seed_message_header: 
                                { 
                                    "surfaces_edges_1_3_mesh_seed_value": *edges_1_3_mesh_seed_value,
                                    "surfaces_edges_2_4_mesh_seed_value": *edges_2_4_mesh_seed_value,
                                    "surface_numbers": surface_numbers,
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Update surfaces mesh seed action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        },
                        MeshSeedActionType::UndoSurfacesMeshSeedUpdate(surface_numbers, 
                            is_action_id_should_be_increased) => 
                        {
                            let uid = uid_generator(action.to_owned(), *add_to_active_actions);
                            let result = json!({ &self.props.undo_surfaces_mesh_seed_update_message_header: 
                                { 
                                    "surface_numbers": surface_numbers,
                                },
                                "action_id": *ref_action_id,
                                "is_action_id_should_be_increased": *is_action_id_should_be_increased,
                                "uid": uid 
                            });
                            let serialized_result = result.serialize(&serializer)
                                .map_err(|_e| "Actions router: Undo surfaces mesh seed update action: \
                                    Data could not be serialized!")?;
                            action_handling_result = serialized_result;
                        }  
                    }
                }

            }
            self.current_action = None;
        }
        Ok(action_handling_result)
    }


    pub fn handle_message(&mut self, message: JsValue) -> Result<JsValue, JsValue>
    {
        let serialized_message: Value = serde_wasm_bindgen::from_value(message).or(Err(JsValue::from(
            "Actions router: Message could not be serialized!")))?;
        let mut is_redo = false;
        if let Some(point_data) = serialized_message.get(&self.props.add_point_message_header)
        {
            self.handle_add_point_message(point_data)?;
        }
        else if let Some(point_data) = serialized_message.get(&self.props.update_point_message_header)
        {
            self.handle_update_point_message(point_data)?;
        }
        else if let Some(point_data) = serialized_message.get(&self.props.delete_point_message_header)
        {
            self.handle_delete_point_message(point_data)?;
        }
        else if let Some(line_data) = serialized_message.get(&self.props.add_line_message_header)
        {
            self.handle_add_line_message(line_data)?;
        }
        else if let Some(line_data) = serialized_message.get(&self.props.update_line_message_header)
        {
            self.handle_update_line_message(line_data)?;
        }
        else if let Some(line_data) = serialized_message.get(&self.props.delete_line_message_header)
        {
            self.handle_delete_line_message(line_data)?;
        }
        else if let Some(surface_data) = serialized_message.get(&self.props.add_surface_message_header)
        {
            self.handle_add_surface_message(surface_data)?;
        }
        else if let Some(surface_data) = serialized_message.get(&self.props.update_surface_message_header)
        {
            self.handle_update_surface_message(surface_data)?;
        }
        else if let Some(surface_data) = serialized_message
            .get(&self.props.rotate_surface_vertices_clockwise_message_header)
        {
            self.handle_rotate_surface_vertices_clockwise_message(surface_data)?;
        }
        else if let Some(surface_data) = serialized_message
            .get(&self.props.rotate_surface_vertices_counter_clockwise_message_header)
        {
            self.handle_rotate_surface_vertices_counter_clockwise_message(surface_data)?;
        }
        else if let Some(surface_data) = serialized_message
            .get(&self.props.flip_surface_normal_axis_message_header)
        {
            self.handle_flip_surface_normal_axis_message(surface_data)?;
        }
        else if let Some(surface_data) = serialized_message.get(&self.props.delete_surface_message_header)
        {
            self.handle_delete_surface_message(surface_data)?;
        }
        else if let Some(material_data) = serialized_message.get(&self.props.add_material_message_header)
        {
            self.handle_add_material_message(material_data)?;
        }
        else if let Some(material_data) = serialized_message.get(&self.props.update_material_message_header)
        {
            self.handle_update_material_message(material_data)?;
        }
        else if let Some(material_data) = serialized_message.get(&self.props.delete_material_message_header)
        {
            self.handle_delete_material_message(material_data)?;
        }
        else if let Some(truss_section_data) = serialized_message.get(&self.props.add_truss_section_message_header)
        {
            self.handle_add_truss_section_message(truss_section_data)?;
        }
        else if let Some(truss_section_data) = serialized_message.get(&self.props.update_truss_section_message_header)
        {
            self.handle_update_truss_section_message(truss_section_data)?;
        }
        else if let Some(truss_section_data) = serialized_message.get(&self.props.delete_truss_section_message_header)
        {
            self.handle_delete_truss_section_message(truss_section_data)?;
        }
        else if let Some(beam_section_data) = serialized_message.get(&self.props.add_beam_section_message_header)
        {
            self.handle_add_beam_section_message(beam_section_data)?;
        }
        else if let Some(beam_section_data) = serialized_message.get(&self.props.update_beam_section_message_header)
        {
            self.handle_update_beam_section_message(beam_section_data)?;
        }
        else if let Some(beam_section_data) = serialized_message.get(&self.props.delete_beam_section_message_header)
        {
            self.handle_delete_beam_section_message(beam_section_data)?;
        }
        else if let Some(plate_section_data) = serialized_message.get(&self.props.add_plate_section_message_header)
        {
            self.handle_add_plate_section_message(plate_section_data)?;
        }
        else if let Some(plate_section_data) = serialized_message.get(&self.props.update_plate_section_message_header)
        {
            self.handle_update_plate_section_message(plate_section_data)?;
        }
        else if let Some(plate_section_data) = serialized_message.get(&self.props.delete_plate_section_message_header)
        {
            self.handle_delete_plate_section_message(plate_section_data)?;
        }
        else if let Some(properties_data) = serialized_message.get(&self.props.add_properties_message_header)
        {
            self.handle_add_properties_message(properties_data)?;
        }
        else if let Some(properties_data) = serialized_message.get(&self.props.update_properties_message_header)
        {
            self.handle_update_properties_message(properties_data)?;
        }
        else if let Some(properties_data) = serialized_message.get(&self.props.delete_properties_message_header)
        {
            self.handle_delete_properties_message(properties_data)?;
        }
        else if let Some(assigned_properties_to_lines_data) = serialized_message
            .get(&self.props.assign_properties_to_lines_message_header)
        {
            self.handle_assign_properties_to_lines_message(assigned_properties_to_lines_data)?;
        }
        else if let Some(local_axis_1_direction_data) = serialized_message
            .get(&self.props.add_beam_section_local_axis_1_direction_message_header)
        {
            self.handle_add_beam_section_local_axis_1_direction_message(
                local_axis_1_direction_data)?;
        }
        else if let Some(local_axis_1_direction_data) = serialized_message
            .get(&self.props.delete_beam_section_local_axis_1_direction_message_header)
        {
            self.handle_delete_beam_section_local_axis_1_direction_message(
                local_axis_1_direction_data)?;
        }
        else if let Some(beam_section_orientation_data) = serialized_message
            .get(&self.props.assign_beam_section_local_axis_1_direction_message_header)
        {
            self.handle_assign_beam_section_local_axis_1_direction_message(
                beam_section_orientation_data)?;
        }
        else if let Some(assigned_properties_to_surfaces_data) = serialized_message
            .get(&self.props.assign_properties_to_surfaces_message_header)
        {
            self.handle_assign_properties_to_surfaces_message(assigned_properties_to_surfaces_data)?;
        }
        else if let Some(concentrated_load_data) = serialized_message
            .get(&self.props.add_concentrated_load_message_header)
        {
            self.handle_add_concentrated_load_message(concentrated_load_data)?;
        }
        else if let Some(concentrated_load_data) = serialized_message
            .get(&self.props.update_concentrated_load_message_header)
        {
            self.handle_update_concentrated_load_message(concentrated_load_data)?;
        }
        else if let Some(concentrated_load_data) = serialized_message
            .get(&self.props.delete_concentrated_load_message_header)
        {
            self.handle_delete_concentrated_load_message(concentrated_load_data)?;
        }
        else if let Some(uniformly_distributed_line_load_data) = serialized_message
            .get(&self.props.add_uniformly_distributed_line_load_message_header)
        {
            self.handle_add_uniformly_distributed_line_load_message(uniformly_distributed_line_load_data)?;
        }
        else if let Some(uniformly_distributed_line_load_data) = serialized_message
            .get(&self.props.update_uniformly_distributed_line_load_message_header)
        {
            self.handle_update_uniformly_distributed_line_load_message(uniformly_distributed_line_load_data)?;
        }
        else if let Some(uniformly_distributed_line_load_data) = serialized_message
            .get(&self.props.delete_uniformly_distributed_line_load_message_header)
        {
            self.handle_delete_uniformly_distributed_line_load_message(uniformly_distributed_line_load_data)?;
        }
        else if let Some(uniformly_distributed_surface_load_data) = serialized_message
            .get(&self.props.add_uniformly_distributed_surface_load_message_header)
        {
            self.handle_add_uniformly_distributed_surface_load_message(uniformly_distributed_surface_load_data)?;
        }
        else if let Some(uniformly_distributed_surface_load_data) = serialized_message
            .get(&self.props.update_uniformly_distributed_surface_load_message_header)
        {
            self.handle_update_uniformly_distributed_surface_load_message(uniformly_distributed_surface_load_data)?;
        }
        else if let Some(uniformly_distributed_surface_load_data) = serialized_message
            .get(&self.props.delete_uniformly_distributed_surface_load_message_header)
        {
            self.handle_delete_uniformly_distributed_surface_load_message(uniformly_distributed_surface_load_data)?;
        }
        else if let Some(point_boundary_condition_data) = serialized_message
            .get(&self.props.add_point_boundary_condition_message_header)
        {
            self.handle_add_point_boundary_condition_message(point_boundary_condition_data)?;
        }
        else if let Some(point_boundary_condition_data) = serialized_message
            .get(&self.props.update_point_boundary_condition_message_header)
        {
            self.handle_update_point_boundary_condition_message(point_boundary_condition_data)?;
        }
        else if let Some(point_boundary_condition_data) = serialized_message
            .get(&self.props.delete_point_boundary_condition_message_header)
        {
            self.handle_delete_point_boundary_condition_message(point_boundary_condition_data)?;
        }
        else if let Some(global_mesh_seed_data) = serialized_message.get(&self.props.update_global_mesh_seed_message_header)
        {
            self.handle_update_global_mesh_seed_message(global_mesh_seed_data)?;
        }
        else if let Some(lines_mesh_seed_data) = serialized_message.get(&self.props.update_lines_mesh_seed_message_header)
        {
            self.handle_update_lines_mesh_seed_message(lines_mesh_seed_data)?;
        }
        else if let Some(surfaces_mesh_seed_data) = serialized_message.get(&self.props.update_surfaces_mesh_seed_message_header)
        {
            self.handle_update_surfaces_mesh_seed_message(surfaces_mesh_seed_data)?;
        }
        else if let Some(undo_data) = serialized_message.get(&self.props.undo_message_header)
        {
            self.handle_undo_message(undo_data)?;
        }
        else if let Some(redo_data) = serialized_message.get(&self.props.redo_message_header)
        {
            self.handle_redo_message(redo_data)?;
            is_redo = true;
        }
        else
        {
            let error_message = "Actions router: Message could not be handled!";
            return Err(JsValue::from(error_message));
        }
        let action_handling_result = self.handle_current_action(is_redo)?;

        log("Actions router: Actions in pool: \n");
        for (uid, action_in_pool) in self.action_pool.iter()
        {
            log(&format!("\tUid: {:?}, Action: {:?} \n", uid, action_in_pool));
        }
        log(&format!("Actions router: The number of actions in pool: {}",
            self.action_pool.len()));

        Ok(action_handling_result)
    }


    pub fn approve_action(&mut self, uid: u32) -> Promise
    {
        if let Some(action_in_pool) = self.action_pool.remove(&uid)
        {
            let ActionInPool { action, add_to_active_actions, is_redo} = action_in_pool;

            if add_to_active_actions && !is_redo
            {
                self.undo_actions.clear();
            }

            if add_to_active_actions
            {
                self.active_actions.push(action.clone());
            }

            let active_actions = self.active_actions.clone();
            let undo_actions = self.undo_actions.clone();
            let action_pool_length = self.action_pool.len();

            future_to_promise(async move {
                log("Actions router active actions: \n");
                for action in active_actions.iter()
                {
                    let action_id = &action.ref_action_id();
                    let action_type = &action.ref_action_type();
                    log(&format!("\tAction id: {:?}, action type: {:?} \n", action_id, action_type));
                }
                log(&format!("Actions router: The number of active actions: {}",
                    active_actions.len()));

                log("Actions router undo actions: \n");
                for action in undo_actions.iter()
                {
                    let action_id = &action.ref_action_id();
                    let action_type = &action.ref_action_type();
                    log(&format!("\tAction id: {:?}, action type: {:?} \n", action_id, action_type));
                }
                log(&format!("Actions router: The number of undo actions: {}",
                    undo_actions.len()));

                log(&format!("Actions router: The number of actions in pool: {}",
                    action_pool_length));

                Ok(JsValue::null())
            })
        }
        else
        {
            let error_message = format!("Actions router: Approve action: There are \
                no action {} in action pool!", uid);
            future_to_promise(async move {
                
                Err(JsValue::from(error_message))
            })
        }
    }


    pub fn discard_action(&mut self, uid: u32) -> Result<(), JsValue>
    {
        if self.action_pool.remove(&uid).is_some()
        {
            log(&format!("Actions router: The number of actions in pool: {}",
                self.action_pool.len()));

            Ok(())
        }
        else
        {
            let error_message = &format!("Actions router: Discard action: There are 
                no action {} in action pool!", uid);
            Err(JsValue::from(error_message))
        }
    }


    pub fn extract_active_actions(&self) -> Result<JsValue, JsValue>
    {
        let active_actions = Array::new();
        for action in self.active_actions.iter()
        {
            match action.convert_to_message(&self.props)
            {
                Ok(action) => { let _ = active_actions.push(&action); },
                Err(e) => return Err(e),
            }
        }

        Ok(active_actions.into())
    }


    pub fn reset(&mut self) {
        self.current_action = None;
        self.action_pool = HashMap::new();
        self.active_actions = Vec::new();
        self.undo_actions = Vec::new();
    }
}
