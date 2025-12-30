use wasm_bindgen::prelude::JsValue;

use std::collections::{HashMap, HashSet};

use crate::types::FEFloat;
use crate::enums::Direction;
use crate::methods_for_nodes_data_handle::add_edge_nodes;


fn get_edge_number(
    point_1_number: String, 
    point_2_number: String,
) 
    -> Result<(String, Direction), JsValue>
{
    if point_1_number
            .parse::<u32>()
            .or(Err(JsValue::from("Mesher: Point number could not be converted to u32!")))? < 
        point_2_number
            .parse::<u32>()
            .or(Err(JsValue::from("Mesher: Point number could not be converted to u32!")))?
    {
        Ok((format!("{point_1_number}_{point_2_number}"), Direction::Front))
    }
    else
    {
        Ok((format!("{point_2_number}_{point_1_number}"), Direction::Reverse))
    }
}


pub(super) fn add_edge(
    point_1_number: String, 
    point_2_number: String, 
    nodes: &mut HashMap<String, (u32, [FEFloat; 3])>, 
    edges: &mut HashMap<String, u8>, 
    mesh_seed: u8,
    uids: &mut HashSet<u32>,
) 
    -> Result<(Direction, Vec<String>), JsValue>
{
    let (edge_number, direction) = get_edge_number(point_1_number.clone(), point_2_number.clone())?;
    if let Some(existed_mesh_seed) = edges.insert(edge_number.clone(), mesh_seed)
    {
        if existed_mesh_seed != mesh_seed
        {
            return Err(JsValue::from(&format!("Mesher: Different mesh seed values was set for edge {edge_number}!")));
        }
    }

    let mut edge_nodes_numbers = Vec::new();

    let node_1_coordinates = nodes.get(&point_1_number).expect("Node is absent!").1.clone();
    let node_2_coordinates = nodes.get(&point_2_number).expect("Node is absent!").1.clone();
    match direction
    {
        Direction::Front => 
        {
            edge_nodes_numbers.push(point_1_number);
            add_edge_nodes(
                nodes,
                node_1_coordinates,
                node_2_coordinates,
                edge_number,
                mesh_seed,
                &mut edge_nodes_numbers,
                uids,
            )?;
            edge_nodes_numbers.push(point_2_number);
        },
        Direction::Reverse => 
        {
            edge_nodes_numbers.push(point_2_number);
            add_edge_nodes(
                nodes,
                node_2_coordinates,
                node_1_coordinates, 
                edge_number,
                mesh_seed,
                &mut edge_nodes_numbers,
                uids,
            )?;
            edge_nodes_numbers.push(point_1_number);
        }
    }

    Ok((direction, edge_nodes_numbers))
}
