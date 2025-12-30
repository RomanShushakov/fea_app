use extended_matrix::VectorTrait;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::collections::HashMap;
use serde::Serialize;
use serde_wasm_bindgen::Serializer;

use extended_matrix::Vector3;

use crate::types::FEFloat;
use crate::traits::{RelativeTrait, StatusTrait, ServerNotificationTrait};
use crate::enums::{RelativeKey, Status, NotificationType};
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
    let custom_event = web_sys::CustomEvent::new_with_event_init_dict(event_type,
        web_sys::CustomEventInit::new()
            .bubbles(true)
            .composed(true)
            .detail(&detail.serialize(&serializer).or(Err("Geometry: Dispatch event: \
                detail could not be converted into JsValue!"))?))
            .or(Err(JsValue::from("Dispatch event: custom event could not be \
                constructed!")))?;
    web_sys::window().expect("no global `window` exists")
        .document().expect("should have a document on window")
        .query_selector(query_selector).or(Err(JsValue::from("Geometry: Dispatch event: No \
            element find by current selector!")))?
        .unwrap()
        .dyn_into::<web_sys::EventTarget>()
        .unwrap()
        .dispatch_event(&custom_event)?;
    Ok(())
}


pub fn find_projection_of_vector_a_perpendicular_to_vector_b(vec_a_components: &[FEFloat; 3], 
    vec_b_components: &[FEFloat; 3]) -> [FEFloat; 3]
{
    let a = Vector3::create(vec_a_components);
    let b = Vector3::create(vec_b_components);
    a.projection_perpendicular_to_vector(&b).get_components()
}


pub fn recursive_permutations<T>(seq: Vec<T>) -> Vec<Vec<T>>
    where T: Copy
{
    if seq.len() == 0
    {
        return vec![];
    }
    if seq.len() == 1
    {
        return vec![seq];
    }
    let mut permutations = Vec::new();
    for i in 0..seq.len()
    {
        let m = seq[i];
        let mut rem_seq = Vec::new();
        for n in 0..i
        {
            rem_seq.push(seq[n]);
        }
        for n in i + 1..seq.len()
        {
            rem_seq.push(seq[n]);
        }
        for permutation in recursive_permutations(rem_seq)
        {
            let mut current_permutation = vec![m];
            for i in 0..permutation.len()
            {
                current_permutation.push(permutation[i]);
            }
            permutations.push(current_permutation);
        }
    }
    return permutations;
}


pub fn are_all_numbers_unique<T>(ref_seq: &[T]) -> bool
    where T: Copy + PartialEq
{
    for i in 1..ref_seq.len()
    {
        let number = ref_seq[i - 1];
        let slice = &ref_seq[i..];
        if slice.iter().any(|n| *n == number)
        {
            return false;
        }
    }
    true
}


pub fn check_value_positive(name: &str, value: FEFloat) -> Result<(), JsValue>
{
    if value <= 0.0 as FEFloat
    {
        let error_message = &format!("{name} is less or equal to zero!");
        return Err(JsValue::from(error_message));
    }
    Ok(())
}


pub fn check_optional_value_positive(name: &str, optional_value: Option<FEFloat>) -> Result<(), JsValue>
{
    if let Some(value) = optional_value
    {
        check_value_positive(name, value)?;
    }
    Ok(())
}


pub fn find_vector_length(vector: &[FEFloat; 3]) -> FEFloat
{
    let v = Vector3::create(vector);
    v.norm().unwrap()
}


pub fn find_surface_normal_components(points_coordinates: &Vec<[FEFloat; 3]>) -> Result<[FEFloat; 3], JsValue>
{
    if points_coordinates.len() != 3
    {
        let error_message = &format!("Incorrect number of points in {points_coordinates:?}!");
        return Err(JsValue::from(error_message));
    }
    let edge_1_direction = Vector3::create(&[
        points_coordinates[0][0] - points_coordinates[1][0],
        points_coordinates[0][1] - points_coordinates[1][1],
        points_coordinates[0][2] - points_coordinates[1][2],
    ]);

    let edge_2_direction = Vector3::create(&[
        points_coordinates[2][0] - points_coordinates[1][0],
        points_coordinates[2][1] - points_coordinates[1][1],
        points_coordinates[2][2] - points_coordinates[1][2],
    ]);

    let components = edge_2_direction.cross_product(&edge_1_direction).get_components();

    Ok(components)
}


pub fn get_objects_numbers_with_relative_keys<'a, T>(src: &HashMap<u32, T>, relative_keys: &'a [RelativeKey]) 
    -> Vec<(u32, &'a RelativeKey)>
    where T: RelativeTrait
{
    let mut data = Vec::new();
    for (number, object) in src.iter()
    {
        for relative_key in relative_keys.iter()
        {
            if object.is_relative_of(relative_key)
            {
                data.push((*number, relative_key));
            }
        }
    }
    data
}


pub fn move_objects_to_dst<T>(src: Vec<T>, dst: &mut HashMap<u32, Vec<T>>, action_id: u32)
{
    if let Some(objects) = dst.get_mut(&action_id)
    {
        for object in src.into_iter()
        {
            objects.push(object);
        }
    }
    else
    {
        dst.insert(action_id, src);
    }
}


pub fn move_objects_with_relative_keys_to_changed<T>(src: &mut HashMap<u32, T>, dst: &mut HashMap<u32, Vec<T>>,
    relative_keys: &[RelativeKey], action_id: u32, props: &Props) -> Result<(), JsValue>
    where T: RelativeTrait + StatusTrait<Key = u32> + ServerNotificationTrait<Key = u32> + Clone
{
    let data = get_objects_numbers_with_relative_keys(src, relative_keys);
    if !data.is_empty()
    {
        let mut changed_objects = Vec::new();
        for (object_number, relative_key) in data.into_iter()
        {
            let object = src.get_mut(&object_number).expect("Object is absent");
            let mut changed_object = object.clone();
            changed_object.set_status(Status::Changed(object_number));
            changed_objects.push(changed_object);
            object.set_relative_to_none(relative_key);
            object.notify_server(NotificationType::Update(false), object_number, props)?;
        }
        move_objects_to_dst(changed_objects, dst, action_id);
    }
    Ok(())
}


pub fn move_objects_with_relative_keys_to_active<T>(src: &mut HashMap<u32, Vec<T>>, dst: &mut HashMap<u32, T>,
    relative_keys: &[RelativeKey], action_id: u32, props: &Props) -> Result<(), JsValue>
    where T: StatusTrait<Key = u32> + RelativeTrait + ServerNotificationTrait<Key = u32>
{   
    if let Some(objects) = src.remove(&action_id)
    {
        for mut object in objects.into_iter()
        {
            match object.get_status()
            {
                Status::Active | Status::Deleted(_) =>
                {
                    let error_message = &format!("Incorrect status for object changed by action {}!",
                        action_id);
                    return Err(JsValue::from(error_message));
                },
                Status::Changed(n) =>
                {
                    if !relative_keys.iter().any(|rk| object.is_relative_of(rk))
                    {
                        let error_message = &format!("There are no object with relative keys \
                            {relative_keys:?}!");
                        return Err(JsValue::from(error_message));
                    }
                    object.set_status(Status::Active);
                    object.notify_server(NotificationType::Update(false), n, props)?;
                    dst.insert(n, object);
                }
            }
        }
    }
    Ok(())
}


pub fn clear_deleted_objects_by_action_id<T>(deleted_objects: &mut HashMap<u32, T>, action_id: u32)
    where T: Clone
{
    for action_id in deleted_objects.clone()
        .keys()
        .filter(|deleted_action_id| **deleted_action_id >= action_id)
        .collect::<Vec<&u32>>()
        .iter()
    {
        let _ = deleted_objects.remove(action_id);
    }
}
