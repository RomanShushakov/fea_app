use serde_json::{Map, Value};
use wasm_bindgen::prelude::JsValue;
use std::collections::HashMap;

use crate::structs::
{
    NodeData, TrussElementData, ExtremeElementsAnalysisValues, BeamElementData, LineElementData, QuadElementData,
    PlateElementData,
};
use crate::functions::convert_vec_to_array;
use crate::props::Props;


fn convert_analysis_result_value_to_extreme_values_array(
    key: &str, analysis_result_value: &Value,
) 
    -> Result<[f32; 2], JsValue>
{
    let extreme_analysis_result_value_vec = analysis_result_value
        .as_array()
        .ok_or(JsValue::from(format!("Renderer: Extreme {key} values could not be converted into array!")))?
        .iter()
        .map(
            |value| 
            value
                .to_string()
                .parse::<f32>()
                .or(Err(JsValue::from(format!("Renderer: Extreme {key} value could not be converted to f32!"))))
            )
        .collect::<Result<Vec<f32>, JsValue>>()?;
    let extreme_analysis_result_value_array = convert_vec_to_array(extreme_analysis_result_value_vec);
    Ok(extreme_analysis_result_value_array)
}


pub fn get_converted_extreme_elements_loads_values(
    extreme_elements_loads_values: &Map<String, Value>,
) 
    -> Result<ExtremeElementsAnalysisValues, JsValue>
{
    let mut extreme_elements_loads_data = ExtremeElementsAnalysisValues::init();

    if let Some(force_r) = extreme_elements_loads_values.get("force_r")
    {
        let extreme_force_r = convert_analysis_result_value_to_extreme_values_array(
            "force_r", force_r,
        )?;
        extreme_elements_loads_data.update_f_r_loads(extreme_force_r);
    }

    if let Some(force_s) = extreme_elements_loads_values.get("force_s")
    {
        let extreme_force_s = convert_analysis_result_value_to_extreme_values_array(
            "force_s", force_s,
        )?;
        extreme_elements_loads_data.update_f_s_loads(extreme_force_s);
    }

    if let Some(force_t) = extreme_elements_loads_values.get("force_t")
    {
        let extreme_force_t = convert_analysis_result_value_to_extreme_values_array(
            "force_t", force_t,
        )?;
        extreme_elements_loads_data.update_f_t_loads(extreme_force_t);
    }

    if let Some(moment_r) = extreme_elements_loads_values.get("moment_r")
    {
        let extreme_moment_r = convert_analysis_result_value_to_extreme_values_array(
            "moment_r", moment_r,
        )?;
        extreme_elements_loads_data.update_m_r_loads(extreme_moment_r);
    }

    if let Some(moment_s) = extreme_elements_loads_values.get("moment_s")
    {
        let extreme_moment_s = convert_analysis_result_value_to_extreme_values_array(
            "moment_s", moment_s,
        )?;
        extreme_elements_loads_data.update_m_s_loads(extreme_moment_s);
    }

    if let Some(moment_t) = extreme_elements_loads_values.get("moment_t")
    {
        let extreme_moment_t = convert_analysis_result_value_to_extreme_values_array(
            "moment_t", moment_t,
        )?;
        extreme_elements_loads_data.update_m_t_loads(extreme_moment_t);
    }

    if let Some(membrane_force_r) = extreme_elements_loads_values.get("membrane_force_r")
    {
        let extreme_membrane_force_r = convert_analysis_result_value_to_extreme_values_array(
            "membrane_force_r", membrane_force_r,
        )?;
        extreme_elements_loads_data.update_mem_f_r_loads(extreme_membrane_force_r);
    }

    if let Some(membrane_force_s) = extreme_elements_loads_values.get("membrane_force_s")
    {
        let extreme_membrane_force_s = convert_analysis_result_value_to_extreme_values_array(
            "membrane_force_s", membrane_force_s,
        )?;
        extreme_elements_loads_data.update_mem_f_s_loads(extreme_membrane_force_s);
    }

    if let Some(membrane_force_r_s) = extreme_elements_loads_values.get("membrane_force_r_s")
    {
        let extreme_membrane_force_r_s = convert_analysis_result_value_to_extreme_values_array(
            "membrane_force_r_s", membrane_force_r_s,
        )?;
        extreme_elements_loads_data.update_mem_f_r_s_loads(extreme_membrane_force_r_s);
    }

    if let Some(bending_moment_r) = extreme_elements_loads_values.get("bending_moment_r")
    {
        let extreme_bending_moment_r = convert_analysis_result_value_to_extreme_values_array(
            "bending_moment_r", bending_moment_r,
        )?;
        extreme_elements_loads_data.update_bend_m_r_loads(extreme_bending_moment_r);
    }

    if let Some(bending_moment_s) = extreme_elements_loads_values.get("bending_moment_s")
    {
        let extreme_bending_moment_s = convert_analysis_result_value_to_extreme_values_array(
            "bending_moment_s", bending_moment_s,
        )?;
        extreme_elements_loads_data.update_bend_m_s_loads(extreme_bending_moment_s);
    }

    if let Some(bending_moment_r_s) = extreme_elements_loads_values.get("bending_moment_r_s")
    {
        let extreme_bending_moment_r_s = convert_analysis_result_value_to_extreme_values_array(
            "bending_moment_r_s", bending_moment_r_s,
        )?;
        extreme_elements_loads_data.update_bend_m_r_s_loads(extreme_bending_moment_r_s);
    }

    if let Some(shear_force_r_t) = extreme_elements_loads_values.get("shear_force_r_t")
    {
        let extreme_shear_force_r_t = convert_analysis_result_value_to_extreme_values_array(
            "shear_force_r_t", shear_force_r_t,
        )?;
        extreme_elements_loads_data.update_shear_f_r_t_loads(extreme_shear_force_r_t);
    }

    if let Some(shear_force_s_t) = extreme_elements_loads_values.get("shear_force_s_t")
    {
        let extreme_shear_force_s_t = convert_analysis_result_value_to_extreme_values_array(
            "shear_force_s_t", shear_force_s_t,
        )?;
        extreme_elements_loads_data.update_shear_f_s_t_loads(extreme_shear_force_s_t);
    }

    Ok(extreme_elements_loads_data)
}


fn convert_line_element_data(
    stringified_line_element_number: &str, 
    line_element_data: &Value,
    converted_nodes_data: &HashMap<String, NodeData>,
) 
    -> Result<(u32, LineElementData), JsValue>
{
    let line_element_number = stringified_line_element_number
        .parse::<u32>()
        .or(Err(JsValue::from("Renderer: Line element number could not be converted to u32!")))?;

    let line_element_data_array = line_element_data
        .as_array()
        .ok_or(JsValue::from("Renderer: Line element data array could not be extracted!"))?;

    let uid = line_element_data_array[0]
        .to_string()
        .parse::<u32>()
        .or(Err(JsValue::from("Renderer: Line element uid could not be converted to u32!")))?;

    let line_element_nodes_names_array = line_element_data_array[1]
        .as_array()
        .ok_or(JsValue::from("Renderer: Line element nodes names array could not be extracted!"))?;

    let node_1_data = converted_nodes_data
        .get(
            line_element_nodes_names_array[0]
                .as_str()
                .ok_or(
                    JsValue::from("Renderer: Line element node 1 name array could not be converted to str!"),
                )?
        )
        .ok_or(JsValue::from("Renderer: Line element node 1 could not be extracted!"))?;

    let node_2_data = converted_nodes_data
        .get(line_element_nodes_names_array[1]
            .as_str()
            .ok_or(
                JsValue::from("Renderer: Line element node 2 name array could not be converted to str!"),
            )?
        )
        .ok_or(JsValue::from("Renderer: Line element node 2 could not be extracted!"))?;

    Ok
    (
        (
            line_element_number,
            LineElementData::create(
                uid, 
                [node_1_data.x, node_1_data.y, node_1_data.z],
                [node_2_data.x, node_2_data.y, node_2_data.z],
                [node_1_data.ux, node_1_data.uy, node_1_data.uz],
                [node_2_data.ux, node_2_data.uy, node_2_data.uz],
                node_1_data.optional_u_result_coeff,
                node_2_data.optional_u_result_coeff,
            ),
        )
    )
}


pub fn convert_truss_elements_data(
    truss_elements_data: &Map<String, Value>, 
    converted_nodes_data: &HashMap<String, NodeData>,
    converted_truss_elements_data: &mut HashMap<u32, TrussElementData>,
)
    -> Result<(), JsValue>
{
    for (stringified_truss_element_number, truss_element_data) in truss_elements_data.iter()
    {
        let (line_element_number, line_element_data) = convert_line_element_data(
            stringified_truss_element_number,
            truss_element_data, 
            converted_nodes_data,
        )?;

        let truss_element_data = TrussElementData::init(line_element_data);
        converted_truss_elements_data.insert(line_element_number, truss_element_data);
    }
    Ok(())
}


pub fn update_truss_elements_data(
    truss_elements_analysis_result_data: &Map<String, Value>,
    converted_truss_elements_data: &mut HashMap<u32, TrussElementData>,
    converted_extreme_elements_loads_data: &ExtremeElementsAnalysisValues,
    props: &Props,
)
    -> Result<(), JsValue>
{
    for (stringified_truss_element_number, truss_element_data) in 
        truss_elements_analysis_result_data.iter()
    {
        let truss_element_number = stringified_truss_element_number
            .parse::<u32>()
            .or(Err(JsValue::from("Renderer: Truss element number could not be converted to u32!")))?;

        let truss_element_data_array = truss_element_data
            .as_array()
            .ok_or(
                JsValue::from("Renderer: Truss element analysis result data could not be converted into array!"),
            )?;

        let rotation_matrix_elements_vec = truss_element_data_array[..9]
            .iter()
            .map(
                |value| 
                value
                    .to_string()
                    .parse::<f32>()
                    .or(Err(JsValue::from("Renderer: Rotation matrix element value \
                        could not be converted to f32!")))
                )
            .collect::<Result<Vec<f32>, JsValue>>()?;
        let rotation_matrix_elements = convert_vec_to_array(rotation_matrix_elements_vec);

        let force_r = truss_element_data_array[9]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Force r value could not be converted to f32!")))?;

        let truss_element_data = converted_truss_elements_data
            .get_mut(&truss_element_number)
            .ok_or(JsValue::from(format!("Renderer: Truss element number {truss_element_number} is absent!")))?;

        truss_element_data.update(rotation_matrix_elements, force_r, converted_extreme_elements_loads_data, props);
    }
    Ok(())
}


pub fn convert_beam_elements_data(
    beam_elements_data: &Map<String, Value>, 
    converted_nodes_data: &HashMap<String, NodeData>,
    converted_beam_elements_data: &mut HashMap<u32, BeamElementData>,
)
    -> Result<(), JsValue>
{
    for (stringified_beam_element_number, beam_element_data) in beam_elements_data.iter()
    {
        let (line_element_number, line_element_data) = convert_line_element_data(
            stringified_beam_element_number,
            beam_element_data, 
            converted_nodes_data,
        )?;

        let beam_element_data = BeamElementData::init(line_element_data);
        converted_beam_elements_data.insert(line_element_number, beam_element_data);
    }
    Ok(())
}


pub fn update_beam_elements_data(
    beam_elements_analysis_result_data: &Map<String, Value>,
    converted_beam_elements_data: &mut HashMap<u32, BeamElementData>,
    converted_extreme_elements_loads_data: &ExtremeElementsAnalysisValues,
    props: &Props,
)
    -> Result<(), JsValue>
{
    for (stringified_beam_element_number, beam_element_data) in 
        beam_elements_analysis_result_data.iter()
    {
        let beam_element_number = stringified_beam_element_number
            .parse::<u32>()
            .or(Err(JsValue::from("Renderer: Beam element number could not be converted to u32!")))?;

        let beam_element_data_array = beam_element_data
            .as_array()
            .ok_or(
                JsValue::from("Renderer: Beam element analysis result data could not be converted into array!"),
            )?;

        let rotation_matrix_elements_vec = beam_element_data_array[..9]
            .iter()
            .map(
                |value| 
                value
                    .to_string()
                    .parse::<f32>()
                    .or(Err(JsValue::from("Renderer: Rotation matrix element value \
                        could not be converted to f32!")))
                )
            .collect::<Result<Vec<f32>, JsValue>>()?;
        let rotation_matrix_elements = convert_vec_to_array(rotation_matrix_elements_vec);

        let force_r = beam_element_data_array[9]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Force r value could not be converted to f32!")))?;

        let force_s = beam_element_data_array[10]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Force s value could not be converted to f32!")))?;

        let force_t = beam_element_data_array[11]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Force t value could not be converted to f32!")))?;

        let moment_r = beam_element_data_array[12]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Moment r value could not be converted to f32!")))?;

        let moment_s_node_1 = beam_element_data_array[13]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Moment s at node 1 value could not be converted to f32!")))?;

        let moment_s = beam_element_data_array[14]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Moment s value could not be converted to f32!")))?;

        let moment_s_node_2 = beam_element_data_array[15]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Moment s at node 2 value could not be converted to f32!")))?;

        let moment_t_node_1 = beam_element_data_array[16]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Moment t at node 1 value could not be converted to f32!")))?;

        let moment_t = beam_element_data_array[17]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Moment t value could not be converted to f32!")))?;

        let moment_t_node_2 = beam_element_data_array[18]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Moment t at node 2 value could not be converted to f32!")))?;

        let beam_element_data = converted_beam_elements_data
            .get_mut(&beam_element_number)
            .ok_or(JsValue::from(format!("Renderer: Beam element number {beam_element_number} is absent!")))?;

        beam_element_data.update(
            rotation_matrix_elements, 
            force_r,
            force_s,
            force_t,
            moment_r,
            moment_s_node_1,
            moment_s,
            moment_s_node_2,
            moment_t_node_1,
            moment_t,
            moment_t_node_2,
            converted_extreme_elements_loads_data,
            props,
        );
    }
    Ok(())
}


fn convert_quad_element_data(
    stringified_quad_element_number: &str, 
    quad_element_data: &Value,
    converted_nodes_data: &HashMap<String, NodeData>,
) 
    -> Result<(u32, QuadElementData), JsValue>
{
    let quad_element_number = stringified_quad_element_number
        .parse::<u32>()
        .or(Err(JsValue::from("Renderer: Quad element number could not be converted to u32!")))?;

    let quad_element_data_array = quad_element_data
        .as_array()
        .ok_or(JsValue::from("Renderer: Quad element data array could not be extracted!"))?;

    let uid = quad_element_data_array[0]
        .to_string()
        .parse::<u32>()
        .or(Err(JsValue::from("Renderer: Quad element uid could not be converted to u32!")))?;

    let quad_element_nodes_names_array = quad_element_data_array[1]
        .as_array()
        .ok_or(JsValue::from("Renderer: Quad element nodes names array could not be extracted!"))?;

    let node_1_data = converted_nodes_data
        .get(
            quad_element_nodes_names_array[0]
                .as_str()
                .ok_or(
                    JsValue::from("Renderer: Quad element node 1 name array could not be converted to str!"),
                )?
        )
        .ok_or(JsValue::from("Renderer: Quad element node 1 could not be extracted!"))?;

    let node_2_data = converted_nodes_data
        .get(quad_element_nodes_names_array[1]
            .as_str()
            .ok_or(
                JsValue::from("Renderer: Quad element node 2 name array could not be converted to str!"),
            )?
        )
        .ok_or(JsValue::from("Renderer: Quad element node 2 could not be extracted!"))?;

    let node_3_data = converted_nodes_data
        .get(quad_element_nodes_names_array[2]
            .as_str()
            .ok_or(
                JsValue::from("Renderer: Quad element node 3 name array could not be converted to str!"),
            )?
        )
        .ok_or(JsValue::from("Renderer: Quad element node 3 could not be extracted!"))?;

    let node_4_data = converted_nodes_data
        .get(quad_element_nodes_names_array[3]
            .as_str()
            .ok_or(
                JsValue::from("Renderer: Quad element node 3 name array could not be converted to str!"),
            )?
        )
        .ok_or(JsValue::from("Renderer: Quad element node 3 could not be extracted!"))?;

    Ok
    (
        (
            quad_element_number,
            QuadElementData::create(
                uid, 
                [node_1_data.x, node_1_data.y, node_1_data.z],
                [node_2_data.x, node_2_data.y, node_2_data.z],
                [node_3_data.x, node_3_data.y, node_3_data.z],
                [node_4_data.x, node_4_data.y, node_4_data.z],
                [node_1_data.ux, node_1_data.uy, node_1_data.uz],
                [node_2_data.ux, node_2_data.uy, node_2_data.uz],
                [node_3_data.ux, node_3_data.uy, node_3_data.uz],
                [node_4_data.ux, node_4_data.uy, node_4_data.uz],
                node_1_data.optional_u_result_coeff,
                node_2_data.optional_u_result_coeff,
                node_3_data.optional_u_result_coeff,
                node_4_data.optional_u_result_coeff,
            ),
        )
    )
}


pub fn convert_plate_elements_data(
    plate_elements_data: &Map<String, Value>, 
    converted_nodes_data: &HashMap<String, NodeData>,
    converted_plate_elements_data: &mut HashMap<u32, PlateElementData>,
)
    -> Result<(), JsValue>
{
    for (stringified_plate_element_number, beam_element_data) in plate_elements_data.iter()
    {
        let (quad_element_number, quad_element_data) = convert_quad_element_data(
            stringified_plate_element_number,
            beam_element_data, 
            converted_nodes_data,
        )?;

        let plate_element_data = PlateElementData::init(quad_element_data);
        converted_plate_elements_data.insert(quad_element_number, plate_element_data);
    }
    Ok(())
}


pub fn update_plate_elements_data(
    plate_elements_analysis_result_data: &Map<String, Value>,
    converted_plate_elements_data: &mut HashMap<u32, PlateElementData>,
    converted_extreme_elements_loads_data: &ExtremeElementsAnalysisValues,
    props: &Props,
)
    -> Result<(), JsValue>
{
    for (stringified_plate_element_number, plate_element_data) in 
        plate_elements_analysis_result_data.iter()
    {
        let plate_element_number = stringified_plate_element_number
            .parse::<u32>()
            .or(Err(JsValue::from("Renderer: Plate element number could not be converted to u32!")))?;

        let plate_element_data_array = plate_element_data
            .as_array()
            .ok_or(
                JsValue::from("Renderer: Plate element analysis result data could not be converted into array!"),
            )?;

        let rotation_matrix_elements_vec = plate_element_data_array[..9]
            .iter()
            .map(
                |value| 
                value
                    .to_string()
                    .parse::<f32>()
                    .or(Err(JsValue::from("Renderer: Rotation matrix element value \
                        could not be converted to f32!")))
                )
            .collect::<Result<Vec<f32>, JsValue>>()?;
        let rotation_matrix_elements = convert_vec_to_array(rotation_matrix_elements_vec);

        let mem_force_r = plate_element_data_array[9]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Membrane force r value could not be converted to f32!")))?;

        let mem_force_s = plate_element_data_array[10]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Membrane force s value could not be converted to f32!")))?;

        let mem_force_r_s = plate_element_data_array[11]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Membrane force r-s value could not be converted to f32!")))?;

        let bend_moment_r = plate_element_data_array[12]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Bending moment r value could not be converted to f32!")))?;

        let bend_moment_s = plate_element_data_array[13]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Bending moment s value could not be converted to f32!")))?;

        let bend_moment_r_s = plate_element_data_array[14]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Bending moment r-s value could not be converted to f32!")))?;

        let shear_force_r_t = plate_element_data_array[15]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Shear force r-t value could not be converted to f32!")))?;

        let shear_force_s_t = plate_element_data_array[16]
            .to_string()
            .parse::<f32>()
            .or(Err(JsValue::from("Renderer: Shear force s-t value could not be converted to f32!")))?;

        let plate_element_data = converted_plate_elements_data
            .get_mut(&plate_element_number)
            .ok_or(JsValue::from(format!("Renderer: Plate element number {plate_element_number} is absent!")))?;

        plate_element_data.update(
            rotation_matrix_elements, 
            mem_force_r,
            mem_force_s,
            mem_force_r_s,
            bend_moment_r,
            bend_moment_s,
            bend_moment_r_s,
            shear_force_r_t,
            shear_force_s_t,
            converted_extreme_elements_loads_data,
            props,
        );
    }
    Ok(())
}
