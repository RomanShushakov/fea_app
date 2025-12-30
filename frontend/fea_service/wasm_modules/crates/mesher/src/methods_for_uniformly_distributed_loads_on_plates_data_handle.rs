use serde_json::Value;
use wasm_bindgen::prelude::JsValue;

use std::collections::HashMap;

use crate::types::FEFloat;
use crate::enums::{BCType, GlobalDOFParameter};
use crate::methods_for_plate_elements_data_handle::check_plate_element_existence;


pub(super) fn extract_uniformly_distributed_loads_on_plates_from_input_data(
    extracted_uniformly_distributed_surface_loads_data: &Value,
    surfaces: &HashMap<String, Vec<u32>>,
    plate_elements: &HashMap<u32, (u32, [String; 4], [FEFloat; 4])>, 
    uniformly_distributed_loads_on_plates: &mut HashMap<u32, Vec<(String, String, FEFloat)>>, 
) 
    -> Result<(), JsValue>
{
    let uniformly_distributed_surface_loads_data = extracted_uniformly_distributed_surface_loads_data
        .as_object()
        .ok_or(JsValue::from("Mesher: Uniformly distributed surface loads data could not be converted into object!"))?;
    for (serialized_surface_number, serialized_uniformly_distributed_surface_load_data) in 
        uniformly_distributed_surface_loads_data.iter()
    {
        let uniformly_distributed_surface_load_data = serialized_uniformly_distributed_surface_load_data
            .as_array()
            .ok_or(JsValue::from("Mesher: Serialized uniformly distributed surface load data could not be converted into array!"))?;

        let px_value = uniformly_distributed_surface_load_data[0]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Px load could not be converted to FEFloat!")))?;

        let py_value = uniformly_distributed_surface_load_data[1]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Py load could not be converted to FEFloat!")))?;

        let pz_value = uniformly_distributed_surface_load_data[2]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Pz load could not be converted to FEFloat!")))?;

        if let Some(plate_elements_numbers) = surfaces.get(serialized_surface_number)
        {
            for plate_element_number in plate_elements_numbers
            {
                check_plate_element_existence(*plate_element_number, plate_elements)?;
                if px_value != 0 as FEFloat
                {
                    let p_data = uniformly_distributed_loads_on_plates
                        .entry(*plate_element_number)
                        .or_insert(vec![]);
                    p_data.push((BCType::Force.as_string(), GlobalDOFParameter::X.as_string(), px_value));
                }

                if py_value != 0 as FEFloat
                {
                    let p_data = uniformly_distributed_loads_on_plates
                        .entry(*plate_element_number)
                        .or_insert(vec![]);
                    p_data.push((BCType::Force.as_string(), GlobalDOFParameter::Y.as_string(), py_value));
                }

                if pz_value != 0 as FEFloat
                {
                    let p_data = uniformly_distributed_loads_on_plates
                        .entry(*plate_element_number)
                        .or_insert(vec![]);
                    p_data.push((BCType::Force.as_string(), GlobalDOFParameter::Z.as_string(), pz_value));
                }
            }
        }
        else
        {
            return Err(JsValue::from(format!("Mesher: There are no plate elements lied on surface {}", 
                serialized_surface_number)));
        }
    }
    Ok(())
}
