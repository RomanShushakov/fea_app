use serde_json::Value;
use wasm_bindgen::prelude::JsValue;

use std::collections::HashMap;

use crate::types::FEFloat;
use crate::enums::{BCType, GlobalDOFParameter};
use crate::methods_for_nodes_data_handle::check_node_number_existence;


pub(super) fn extract_point_boundary_conditions_from_input_data(
    extracted_point_boundary_conditions_data: &Value, 
    nodes: &HashMap<String, (u32, [FEFloat; 3])>, 
    boundary_conditions: &mut HashMap<String, Vec<(String, String, FEFloat)>>, 
) 
    -> Result<(), JsValue>
{
    let point_boundary_conditions_data = extracted_point_boundary_conditions_data
        .as_object()
        .ok_or(JsValue::from("Mesher: Point boundary conditions data could not be converted into object!"))?;
    for (serialized_point_number, serialized_point_boundary_condition_data) in point_boundary_conditions_data.iter()
    {
        check_node_number_existence(serialized_point_number.to_string(), nodes)?;

        let point_boundary_condition_data = serialized_point_boundary_condition_data
            .as_array()
            .ok_or(JsValue::from("Mesher: Serialized point boundary condition data could not be converted into array!"))?;

        if point_boundary_condition_data[0].is_number()
        {
            let bc_value = point_boundary_condition_data[0]
                .to_string()
                .parse::<FEFloat>()
                .or(Err(JsValue::from("Mesher: X displacement could not be converted to FEFloat!")))?;
            let bc_data = boundary_conditions
                .entry(serialized_point_number.to_string())
                .or_insert(vec![]);
            bc_data.push((BCType::Displacment.as_string(), GlobalDOFParameter::X.as_string(), bc_value));
        }

        if !point_boundary_condition_data[1].is_null()
        {
            let bc_value = point_boundary_condition_data[1]
                .to_string()
                .parse::<FEFloat>()
                .or(Err(JsValue::from("Mesher: Y displacement could not be converted to FEFloat!")))?;
            let bc_data = boundary_conditions
                .entry(serialized_point_number.to_string())
                .or_insert(vec![]);
            bc_data.push((BCType::Displacment.as_string(), GlobalDOFParameter::Y.as_string(), bc_value));
        }

        if !point_boundary_condition_data[2].is_null()
        {
            let bc_value = point_boundary_condition_data[2]
                .to_string()
                .parse::<FEFloat>()
                .or(Err(JsValue::from("Mesher: Z displacement could not be converted to FEFloat!")))?;
            let bc_data = boundary_conditions
                .entry(serialized_point_number.to_string())
                .or_insert(vec![]);
            bc_data.push((BCType::Displacment.as_string(), GlobalDOFParameter::Z.as_string(), bc_value));
        }

        if !point_boundary_condition_data[3].is_null()
        {
            let bc_value = point_boundary_condition_data[3]
                .to_string()
                .parse::<FEFloat>()
                .or(Err(JsValue::from("Mesher: ThX displacement could not be converted to FEFloat!")))?;
            let bc_data = boundary_conditions
                .entry(serialized_point_number.to_string())
                .or_insert(vec![]);
            bc_data.push((BCType::Displacment.as_string(), GlobalDOFParameter::ThX.as_string(), bc_value));
        }

        if !point_boundary_condition_data[4].is_null()
        {
            let bc_value = point_boundary_condition_data[4]
                .to_string()
                .parse::<FEFloat>()
                .or(Err(JsValue::from("Mesher: ThY displacement could not be converted to FEFloat!")))?;
            let bc_data = boundary_conditions
                .entry(serialized_point_number.to_string())
                .or_insert(vec![]);
            bc_data.push((BCType::Displacment.as_string(), GlobalDOFParameter::ThY.as_string(), bc_value));
        }

        if !point_boundary_condition_data[5].is_null()
        {
            let bc_value = point_boundary_condition_data[5]
                .to_string()
                .parse::<FEFloat>()
                .or(Err(JsValue::from("Mesher: ThZ displacement could not be converted to FEFloat!")))?;
            let bc_data = boundary_conditions
                .entry(serialized_point_number.to_string())
                .or_insert(vec![]);
            bc_data.push((BCType::Displacment.as_string(), GlobalDOFParameter::ThZ.as_string(), bc_value));
        }
    }
    Ok(())
}
