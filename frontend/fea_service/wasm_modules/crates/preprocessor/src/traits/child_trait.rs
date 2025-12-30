use wasm_bindgen::JsValue;

use crate::enums::ParentKey;


pub trait ChildTrait
{
    fn get_parent_keys(&self) -> Vec<ParentKey>;
    fn get_parent_index(&self, parent_key: ParentKey) -> Result<usize, JsValue>
    {
        if let Some(index) = self.get_parent_keys().iter().position(|pk| *pk == parent_key)
        {
            Ok(index)
        }
        else
        {
            let error_message = format!("Parent key {:?} missing!", parent_key);
            Err(JsValue::from(&error_message))
        }
    }
    fn is_child_of_parent(&self, parent_key: &ParentKey) -> bool
    {
        self.get_parent_keys().contains(parent_key)
    }
}
