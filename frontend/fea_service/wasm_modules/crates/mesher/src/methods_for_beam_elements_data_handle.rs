use serde_json::Value;
use wasm_bindgen::prelude::JsValue;

use std::collections::{HashMap, HashSet};

use crate::functions::generate_uid;
use crate::types::FEFloat;
use crate::enums::Direction;
use crate::props::Props;
use crate::methods_for_nodes_data_handle::check_node_number_existence;
use crate::methods_for_edges_data_handle::add_edge;


pub(super) fn extract_beams_from_input_data(
    extracted_beams_data: &Value, 
    nodes: &mut HashMap<String, (u32, [FEFloat; 3])>, 
    edges: &mut HashMap<String, u8>, 
    beam_elements: &mut HashMap<u32, (u32, [String; 2], [FEFloat; 11])>, 
    lines: &mut HashMap<String, Vec<u32>>, 
    props: &Props,
    uids: &mut HashSet<u32>,
) 
    -> Result<(), JsValue>
{
    let beams_data = extracted_beams_data.as_object().ok_or(
        JsValue::from("Mesher: Beams data could not be converted into object!"))?;
    for (serialized_line_number, serialized_beam_data) in beams_data.iter()
    {
        let beam_data = serialized_beam_data
            .as_array()
            .ok_or(JsValue::from("Mesher: Serialized beam data could not be converted into array!"))?;

        let point_1_number = beam_data[0].to_string();
        check_node_number_existence(point_1_number.clone(), nodes)?;

        let point_2_number = beam_data[1].to_string();
        check_node_number_existence(point_2_number.clone(), nodes)?;

        let young_modulus = beam_data[2]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Young's modulus could not be converted to FEFloat!")))?;

        let poisson_ratio = beam_data[3]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Poisson's ratio could not be converted to FEFloat!")))?;

        let area = beam_data[4]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Area could not be converted to FEFloat!")))?;

        let i11 = beam_data[5]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: I11 could not be converted to FEFloat!")))?;

        let i22 = beam_data[6]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: I22 could not be converted to FEFloat!")))?;

        let i12 = beam_data[7]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: I12 could not be converted to FEFloat!")))?;

        let it = beam_data[8]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: It could not be converted to FEFloat!")))?;

        let shear_factor = beam_data[9]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Shear factor could not be converted to FEFloat!")))?;

        let local_axis_1_direction_x = beam_data[10]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Local axis 1 direction x could not be converted to FEFloat!")))?;

        let local_axis_1_direction_y = beam_data[11]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Local axis 1 direction y could not be converted to FEFloat!")))?;

        let local_axis_1_direction_z = beam_data[12]
            .to_string()
            .parse::<FEFloat>()
            .or(Err(JsValue::from("Mesher: Local axis 1 direction z could not be converted to FEFloat!")))?;

        let mesh_seed = beam_data[13]
            .to_string()
            .parse::<u8>()
            .or(Err(JsValue::from("Mesher: Mesh seed could not be converted to u8!")))?;

        let (direction, edge_nodes_numbers) = add_edge(
            point_1_number, point_2_number, nodes, edges, mesh_seed, uids,
        )?;

        let range = 
        {   
            match direction 
            {
                Direction::Front => (1..edge_nodes_numbers.len()).collect::<Vec<usize>>(),
                Direction::Reverse => (1..edge_nodes_numbers.len()).rev().collect::<Vec<usize>>(),              
            }
        };

        let mut beam_elements_numbers = Vec::new();

        for i in range
        {
            let mut str_beam_element_number = format!("{}", props.beam_elements_group_number);
            (0..props.max_line_number.to_string().len() - serialized_line_number.len())
                .for_each(|_| str_beam_element_number += "0");
            str_beam_element_number += &serialized_line_number.to_string();

            match direction
            {
                Direction::Front => str_beam_element_number += &i.to_string(),
                Direction::Reverse => str_beam_element_number += &(edge_nodes_numbers.len() - i).to_string(),
            }

            let beam_element_number = str_beam_element_number
                .parse::<u32>()
                .or(Err(JsValue::from("Mesher: Beam element number could not be converted to u32!")))?;

            let nodes_numbers = 
            {
                match direction
                {
                    Direction::Front => [edge_nodes_numbers[i - 1].clone(), edge_nodes_numbers[i].clone()],
                    Direction::Reverse => [edge_nodes_numbers[i].clone(), edge_nodes_numbers[i - 1].clone()],
                }
            };

            let uid = generate_uid(uids);

            let beam_element_data = 
            (
                uid,
                nodes_numbers,
                [
                    young_modulus, 
                    poisson_ratio, 
                    area, 
                    i11, 
                    i22, 
                    i12, 
                    it, 
                    shear_factor, 
                    local_axis_1_direction_x,
                    local_axis_1_direction_y,
                    local_axis_1_direction_z,
                ]
            );

            beam_elements_numbers.push(beam_element_number);
            beam_elements.insert(beam_element_number, beam_element_data);
        }

        lines.insert(serialized_line_number.to_string(), beam_elements_numbers);
    }
    Ok(())
}


pub(super) fn check_beam_element_existence(
    beam_element_number: u32, 
    beam_elements: &HashMap<u32, (u32, [String; 2], [FEFloat; 11])>, 
) 
    -> Result<(), JsValue>
{
    if !beam_elements.contains_key(&beam_element_number)
    {
        return Err(JsValue::from(&format!("Mesher: Beam element with number {beam_element_number} doesn't exist!")));
    }
    Ok(())
}
