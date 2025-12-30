use wasm_bindgen::JsValue;

#[derive(PartialEq, Eq)]
pub enum BCType {
    Displacement,
    Force,
}

impl BCType {
    pub fn from_str(name: &str) -> Result<Self, JsValue> {
        match name {
            "displacement" => Ok(Self::Displacement),
            "force" => Ok(Self::Force),
            _ => Err(JsValue::from(format!("Unknown bc type {name}!"))),
        }
    }
}
