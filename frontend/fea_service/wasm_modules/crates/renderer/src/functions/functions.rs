use wasm_bindgen::prelude::{JsCast, JsValue, wasm_bindgen};
use std::array::TryFromSliceError;
use std::cell::RefCell;
use std::rc::Rc;
use std::convert::TryInto;
use serde::Serialize;
use serde_wasm_bindgen::Serializer;

use std::collections::{HashMap, HashSet};

use crate::props::Props;


#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(value: &str);
}


pub fn dispatch_custom_event(detail: serde_json::Value, event_type: &str, query_selector: &str)
    -> Result<(), JsValue>
{
    let serializer = Serializer::json_compatible();
    let custom_event = web_sys::CustomEvent::new_with_event_init_dict(
        event_type,
        web_sys::CustomEventInit::new()
            .bubbles(true)
            .composed(true)
            .detail(&detail.serialize(&serializer)
                .or(Err("Renderer: Dispatch event: detail could not be \
                converted into JsValue!"))?))
                    .or(Err(JsValue::from("Renderer: Dispatch event: \
                    custom event could not be constructed!")))?;
    web_sys::window().expect("no global `window` exists")
        .document().expect("should have a document on window")
        .query_selector(query_selector).or(Err(JsValue::from("Renderer: Dispatch event: No \
            element find by current selector!")))?.unwrap()
        .dyn_into::<web_sys::EventTarget>().unwrap()
        .dispatch_event(&custom_event)?;
    Ok(())
}


pub fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4]
{
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    [b1, b2, b3, b4]
}


pub fn transform_array_of_u8_to_u32(b: &[u8; 4]) -> u32
{
    ((b[0] as u32) << 24) + ((b[1] as u32) << 16) + ((b[2] as u32) << 8) + (b[3] as u32)
}


pub fn get_value_coeff(optional_extreme_value: &Option<[f32; 2]>, value: f32, props: &Props) -> Option<[f32; 5]>
{
    if let Some([min_value, max_value]) = optional_extreme_value
    {
        let abs_coeff = value / min_value.abs().max(max_value.abs());
        let color_coeff = if min_value == max_value { 0.0 } else { (value - min_value) / (max_value - min_value) };
        Some([
            abs_coeff, 
            props.color_bar_min_color[0] + color_coeff, 
            props.color_bar_min_color[1] - color_coeff, 
            props.color_bar_min_color[2] - color_coeff, 
            props.color_bar_min_color[3],
        ])
    }
    else
    {
        None
    }
}


pub fn convert_vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] 
{
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}


pub fn convert_slice_to_array<T, const N: usize>(s: &[T]) -> [T; N]
    where T: Copy,
{
    s.try_into()
        .unwrap_or_else(|e: TryFromSliceError| panic!("Expected a Slice of length {}, but error {} appears!", N, e))
}


pub fn find_grid_points_coordinates(n: u32, point_1_coordinates: &[f32; 3], point_2_coordinates: &[f32; 3],
    point_3_coordinates: &[f32; 3], point_4_coordinates: &[f32; 3]) -> Vec<[f32; 3]>
{
    let mut grid_points = Vec::new();

    let step_1 = (0..3).map(|i| (point_2_coordinates[i] - point_1_coordinates[i]) / (n - 1) as f32)
        .collect::<Vec<f32>>();
    let step_3 = (0..3).map(|i| (point_4_coordinates[i] - point_3_coordinates[i]) / (n - 1) as f32)
        .collect::<Vec<f32>>();
    
    for i in 0..n
    {
        let start_gp = (0..3).map(|k| point_1_coordinates[k] + step_1[k] * i as f32).collect::<Vec<f32>>();
        let end_gp = (0..3).map(|k| point_4_coordinates[k] - step_3[k] * i as f32).collect::<Vec<f32>>();
        let step = (0..3).map(|k| (end_gp[k] - start_gp[k]) / (n - 1) as f32).collect::<Vec<f32>>();
        for j in 0..n
        {
            let gp = (0..3).map(|k| start_gp[k] + step[k] * j as f32).collect::<Vec<f32>>();
            let grid_point = convert_vec_to_array(gp);
            grid_points.push(grid_point);
        }
    }

    grid_points
}


pub fn move_selected_objects_into_regular<T>(
    selected_colors: &HashSet<[u8; 4]>,
    selected_objects: &mut HashMap<[u8; 4], T>,
    regular_objects: &mut HashMap<[u8; 4], T>,
)
    where T: Clone
{
    for key in selected_objects.clone().keys()
    {
        if !selected_colors.contains(key)
        {
            let (transformed_uid, obj) = 
                selected_objects.remove_entry(key).unwrap();
            {
                regular_objects.insert(transformed_uid, obj);
            }
        }
    }
}


pub fn move_rc_selected_objects_into_rc_regular<T>(
    selected_colors: &HashSet<[u8; 4]>,
    selected_objects: &mut HashMap<[u8; 4], Rc<RefCell<T>>>,
    regular_objects: &mut HashMap<[u8; 4], Rc<RefCell<T>>>,
)
    where T: Clone
{
    for key in selected_objects.clone().keys()
    {
        if !selected_colors.contains(key)
        {
            let (transformed_uid, obj) = 
                selected_objects.remove_entry(key).unwrap();
            {
                regular_objects.insert(transformed_uid, obj);
            }
        }
    }
}


pub fn move_regular_object_into_selected_objects<T>(
    selected_color: &[u8; 4],
    selected_uids: &mut Vec<u32>,
    regular_objects: &mut HashMap<[u8; 4], T>,
    selected_objects: &mut HashMap<[u8; 4], T>,
)
{
    if let Some((transformed_uid, selected_object)) = regular_objects.remove_entry(selected_color)
    {
        let uid = transform_array_of_u8_to_u32(&transformed_uid);
        selected_uids.push(uid);
        selected_objects.insert(transformed_uid, selected_object);
    }
}


pub fn move_rc_regular_object_into_rc_selected_objects<T>(
    selected_color: &[u8; 4],
    selected_uids: &mut Vec<u32>,
    regular_objects: &mut HashMap<[u8; 4], Rc<RefCell<T>>>,
    selected_objects: &mut HashMap<[u8; 4], Rc<RefCell<T>>>,
)
{
    if let Some((transformed_uid, selected_object)) = regular_objects.remove_entry(selected_color)
    {
        let uid = transform_array_of_u8_to_u32(&transformed_uid);
        selected_uids.push(uid);
        selected_objects.insert(transformed_uid, selected_object);
    }
}
