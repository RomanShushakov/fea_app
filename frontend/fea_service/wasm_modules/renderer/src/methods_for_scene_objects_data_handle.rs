use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use serde_json::Value;

use std::collections::{HashMap, HashSet};

use crate::enums::{LineObjectType, SurfaceObjectType, MeshSeedType, SceneState};
use crate::methods_for_converting_nodes_data::
{
   convert_nodes_data, get_converted_extreme_global_analysis_values, update_nodes_data, 
};
use crate::methods_for_converting_elements_data::
{
    convert_truss_elements_data, update_truss_elements_data, get_converted_extreme_elements_loads_values,
    convert_beam_elements_data, update_beam_elements_data, convert_plate_elements_data, update_plate_elements_data,
};
use crate::functions::transform_u32_to_array_of_u8;
use crate::Renderer;


#[wasm_bindgen]
impl Renderer
{
    pub fn activate_preprocessor_state(&mut self)
    {
        self.scene.set_scene_state(SceneState::Preprocessor);
    }


    pub fn add_point(&mut self, number: u32, uid: u32, x: f32, y: f32, z: f32) -> Result<(), JsValue>
    {
        self.scene.add_point(number, uid, x, y, z, &self.props)
    }


    pub fn update_point(&mut self, number: u32, uid: u32, x: f32, y: f32, z: f32) -> Result<(), JsValue>
    {
        self.scene.update_point(number, uid, x, y, z, &self.props)
    }


    pub fn delete_point(&mut self, number: u32, uid: u32) -> Result<(), JsValue>
    {
        self.scene.delete_point(number, uid)
    }


    pub fn add_line(
        &mut self, 
        number: u32, 
        uid: u32, 
        point_1_number: u32, 
        point_2_number: u32, 
        optional_transformed_local_axis_1_direction: Option<Box<[f32]>>, 
        line_object_type: LineObjectType,
        line_mesh_seed_value: Option<u8>, 
        mesh_seed_type: MeshSeedType, 
        optional_uniformly_distributed_line_load_uid: Option<u32>, 
        optional_uniformly_distributed_line_load_components: Option<Box<[f32]>>,
    ) 
        -> Result<(), JsValue>
    {
        self.scene.add_line(
            number, 
            uid, 
            point_1_number, 
            point_2_number,
            optional_transformed_local_axis_1_direction, 
            line_object_type, line_mesh_seed_value, mesh_seed_type,
            optional_uniformly_distributed_line_load_uid, 
            optional_uniformly_distributed_line_load_components, 
            &self.props,
        )
    }


    pub fn update_line(
        &mut self, 
        number: u32, 
        uid: u32, 
        point_1_number: u32, 
        point_2_number: u32,
        optional_transformed_local_axis_1_direction: Option<Box<[f32]>>, 
        line_object_type: LineObjectType,
        line_mesh_seed_value: Option<u8>, 
        mesh_seed_type: MeshSeedType, 
        optional_uniformly_distributed_line_load_uid: Option<u32>,
        optional_uniformly_distributed_line_load_components: Option<Box<[f32]>>,
    ) 
        -> Result<(), JsValue>
    {
        self.scene.update_line(
            number, uid, 
            point_1_number, 
            point_2_number, 
            optional_transformed_local_axis_1_direction, 
            line_object_type, 
            line_mesh_seed_value, 
            mesh_seed_type,
            optional_uniformly_distributed_line_load_uid, 
            optional_uniformly_distributed_line_load_components, 
            &self.props,
        )
    }


    pub fn delete_line(&mut self, number: u32, uid: u32) -> Result<(), JsValue>
    {
        self.scene.delete_line(number, uid)
    }


    pub fn add_surface(
        &mut self, 
        number: u32, 
        uid: u32, 
        point_1_number: u32, 
        point_2_number: u32, 
        point_3_number: u32, 
        point_4_number: u32, 
        normal: &[f32], 
        surface_object_type: SurfaceObjectType, 
        surface_mesh_seed_values: Option<Box<[u8]>>, 
        mesh_seed_type: MeshSeedType,
        optional_uniformly_distributed_surface_load_uid: Option<u32>, 
        optional_uniformly_distributed_surface_load_components: Option<Box<[f32]>>,
    )
        -> Result<(), JsValue>
    {
        self.scene.add_surface(
            number, 
            uid, 
            point_1_number, 
            point_2_number, 
            point_3_number, 
            point_4_number, normal,
            surface_object_type, 
            surface_mesh_seed_values, 
            mesh_seed_type,
            optional_uniformly_distributed_surface_load_uid, 
            optional_uniformly_distributed_surface_load_components,
            &self.props,
        )
    }


    pub fn update_surface(
        &mut self, 
        number: u32, 
        uid: u32, 
        point_1_number: u32, 
        point_2_number: u32, 
        point_3_number: u32, 
        point_4_number: u32, 
        normal: &[f32], 
        surface_object_type: SurfaceObjectType,
        surface_mesh_seed_values: Option<Box<[u8]>>, 
        mesh_seed_type: MeshSeedType,
        optional_uniformly_distributed_surface_load_uid: Option<u32>, 
        optional_uniformly_distributed_surface_load_components: Option<Box<[f32]>>,
    ) 
        -> Result<(), JsValue>
    {
        self.scene.update_surface(
            number, 
            uid, 
            point_1_number, 
            point_2_number, 
            point_3_number, 
            point_4_number, normal,
            surface_object_type, 
            surface_mesh_seed_values, 
            mesh_seed_type,
            optional_uniformly_distributed_surface_load_uid, 
            optional_uniformly_distributed_surface_load_components,
            &self.props,
        )
    }


    pub fn delete_surface(&mut self, number: u32, uid: u32) -> Result<(), JsValue>
    {
        self.scene.delete_surface(number, uid)
    }


    pub fn add_concentrated_load(
        &mut self, point_number: u32, uid: u32, fx: f32, fy: f32, fz: f32, mx: f32, my: f32, mz: f32,
    ) 
        -> Result<(), JsValue>
    {
        self.scene.add_concentrated_load(point_number, uid, fx, fy, fz, mx, my, mz, &self.props)
    }


    pub fn update_concentrated_load(
        &mut self, point_number: u32, uid: u32, fx: f32, fy: f32, fz: f32, mx: f32, my: f32, mz: f32,
    ) 
        -> Result<(), JsValue>
    {
        self.scene.update_concentrated_load(point_number, uid, fx, fy, fz, mx, my, mz, &self.props)
    }


    pub fn delete_concentrated_load(&mut self, point_number: u32, uid: u32) -> Result<(), JsValue>
    {
        self.scene.delete_concentrated_load(point_number, uid)
    }


    pub fn add_point_boundary_condition(
        &mut self, 
        point_number: u32, 
        uid: u32, 
        optional_ux: Option<f32>, 
        optional_uy: Option<f32>, 
        optional_uz: Option<f32>,
        optional_rx: Option<f32>, 
        optional_ry: Option<f32>,
        optional_rz: Option<f32>,
    ) 
        -> Result<(), JsValue>
    {
        self.scene.add_point_boundary_condition(
            point_number, 
            uid, 
            optional_ux, 
            optional_uy, 
            optional_uz, 
            optional_rx, 
            optional_ry, 
            optional_rz, 
            &self.props,
        )
    }


    pub fn update_point_boundary_condition(
        &mut self, 
        point_number: u32,
        uid: u32, 
        optional_ux: Option<f32>, 
        optional_uy: Option<f32>, 
        optional_uz: Option<f32>, 
        optional_rx: Option<f32>, 
        optional_ry: Option<f32>,
        optional_rz: Option<f32>,
    ) 
        -> Result<(), JsValue>
    {
        self.scene.update_point_boundary_condition(
            point_number, 
            uid, 
            optional_ux, 
            optional_uy, 
            optional_uz, 
            optional_rx,
            optional_ry, 
            optional_rz, 
            &self.props,
        )
    }


    pub fn delete_point_boundary_condition(&mut self, point_number: u32, uid: u32) -> Result<(), JsValue>
    {
        self.scene.delete_point_boundary_condition(point_number, uid)
    }


    pub fn activate_postprocessor_state(&mut self, job: JsValue) -> Result<(), JsValue>
    {
        let serialized_data: Value = serde_wasm_bindgen::from_value(job)
            .or(Err(JsValue::from("Renderer: Job data could not be serialized!")))?;

        let mesh_data = serialized_data
            .get("mesh")
            .ok_or(JsValue::from("Renderer: Mesh data could not be extracted!"))?
            .as_object()
            .ok_or(JsValue::from("Renderer: Mesh data could not be converted into object!"))?;

        let global_analysis_result_data = serialized_data
            .get("global_analysis_result")
            .ok_or(JsValue::from("Renderer: Global analysis result data could not be extracted!"))?
            .as_object()
            .ok_or(JsValue::from("Renderer: Global analysis result data could not be converted into object!"))?;

        let extreme_global_displacements_data = serialized_data
            .get("extreme_global_displacements")
            .ok_or(JsValue::from("Renderer: Extreme global displacements data could not be extracted!"))?
            .as_object()
            .ok_or(JsValue::from("Renderer: Extreme global displacements data could not be converted into object!"))?;
        let converted_extreme_global_displacements_data = 
            get_converted_extreme_global_analysis_values(extreme_global_displacements_data, true)?;

        let extreme_global_loads_data = serialized_data
            .get("extreme_global_loads")
            .ok_or(JsValue::from("Renderer: Extreme global loads data could not be extracted!"))?
            .as_object()
            .ok_or(JsValue::from("Renderer: Extreme global loads data could not be converted into object!"))?;
        let converted_extreme_global_loads_data = 
            get_converted_extreme_global_analysis_values(extreme_global_loads_data, false)?;

        let elements_analysis_result_data = serialized_data
            .get("elements_analysis_result")
            .ok_or(JsValue::from("Renderer: Elements analysis result data could not be extracted!"))?
            .as_object()
            .ok_or(JsValue::from("Renderer: Elements analysis result data could not be converted into object!"))?;

        let extreme_elements_loads_data = serialized_data
            .get("extreme_elements_loads")
            .ok_or(JsValue::from("Renderer: Extreme elements loads data could not be extracted!"))?
            .as_object()
            .ok_or(JsValue::from("Renderer: Extreme elements loads data could not be converted into object!"))?;
        let converted_extreme_elements_loads_data = 
            get_converted_extreme_elements_loads_values(extreme_elements_loads_data)?;

        let nodes_data = mesh_data
            .get("nodes")
            .ok_or(JsValue::from("Renderer: Nodes data could not be extracted!"))?
            .as_object()
            .ok_or(JsValue::from("Renderer: Nodes data could not be converted into object!"))?;

        let mut converted_nodes_data = HashMap::new();
        convert_nodes_data(nodes_data, &mut converted_nodes_data)?;

        update_nodes_data(
            global_analysis_result_data, 
            &mut converted_nodes_data, 
            &converted_extreme_global_displacements_data,
            &converted_extreme_global_loads_data,
            &self.props,
        )?;

        let truss_elements_data = mesh_data
            .get("truss_elements")
            .ok_or(JsValue::from("Renderer: Truss elements data could not be extracted!"))?
            .as_object()
            .ok_or(JsValue::from("Renderer: Truss elements data could not be converted into object!"))?;

        let mut converted_truss_elements_data = HashMap::new();
        convert_truss_elements_data(truss_elements_data, &converted_nodes_data, &mut converted_truss_elements_data)?;

        let truss_elements_analysis_result_data = elements_analysis_result_data
            .get("truss_elements")
            .ok_or(JsValue::from("Renderer: Truss elements analysis result data could not be extracted!"))?
            .as_object()
            .ok_or(JsValue::from("Renderer: Truss elements analysis result data could not be converted into object!"))?;
        update_truss_elements_data(
            truss_elements_analysis_result_data, 
            &mut converted_truss_elements_data, 
            &converted_extreme_elements_loads_data,
            &self.props,
        )?;

        let beam_elements_data = mesh_data
            .get("beam_elements")
            .ok_or(JsValue::from("Renderer: Beam elements data could not be extracted!"))?
            .as_object()
            .ok_or(JsValue::from("Renderer: Beam elements data could not be converted into object!"))?;

        let mut converted_beam_elements_data = HashMap::new();
        convert_beam_elements_data(beam_elements_data, &converted_nodes_data, &mut converted_beam_elements_data)?;

        let beam_elements_analysis_result_data = elements_analysis_result_data
            .get("beam_elements")
            .ok_or(JsValue::from("Renderer: Beam elements analysis result data could not be extracted!"))?
            .as_object()
            .ok_or(JsValue::from("Renderer: Beam elements analysis result data could not be converted into object!"))?;
        update_beam_elements_data(
            beam_elements_analysis_result_data, 
            &mut converted_beam_elements_data, 
            &converted_extreme_elements_loads_data,
            &self.props,
        )?;

        let plate_elements_data = mesh_data
            .get("plate_elements")
            .ok_or(JsValue::from("Renderer: Plate elements data could not be extracted!"))?
            .as_object()
            .ok_or(JsValue::from("Renderer: Plate elements data could not be converted into object!"))?;

        let mut converted_plate_elements_data = HashMap::new();
        convert_plate_elements_data(plate_elements_data, &converted_nodes_data, &mut converted_plate_elements_data)?;

        let plate_elements_analysis_result_data = elements_analysis_result_data
            .get("plate_elements")
            .ok_or(JsValue::from("Renderer: Plate elements analysis result data could not be extracted!"))?
            .as_object()
            .ok_or(JsValue::from("Renderer: Plate elements analysis result data could not be converted into object!"))?;
        update_plate_elements_data(
            plate_elements_analysis_result_data, 
            &mut converted_plate_elements_data, 
            &converted_extreme_elements_loads_data,
            &self.props,
        )?;

        self.scene.set_scene_state(
            SceneState::Postprocessor((
                None, 
                converted_extreme_global_displacements_data,
                converted_extreme_global_loads_data,
                converted_extreme_elements_loads_data,
            )),
        );

        self.scene.add_nodes(converted_nodes_data, &self.props)?;
        self.scene.add_truss_elements(converted_truss_elements_data, &self.props)?;
        self.scene.add_beam_elements(converted_beam_elements_data, &self.props)?;
        self.scene.add_plate_elements(converted_plate_elements_data, &self.props)?;

        Ok(())
    }


    pub fn select_objects(&mut self, drop_selection: &js_sys::Function) -> Result<(), JsValue>
    {
        let selected_colors = self.manipulation.under_selection_box_colors
            .chunks(4)
            .map(|chunk| <[u8; 4]>::try_from(chunk).unwrap())
            .collect::<HashSet<[u8; 4]>>();

        self.scene.select_objects(&selected_colors, drop_selection, true, &self.props)?;

        Ok(())
    }


    pub fn preview_selected_objects(&mut self, uids: &[u32], drop_selection: &js_sys::Function) -> Result<(), JsValue>
    {
        let selected_colors = uids.into_iter()
            .map(|uid| transform_u32_to_array_of_u8(*uid))
            .collect::<HashSet<[u8; 4]>>();
        self.scene.select_objects(&selected_colors, drop_selection, false, &self.props)?;
        Ok(())
    }
}
