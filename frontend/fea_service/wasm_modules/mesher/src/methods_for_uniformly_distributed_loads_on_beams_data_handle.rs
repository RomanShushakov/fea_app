use serde_json::Value;
use wasm_bindgen::prelude::JsValue;

use std::collections::HashMap;

use crate::types::FEFloat;
use crate::enums::{BCType, GlobalDOFParameter};
use crate::methods_for_beam_elements_data_handle::check_beam_element_existence;


pub(super) fn extract_uniformly_distributed_loads_on_beams_from_input_data(
    extracted_uniformly_distributed_line_loads_data: &Value,
    lines: &HashMap<String, Vec<u32>>,
    beam_elements: &HashMap<u32, (u32, [String; 2], [FEFloat; 11])>, 
    uniformly_distributed_loads_on_beams: &mut HashMap<u32, Vec<(String, String, FEFloat)>>, 
) 
    -> Result<(), JsValue>
{
    let uniformly_distributed_line_loads_data = extracted_uniformly_distributed_line_loads_data
        .as_object()
        .ok_or(JsValue::from("Mesher: Uniformly distributed line loads data could not be converted into object!"))?;
    for (serialized_line_number, serialized_uniformly_distributed_line_load_data) in 
        uniformly_distributed_line_loads_data.iter()
    {
        let uniformly_distributed_line_load_data = serialized_uniformly_distributed_line_load_data
            .as_array()
            .ok_or(JsValue::from("Mesher: Serialized uniformly distributed line load data could not be converted into array!"))?;

        let qx_value = uniformly_distributed_line_load_data[0]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Qx load could not be converted to FEFloat!")))?;

        let qy_value = uniformly_distributed_line_load_data[1]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Qy load could not be converted to FEFloat!")))?;

        let qz_value = uniformly_distributed_line_load_data[2]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Qz load could not be converted to FEFloat!")))?;

        if let Some(beam_elements_numbers) = lines.get(serialized_line_number)
        {
            for beam_element_number in beam_elements_numbers
            {
                check_beam_element_existence(*beam_element_number, beam_elements)?;
                if qx_value != 0 as FEFloat
                {
                    let q_data = uniformly_distributed_loads_on_beams
                        .entry(*beam_element_number)
                        .or_insert(vec![]);
                    q_data.push((BCType::Force.as_string(), GlobalDOFParameter::X.as_string(), qx_value));
                }

                if qy_value != 0 as FEFloat
                {
                    let q_data = uniformly_distributed_loads_on_beams
                        .entry(*beam_element_number)
                        .or_insert(vec![]);
                    q_data.push((BCType::Force.as_string(), GlobalDOFParameter::Y.as_string(), qy_value));
                }

                if qz_value != 0 as FEFloat
                {
                    let q_data = uniformly_distributed_loads_on_beams
                        .entry(*beam_element_number)
                        .or_insert(vec![]);
                    q_data.push((BCType::Force.as_string(), GlobalDOFParameter::Z.as_string(), qz_value));
                }
            }
        }
        else
        {
            return Err(JsValue::from(format!("Mesher: There are no beam elements lied on line {}", 
                serialized_line_number)));
        }
    }
    Ok(())
}
