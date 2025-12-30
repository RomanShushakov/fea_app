use serde_json::Value;
use wasm_bindgen::prelude::JsValue;

use std::collections::{HashMap, HashSet};

use crate::types::FEFloat;
use crate::enums::Direction;
use crate::props::Props;
use crate::methods_for_nodes_data_handle::check_node_number_existence;
use crate::methods_for_edges_data_handle::add_edge;
use crate::functions::{find_lines_intersection, generate_uid};


fn add_edge_nodes_to_nodes_matrix(
    point_1_number: String, 
    point_2_number: String, 
    point_3_number: String,
    point_4_number: String, 
    nodes: &mut HashMap<String, (u32, [FEFloat; 3])>, 
    edges: &mut HashMap<String, u8>,
    edges_1_3_mesh_seed: u8, 
    edges_2_4_mesh_seed: u8, 
    nodes_matrix: &mut HashMap<(usize, usize), String>,
    uids: &mut HashSet<u32>,
)
    -> Result<(), JsValue>
{
    let (edge_1_direction, edge_1_nodes_numbers) = add_edge(
        point_1_number.clone(), 
        point_2_number.clone(), 
        nodes, 
        edges, 
        edges_1_3_mesh_seed, 
        uids,
    )?;
    let (edge_2_direction, edge_2_nodes_numbers) = add_edge(
        point_2_number.clone(),
        point_3_number.clone(),
        nodes,
        edges, 
        edges_2_4_mesh_seed,
        uids,
    )?;
    let (edge_3_direction, edge_3_nodes_numbers) = add_edge(
        point_3_number,
        point_4_number.clone(),
        nodes, edges,
        edges_1_3_mesh_seed,
        uids,
    )?;
    let (edge_4_direction, edge_4_nodes_numbers) = add_edge(
        point_4_number,
        point_1_number,
        nodes,
        edges,
        edges_2_4_mesh_seed,
        uids,
    )?;
    let edge_1_range = 
    {   
        match edge_1_direction 
        {
            Direction::Front => (0..edge_1_nodes_numbers.len()).collect::<Vec<usize>>(),
            Direction::Reverse => (0..edge_1_nodes_numbers.len()).rev().collect::<Vec<usize>>(),              
        }
    };
    for (column, i) in edge_1_range.iter().enumerate()
    {
        nodes_matrix.insert((0, column), edge_1_nodes_numbers[*i].clone());
    }

    let edge_2_range = 
    {   
        match edge_2_direction 
        {
            Direction::Front => (0..edge_2_nodes_numbers.len()).collect::<Vec<usize>>(),
            Direction::Reverse => (0..edge_2_nodes_numbers.len()).rev().collect::<Vec<usize>>(),              
        }
    };
    for (row, i) in edge_2_range.iter().enumerate()
    {
        nodes_matrix.insert((row, edges_1_3_mesh_seed as usize), edge_2_nodes_numbers[*i].clone());
    }

    let edge_3_range = 
    {   
        match edge_3_direction 
        {
            Direction::Front => (0..edge_3_nodes_numbers.len()).rev().collect::<Vec<usize>>(),   
            Direction::Reverse => (0..edge_3_nodes_numbers.len()).collect::<Vec<usize>>(),             
        }
    };
    for (column, i) in edge_3_range.iter().enumerate()
    {
        nodes_matrix.insert((edges_2_4_mesh_seed as usize, column), edge_3_nodes_numbers[*i].clone());
    }

    let edge_4_range = 
    {   
        match edge_4_direction 
        {
            Direction::Front => (0..edge_4_nodes_numbers.len()).rev().collect::<Vec<usize>>(),   
            Direction::Reverse => (0..edge_4_nodes_numbers.len()).collect::<Vec<usize>>(),           
        }
    };
    for (row, i) in edge_4_range.iter().enumerate()
    {
        nodes_matrix.insert((row, 0), edge_4_nodes_numbers[*i].clone());
    }

    Ok(())
}


fn add_inner_nodes_to_nodes_matrix(
    serialized_surface_number: String, 
    nodes: &mut HashMap<String, (u32, [FEFloat; 3])>, 
    edges_1_3_mesh_seed: u8,
    edges_2_4_mesh_seed: u8,
    nodes_matrix: &mut HashMap<(usize, usize), String>,
    props: &Props,
    uids: &mut HashSet<u32>,
)
    -> Result<(), JsValue>
{
    let mut count = 0;
    for row in 1..edges_2_4_mesh_seed as usize
    {
        for column in 1..edges_1_3_mesh_seed as usize
        {
            count += 1;
            let l_1_p_1_number = nodes_matrix.get(&(row, 0)).expect("Element number is absent!");
            let l_1_p_1 = nodes.get(l_1_p_1_number).expect("Element is absent").1;
            let l_1_p_2_number = nodes_matrix.get(&(row, edges_1_3_mesh_seed as usize))
                .expect("Element is absent!");
            let l_1_p_2 = nodes.get(l_1_p_2_number).expect("Element is absent").1;
            let l_2_p_1_number = nodes_matrix.get(&(0, column)).expect("Element number is absent!");
            let l_2_p_1 = nodes.get(l_2_p_1_number).expect("Element is absent").1;
            let l_2_p_2_number = nodes_matrix.get(&(edges_2_4_mesh_seed as usize, column))
                .expect("Element is absent!");
            let l_2_p_2 = nodes.get(l_2_p_2_number).expect("Element is absent").1;
            let inner_node_number = format!("{serialized_surface_number}_{count}");
            let inner_node_components = find_lines_intersection(
                &l_1_p_1, &l_1_p_2, &l_2_p_1, &l_2_p_2, props.abs_tol, props.rel_tol,
            )?;

            let uid = generate_uid(uids);

            nodes.insert(inner_node_number.clone(), (uid, inner_node_components));
            nodes_matrix.insert((row, column), inner_node_number);
        }
    }

    Ok(())
}


pub(super) fn extract_plates_from_input_data(
    extracted_plates_data: &Value, 
    nodes: &mut HashMap<String, (u32, [FEFloat; 3])>, 
    edges: &mut HashMap<String, u8>, 
    plate_elements: &mut HashMap<u32, (u32, [String; 4], [FEFloat; 4])>, 
    surfaces: &mut HashMap<String, Vec<u32>>, 
    props: &Props,
    uids: &mut HashSet<u32>,
) 
    -> Result<(), JsValue>
{
    let plates_data = extracted_plates_data.as_object().ok_or(
        JsValue::from("Mesher: Plates data could not be converted into object!"))?;
    for (serialized_surface_number, serialized_plate_data) in plates_data.iter()
    {
        let plate_data = serialized_plate_data
            .as_array()
            .ok_or(JsValue::from("Mesher: Serialized plate data could not be converted into array!"))?;

        let point_1_number = plate_data[0].to_string();
        check_node_number_existence(point_1_number.clone(), nodes)?;

        let point_2_number = plate_data[1].to_string();
        check_node_number_existence(point_2_number.clone(), nodes)?;

        let point_3_number = plate_data[2].to_string();
        check_node_number_existence(point_3_number.clone(), nodes)?;

        let point_4_number = plate_data[3].to_string();
        check_node_number_existence(point_4_number.clone(), nodes)?;

        let young_modulus = plate_data[4]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Young's modulus could not be converted to FEFloat!")))?;

        let poisson_ratio = plate_data[5]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Poisson's ratio could not be converted to FEFloat!")))?;

        let thickness = plate_data[6]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Thickness could not be converted to FEFloat!")))?;

        let shear_factor = plate_data[7]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Shear factor could not be converted to FEFloat!")))?;

        let edges_1_3_mesh_seed = plate_data[8]
            .to_string()
            .parse::<u8>()
            .or(Err(JsValue::from("Mesher: Edges 1 3 mesh seed could not be converted to u8!")))?;

        let edges_2_4_mesh_seed = plate_data[9]
            .to_string()
            .parse::<u8>()
            .or(Err(JsValue::from("Mesher: Edges 2 4 mesh seed could not be converted to u8!")))?;

        let mut nodes_matrix = HashMap::new();
        add_edge_nodes_to_nodes_matrix(
            point_1_number,
            point_2_number,
            point_3_number,
            point_4_number,
            nodes,
            edges, 
            edges_1_3_mesh_seed,
            edges_2_4_mesh_seed,
            &mut nodes_matrix,
            uids,
        )?;
        add_inner_nodes_to_nodes_matrix(
            serialized_surface_number.to_string(), 
            nodes, 
            edges_1_3_mesh_seed, 
            edges_2_4_mesh_seed, 
            &mut nodes_matrix, 
            props,
            uids,
        )?;

        let mut plate_elements_numbers = Vec::new();
        let mut count = 0;
        for row in 0..edges_2_4_mesh_seed as usize
        {
            for column in 0..edges_1_3_mesh_seed as usize
            {
                count += 1;
                let mut str_plate_element_number = format!("{}", props.plate_elements_group_number);
                (0..props.max_surface_number.to_string().len() - serialized_surface_number.len())
                    .for_each(|_| str_plate_element_number += "0");
                str_plate_element_number += &format!("{serialized_surface_number}{count}");
                let plate_element_number = str_plate_element_number
                    .parse::<u32>()
                    .or(Err(JsValue::from("Mesher: Beam element number could not be converted to u32!")))?;
                let node_1_number = nodes_matrix.get(&(row, column)).expect("Element is absent!");
                let node_2_number = nodes_matrix.get(&(row, column + 1)).expect("Element is absent!");
                let node_3_number = nodes_matrix.get(&(row + 1, column + 1)).expect("Element is absent!");
                let node_4_number = nodes_matrix.get(&(row + 1, column)).expect("Element is absent!");

                let uid = generate_uid(uids);

                let plate_element_data = 
                    (
                        uid,
                        [
                            node_1_number.to_string(),
                            node_2_number.to_string(),
                            node_3_number.to_string(),
                            node_4_number.to_string(),
                        ],
                        [
                            young_modulus, 
                            poisson_ratio, 
                            thickness, 
                            shear_factor, 
                        ],
                    );
                plate_elements.insert(plate_element_number, plate_element_data);
                plate_elements_numbers.push(plate_element_number);
            }
        }

        surfaces.insert(serialized_surface_number.to_string(), plate_elements_numbers);
    }
    Ok(())
}


pub(super) fn check_plate_element_existence(
    plate_element_number: u32, 
    plate_elements: &HashMap<u32, (u32, [String; 4], [FEFloat; 4])>, 
) 
    -> Result<(), JsValue>
{
    if !plate_elements.contains_key(&plate_element_number)
    {
        return Err(JsValue::from(&format!("Mesher: Plate element with number {plate_element_number} doesn't exist!")));
    }
    Ok(())
}
