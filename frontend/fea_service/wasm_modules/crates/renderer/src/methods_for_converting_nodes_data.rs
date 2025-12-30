use serde_json::{Map, Value};
use wasm_bindgen::prelude::JsValue;
use std::collections::HashMap;

use crate::structs::{NodeData, ExtremeGlobalAnalysisValues};
use crate::functions::convert_vec_to_array;
use crate::props::Props;


pub fn convert_nodes_data(
    nodes_data: &Map<String, Value>, converted_nodes_data: &mut HashMap<String, NodeData>,
)
    -> Result<(), JsValue>
{
    for (node_name, node_data) in nodes_data.iter()
    {
        let node_data_array = node_data
            .as_array()
            .ok_or(JsValue::from("Renderer: Node data array could not be extracted!"))?;

        let uid = node_data_array[0]
            .to_string()
            .parse::<u32>()
            .or(Err(JsValue::from("Renderer: Node uid could not be converted to u32!")))?;

        let coordinates_array = node_data_array[1]
            .as_array()
            .ok_or(JsValue::from("Renderer: Node coordinates array could not be extracted!"))?;

        let x = coordinates_array[0]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Node x coordinate could not be converted to f32!")))?;
        let y = coordinates_array[1]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Node y coordinate could not be converted to f32!")))?;
        let z = coordinates_array[2]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Node z coordinate could not be converted to f32!")))?;

        let node_data = NodeData::init(uid, x, y, z);
        converted_nodes_data.insert(node_name.to_string(), node_data);
    }
    Ok(())
}


fn get_extreme_analysis_result_values_array(
    key: &str, extreme_global_analysis_values: &Map<String, Value>
) 
    -> Result<[f32; 2], JsValue>
{
    let extreme_analysis_result_values_vec = extreme_global_analysis_values
        .get(key)
        .ok_or(JsValue::from(format!("Renderer: Extreme {key} values could not be extracted!")))?
        .as_array()
        .ok_or(JsValue::from(format!("Renderer: Extreme {key} values could not be converted into array!")))?
        .iter()
        .map(
            |value| 
            Ok
            (
                value
                .to_string()
                .parse::<f32>()
                .or(Err(JsValue::from(format!("Renderer: Extreme {key} value could not be converted to f32!"))))?)
            )
        .collect::<Result<Vec<f32>, JsValue>>()?;
    let extreme_analysis_result_values_array = convert_vec_to_array(extreme_analysis_result_values_vec);
    Ok(extreme_analysis_result_values_array)
}


pub fn get_converted_extreme_global_analysis_values(
    extreme_global_analysis_values: &Map<String, Value>, is_displacements: bool,
) 
    -> Result<ExtremeGlobalAnalysisValues, JsValue>
{
    let extreme_u_f_x = get_extreme_analysis_result_values_array("x", extreme_global_analysis_values)?;
    let extreme_u_f_y = get_extreme_analysis_result_values_array("y", extreme_global_analysis_values)?;
    let extreme_u_f_z = get_extreme_analysis_result_values_array("z", extreme_global_analysis_values)?;
    let extreme_r_m_x = get_extreme_analysis_result_values_array("th_x", extreme_global_analysis_values)?;
    let extreme_r_m_y = get_extreme_analysis_result_values_array("th_y", extreme_global_analysis_values)?;
    let extreme_r_m_z = get_extreme_analysis_result_values_array("th_z", extreme_global_analysis_values)?;

    let u_f_key = if is_displacements { "u_result" } else { "f_result" };

    let extreme_u_f_result = get_extreme_analysis_result_values_array(
        u_f_key, extreme_global_analysis_values,
    )?;

    let r_m_key = if is_displacements { "r_result" } else { "m_result" };
    let extreme_r_m_result = get_extreme_analysis_result_values_array(
        r_m_key, extreme_global_analysis_values,
    )?;

    let converted_extreme_global_displacements_data = ExtremeGlobalAnalysisValues::create(
        extreme_u_f_x,
        extreme_u_f_y,
        extreme_u_f_z,
        extreme_r_m_x,
        extreme_r_m_y,
        extreme_r_m_z,
        extreme_u_f_result,
        extreme_r_m_result,
    );
    Ok(converted_extreme_global_displacements_data)
}


pub fn update_nodes_data(
    global_analysis_result_data: &Map<String, Value>,
    converted_nodes_data: &mut HashMap<String, NodeData>,
    converted_extreme_global_displacements_data: &ExtremeGlobalAnalysisValues,
    converted_extreme_global_loads_data: &ExtremeGlobalAnalysisValues,
    props: &Props,
)
    -> Result<(), JsValue>
{
    for (node_name, nodal_data) in global_analysis_result_data.iter()
    {
        let nodal_data_array = nodal_data
            .as_array()
            .ok_or(
                JsValue::from("Renderer: Global analysis result nodal data could not be converted into array!"),
            )?;

        let ux = nodal_data_array[0]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Ux value could not be converted to f32!")))?;
        let uy = nodal_data_array[1]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Uy value could not be converted to f32!")))?;
        let uz = nodal_data_array[2]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Uz value could not be converted to f32!")))?;

        let fx = nodal_data_array[6]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Fx value could not be converted to f32!")))?;
        let fy = nodal_data_array[7]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Fy value could not be converted to f32!")))?;
        let fz = nodal_data_array[8]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Fz value could not be converted to f32!")))?;
        let mx = nodal_data_array[9]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Mx value could not be converted to f32!")))?;
        let my = nodal_data_array[10]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: My value could not be converted to f32!")))?;
        let mz = nodal_data_array[11]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Mz value could not be converted to f32!")))?;

        let node_data = converted_nodes_data
            .get_mut(node_name)
            .ok_or(JsValue::from(format!("Renderer: Node name {node_name} is absent!")))?;

        node_data.update(
            ux, 
            uy, 
            uz, 
            fx, 
            fy, 
            fz, 
            mx, 
            my, 
            mz, 
            converted_extreme_global_displacements_data,
            converted_extreme_global_loads_data,
            props,
        );
    }
    Ok(())
}
