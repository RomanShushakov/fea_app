use serde_json::Value;
use wasm_bindgen::prelude::JsValue;

use std::collections::HashMap;

use crate::types::FEFloat;
use crate::enums::{BCType, GlobalDOFParameter};
use crate::methods_for_nodes_data_handle::check_node_number_existence;


pub(super) fn extract_concentrated_loads_from_input_data(
    extracted_concentrated_loads_data: &Value, 
    nodes: &HashMap<String, (u32, [FEFloat; 3])>, 
    boundary_conditions: &mut HashMap<String, Vec<(String, String, FEFloat)>>, 
) 
    -> Result<(), JsValue>
{
    let concentrated_loads_data = extracted_concentrated_loads_data.as_object().ok_or(
        JsValue::from("Mesher: Concentrated loads data could not be converted into object!"))?;
    for (serialized_point_number, serialized_concentrated_load_data) in concentrated_loads_data.iter()
    {
        check_node_number_existence(serialized_point_number.to_string(), nodes)?;

        let concentrated_load_data = serialized_concentrated_load_data
            .as_array()
            .ok_or(JsValue::from("Mesher: Serialized concentrated load data could not be converted into array!"))?;

        let force_x_value = concentrated_load_data[0]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: X force could not be converted to FEFloat!")))?;
        if force_x_value != 0 as FEFloat
        {
            let bc_data = boundary_conditions
                .entry(serialized_point_number.to_string())
                .or_insert(vec![]);
            bc_data.push((BCType::Force.as_string(), GlobalDOFParameter::X.as_string(), force_x_value));
        }

        let force_y_value = concentrated_load_data[1]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Y force could not be converted to FEFloat!")))?;
        if force_y_value != 0 as FEFloat
        {
            let bc_data = boundary_conditions
                .entry(serialized_point_number.to_string())
                .or_insert(vec![]);
            bc_data.push((BCType::Force.as_string(), GlobalDOFParameter::Y.as_string(), force_y_value));
        }

        let force_z_value = concentrated_load_data[2]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Z force could not be converted to FEFloat!")))?;
        if force_z_value != 0 as FEFloat
        {
            let bc_data = boundary_conditions
                .entry(serialized_point_number.to_string())
                .or_insert(vec![]);
            bc_data.push((BCType::Force.as_string(), GlobalDOFParameter::Z.as_string(), force_z_value));
        }

        let moment_x_value = concentrated_load_data[3]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: X moment could not be converted to FEFloat!")))?;
        if moment_x_value != 0 as FEFloat
        {
            let bc_data = boundary_conditions
                .entry(serialized_point_number.to_string())
                .or_insert(vec![]);
            bc_data.push((BCType::Force.as_string(), GlobalDOFParameter::ThX.as_string(), moment_x_value));
        }

        let moment_y_value = concentrated_load_data[4]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Y moment could not be converted to FEFloat!")))?;
        if moment_y_value != 0 as FEFloat
        {
            let bc_data = boundary_conditions
                .entry(serialized_point_number.to_string())
                .or_insert(vec![]);
            bc_data.push((BCType::Force.as_string(), GlobalDOFParameter::ThY.as_string(), moment_y_value));
        }

        let moment_z_value = concentrated_load_data[5]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Z moment could not be converted to FEFloat!")))?;
        if moment_z_value != 0 as FEFloat
        {
            let bc_data = boundary_conditions
                .entry(serialized_point_number.to_string())
                .or_insert(vec![]);
            bc_data.push((BCType::Force.as_string(), GlobalDOFParameter::ThZ.as_string(), moment_z_value));
        }
    }
    Ok(())
}
