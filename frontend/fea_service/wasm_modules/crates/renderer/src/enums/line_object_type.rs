use wasm_bindgen::prelude::wasm_bindgen;


#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug)]
pub enum LineObjectType
{
    LineDefault,
    LineTruss,
    LineBeam,
    Element,
}
