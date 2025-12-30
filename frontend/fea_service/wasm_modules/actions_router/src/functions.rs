use wasm_bindgen::prelude::*;
use rand;
use std::collections::HashMap;

use crate::action::ActionInPool;


#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(value: &str);
}


pub fn generate_uid(ref_action_pool: &HashMap<u32, ActionInPool>) -> u32
{
    let uid =
    {
        let mut current_uid = rand::random::<u32>();
        while ref_action_pool.contains_key(&current_uid)
        {
            current_uid = rand::random::<u32>();
        }
        current_uid
    };
    uid
}
