use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::JsValue;

use finite_element_method::{DOFParameter, ElementForceComponent, FEM};

use crate::types::FEFloat;

use crate::enums::BCType;

fn dof_parameter_from_str(name: &str) -> Result<DOFParameter, JsValue> {
    match name {
        "x" => Ok(DOFParameter::X),
        "y" => Ok(DOFParameter::Y),
        "z" => Ok(DOFParameter::Z),
        "th_x" => Ok(DOFParameter::ThX),
        "th_y" => Ok(DOFParameter::ThY),
        "th_z" => Ok(DOFParameter::ThZ),
        _ => Err(JsValue::from(format!("Unknown bc dof {name}!"))),
    }
}

pub fn dof_parameter_into_str(dof_parameter: &DOFParameter) -> &str {
    match dof_parameter {
        DOFParameter::X => "x",
        DOFParameter::Y => "y",
        DOFParameter::Z => "z",
        DOFParameter::ThX => "th_x",
        DOFParameter::ThY => "th_y",
        DOFParameter::ThZ => "th_z",
    }
}

pub fn element_force_component_into_str(element_force_component: &ElementForceComponent) -> &str {
    match element_force_component {
        ElementForceComponent::ForceR => "force_r",
        ElementForceComponent::ForceS => "force_s",
        ElementForceComponent::ForceT => "force_t",
        ElementForceComponent::MembraneForceR => "membrane_force_r",
        ElementForceComponent::MembraneForceS => "membrane_force_s",
        ElementForceComponent::MembraneForceRS => "membrane_force_r_s",
        ElementForceComponent::ShearForceRT => "shear_force_r_t",
        ElementForceComponent::ShearForceST => "shear_force_s_t",
        ElementForceComponent::MomentR => "moment_r",
        ElementForceComponent::MomentS => "moment_s",
        ElementForceComponent::MomentT => "moment_t",
        ElementForceComponent::BendingMomentR => "bending_moment_r",
        ElementForceComponent::BendingMomentS => "bending_moment_s",
        ElementForceComponent::BendingMomentRS => "bending_moment_r_s",
    }
}

pub fn extract_nodal_bc_from_input_data(
    extracted_nodal_bc_data: &Value,
    fem: &mut FEM<FEFloat>,
    node_name_node_number_map: &mut HashMap<String, u32>,
) -> Result<(), JsValue> {
    let nodal_bc_data = extracted_nodal_bc_data.as_object().ok_or(JsValue::from(
        "Solver: Nodal BC data could not be converted into object!",
    ))?;

    for (node_name, applied_bcs) in nodal_bc_data.iter() {
        let applied_bcs_array = applied_bcs.as_array().ok_or(JsValue::from(
            "Solver: Applied BCs data could not be converted into array!",
        ))?;

        for applied_bc in applied_bcs_array {
            let bc_array = applied_bc.as_array().ok_or(JsValue::from(
                "Solver: Applied BC data could not be converted into array!",
            ))?;

            let bc_type_name = bc_array[0].to_string();
            let bc_type = BCType::from_str(&bc_type_name[1..bc_type_name.len() - 1])?;

            let bc_dof_name = bc_array[1].to_string();
            let dof_parameter = dof_parameter_from_str(&bc_dof_name[1..bc_dof_name.len() - 1])?;

            let value = bc_array[2]
                .to_string()
                .parse::<FEFloat>()
                .or(Err(JsValue::from(
                    "Solver: BC value could not be converted to FEFloat!",
                )))?;

            let node_number = node_name_node_number_map
                .get(node_name)
                .ok_or(JsValue::from(format!(
                    "Solver: BC node {node_name} is absent!"
                )))?;

            match bc_type {
                BCType::Displacement => fem.add_displacement(*node_number, dof_parameter, value)?,
                BCType::Force => fem.add_concentrated_load(*node_number, dof_parameter, value)?,
            }
        }
    }
    Ok(())
}

pub fn extract_udls_on_beams_from_input_data(
    extracted_udls_on_beams_data: &Value,
    fem: &mut FEM<FEFloat>,
) -> Result<(), JsValue> {
    let udls_on_beams_data = extracted_udls_on_beams_data
        .as_object()
        .ok_or(JsValue::from(
            "Solver: Uniformly distributed loads on beams data could not be converted \
            into object!",
        ))?;

    for (stringified_beam_element_number, applied_load) in udls_on_beams_data.iter() {
        let applied_load_array = applied_load.as_array().ok_or(JsValue::from(
            "Solver: Applied BCs data could not be converted into array!",
        ))?[0]
            .as_array()
            .ok_or(JsValue::from(
                "Solver: Applied BCs data could not be converted into array!",
            ))?;

        let load_type_name = applied_load_array[0].to_string();
        let load_type = BCType::from_str(&load_type_name[1..load_type_name.len() - 1])?;

        let load_dof_name = applied_load_array[1].to_string();
        let dof_parameter = dof_parameter_from_str(&load_dof_name[1..load_dof_name.len() - 1])?;

        let value = applied_load_array[2]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: UDL load on beam value could not be converted to FEFloat!",
            )))?;

        let beam_element_number = stringified_beam_element_number
            .to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(format!(
                "Solver: Beam element number {} could not be converted to u32!",
                stringified_beam_element_number
            ))))?;

        if load_type == BCType::Displacement {
            return Err(JsValue::from(
                "Incorrect type of uniformly distributed load on beam!",
            ));
        }

        fem.add_uniformly_distributed_line_load(beam_element_number, dof_parameter, value)
            .map_err(|e| JsValue::from(e))?;
    }
    Ok(())
}

pub fn extract_udls_on_plates_from_input_data(
    extracted_udls_on_plates_data: &Value,
    fem: &mut FEM<FEFloat>,
) -> Result<(), JsValue> {
    let udls_on_plates_data = extracted_udls_on_plates_data
        .as_object()
        .ok_or(JsValue::from(
            "Solver: Uniformly distributed loads on plates data could not be converted \
            into object!",
        ))?;

    for (stringified_plate_element_number, applied_load) in udls_on_plates_data.iter() {
        let applied_load_array = applied_load.as_array().ok_or(JsValue::from(
            "Solver: Applied BCs data could not be converted into array!",
        ))?[0]
            .as_array()
            .ok_or(JsValue::from(
                "Solver: Applied BCs data could not be converted into array!",
            ))?;

        let load_type_name = applied_load_array[0].to_string();
        let load_type = BCType::from_str(&load_type_name[1..load_type_name.len() - 1])?;

        let load_dof_name = applied_load_array[1].to_string();
        let dof_parameter = dof_parameter_from_str(&load_dof_name[1..load_dof_name.len() - 1])?;

        let value = applied_load_array[2]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from(
                "Solver: UDL load on plate value could not be converted to FEFloat!",
            )))?;

        let plate_element_number = stringified_plate_element_number
            .to_string()
            .parse::<u32>()
            .or(Err(JsValue::from(format!(
                "Solver: Plate element number {} could not be converted to u32!",
                stringified_plate_element_number
            ))))?;

        if load_type == BCType::Displacement {
            return Err(JsValue::from(
                "Incorrect type of uniformly distributed load on plate!",
            ));
        }

        fem.add_uniformly_distributed_surface_load(plate_element_number, dof_parameter, value)
            .map_err(|e| JsValue::from(e))?;
    }
    Ok(())
}
