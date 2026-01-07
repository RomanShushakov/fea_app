use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::JsValue;

use finite_element_method::FEM;

use crate::types::FEFloat;

pub fn extract_beam_elements_from_input_data(
    extracted_beam_elements_data: &Value,
    fem: &mut FEM<FEFloat>,
    node_name_node_number_map: &mut HashMap<String, u32>,
) -> Result<(), JsValue> {
    let beam_elements_data = extracted_beam_elements_data
        .as_object()
        .ok_or(JsValue::from(
            "Solver: Beam elements data could not be converted into object!",
        ))?;

    for (stringified_beam_element_number, beam_element_properties) in beam_elements_data.iter() {
        let stringified_beam_elements_properties = beam_element_properties.as_array().ok_or(
            JsValue::from("Solver: Beam element all properties array could not be extracted!"),
        )?;

        let nodes_names =
            stringified_beam_elements_properties[1]
                .as_array()
                .ok_or(JsValue::from(
                    "Solver: Beam element nodes numbers array could not be extracted!",
                ))?;

        let node_1_name = nodes_names[0].to_string();
        let node_1_number = node_name_node_number_map
            .get(&node_1_name[1..node_1_name.len() - 1])
            .ok_or(JsValue::from(format!(
                "Solver: Beam element node {} is absent!",
                node_1_name
            )))?;

        let node_2_name = nodes_names[1].to_string();
        let node_2_number = node_name_node_number_map
            .get(&node_2_name[1..node_2_name.len() - 1])
            .ok_or(JsValue::from(format!(
                "Solver: Beam element node {} is absent!",
                node_2_name
            )))?;

        let properties =
            stringified_beam_elements_properties[2]
                .as_array()
                .ok_or(JsValue::from(
                    "Solver: Beam element properties array could not be extracted!",
                ))?;

        let young_modulus = properties[0]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Beam element Young's modulus could not be converted to FEFloat!",
            )))?;

        let poisson_ratio = properties[1]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Beam element Poisson's ratio could not be converted to FEFloat!",
            )))?;

        let area = properties[2]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Beam element area could not be converted to FEFloat!",
            )))?;

        let i11 = properties[3]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Beam element I11 could not be converted to FEFloat!",
            )))?;

        let i22 = properties[4]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Beam element I22 could not be converted to FEFloat!",
            )))?;

        let i12 = properties[5]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Beam element I12 could not be converted to FEFloat!",
            )))?;

        let it = properties[6]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Beam element It could not be converted to FEFloat!",
            )))?;

        let shear_factor = properties[7]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: Beam element shear factor could not be converted to FEFloat!",
            )))?;

        let local_axis_1_direction_x = properties[8].to_string().parse::<FEFloat>().or(Err(
            JsValue::from(
                "Solver: Beam element local axis 1 direction x component could not be converted \
                to FEFloat!",
            ),
        ))?;

        let local_axis_1_direction_y = properties[9].to_string().parse::<FEFloat>().or(Err(
            JsValue::from(
                "Solver: Beam element local axis 1 direction y component could not be converted \
                to FEFloat!",
            ),
        ))?;

        let local_axis_1_direction_z = properties[10].to_string().parse::<FEFloat>().or(Err(
            JsValue::from(
                "Solver: Beam element local axis 1 direction z component could not be converted \
                to FEFloat!",
            ),
        ))?;

        let beam_element_number = stringified_beam_element_number
            .to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(
                "Solver: Beam element number could not be converted to u32!",
            )))?;

        fem.add_beam(
            beam_element_number,
            *node_1_number,
            *node_2_number,
            young_modulus,
            poisson_ratio,
            area,
            i11,
            i22,
            i12,
            it,
            shear_factor,
            [
                local_axis_1_direction_x,
                local_axis_1_direction_y,
                local_axis_1_direction_z,
            ],
        )
        .map_err(JsValue::from)?;
    }
    Ok(())
}
