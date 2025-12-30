use wasm_bindgen::prelude::wasm_bindgen;

use crate::Renderer;


#[wasm_bindgen]
impl Renderer
{
    pub fn set_cursor_coordinates(&mut self, x: i32, y: i32)
    {
        self.manipulation.cursor_coord_x = x;
        self.manipulation.cursor_coord_y = y;
    }


    pub fn increment_theta(&mut self, d_theta: f32)
    {
        self.manipulation.theta += d_theta;
    }


    pub fn increment_phi(&mut self, d_phi: f32)
    {
        self.manipulation.phi += d_phi;
    }


    pub fn increment_dx(&mut self, dx: f32)
    {
        self.manipulation.dx += dx;
    }


    pub fn increment_dy(&mut self, dy: f32)
    {
        self.manipulation.dy += dy;
    }


    pub fn get_d_scale(&self) -> f32
    {
        self.manipulation.d_scale
    }


    pub fn set_d_scale(&mut self, d_scale: f32)
    {
        self.manipulation.d_scale = d_scale;
    }


    pub fn set_theta(&mut self, theta: f32)
    {
        self.manipulation.theta = theta;
    }


    pub fn set_phi(&mut self, phi: f32)
    {
        self.manipulation.phi = phi;
    }


    pub fn start_selection(&mut self)
    {
        self.manipulation.selection_box_start_x = Some(self.manipulation.cursor_coord_x);
        self.manipulation.selection_box_start_y = Some(self.manipulation.cursor_coord_y);
    }


    pub fn finish_selection(&mut self)
    {
        self.manipulation.selection_box_start_x = None;
        self.manipulation.selection_box_start_y = None;
    }


    pub fn set_point_visibility(&mut self, is_point_visible: bool)
    {
        self.manipulation.is_point_visible = is_point_visible;
    }


    pub fn set_line_visibility(&mut self, is_line_visible: bool)
    {
        self.manipulation.is_line_visible = is_line_visible;
    }


    pub fn set_surface_visibility(&mut self, is_surface_visible: bool)
    {
        self.manipulation.is_surface_visible = is_surface_visible;
    }


    pub fn set_surface_edges_1_3_visibility(&mut self, is_surface_edges_1_3_visible: bool)
    {
        self.manipulation.is_surface_edges_1_3_visible = is_surface_edges_1_3_visible;
    }


    pub fn set_surface_edges_2_4_visibility(&mut self, is_surface_edges_2_4_visible: bool)
    {
        self.manipulation.is_surface_edges_2_4_visible = is_surface_edges_2_4_visible;
    }


    pub fn set_beam_section_orientation_visibility(&mut self, is_beam_section_orientation_visible: bool)
    {
        self.manipulation.is_beam_section_orientation_visible = is_beam_section_orientation_visible;
    }


    pub fn set_surface_normal_visibility(&mut self, is_surface_normal_visible: bool)
    {
        self.manipulation.is_surface_normal_visible = is_surface_normal_visible;
    }

    
    pub fn set_load_visibility(&mut self, is_load_visible: bool)
    {
        self.manipulation.is_load_visible = is_load_visible;
    }


    pub fn set_boundary_condition_visibility(&mut self, is_boundary_condition_visible: bool)
    {
        self.manipulation.is_boundary_condition_visible = is_boundary_condition_visible;
    }


    pub fn set_mesh_seed_visibility(&mut self, is_mesh_seed_visible: bool)
    {
        self.manipulation.is_mesh_seed_visible = is_mesh_seed_visible;
    }


    pub fn set_node_visibility(&mut self, is_node_visible: bool)
    {
        self.manipulation.is_node_visible = is_node_visible;
    }


    pub fn set_truss_element_visibility(&mut self, is_truss_element_visible: bool)
    {
        self.manipulation.is_truss_element_visible = is_truss_element_visible;
    }


    pub fn set_beam_element_visibility(&mut self, is_beam_element_visible: bool)
    {
        self.manipulation.is_beam_element_visible = is_beam_element_visible;
    }


    pub fn set_plate_element_visibility(&mut self, is_plate_element_visible: bool)
    {
        self.manipulation.is_plate_element_visible = is_plate_element_visible;
    }


    pub fn set_local_axes_visibility(&mut self, is_local_axes_visible: bool)
    {
        self.manipulation.is_local_axes_visible = is_local_axes_visible;
    }


    pub fn auto_fit(&mut self)
    {
        self.manipulation.dx = 0.0;
        self.manipulation.dy = 0.0;
        self.manipulation.d_scale = 0.0;
    }
}
