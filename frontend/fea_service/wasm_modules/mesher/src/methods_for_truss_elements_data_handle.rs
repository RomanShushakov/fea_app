use serde_json::Value;
use wasm_bindgen::prelude::JsValue;

use std::collections::{HashMap, HashSet};

use crate::functions::generate_uid;
use crate::types::FEFloat;
use crate::props::Props;
use crate::methods_for_nodes_data_handle::check_node_number_existence;
use crate::methods_for_edges_data_handle::add_edge;


pub(super) fn extract_trusses_from_input_data(
    extracted_trusses_data: &Value, 
    nodes: &mut HashMap<String, (u32, [FEFloat; 3])>, 
    edges: &mut HashMap<String, u8>, 
    truss_elements: &mut HashMap<u32, (u32, [String; 2], Vec<FEFloat>)>, 
    props: &Props,
    uids: &mut HashSet<u32>,
) 
    -> Result<(), JsValue>
{
    let trusses_data = extracted_trusses_data
        .as_object()
        .ok_or(JsValue::from("Mesher: Trusses data could not be converted into object!"))?;
    for (serialized_line_number, serialized_truss_data) in trusses_data.iter()
    {
        let truss_data = serialized_truss_data
            .as_array()
            .ok_or(JsValue::from("Mesher: Serialized truss data could not be converted into array!"))?;

        let point_1_number = truss_data[0].to_string();
        check_node_number_existence(point_1_number.clone(), nodes)?;

        let point_2_number = truss_data[1].to_string();
        check_node_number_existence(point_2_number.clone(), nodes)?;

        let young_modulus = truss_data[2]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Young's modulus could not be converted to FEFloat!")))?;

        let area = truss_data[3]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Area could not be converted to FEFloat!")))?;

        let uid = generate_uid(uids);

        let mut truss_element_data = (
            uid,
            [point_1_number.clone(), point_2_number.clone()], 
            vec![young_modulus, area]
        );

        if !truss_data[4].is_null()
        {
            let area_2 = truss_data[4]
                .to_string()
                .parse::<FEFloat>()
                .or(Err(JsValue::from("Mesher: Area 2 could not be converted to FEFloat!")))?;
            truss_element_data.2.push(area_2);
        }

        let mut str_truss_element_number = format!("{}", props.truss_elements_group_number);
        (0..props.max_line_number.to_string().len() - serialized_line_number.len())
            .for_each(|_| str_truss_element_number += "0");
        str_truss_element_number += &format!("{serialized_line_number}");
        let truss_element_number = str_truss_element_number
            .parse::<u32>()
            .or(Err(JsValue::from("Mesher: Truss element number could not be converted to u32!")))?;
        truss_elements.insert(truss_element_number, truss_element_data);

        let _ = add_edge(point_1_number, point_2_number, nodes, edges, 1, uids)?;
    }
    Ok(())
}
