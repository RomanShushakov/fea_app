use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::JsValue;

use finite_element_method::FEM;

use crate::types::FEFloat;

pub fn extract_nodes_from_input_data(
    extracted_nodes_data: &Value,
    fem: &mut FEM<FEFloat>,
    node_number_node_name_map: &mut HashMap<u32, String>,
    node_name_node_number_map: &mut HashMap<String, u32>,
) -> Result<(), JsValue> {
    let nodes_data = extracted_nodes_data.as_object().ok_or(JsValue::from(
        "Solver: Nodes data could not be converted into object!",
    ))?;
    let mut nodes_count = 0;

    for (node_name, node_data) in nodes_data.iter() {
        let node_data_array = node_data.as_array().ok_or(JsValue::from(
            "Solver: Node data array could not be extracted!",
        ))?;

        let coordinates_array = node_data_array[1].as_array().ok_or(JsValue::from(
            "Solver: Nodes coordinates array could not be extracted!",
        ))?;

        let x = coordinates_array[0]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Node x coordinate could not be converted to FEFloat!",
            )))?;

        let y = coordinates_array[1]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Node y coordinate could not be converted to FEFloat!",
            )))?;

        let z = coordinates_array[2]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Node z coordinate could not be converted to FEFloat!",
            )))?;

        nodes_count += 1;

        fem.add_node(nodes_count, x, y, z)
            .map_err(JsValue::from)?;

        node_number_node_name_map.insert(nodes_count, node_name.to_string());
        node_name_node_number_map.insert(node_name.to_string(), nodes_count);
    }
    Ok(())
}
