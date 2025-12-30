use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use web_sys::WebGlBuffer;
use web_sys::
{
    HtmlCanvasElement, WebGlRenderingContext as GL, CanvasRenderingContext2d as CTX,
};

mod methods_for_scene_objects_data_handle;

mod functions;
use functions::{get_webgl_rendering_context, get_canvas_rendering_context_2d, add_cs_axes_lines, add_cs_axes_caps};

mod methods_for_plotting_result;

mod props;
use props::Props;

mod structs;
use structs::{Primitives, ShaderPrograms, Manipulation, Scene};

mod traits;

mod enums;
use enums::GLMode;

mod methods_for_associating_buffers_with_shader_programs;
mod methods_for_drawing_cs_axes;
mod methods_for_manipulation;
mod methods_for_drawing_scene;
mod methods_for_converting_nodes_data;

mod methods_for_converting_elements_data;


#[wasm_bindgen]
pub struct Renderer
{
    props: Props,
    canvas_text: HtmlCanvasElement,
    canvas_gl: HtmlCanvasElement,
    optional_vertex_buffer: Option<WebGlBuffer>,
    optional_is_to_scale_buffer: Option<WebGlBuffer>,
    optional_reference_point_buffer: Option<WebGlBuffer>,
    optional_vertex_displacement_buffer: Option<WebGlBuffer>,
    optional_color_buffer: Option<WebGlBuffer>,
    shader_programs: ShaderPrograms,
    cs_axes_primitives: Primitives,
    manipulation: Manipulation,
    scene: Scene,
}


#[wasm_bindgen]
impl Renderer
{
    pub fn create(
        canvas_text: HtmlCanvasElement, 
        canvas_gl: HtmlCanvasElement,

        abs_tol: f32,
        rel_tol: f32,

        init_cursor_coord_x: i32,
        init_cursor_coord_y: i32,
        init_theta: f32,
        init_phi: f32,
        init_dx: f32,
        init_dy: f32,
        init_d_scale: f32,
        init_is_point_visible: bool,
        init_is_line_visible: bool,
        init_is_surface_visible: bool,
        init_is_surface_edges_1_3_visible: bool,
        init_is_surface_edges_2_4_visible: bool,
        init_is_beam_section_orientation_visible: bool,
        init_is_surface_normal_visible: bool,
        init_is_load_visible: bool,
        init_is_boundary_condition_visible: bool,
        is_mesh_seed_visible: bool,
    
        init_is_node_visible: bool,
        init_is_truss_element_visible: bool,
        init_is_beam_element_visible: bool,
        init_is_plate_element_visible: bool,
        init_is_local_axes_visible: bool,

        cs_origin: &[f32],
        cs_axis_x: &[f32],
        cs_axis_x_color: &[f32],
        cs_axis_y: &[f32],
        cs_axis_y_color: &[f32],
        cs_axis_z: &[f32],
        cs_axis_z_color: &[f32],

        cs_axes_caps_height: f32,
        cs_axes_caps_width: f32,
        cs_axes_caps_base_points_number: u32,

        cs_axes_scale: f32,
        cs_axes_x_shift: f32,
        cs_axes_y_shift: f32,
        cs_axes_z_shift: f32,
        axis_x_denotation_shift_x: f32,
        axis_x_denotation_shift_y: f32,
        axis_y_denotation_shift_x: f32,
        axis_y_denotation_shift_y: f32,
        axis_z_denotation_shift_x: f32,
        axis_z_denotation_shift_y: f32,
        axis_z_denotation_shift_z: f32,
        canvas_axes_denotation_color: &str,

        hint_shift_x: f32,
        rotation_hint_shift_y: f32,
        zoom_hint_shift_y: f32,
        pan_hint_shift_y: f32,
        hints_color: &str,

        drawn_object_selected_color: &[f32],
        node_color: &[f32],
        point_color: &[f32],
        drawn_point_object_denotation_shift: f32,

        line_default_color: &[f32],
        line_truss_props_color: &[f32],
        line_beam_props_color: &[f32],
        element_color: &[f32],
        drawn_line_object_denotation_shift: f32,

        surface_default_color: &[f32],
        surface_plate_props_color: &[f32],

        selection_rectangle_stroke_color: &str,
        selection_rectangle_fill_color: &str,

        event_target: &str,
        selected_object_event_name: &str, 

        beam_section_orientation_line_length: f32,
        beam_section_orientation_cap_height: f32,
        beam_section_orientation_cap_width: f32,
        beam_section_orientation_cap_base_points_number: u32,
        beam_section_orientation_color: &[f32],

        surface_normal_line_length: f32,
        surface_normal_cap_height: f32,
        surface_normal_cap_width: f32,
        surface_normal_cap_base_points_number: u32,
        surface_normal_color: &[f32],

        concentrated_load_line_length: f32,
        concentrated_load_cap_height: f32,
        concentrated_load_cap_width: f32,
        concentrated_load_cap_base_points_number: u32,
        concentrated_load_color: &[f32],

        uniformly_distributed_line_load_line_length: f32,
        uniformly_distributed_line_load_cap_height: f32,
        uniformly_distributed_line_load_cap_width: f32,
        uniformly_distributed_line_load_cap_base_points_number: u32,
        uniformly_distributed_line_load_color: &[f32],
        number_of_uniformly_distributed_line_load_arrows: u32,

        uniformly_distributed_surface_load_line_length: f32,
        uniformly_distributed_surface_load_cap_height: f32,
        uniformly_distributed_surface_load_cap_width: f32,
        uniformly_distributed_surface_load_cap_base_points_number: u32,
        uniformly_distributed_surface_load_color: &[f32],
        number_of_uniformly_distributed_surface_load_arrows: u32,

        point_boundary_condition_cap_height: f32,
        point_boundary_condition_cap_width: f32,
        point_boundary_condition_cap_base_point_number: u32,
        point_boundary_condition_color: &[f32],

        global_mesh_seed_color: &[f32],
        local_mesh_seed_color: &[f32],

        symbols_min_line_length: f32,
        symbols_min_cap_height: f32,
        symbols_min_cap_width: f32,
        symbols_max_line_length: f32,
        symbols_max_cap_height: f32,
        symbols_max_cap_width: f32,
        symbols_cap_base_points_number: u32,

        color_bar_shift_x: f32,
        color_bar_y_bottom: f32,
        color_bar_y_top: f32,
        color_bar_width: f32,
        color_bar_min_color: &[f32],
        color_bar_max_color: &[f32],

        color_bar_caption_shift_x: f32,
        color_bar_caption_header_shift_y: f32,
        color_bar_caption_component_shift_y: f32,
        color_bar_caption_extreme_value_shift_x: f32,

        local_axis_line_length: f32,
        local_axis_cap_height: f32,
        local_axis_cap_width: f32,
        local_axis_cap_base_points_number: u32,
        local_axis_r_color: &[f32],
        local_axis_s_color: &[f32],
        local_axis_t_color: &[f32],

        plate_scale_for_load_arrows: f32,
    )
        -> Result<Renderer, JsValue>
    {
        let props = Props::create(
            abs_tol,
            rel_tol,

            init_cursor_coord_x,
            init_cursor_coord_y,
            init_theta,
            init_phi,
            init_dx,
            init_dy,
            init_d_scale,
            init_is_point_visible,
            init_is_line_visible,
            init_is_surface_visible,
            init_is_surface_edges_1_3_visible,
            init_is_surface_edges_2_4_visible,
            init_is_beam_section_orientation_visible,
            init_is_surface_normal_visible,
            init_is_load_visible,
            init_is_boundary_condition_visible,
            is_mesh_seed_visible,
        
            init_is_node_visible,
            init_is_truss_element_visible,
            init_is_beam_element_visible,
            init_is_plate_element_visible,
            init_is_local_axes_visible,

            cs_origin,
            cs_axis_x,
            cs_axis_x_color,
            cs_axis_y,
            cs_axis_y_color,
            cs_axis_z,
            cs_axis_z_color,

            cs_axes_caps_height,
            cs_axes_caps_width,
            cs_axes_caps_base_points_number,

            cs_axes_scale,
            cs_axes_x_shift,
            cs_axes_y_shift,
            cs_axes_z_shift,
            axis_x_denotation_shift_x,
            axis_x_denotation_shift_y,
            axis_y_denotation_shift_x,
            axis_y_denotation_shift_y,
            axis_z_denotation_shift_x,
            axis_z_denotation_shift_y,
            axis_z_denotation_shift_z,
            canvas_axes_denotation_color,

            hint_shift_x,
            rotation_hint_shift_y,
            zoom_hint_shift_y,
            pan_hint_shift_y,
            hints_color,

            drawn_object_selected_color,
            node_color,
            point_color,
            drawn_point_object_denotation_shift,

            line_default_color,
            line_truss_props_color,
            line_beam_props_color,
            element_color,
            drawn_line_object_denotation_shift,

            surface_default_color,
            surface_plate_props_color,

            selection_rectangle_stroke_color,
            selection_rectangle_fill_color,

            event_target,
            selected_object_event_name,

            beam_section_orientation_line_length,
            beam_section_orientation_cap_height,
            beam_section_orientation_cap_width,
            beam_section_orientation_cap_base_points_number,
            beam_section_orientation_color,

            surface_normal_line_length,
            surface_normal_cap_height,
            surface_normal_cap_width,
            surface_normal_cap_base_points_number,
            surface_normal_color,

            concentrated_load_line_length,
            concentrated_load_cap_height,
            concentrated_load_cap_width,
            concentrated_load_cap_base_points_number,
            concentrated_load_color,

            uniformly_distributed_line_load_line_length,
            uniformly_distributed_line_load_cap_height,
            uniformly_distributed_line_load_cap_width,
            uniformly_distributed_line_load_cap_base_points_number,
            uniformly_distributed_line_load_color,
            number_of_uniformly_distributed_line_load_arrows,

            uniformly_distributed_surface_load_line_length,
            uniformly_distributed_surface_load_cap_height,
            uniformly_distributed_surface_load_cap_width,
            uniformly_distributed_surface_load_cap_base_points_number,
            uniformly_distributed_surface_load_color,
            number_of_uniformly_distributed_surface_load_arrows,

            point_boundary_condition_cap_height,
            point_boundary_condition_cap_width,
            point_boundary_condition_cap_base_point_number,
            point_boundary_condition_color,

            global_mesh_seed_color,
            local_mesh_seed_color,

            symbols_min_line_length,
            symbols_min_cap_height,
            symbols_min_cap_width,
            symbols_max_line_length,
            symbols_max_cap_height,
            symbols_max_cap_width,
            symbols_cap_base_points_number,

            color_bar_shift_x,
            color_bar_y_bottom,
            color_bar_y_top,
            color_bar_width,
            color_bar_min_color,
            color_bar_max_color,

            color_bar_caption_shift_x,
            color_bar_caption_header_shift_y,
            color_bar_caption_component_shift_y,
            color_bar_caption_extreme_value_shift_x,

            local_axis_line_length,
            local_axis_cap_height,
            local_axis_cap_width,
            local_axis_cap_base_points_number,
            local_axis_r_color,
            local_axis_s_color,
            local_axis_t_color,

            plate_scale_for_load_arrows,
        );


        let gl = get_webgl_rendering_context(&canvas_gl)?;

        let optional_vertex_buffer = gl.create_buffer();
        let optional_is_to_scale_buffer = gl.create_buffer();
        let optional_reference_point_buffer = gl.create_buffer();
        let optional_vertex_displacement_buffer = gl.create_buffer();
        let optional_color_buffer = gl.create_buffer();
        let shader_programs = ShaderPrograms::initialize(&gl)?;

        let mut cs_axes_primitives = Primitives::create();
        add_cs_axes_lines(&props, &mut cs_axes_primitives);
        add_cs_axes_caps(&props, &mut cs_axes_primitives)?;

        let manipulation = Manipulation::init(&props);
        let scene = Scene::create();

        Ok(Renderer 
            { 
                props, 
                canvas_text,
                canvas_gl,
                optional_vertex_buffer, 
                optional_is_to_scale_buffer,
                optional_reference_point_buffer,
                optional_vertex_displacement_buffer,
                optional_color_buffer, 
                shader_programs, 
                cs_axes_primitives,
                manipulation,
                scene,
            }
        )
    }


    pub fn set_canvas_size(&mut self, canvas_width: f32, canvas_height: f32)
    {
        self.canvas_text.set_width(canvas_width as u32);
        self.canvas_text.set_height(canvas_height as u32);
        self.canvas_gl.set_width(canvas_width as u32);
        self.canvas_gl.set_height(canvas_height as u32);
    }


    fn get_under_selection_box_colors(&mut self, gl: &GL) -> Result<(), JsValue>
    {
        if let (Some(start_x), Some(start_y)) =
            (self.manipulation.selection_box_start_x, self.manipulation.selection_box_start_y)
        {
            let selection_rectangle_width = self.manipulation.cursor_coord_x - start_x;
            let selection_rectangle_height = self.manipulation.cursor_coord_y - start_y;
            if selection_rectangle_width > 0 && selection_rectangle_height > 0
            {
                let mut pixels = vec![0u8; (selection_rectangle_width *
                    selection_rectangle_height).abs() as usize * 4];
                match gl.read_pixels_with_opt_u8_array(
                    start_x,
                    start_y,
                    selection_rectangle_width.abs(),
                    selection_rectangle_height.abs(),
                    GL::RGBA,
                    GL::UNSIGNED_BYTE,
                    Some(&mut pixels))
                {
                    Ok(_) => self.manipulation.under_selection_box_colors = pixels,
                    Err(msg) => return Err(JsValue::from(&format!("{:?}", msg))),
                }
            }
            else if selection_rectangle_width < 0 && selection_rectangle_height > 0
            {
                let mut pixels = vec![0u8; (selection_rectangle_width *
                    selection_rectangle_height).abs() as usize * 4];
                match gl.read_pixels_with_opt_u8_array(
                    self.manipulation.cursor_coord_x,
                    start_y,
                    selection_rectangle_width.abs(),
                    selection_rectangle_height.abs(),
                    GL::RGBA,
                    GL::UNSIGNED_BYTE,
                    Some(&mut pixels))
                {
                    Ok(_) => self.manipulation.under_selection_box_colors = pixels,
                    Err(msg) => return Err(JsValue::from(&format!("{:?}", msg))),
                }
            }
            else if selection_rectangle_width > 0 && selection_rectangle_height < 0
            {
                let mut pixels = vec![0u8; (selection_rectangle_width *
                    selection_rectangle_height).abs() as usize * 4];
                match gl.read_pixels_with_opt_u8_array(
                    start_x,
                    self.manipulation.cursor_coord_y,
                    selection_rectangle_width.abs(),
                    selection_rectangle_height.abs(),
                    GL::RGBA,
                    GL::UNSIGNED_BYTE,
                    Some(&mut pixels))
                {
                    Ok(_) => self.manipulation.under_selection_box_colors = pixels,
                    Err(msg) => return Err(JsValue::from(&format!("{:?}", msg))),
                }
            }
            else if selection_rectangle_width < 0 && selection_rectangle_height < 0
            {
                let mut pixels = vec![0u8; (selection_rectangle_width *
                    selection_rectangle_height).abs() as usize * 4];
                match gl.read_pixels_with_opt_u8_array(
                    self.manipulation.cursor_coord_x,
                    self.manipulation.cursor_coord_y,
                    selection_rectangle_width.abs(),
                    selection_rectangle_height.abs(),
                    GL::RGBA,
                    GL::UNSIGNED_BYTE,
                    Some(&mut pixels))
                {
                    Ok(_) => self.manipulation.under_selection_box_colors = pixels,
                    Err(msg) => return Err(JsValue::from(&format!("{:?}", msg))),
                }
            }
            else
            {
                let mut pixels = vec![0u8; 12 * 12 * 4];
                match gl.read_pixels_with_opt_u8_array(
                    self.manipulation.cursor_coord_x - 6, 
                    self.manipulation.cursor_coord_y - 6, 
                    12, 
                    12, 
                    GL::RGBA,
                    GL::UNSIGNED_BYTE, 
                    Some(&mut pixels))
                {
                    Ok(_) => self.manipulation.under_selection_box_colors = pixels,
                    Err(msg) => return Err(JsValue::from(&format!("{:?}", msg))),
                }
            }
        }
        else
        {
            let mut pixels = vec![0u8; 12 * 12 * 4];
            match gl.read_pixels_with_opt_u8_array(
                self.manipulation.cursor_coord_x - 6, 
                self.manipulation.cursor_coord_y - 6, 
                12, 
                12, 
                GL::RGBA,
                GL::UNSIGNED_BYTE, 
                Some(&mut pixels))
            {
                Ok(_) => self.manipulation.under_selection_box_colors = pixels,
                Err(msg) => return Err(JsValue::from(&format!("{:?}", msg))),
            }
        }
        Ok(())
    }


    fn draw_selection_box(&self, ctx: &CTX)
    {
        if let (Some(start_x), Some(start_y)) =
            (self.manipulation.selection_box_start_x, self.manipulation.selection_box_start_y)
        {
            let selection_rectangle_width = self.manipulation.cursor_coord_x - start_x;
            let selection_rectangle_height = self.manipulation.cursor_coord_y - start_y;
            ctx.set_stroke_style(&self.props.selection_rectangle_stroke_color.clone().into());
            ctx.stroke_rect(
                start_x as f64, 
                self.canvas_text.height() as f64 - start_y as f64, 
                selection_rectangle_width as f64,
                - selection_rectangle_height as f64,
            );
            ctx.set_fill_style(&self.props.selection_rectangle_fill_color.clone().into());
            ctx.fill_rect(
                start_x as f64, 
                self.canvas_text.height() as f64 - start_y as f64,
                selection_rectangle_width as f64,
                - selection_rectangle_height as f64,
            );
        }
    }


    pub fn tick(&mut self) -> Result<(), JsValue>
    {
        self.render()?;
        Ok(())
    }


    fn render(&mut self) -> Result<(), JsValue>
    {
        let width = self.canvas_gl.width();
        let height = self.canvas_gl.height();
        let gl = get_webgl_rendering_context(&self.canvas_gl)?;
        let ctx = get_canvas_rendering_context_2d(&self.canvas_text)?;
        let line_width = if js_sys::Float32Array::new(
            &gl.get_parameter(GL::ALIASED_LINE_WIDTH_RANGE)?).to_vec()[1] > 2.0
        {
            2.0
        }
        else
        {
            1.0
        };

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        ctx.clear_rect(0.0, 0.0, width as f64, height as f64);
        gl.enable(GL::DEPTH_TEST);
        gl.clear(GL::COLOR_BUFFER_BIT);
        gl.clear(GL::DEPTH_BUFFER_BIT);

        gl.viewport(0, 0, width as i32, height as i32);
        gl.line_width(line_width);

        let z_near = 1.0;
        let z_far = 101.0;

        self.draw_scene(&gl, GLMode::Selection, z_far, z_near, &ctx)?;

        self.get_under_selection_box_colors(&gl)?;

        gl.clear(GL::COLOR_BUFFER_BIT);
        gl.clear(GL::DEPTH_BUFFER_BIT);

        self.draw_scene(&gl, GLMode::Visible, z_far, z_near, &ctx)?;

        self.draw_cs_axes(&gl, z_near, z_far, &ctx)?;

        self.draw_selection_box(&ctx);

        Ok(())
    }

    pub fn reset_scene(&mut self)
    {
        self.scene.reset();
    }
}
