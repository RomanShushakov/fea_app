use web_sys::{WebGlBuffer, WebGlRenderingContext as GL};
use js_sys::Object;


pub trait BufferDataTrait
{
    fn convert(&self) -> Object;
    fn binding_point(&self) -> u32;
    fn store(&self, gl: &GL, optional_ref_buffer: Option<&WebGlBuffer>)
    {
        let data = self.convert();
        let target = self.binding_point();
        gl.bind_buffer(target, optional_ref_buffer);
        gl.buffer_data_with_array_buffer_view(target, &data, GL::DYNAMIC_DRAW);
    }
}
