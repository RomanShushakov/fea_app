mod canvas;
pub use canvas::
{
    get_webgl_rendering_context, get_canvas_rendering_context_2d, add_denotation, add_hints, add_color_bar, 
    add_color_bar_caption,
};

mod geometric_figures;
pub use geometric_figures::monochrome_cone;

mod cs_axes;
pub use cs_axes::{add_cs_axes_lines, add_cs_axes_caps};

mod functions;
pub use functions::
{
    dispatch_custom_event, transform_u32_to_array_of_u8, convert_vec_to_array,
    convert_slice_to_array, find_grid_points_coordinates, get_value_coeff, move_selected_objects_into_regular,
    move_rc_selected_objects_into_rc_regular, move_regular_object_into_selected_objects,
    move_rc_regular_object_into_rc_selected_objects,
};
