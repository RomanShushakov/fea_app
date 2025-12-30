use serde_json::Value;
use wasm_bindgen::prelude::JsValue;

use std::collections::{HashMap, HashSet};

use crate::types::FEFloat;
use crate::functions::generate_uid;


pub(super) fn extract_points_from_input_data(
    extracted_points_data: &Value, 
    nodes: &mut HashMap<String, (u32, [FEFloat; 3])>,
    uids: &mut HashSet<u32>,
)
    -> Result<(), JsValue>
{
    let points_data = extracted_points_data.as_object().ok_or(
        JsValue::from("Mesher: Points data could not be converted into object!"))?;
    for (stringified_point_number, coordinates_array) in points_data.iter()
    {
        let stringified_coordinates = coordinates_array
            .as_array()
            .ok_or(JsValue::from("Mesher: Point coordinates array could not be extracted!"))?;

        let x = stringified_coordinates[0]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Point x coordinate could not be converted to FEFloat!")))?;

        let y = stringified_coordinates[1]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Point y coordinate could not be converted to FEFloat!")))?;

        let z = stringified_coordinates[2]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Point z coordinate could not be converted to FEFloat!")))?;

        let uid = generate_uid(uids);
        
        nodes.insert(stringified_point_number.to_string(), (uid, [x, y, z]));
    }
    Ok(())
}


pub(super) fn add_edge_nodes(
    nodes: &mut HashMap<String, (u32, [FEFloat; 3])>, 
    node_1_coordinates: [FEFloat; 3], 
    node_2_coordinates: [FEFloat; 3], 
    edge_number: String, 
    mesh_seed: u8, 
    edge_nodes_numbers: &mut Vec<String>,
    uids: &mut HashSet<u32>,
) 
    -> Result<(), JsValue> 
{
    let step = [
        (node_2_coordinates[0] - node_1_coordinates[0]) / mesh_seed as FEFloat,
        (node_2_coordinates[1] - node_1_coordinates[1]) / mesh_seed as FEFloat,
        (node_2_coordinates[2] - node_1_coordinates[2]) / mesh_seed as FEFloat,
    ];

    for i in 1..mesh_seed
    {
        let node_number = format!("{edge_number}_{i}");
        let node_coordinates = [
            node_1_coordinates[0] + step[0] * i as FEFloat, 
            node_1_coordinates[1] + step[1] * i as FEFloat,
            node_1_coordinates[2] + step[2] * i as FEFloat,
        ];

        let uid = generate_uid(uids);

        nodes.insert(node_number.clone(), (uid, node_coordinates));
        edge_nodes_numbers.push(node_number);
    }
    Ok(())
}


pub(super) fn check_node_number_existence(
    node_number: String, 
    nodes: &HashMap<String, (u32, [FEFloat; 3])>
) 
    -> Result<(), JsValue>
{
    if !nodes.contains_key(&node_number)
    {
        return Err(JsValue::from(&format!("Mesher: Node with number {node_number} doesn't exist!")));
    }
    Ok(())
}
