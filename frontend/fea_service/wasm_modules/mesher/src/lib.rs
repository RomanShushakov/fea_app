use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use serde_json::Value;
use serde_json::json;
use serde_wasm_bindgen::Serializer;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;

mod types;
use types::FEFloat;

mod props;
use props::Props;

mod methods_for_nodes_data_handle;
use methods_for_nodes_data_handle::extract_points_from_input_data;

mod methods_for_edges_data_handle;

mod methods_for_truss_elements_data_handle;
use methods_for_truss_elements_data_handle::extract_trusses_from_input_data;

mod methods_for_beam_elements_data_handle;
use methods_for_beam_elements_data_handle::extract_beams_from_input_data;

mod methods_for_plate_elements_data_handle;
use methods_for_plate_elements_data_handle::extract_plates_from_input_data;

mod methods_for_point_boundary_conditions_data_handle;
use methods_for_point_boundary_conditions_data_handle::extract_point_boundary_conditions_from_input_data;

mod methods_for_concentrated_loads_data_handle;
use methods_for_concentrated_loads_data_handle::extract_concentrated_loads_from_input_data;

mod methods_for_uniformly_distributed_loads_on_beams_data_handle;
use methods_for_uniformly_distributed_loads_on_beams_data_handle::
    extract_uniformly_distributed_loads_on_beams_from_input_data;

mod methods_for_uniformly_distributed_loads_on_plates_data_handle;
use methods_for_uniformly_distributed_loads_on_plates_data_handle::
    extract_uniformly_distributed_loads_on_plates_from_input_data;

mod functions;

mod enums;


#[wasm_bindgen]
pub struct Mesher 
{
    props: Props,
}


#[wasm_bindgen]
impl Mesher
{
    pub fn create(
        abs_tol: FEFloat,
        rel_tol: FEFloat,
        max_point_number: u32,
        max_line_number: u32,
        max_surface_number: u32,
        truss_elements_group_number: u32,
        beam_elements_group_number: u32,
        plate_elements_group_number: u32,
    ) 
        -> Self
    {
        let props = Props::create(
            abs_tol,
            rel_tol,
            max_point_number,
            max_line_number,
            max_surface_number,
            truss_elements_group_number,
            beam_elements_group_number,
            plate_elements_group_number,
        );
        Mesher { props }
    }


    pub fn generate_mesh(&mut self, data_for_mesher: JsValue) -> Result<JsValue, JsValue>
    {
        let mut uids = HashSet::new();

        let serialized_data: Value = serde_wasm_bindgen::from_value(data_for_mesher).or(Err(JsValue::from(
            "Mesher: Data could not be serialized!")))?;

        let extracted_points_data = serialized_data
            .get("points")
            .ok_or(JsValue::from("Mesher: Points data could not be extracted!"))?;
        let mut nodes = HashMap::new();
        extract_points_from_input_data(extracted_points_data, &mut nodes, &mut uids)?;

        let extracted_trusses_data = serialized_data
            .get("trusses")
            .ok_or(JsValue::from("Mesher: Trusses data could not be extracted!"))?;
        let mut edges = HashMap::new();
        let mut truss_elements = HashMap::new();
        extract_trusses_from_input_data(
            extracted_trusses_data, &mut nodes, &mut edges, &mut truss_elements, &self.props, &mut uids,
        )?;

        let extracted_beams_data = serialized_data
            .get("beams")
            .ok_or(JsValue::from("Mesher: Beams data could not be extracted!"))?;
        let mut lines = HashMap::new();
        let mut beam_elements = HashMap::new();
        extract_beams_from_input_data(
            extracted_beams_data, &mut nodes, &mut edges, &mut beam_elements, &mut lines, &self.props, &mut uids,
        )?;

        let extracted_plates_data = serialized_data
            .get("plates")
            .ok_or(JsValue::from("Mesher: Plates data could not be extracted!"))?;
        let mut surfaces = HashMap::new();
        let mut plate_elements = HashMap::new();
        extract_plates_from_input_data(
            extracted_plates_data, &mut nodes, &mut edges, &mut plate_elements, &mut surfaces, &self.props, &mut uids,
        )?;

        let extracted_point_boundary_conditions_data = serialized_data
            .get("point_boundary_conditions")
            .ok_or(JsValue::from("Mesher: Point boundary conditions data could not be extracted!"))?;
        let mut boundary_conditions = HashMap::new();
        extract_point_boundary_conditions_from_input_data(extracted_point_boundary_conditions_data, &nodes, 
            &mut boundary_conditions)?;

        let extracted_concentrated_loads_data = serialized_data
            .get("concentrated_loads")
            .ok_or(JsValue::from("Mesher: Concentrated loads data could not be extracted!"))?;
        extract_concentrated_loads_from_input_data(extracted_concentrated_loads_data, &nodes, 
            &mut boundary_conditions)?;

        let extracted_uniformly_distributed_line_loads_data = serialized_data
            .get("uniformly_distributed_line_loads")
            .ok_or(JsValue::from("Mesher: Uniformly distributed line loads data could not be extracted!"))?;
        let mut uniformly_distributed_loads_on_beams = HashMap::new();
        extract_uniformly_distributed_loads_on_beams_from_input_data(extracted_uniformly_distributed_line_loads_data, 
            &lines, &beam_elements, &mut uniformly_distributed_loads_on_beams)?;

        let extracted_uniformly_distributed_surface_loads_data = serialized_data
            .get("uniformly_distributed_surface_loads")
            .ok_or(JsValue::from("Mesher: Uniformly distributed surface loads data could not be extracted!"))?;
        let mut uniformly_distributed_loads_on_plates = HashMap::new();
        extract_uniformly_distributed_loads_on_plates_from_input_data(extracted_uniformly_distributed_surface_loads_data, 
            &surfaces, &plate_elements, &mut uniformly_distributed_loads_on_plates)?;

        let mesh_data = json!({ 
            "nodes": nodes,
            "truss_elements": truss_elements,
            "beam_elements": beam_elements,
            "plate_elements": plate_elements,
            "boundary_conditions": boundary_conditions,
            "uniformly_distributed_loads_on_beams": uniformly_distributed_loads_on_beams,
            "uniformly_distributed_loads_on_plates": uniformly_distributed_loads_on_plates,
            "statistics": {
                "nodes_number": nodes.len(),
                "truss_elements_number": truss_elements.len(),
                "beam_elements_number": beam_elements.len(),
                "plate_elements_number": plate_elements.len(),
            }
        });

        let serializer = Serializer::json_compatible();
        let mesh = mesh_data
            .serialize(&serializer)
            .or(Err(JsValue::from("Mesher: Mesh could not be composed for extraction!")))?;

        Ok(mesh)
    }
}
