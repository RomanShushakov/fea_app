use wasm_bindgen::JsValue;

use crate::Props;
use crate::structs::Primitives;
use crate::functions::monochrome_cone;


pub fn add_cs_axes_lines(props: &Props, primitives: &mut Primitives)
{
    primitives.extend_lines_endpoints_coordinates(&props.cs_origin);
    primitives.extend_lines_endpoints_coordinates(&props.cs_axis_x);
    primitives.extend_lines_endpoints_colors(&props.cs_axis_x_color);
    primitives.extend_lines_endpoints_colors(&props.cs_axis_x_color);
    primitives.extend_lines_endpoints_coordinates(&props.cs_origin);
    primitives.extend_lines_endpoints_coordinates(&props.cs_axis_y);
    primitives.extend_lines_endpoints_colors(&props.cs_axis_y_color);
    primitives.extend_lines_endpoints_colors(&props.cs_axis_y_color);
    primitives.extend_lines_endpoints_coordinates(&props.cs_origin);
    primitives.extend_lines_endpoints_coordinates(&props.cs_axis_z);
    primitives.extend_lines_endpoints_colors(&props.cs_axis_z_color);
    primitives.extend_lines_endpoints_colors(&props.cs_axis_z_color);
    primitives.extend_lines_endpoints_is_to_scale(&[1.0; 6]);
    primitives.extend_lines_endpoints_reference_points(&[0.0; 3 * 6]);
    primitives.extend_lines_endpoints_displacements(&[0.0; 3 * 6]);
}


pub fn add_cs_axes_caps(props: &Props, primitives: &mut Primitives) -> Result<(), JsValue>
{
    let vertices_coordinates = [props.cs_axis_x, props.cs_axis_y, props.cs_axis_z];
    let base_center_points_coordinates = [
        [1.0 - props.cs_axes_caps_height, 0.0, 0.0], 
        [0.0, 1.0 - props.cs_axes_caps_height, 0.0], 
        [0.0, 0.0, 1.0 - props.cs_axes_caps_height],
    ];
    let vertices_colors = [props.cs_axis_x_color, props.cs_axis_y_color, props.cs_axis_z_color];
    for ((vertex_coordinates, base_center_point_coordinates), vertex_color) in 
        vertices_coordinates.iter().zip(base_center_points_coordinates.iter()).zip(vertices_colors.iter())
    {
        let (axes_cap_vertices_coordinates, axis_cap_vertices_colors_values) = 
            monochrome_cone(
                vertex_coordinates, 
                base_center_point_coordinates,
                props.cs_axes_caps_height, 
                props.cs_axes_caps_width, 
                props.cs_axes_caps_base_points_number, 
                vertex_color, 
                props.abs_tol,
                props.rel_tol
            )?;
        primitives.extend_triangles_vertices_coordinates(&axes_cap_vertices_coordinates);
        primitives.extend_triangles_vertices_is_to_scale(
            &vec![1.0; axes_cap_vertices_coordinates.len() / 3]);
        primitives.extend_triangles_vertices_reference_points(
            &vec![0.2; axes_cap_vertices_coordinates.len()]);
        primitives.extend_triangles_vertices_displacements(
            &vec![0.0; axes_cap_vertices_coordinates.len()]);
        primitives.extend_triangles_vertices_colors(&axis_cap_vertices_colors_values);
    }

    Ok(())
}
