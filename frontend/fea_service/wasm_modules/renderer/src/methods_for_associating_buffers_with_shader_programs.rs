use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;

use crate::Renderer;
use crate::functions::get_webgl_rendering_context;


#[wasm_bindgen]
impl Renderer
{
    pub fn associate_vertex_buffer_with_shader_programs(&self) -> Result<(), JsValue>
    {
        let gl = get_webgl_rendering_context(&self.canvas_gl)?;

        gl.bind_buffer(GL::ARRAY_BUFFER, self.optional_vertex_buffer.as_ref());
        gl.vertex_attrib_pointer_with_i32(
            self.shader_programs.get_vertex_position(), 3, GL::FLOAT, false, 0, 0,
        );
        gl.enable_vertex_attrib_array(self.shader_programs.get_vertex_position());
        Ok(())
    }


    pub fn associate_is_to_scale_buffer_with_shader_programs(&self) -> Result<(), JsValue>
    {
        let gl = get_webgl_rendering_context(&self.canvas_gl)?;

        gl.bind_buffer(GL::ARRAY_BUFFER, self.optional_is_to_scale_buffer.as_ref());
        gl.vertex_attrib_pointer_with_i32(
            self.shader_programs.get_is_to_scale(), 1, GL::FLOAT, false, 0, 0,
        );
        gl.enable_vertex_attrib_array(self.shader_programs.get_is_to_scale());
        Ok(())
    }


    pub fn associate_reference_point_buffer_with_shader_programs(&self) -> Result<(), JsValue>
    {
        let gl = get_webgl_rendering_context(&self.canvas_gl)?;

        gl.bind_buffer(GL::ARRAY_BUFFER, self.optional_reference_point_buffer.as_ref());
        gl.vertex_attrib_pointer_with_i32(
            self.shader_programs.get_reference_point(), 3, GL::FLOAT, false, 0, 0,
        );
        gl.enable_vertex_attrib_array(self.shader_programs.get_reference_point());
        Ok(())
    }


    pub fn associate_vertex_displacement_buffer_with_shader_programs(&self) -> Result<(), JsValue>
    {
        let gl = get_webgl_rendering_context(&self.canvas_gl)?;

        gl.bind_buffer(GL::ARRAY_BUFFER, self.optional_vertex_displacement_buffer.as_ref());
        gl.vertex_attrib_pointer_with_i32(self.shader_programs.get_vertex_displacement(), 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(self.shader_programs.get_vertex_displacement());
        Ok(())
    }


    pub fn associate_color_buffer_with_shader_programs(&self) -> Result<(), JsValue>
    {
        let gl = get_webgl_rendering_context(&self.canvas_gl)?;

        gl.bind_buffer(GL::ARRAY_BUFFER, self.optional_color_buffer.as_ref());
        gl.vertex_attrib_pointer_with_i32(self.shader_programs.get_vertex_color(), 4, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(self.shader_programs.get_vertex_color());
        Ok(())
    }
}
