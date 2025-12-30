use serde::Serialize;
use wasm_bindgen::JsValue;

use crate::types::FEFloat;


#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Normal(FEFloat, FEFloat, FEFloat);


impl Normal
{
    pub fn create(components: &[FEFloat]) -> Result<Self, JsValue>
    {
        if components.len() != 3
        {
            let error_message = &format!("Incorrect number of components in {components:?}!");
            return Err(JsValue::from(error_message));
        }

        if components[0] == 0 as FEFloat && components[1] == 0 as FEFloat && components[2] == 0 as FEFloat
        {
            let error_message = &format!("All components in {components:?} are equal to zero!");
            return Err(JsValue::from(error_message));
        }

        Ok(Normal(components[0], components[1], components[2]))
    }


    pub fn flip(&mut self)
    {
        let (u, v, w) = (self.0, self.1, self.2);
        self.0 = -u;
        self.1 = -v;
        self.2 = -w;
    }
}
