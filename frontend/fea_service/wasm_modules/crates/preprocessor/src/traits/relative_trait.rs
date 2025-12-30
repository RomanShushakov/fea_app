use wasm_bindgen::JsValue;

use crate::enums::RelativeKey;


pub trait RelativeTrait
{
    fn get_relative_keys(&self) -> Vec<RelativeKey>;
    fn get_relative_index(&self, relative_key: RelativeKey) -> Result<usize, JsValue>
    {
        if let Some(index) = self.get_relative_keys().iter().position(|rk| *rk == relative_key)
        {
            Ok(index)
        }
        else
        {
            let error_message = format!("Relative key {:?} missing!", relative_key);
            Err(JsValue::from(&error_message))
        }
    }
    fn is_relative_of(&self, relative_key: &RelativeKey) -> bool
    {
        self.get_relative_keys().contains(relative_key)
    }
    fn set_relative_to_none(&mut self, relative_key: &RelativeKey);
}
