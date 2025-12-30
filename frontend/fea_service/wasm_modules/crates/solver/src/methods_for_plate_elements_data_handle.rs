use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::JsValue;

use finite_element_method::FEM;

use crate::types::FEFloat;

pub fn extract_plate_elements_from_input_data(
    extracted_plate_elements_data: &Value,
    fem: &mut FEM<FEFloat>,
    node_name_node_number_map: &mut HashMap<String, u32>,
) -> Result<(), JsValue> {
    let plate_elements_data = extracted_plate_elements_data
        .as_object()
        .ok_or(JsValue::from(
            "Solver: Plate elements data could not be converted into object!",
        ))?;

    for (stringified_plate_element_number, plate_element_properties) in plate_elements_data.iter() {
        let stringified_plate_elements_properties = plate_element_properties.as_array().ok_or(
            JsValue::from("Solver: Plate element all properties array could not be extracted!"),
        )?;

        let nodes_names =
            stringified_plate_elements_properties[1]
                .as_array()
                .ok_or(JsValue::from(
                    "Solver: Plate element nodes numbers array could not be extracted!",
                ))?;

        let node_1_name = nodes_names[0].to_string();
        let node_1_number = node_name_node_number_map
            .get(&node_1_name[1..node_1_name.len() - 1])
            .ok_or(JsValue::from(format!(
                "Solver: Plate element node {} is absent!",
                node_1_name
            )))?;

        let node_2_name = nodes_names[1].to_string();
        let node_2_number = node_name_node_number_map
            .get(&node_2_name[1..node_2_name.len() - 1])
            .ok_or(JsValue::from(format!(
                "Solver: Plate element node {} is absent!",
                node_2_name
            )))?;

        let node_3_name = nodes_names[2].to_string();
        let node_3_number = node_name_node_number_map
            .get(&node_3_name[1..node_3_name.len() - 1])
            .ok_or(JsValue::from(format!(
                "Solver: Plate element node {} is absent!",
                node_3_name
            )))?;

        let node_4_name = nodes_names[3].to_string();
        let node_4_number = node_name_node_number_map
            .get(&node_4_name[1..node_4_name.len() - 1])
            .ok_or(JsValue::from(format!(
                "Solver: Plate element node {} is absent!",
                node_4_name
            )))?;

        let properties =
            stringified_plate_elements_properties[2]
                .as_array()
                .ok_or(JsValue::from(
                    "Solver: Plate element properties array could not be extracted!",
                ))?;

        let young_modulus = properties[0]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Plate element Young's modulus could not be converted to FEFloat!",
            )))?;

        let poisson_ratio = properties[1]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Plate element Poisson's ratio could not be converted to FEFloat!",
            )))?;

        let thickness = properties[2]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Plate element thickness could not be converted to FEFloat!",
            )))?;

        let shear_factor = properties[3]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Plate element shear factor could not be converted to FEFloat!",
            )))?;

        let plate_element_number = stringified_plate_element_number
            .to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Solver: Plate element number could not be converted to u32!",
            )))?;

        fem.add_plate(
            plate_element_number,
            *node_1_number,
            *node_2_number,
            *node_3_number,
            *node_4_number,
            young_modulus,
            poisson_ratio,
            thickness,
            shear_factor,
        )
        .map_err(|e| JsValue::from(e))?;
    }
    Ok(())
}
