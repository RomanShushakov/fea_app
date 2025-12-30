use std::collections::HashSet;

use crate::Props;


pub struct Manipulation
{
    pub cursor_coord_x: i32,
    pub cursor_coord_y: i32,
    pub theta: f32,
    pub phi: f32,
    pub dx: f32,
    pub dy: f32,
    pub d_scale: f32,
    pub under_selection_box_colors: Vec<u8>,

    pub selected_colors: HashSet<[u8; 4]>,

    pub selection_box_start_x: Option<i32>,
    pub selection_box_start_y: Option<i32>,

    pub is_point_visible: bool,
    pub is_line_visible: bool,
    pub is_surface_visible: bool,
    pub is_surface_edges_1_3_visible: bool,
    pub is_surface_edges_2_4_visible: bool,
    pub is_beam_section_orientation_visible: bool,
    pub is_surface_normal_visible: bool,
    pub is_load_visible: bool,
    pub is_boundary_condition_visible: bool,
    pub is_mesh_seed_visible: bool,

    pub is_node_visible: bool,
    pub is_truss_element_visible: bool,
    pub is_beam_element_visible: bool,
    pub is_plate_element_visible: bool,
    pub is_local_axes_visible: bool,
}


impl Manipulation
{
    pub fn init(props: &Props) -> Self
    {
        Manipulation
        {
            cursor_coord_x: props.init_cursor_coord_x,
            cursor_coord_y: props.init_cursor_coord_y,
            theta: props.init_theta,
            phi: props.init_phi, 
            dx: props.init_dx, 
            dy: props.init_dy, 
            d_scale: props.init_d_scale,
            under_selection_box_colors: Vec::new(),

            selected_colors: HashSet::new(),

            selection_box_start_x: None,
            selection_box_start_y: None,

            is_point_visible: props.init_is_point_visible,
            is_line_visible: props.init_is_line_visible,
            is_surface_visible: props.init_is_surface_visible,
            is_surface_edges_1_3_visible: props.init_is_surface_edges_1_3_visible,
            is_surface_edges_2_4_visible: props.init_is_surface_edges_2_4_visible,
            is_beam_section_orientation_visible: props.init_is_beam_section_orientation_visible,
            is_surface_normal_visible: props.init_is_surface_normal_visible,
            is_load_visible: props.init_is_load_visible,
            is_boundary_condition_visible: props.init_is_boundary_condition_visible,
            is_mesh_seed_visible: props.is_mesh_seed_visible,

            is_node_visible: props.init_is_node_visible,
            is_truss_element_visible: props.init_is_truss_element_visible,
            is_beam_element_visible: props.init_is_beam_element_visible,
            is_plate_element_visible: props.init_is_plate_element_visible,
            is_local_axes_visible: props.init_is_local_axes_visible,
        }
    }
}
