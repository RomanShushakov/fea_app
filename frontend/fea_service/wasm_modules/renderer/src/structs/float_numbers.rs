use js_sys::{Object, Float32Array};
use web_sys::WebGlRenderingContext as GL;

use crate::traits::BufferDataTrait;


#[derive(Clone, Debug)]
pub struct FloatNumbers(Vec<f32>);


impl FloatNumbers
{
    pub fn create() -> Self
    {
        FloatNumbers(Vec::new())
    }


    pub fn extend(&mut self, float_numbers: &[f32])
    {
        self.0.extend(float_numbers);
    }


    pub fn len(&self) -> usize
    {
        self.0.len()
    }


    pub fn to_vec(&self) -> Vec<f32>
    {
        self.0.clone()
    }
}


impl BufferDataTrait for FloatNumbers
{
    fn convert(&self) -> Object 
    {
        let data = Float32Array::from(self.0.as_slice());
        data.into()
    }


    fn binding_point(&self) -> u32
    {
        GL::ARRAY_BUFFER 
    }
}
