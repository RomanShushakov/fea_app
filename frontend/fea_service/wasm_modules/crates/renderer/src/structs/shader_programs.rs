use wasm_bindgen::JsValue;
use web_sys::{WebGlUniformLocation, WebGlRenderingContext as GL};


pub struct ShaderPrograms
{
    vertex_position: u32,
    is_to_scale: u32,
    reference_point: u32,
    vertex_displacement: u32,
    vertex_color: u32,
    displacement_magnitude: WebGlUniformLocation,
    point_size: WebGlUniformLocation,
    shift: WebGlUniformLocation,
    scale: WebGlUniformLocation,
    d_scale: WebGlUniformLocation,
    screen_size: WebGlUniformLocation,
    projection_matrix: WebGlUniformLocation,
    model_view_matrix: WebGlUniformLocation,
}


impl ShaderPrograms
{
    pub fn initialize(gl: &GL) -> Result<Self, JsValue>
    {
        let vertex_shader_code = include_str!("../shaders/main_vert_shader.vert");
        let fragment_shader_code = include_str!("../shaders/main_frag_shader.frag");

        let vertex_shader = gl.create_shader(GL::VERTEX_SHADER)
            .ok_or_else(|| JsValue::from("Vertex shader could not be created!"))?;
        gl.shader_source(&vertex_shader, vertex_shader_code);
        gl.compile_shader(&vertex_shader);
        let fragment_shader = gl.create_shader(GL::FRAGMENT_SHADER)
            .ok_or_else(|| JsValue::from("Fragment shader could not be created!"))?;
        gl.shader_source(&fragment_shader, fragment_shader_code);
        gl.compile_shader(&fragment_shader);
        let shader_program = gl.create_program()
            .ok_or_else(|| JsValue::from("Shader program could not be created!"))?;
        gl.attach_shader(&shader_program, &vertex_shader);
        gl.attach_shader(&shader_program, &fragment_shader);
        gl.link_program(&shader_program);
        gl.use_program(Some(&shader_program));

        let vertex_position = gl.get_attrib_location(&shader_program, "a_vertex_position") as u32;
        let is_to_scale = gl.get_attrib_location(&shader_program, "a_is_to_scale") as u32;
        let reference_point = gl.get_attrib_location(&shader_program, "a_reference_point") as u32;
        let vertex_displacement = gl.get_attrib_location(&shader_program, "a_vertex_displacement") as u32;
        let vertex_color = gl.get_attrib_location(&shader_program, "a_vertex_color") as u32;
        let displacement_magnitude = 
            gl.get_uniform_location(&shader_program, "u_displacement_magnitude")
                .ok_or_else(|| JsValue::from("Could not get displacement magnitude location!"))?;
        let point_size = gl.get_uniform_location(&shader_program, "u_point_size")
            .ok_or_else(|| JsValue::from("Could not get point size location!"))?;
        let shift = gl.get_uniform_location(&shader_program, "u_shift")
            .ok_or_else(|| JsValue::from("Could not get shift location!"))?;
        let scale = gl.get_uniform_location(&shader_program, "u_scale")
            .ok_or_else(|| JsValue::from("Could not get scale location!"))?;
        let d_scale = gl.get_uniform_location(&shader_program, "u_d_scale")
            .ok_or_else(|| JsValue::from("Could not get dscale location!"))?;
        let screen_size = gl.get_uniform_location(&shader_program, "u_screen_size")
            .ok_or_else(|| JsValue::from("Could not get screen size location!"))?;
        let projection_matrix = gl
            .get_uniform_location(&shader_program, "u_projection_matrix")
            .ok_or_else(|| JsValue::from("Could not get projection matrix location!"))?;
        let model_view_matrix = gl
            .get_uniform_location(&shader_program, "u_model_view_matrix")
            .ok_or_else(|| JsValue::from("Could not get view matrix location!"))?;
        Ok(
            ShaderPrograms 
            {
                vertex_position, is_to_scale, reference_point, vertex_displacement, vertex_color, 
                displacement_magnitude, point_size, shift, scale, d_scale, screen_size, projection_matrix, 
                model_view_matrix 
            }
        )
    }


    pub fn get_vertex_position(&self) -> u32
    {
        self.vertex_position
    }


    pub fn get_is_to_scale(&self) -> u32
    {
        self.is_to_scale
    }


    pub fn get_reference_point(&self) -> u32
    {
        self.reference_point
    }


    pub fn get_vertex_displacement(&self) -> u32
    {
        self.vertex_displacement
    }


    pub fn get_vertex_color(&self) -> u32
    {
        self.vertex_color
    }


    pub fn get_ref_displacement_magnitude(&self) -> &WebGlUniformLocation
    {
        &self.displacement_magnitude
    }


    pub fn get_ref_point_size(&self) -> &WebGlUniformLocation
    {
        &self.point_size
    }


    pub fn get_ref_shift(&self) -> &WebGlUniformLocation
    {
        &self.shift
    }


    pub fn get_ref_scale(&self) -> &WebGlUniformLocation
    {
        &self.scale
    }


    pub fn get_ref_d_scale(&self) -> &WebGlUniformLocation
    {
        &self.d_scale
    }


    pub fn get_ref_screen_size(&self) -> &WebGlUniformLocation
    {
        &self.screen_size
    }


    pub fn get_ref_projection_matrix(&self) -> &WebGlUniformLocation
    {
        &self.projection_matrix
    }


    pub fn get_ref_model_view_matrix(&self) -> &WebGlUniformLocation
    {
        &self.model_view_matrix
    }
}
