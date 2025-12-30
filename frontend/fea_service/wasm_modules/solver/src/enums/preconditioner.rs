use wasm_bindgen::JsValue;

#[derive(PartialEq, Eq)]
pub enum Preconditioner {
    Jacobi,
    BlockJacobi,
    BlockJacobiGpu,
}

impl Preconditioner {
    pub fn from_str(name: &str) -> Result<Self, JsValue> {
        match name {
            "jacobi" => Ok(Self::Jacobi),
            "block_jacobi" => Ok(Self::BlockJacobi),
            "block_jacobi_gpu" => Ok(Self::BlockJacobiGpu),
            _ => Err(JsValue::from(format!(
                "Unknown preconditioner type {name}!"
            ))),
        }
    }
}
