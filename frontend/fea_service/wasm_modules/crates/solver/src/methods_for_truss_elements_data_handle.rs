use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::JsValue;

use finite_element_method::FEM;

use crate::types::FEFloat;

pub fn extract_truss_elements_from_input_data(
    extracted_truss_elements_data: &Value,
    fem: &mut FEM<FEFloat>,
    node_name_node_number_map: &mut HashMap<String, u32>,
) -> Result<(), JsValue> {
    let truss_elements_data = extracted_truss_elements_data
        .as_object()
        .ok_or(JsValue::from(
            "Solver: Truss elements data could not be converted into object!",
        ))?;

    for (stringified_truss_element_number, truss_element_properties) in truss_elements_data.iter() {
        let stringified_truss_elements_properties = truss_element_properties.as_array().ok_or(
            JsValue::from("Solver: Truss element all properties array could not be extracted!"),
        )?;

        let nodes_names =
            stringified_truss_elements_properties[1]
                .as_array()
                .ok_or(JsValue::from(
                    "Solver: Truss element nodes numbers array could not be extracted!",
                ))?;

        let node_1_name = nodes_names[0].to_string();
        let node_1_number = node_name_node_number_map
            .get(&node_1_name[1..node_1_name.len() - 1])
            .ok_or(JsValue::from(format!(
                "Solver: Truss element node {} is absent!",
                node_1_name
            )))?;

        let node_2_name = nodes_names[1].to_string();
        let node_2_number = node_name_node_number_map
            .get(&node_2_name[1..node_2_name.len() - 1])
            .ok_or(JsValue::from(format!(
                "Solver: Truss element node {} is absent!",
                node_2_name
            )))?;

        let properties =
            stringified_truss_elements_properties[2]
                .as_array()
                .ok_or(JsValue::from(
                    "Solver: Truss element properties array could not be extracted!",
                ))?;

        let young_modulus = properties[0]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Truss element Young's modulus could not be converted to FEFloat!",
            )))?;

        let area = properties[1]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Truss element area could not be converted to FEFloat!",
            )))?;

        let area_2 = if properties.len() == 3 {
            Some(
                properties[2]
                    .to_string()
                    .parse::<FEFloat>()
                    .or(Err(JsValue::from(
                        "Solver: Truss element area 2 could not be converted to FEFloat!",
                    )))?,
            )
        } else {
            None
        };

        let truss_element_number = stringified_truss_element_number
            .to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Solver: Truss element number could not be converted to u32!",
            )))?;

        fem.add_truss(
            truss_element_number,
            *node_1_number,
            *node_2_number,
            young_modulus,
            area,
            area_2,
        )
        .map_err(JsValue::from)?;
    }
    Ok(())
}
