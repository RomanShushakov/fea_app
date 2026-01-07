use wasm_bindgen::JsValue;

use extended_matrix::{SquareMatrix, Vector3, BasicOperationsTrait, Position};

use crate::enums::GLMode;
use crate::structs::{Primitives, FloatNumbers, PlateElementPrimitives};
use crate::functions::{monochrome_cone, convert_slice_to_array};
use crate::Props;
use crate::traits::SelectedObjectTrait;


#[derive(Debug, Clone)]
pub struct PlateElement
{
    primitives_for_selection: Primitives,
    primitives_visible: Primitives,
    primitives_visible_selected: Primitives,
    optional_primitives_result_u_displacement: Option<Primitives>,
    optional_primitives_local_axes: Option<Primitives>,
    optional_primitives_mem_force_r: Option<Primitives>,
    optional_primitives_mem_force_s: Option<Primitives>,
    optional_primitives_mem_force_r_s: Option<Primitives>,
    optional_primitives_bend_moment_r: Option<Primitives>,
    optional_primitives_bend_moment_s: Option<Primitives>,
    optional_primitives_bend_moment_r_s: Option<Primitives>,
    optional_primitives_shear_force_r_t: Option<Primitives>,
    optional_primitives_shear_force_s_t: Option<Primitives>,
}


impl PlateElement
{
    fn get_primitives_for_selection(
        transformed_uid: [u8; 4], 
        point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3], 
        point_3_coordinates: [f32; 3], 
        point_4_coordinates: [f32; 3],
        point_1_displacement: &[f32], 
        point_2_displacement: &[f32], 
        point_3_displacement: &[f32], 
        point_4_displacement: &[f32], 
    ) 
        -> Primitives
    {
        let mut primitives_for_selection = Primitives::create();
        for (point_coordinates, point_displacements) in 
            [
                point_1_coordinates, point_2_coordinates, point_3_coordinates, 
                point_3_coordinates, point_4_coordinates, point_1_coordinates
            ].iter().zip(
                [
                    point_1_displacement, point_2_displacement, point_3_displacement,
                    point_3_displacement, point_4_displacement, point_1_displacement
                ]
            )
        {
            primitives_for_selection.extend_triangles_vertices_coordinates(point_coordinates);
            primitives_for_selection.extend_triangles_vertices_is_to_scale(&[1.0]);
            primitives_for_selection.extend_triangles_vertices_reference_points(point_coordinates);
            primitives_for_selection.extend_triangles_vertices_displacements(point_displacements);
        }
        let color_for_selection = transformed_uid.map(|v| v as f32 / 255.0);
        for _ in 0..6
        {
            primitives_for_selection.extend_triangles_vertices_colors(&color_for_selection);    
        }
        primitives_for_selection
    }


    fn get_primitives_visible(
        point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3], 
        point_3_coordinates: [f32; 3], 
        point_4_coordinates: [f32; 3],
        point_1_displacement: &[f32], 
        point_2_displacement: &[f32], 
        point_3_displacement: &[f32], 
        point_4_displacement: &[f32], 
        point_1_color: [f32; 4],
        point_2_color: [f32; 4],
        point_3_color: [f32; 4],
        point_4_color: [f32; 4],
    ) 
        -> Primitives
    {
        let mut primitives_visible = Primitives::create();
        for (
            (point_1_coordinates, point_2_coordinates), 
            (
                (point_1_displacement, point_2_displacement), 
                (point_1_color, point_2_color),
            ),
        ) in
            [
                (point_1_coordinates, point_2_coordinates),
                (point_2_coordinates, point_3_coordinates), 
                (point_3_coordinates, point_4_coordinates),
                (point_4_coordinates, point_1_coordinates),
            ].iter().zip(
                [
                    (point_1_displacement, point_2_displacement),
                    (point_2_displacement, point_3_displacement),
                    (point_3_displacement, point_4_displacement),
                    (point_4_displacement, point_1_displacement)
                ].into_iter().zip(
                    [
                        (point_1_color, point_2_color),
                        (point_2_color, point_3_color),
                        (point_3_color, point_4_color),
                        (point_4_color, point_1_color)
                    ].iter()
                )
            )
        {   
            primitives_visible.extend_lines_endpoints_coordinates(point_1_coordinates);
            primitives_visible.extend_lines_endpoints_is_to_scale(&[1.0]);
            primitives_visible.extend_lines_endpoints_reference_points(point_1_coordinates);
            primitives_visible.extend_lines_endpoints_displacements(point_1_displacement);
            primitives_visible.extend_lines_endpoints_colors(point_1_color);

            primitives_visible.extend_lines_endpoints_coordinates(point_2_coordinates);
            primitives_visible.extend_lines_endpoints_is_to_scale(&[1.0]);
            primitives_visible.extend_lines_endpoints_reference_points(point_2_coordinates);
            primitives_visible.extend_lines_endpoints_displacements(point_2_displacement);
            primitives_visible.extend_lines_endpoints_colors(point_2_color);

        }
        primitives_visible
    }


    fn transform_to_local_axis_direction(
        axis_direction: [f32; 3], rotation_matrix: &SquareMatrix<f32>,
    ) 
        -> Result<[f32; 3], JsValue>
    {
        let local_axis_direction_matrix = rotation_matrix
            .transpose()
            .multiply(&Vector3::create(&axis_direction))
            .map_err(JsValue::from)?;
        let local_axis_direction = [
            *local_axis_direction_matrix
                .get_element_value(&Position(0, 0))
                .map_err(JsValue::from)?,
            *local_axis_direction_matrix
                .get_element_value(&Position(1, 0))
                .map_err(JsValue::from)?,
            *local_axis_direction_matrix
                .get_element_value(&Position(2, 0))
                .map_err(JsValue::from)?,
        ];
        Ok(local_axis_direction)
    }


    fn find_optional_transformed_local_axes_directions(
        optional_rotation_matrix: &Option<SquareMatrix<f32>>,
        point_1_coordinates: [f32; 3],
        point_2_coordinates: [f32; 3],
        point_3_coordinates: [f32; 3],
        point_4_coordinates: [f32; 3],
    ) 
        -> Result<Option<[[f32; 3]; 4]>, JsValue>
    {
        if let Some(rotation_matrix) = optional_rotation_matrix
        {
            let center = [
                (
                    point_1_coordinates[0] + point_2_coordinates[0] + point_3_coordinates[0] + point_4_coordinates[0]
                ) / 4.0,
                (
                    point_1_coordinates[1] + point_2_coordinates[1] + point_3_coordinates[1] + point_4_coordinates[1]
                ) / 4.0,
                (
                    point_1_coordinates[2] + point_2_coordinates[2] + point_3_coordinates[2] + point_4_coordinates[2]
                ) / 4.0,
            ];

            let axis_r_direction = PlateElement::transform_to_local_axis_direction(
                [1.0, 0.0, 0.0], rotation_matrix,
            )?;

            let axis_s_direction = PlateElement::transform_to_local_axis_direction(
                [0.0, 1.0, 0.0], rotation_matrix,
            )?;

            let axis_t_direction = PlateElement::transform_to_local_axis_direction(
                [0.0, 0.0, 1.0], rotation_matrix,
            )?;

            Ok(Some([center, axis_r_direction, axis_s_direction, axis_t_direction]))
        }
        else
        {
            Ok(None)
        }
    }


    // fn find_scaled_plate_points_coordinates(
    //     center: &[f32; 3],
    //     point_1_coordinates: &[f32; 3],
    //     point_2_coordinates: &[f32; 3],
    //     point_3_coordinates: &[f32; 3],
    //     point_4_coordinates: &[f32; 3],
    //     scale: f32,
    // ) 
    //     -> [[f32; 3]; 4]
    // {
    //     convert_slice_to_array(
    //         &[point_1_coordinates, point_2_coordinates, point_3_coordinates, point_4_coordinates]
    //             .into_iter()
    //             .map(|point_coordinates|
    //                 {
    //                     convert_slice_to_array(
    //                         &point_coordinates
    //                             .iter()
    //                             .zip(center)
    //                             .map(|(point_coord_component, center_coord_component)|
    //                                 {
    //                                     (point_coord_component - center_coord_component) * scale + 
    //                                     center_coord_component
    //                                 })
    //                             .collect::<Vec<f32>>()
    //                     )
    //                 })
    //             .collect::<Vec<[f32; 3]>>()
    //     )
    // }


    fn add_axis_line(
        primitives: &mut Primitives, center: &[f32; 3], point_2_coordinates: &[f32; 3], color: &[f32; 4], 
    )
    {
        primitives.extend_lines_endpoints_coordinates(&[0.0, 0.0, 0.0]);
        primitives.extend_lines_endpoints_is_to_scale(&[0.0]);
        primitives.extend_lines_endpoints_reference_points(center);
        primitives.extend_lines_endpoints_displacements(&[0.0, 0.0, 0.0]);
        
        primitives.extend_lines_endpoints_coordinates(point_2_coordinates);
        primitives.extend_lines_endpoints_is_to_scale(&[0.0]);
        primitives.extend_lines_endpoints_reference_points(center);
        primitives.extend_lines_endpoints_displacements(&[0.0, 0.0, 0.0]);

        primitives.extend_lines_endpoints_colors(color);
        primitives.extend_lines_endpoints_colors(color);
    }


    fn add_axis_cap( 
        primitives: &mut Primitives, 
        vertex_coordinates: &[f32; 3], 
        base_center_point_coordinates: &[f32; 3],
        vertex_reference_point: &[f32; 3],
        color: &[f32; 4],
        props: &Props,
    ) 
        -> Result<(), JsValue>
    {
        let (axes_cap_vertices_coordinates, axis_cap_vertices_colors_values) = monochrome_cone(
            vertex_coordinates, 
            base_center_point_coordinates,
            props.local_axis_cap_height, 
            props.local_axis_cap_width, 
            props.local_axis_cap_base_points_number, 
            color, 
            props.abs_tol,
            props.rel_tol,
        )?;
        
        primitives.extend_triangles_vertices_coordinates(&axes_cap_vertices_coordinates);
        primitives.extend_triangles_vertices_is_to_scale(
            &vec![0.0; axes_cap_vertices_coordinates.len() / 3]);
        for _ in 0..axes_cap_vertices_coordinates.len() / 3
        {
            primitives.extend_triangles_vertices_reference_points(vertex_reference_point);
        }
        primitives.extend_triangles_vertices_displacements(
            &vec![0.0; axes_cap_vertices_coordinates.len()],
        );
        primitives.extend_triangles_vertices_colors(&axis_cap_vertices_colors_values);

        Ok(())
    }


    pub fn create(
        transformed_uid: [u8; 4], 
        point_1_coordinates: [f32; 3],
        point_2_coordinates: [f32; 3],
        point_3_coordinates: [f32; 3],
        point_4_coordinates: [f32; 3],
        point_1_displacement: &[f32],
        point_2_displacement: &[f32],
        point_3_displacement: &[f32],
        point_4_displacement: &[f32],
        optional_point_1_u_result_coeff: Option<[f32; 5]>,
        optional_point_2_u_result_coeff: Option<[f32; 5]>,
        optional_point_3_u_result_coeff: Option<[f32; 5]>,
        optional_point_4_u_result_coeff: Option<[f32; 5]>,
        optional_rotation_matrix: &Option<SquareMatrix<f32>>,
        optional_mem_force_r_coeff: Option<[f32; 5]>,
        optional_mem_force_s_coeff: Option<[f32; 5]>,
        optional_mem_force_r_s_coeff: Option<[f32; 5]>,
        optional_bend_moment_r_coeff: Option<[f32; 5]>,
        optional_bend_moment_s_coeff: Option<[f32; 5]>,
        optional_bend_moment_r_s_coeff: Option<[f32; 5]>,
        optional_shear_force_r_t_coeff: Option<[f32; 5]>,
        optional_shear_force_s_t_coeff: Option<[f32; 5]>,
        props: &Props
    ) 
        -> Result<Self, JsValue>
    {
        let primitives_for_selection = PlateElement::get_primitives_for_selection(
            transformed_uid, 
            point_1_coordinates,
            point_2_coordinates,
            point_3_coordinates,
            point_4_coordinates,
            point_1_displacement, 
            point_2_displacement,
            point_3_displacement, 
            point_4_displacement,
        );

        let primitives_visible = PlateElement::get_primitives_visible(
            point_1_coordinates,
            point_2_coordinates,
            point_3_coordinates,
            point_4_coordinates,
            point_1_displacement,
            point_2_displacement,
            point_3_displacement,
            point_4_displacement,
            props.element_color,
            props.element_color,
            props.element_color,
            props.element_color,
        );

        let mut primitives_visible_selected = primitives_visible.clone();
        let mut selected_lines_endpoints_colors = FloatNumbers::create();
        for _ in 0..8
        {
            selected_lines_endpoints_colors.extend(&props.drawn_object_selected_color);
        }
        primitives_visible_selected.set_lines_endpoints_colors(selected_lines_endpoints_colors);

        let point_1_color = 
            if let Some(point_1_u_result_coeff) = optional_point_1_u_result_coeff
            {
                convert_slice_to_array(&point_1_u_result_coeff[1..])
            }
            else
            {
                props.color_bar_min_color
            };

        let point_2_color = 
            if let Some(point_2_u_result_coeff) = optional_point_2_u_result_coeff
            {
                convert_slice_to_array(&point_2_u_result_coeff[1..])
            }
            else
            {
                props.color_bar_min_color
            };

        let point_3_color = 
            if let Some(point_3_u_result_coeff) = optional_point_3_u_result_coeff
            {
                convert_slice_to_array(&point_3_u_result_coeff[1..])
            }
            else
            {
                props.color_bar_min_color
            };

        let point_4_color = 
            if let Some(point_4_u_result_coeff) = optional_point_4_u_result_coeff
            {
                convert_slice_to_array(&point_4_u_result_coeff[1..])
            }
            else
            {
                props.color_bar_min_color
            };

        let primitives_result_u_displacement = PlateElement::get_primitives_visible(
            point_1_coordinates,
            point_2_coordinates,
            point_3_coordinates,
            point_4_coordinates,
            point_1_displacement,
            point_2_displacement,
            point_3_displacement,
            point_4_displacement,
            point_1_color,
            point_2_color,
            point_3_color,
            point_4_color,
        );

        let optional_primitives_result_u_displacement = Some(primitives_result_u_displacement);

        let optional_transformed_local_axes_directions = 
            PlateElement::find_optional_transformed_local_axes_directions(
                optional_rotation_matrix, 
                point_1_coordinates,
                point_2_coordinates,
                point_3_coordinates,
                point_4_coordinates,
            )?;
        
        let optional_primitives_local_axes = 
            if let Some(transformed_local_axes_directions) = optional_transformed_local_axes_directions
            {
                let center = transformed_local_axes_directions[0];
                let mut primitives_local_axes = Primitives::create();
                for (axis_direction, axis_color) in transformed_local_axes_directions[1..].iter()
                    .zip([props.local_axis_r_color, props.local_axis_s_color, props.local_axis_t_color].iter())
                {
                    let norm = props.local_axis_line_length / 
                        axis_direction.iter().fold(0.0, |acc, x| acc + x * x).sqrt();
                    let point_2_coordinates = [
                        axis_direction[0] * norm, axis_direction[1] * norm, axis_direction[2] * norm,
                    ];
                    PlateElement::add_axis_line(
                        &mut primitives_local_axes, 
                        &center, 
                        &point_2_coordinates, 
                        axis_color,
                    );
                    let norm_2 = (props.local_axis_line_length - props.local_axis_cap_height) / 
                        axis_direction.iter().fold(0.0, |acc, x| acc + x * x).sqrt();
                    let base_center_point_coordinates = [
                        axis_direction[0] * norm_2, axis_direction[1] * norm_2, axis_direction[2] * norm_2,
                    ];
                    PlateElement::add_axis_cap(
                        &mut primitives_local_axes, 
                        &point_2_coordinates,
                        &base_center_point_coordinates, 
                        &center,
                        axis_color,
                        props,
                    )?;
                }
                Some(primitives_local_axes)
            }
            else
            {
                None
            };

        let load_axis_point_2_coordinates_handle = |coeff: f32, axis_direction: [f32; 3]| 
            {
                let norm = 
                    (
                        props.symbols_min_line_length * coeff / coeff.abs() + 
                        (props.symbols_max_line_length - props.symbols_min_line_length) * coeff
                    ) / 
                    axis_direction.iter().fold(0.0, |acc, x| acc + x * x).sqrt();
                [axis_direction[0] * norm, axis_direction[1] * norm, axis_direction[2] * norm]
            };

        let load_axis_base_center_point_coordinates_handle = 
            |coeff: f32, axis_direction: [f32; 3]| 
            {
                let norm = 
                    (
                        (
                            props.symbols_min_line_length * coeff / coeff.abs() + 
                            (props.symbols_max_line_length - props.symbols_min_line_length) * coeff
                        ) -
                        (
                            props.symbols_min_cap_height * coeff / coeff.abs() + 
                            (props.symbols_max_cap_height - props.symbols_min_cap_height) * coeff
                        )
                    ) / 
                    axis_direction.iter().fold(0.0, |acc, x| acc + x * x).sqrt();
                [axis_direction[0] * norm, axis_direction[1] * norm, axis_direction[2] * norm]
            };

        let optional_primitives_mem_force_r = 
            if let (
                Some(transformed_local_axes_directions), 
                Some([coeff, r, g, b, a]),
            ) = 
                (optional_transformed_local_axes_directions, optional_mem_force_r_coeff)
            {
                let center = transformed_local_axes_directions[0];
                let axis_r_direction = transformed_local_axes_directions[1];
                let mut primitives_mem_force_r = Primitives::create();

                let point_2_coordinates = load_axis_point_2_coordinates_handle(
                    coeff, axis_r_direction,
                );

                PlateElement::add_axis_line(
                    &mut primitives_mem_force_r, &center, &point_2_coordinates, &[r, g, b, a],
                );

                let base_center_point_coordinates = load_axis_base_center_point_coordinates_handle(
                    coeff, axis_r_direction,
                );

                PlateElement::add_axis_cap(
                    &mut primitives_mem_force_r, 
                    &point_2_coordinates,
                    &base_center_point_coordinates, 
                    &center,
                    &[r, g, b, a],
                    props,
                )?;

                Some(primitives_mem_force_r)
            }
            else
            {
                None
            };

        let optional_primitives_mem_force_s = 
            if let (
                Some(transformed_local_axes_directions), 
                Some([coeff, r, g, b, a]),
            ) = 
                (optional_transformed_local_axes_directions, optional_mem_force_s_coeff)
            {
                let center = transformed_local_axes_directions[0];
                let axis_s_direction = transformed_local_axes_directions[2];
                let mut primitives_mem_force_s = Primitives::create();

                let point_2_coordinates = load_axis_point_2_coordinates_handle(
                    coeff, axis_s_direction,
                );

                PlateElement::add_axis_line(
                    &mut primitives_mem_force_s, &center, &point_2_coordinates, &[r, g, b, a],
                );

                let base_center_point_coordinates = load_axis_base_center_point_coordinates_handle(
                    coeff, axis_s_direction,
                );

                PlateElement::add_axis_cap(
                    &mut primitives_mem_force_s, 
                    &point_2_coordinates,
                    &base_center_point_coordinates, 
                    &center,
                    &[r, g, b, a],
                    props,
                )?;

                Some(primitives_mem_force_s)
            }
            else
            {
                None
            };

        let optional_primitives_mem_force_r_s = 
            if let (
                Some(transformed_local_axes_directions), 
                Some([coeff, r, g, b, a]),
            ) = 
                (optional_transformed_local_axes_directions, optional_mem_force_r_s_coeff)
            {
                let center = transformed_local_axes_directions[0];
                let axis_r_direction = transformed_local_axes_directions[1];
                let axis_s_direction = transformed_local_axes_directions[2];

                let mut primitives_mem_force_r_s = Primitives::create();

                let point_2_coordinates_r = load_axis_point_2_coordinates_handle(
                    coeff, axis_r_direction,
                );

                PlateElement::add_axis_line(
                    &mut primitives_mem_force_r_s, &center, &point_2_coordinates_r, &[r, g, b, a],
                );

                let base_center_point_coordinates_r = load_axis_base_center_point_coordinates_handle(
                    coeff, axis_r_direction,
                );

                PlateElement::add_axis_cap(
                    &mut primitives_mem_force_r_s, 
                    &point_2_coordinates_r,
                    &base_center_point_coordinates_r, 
                    &center,
                    &[r, g, b, a],
                    props,
                )?;

                let point_2_coordinates_s = load_axis_point_2_coordinates_handle(
                    coeff, axis_s_direction,
                );

                PlateElement::add_axis_line(
                    &mut primitives_mem_force_r_s, &center, &point_2_coordinates_s, &[r, g, b, a],
                );

                let base_center_point_coordinates_s = load_axis_base_center_point_coordinates_handle(
                    coeff, axis_s_direction,
                );

                PlateElement::add_axis_cap(
                    &mut primitives_mem_force_r_s, 
                    &point_2_coordinates_s,
                    &base_center_point_coordinates_s, 
                    &center,
                    &[r, g, b, a],
                    props,
                )?;
                
                Some(primitives_mem_force_r_s)
            }
            else
            {
                None
            };

        let optional_primitives_bend_moment_r = 
            if let (
                Some(transformed_local_axes_directions), 
                Some([coeff, r, g, b, a]),
            ) = 
                (optional_transformed_local_axes_directions, optional_bend_moment_r_coeff)
            {
                let center = transformed_local_axes_directions[0];
                let axis_r_direction = transformed_local_axes_directions[1];
                let mut primitives_bend_moment_r = Primitives::create();

                let point_2_coordinates = load_axis_point_2_coordinates_handle(
                    coeff, axis_r_direction,
                );

                PlateElement::add_axis_line(
                    &mut primitives_bend_moment_r, &center, &point_2_coordinates, &[r, g, b, a],
                );

                let base_center_point_coordinates = load_axis_base_center_point_coordinates_handle(
                    coeff, axis_r_direction,
                );

                PlateElement::add_axis_cap(
                    &mut primitives_bend_moment_r, 
                    &point_2_coordinates,
                    &base_center_point_coordinates, 
                    &center,
                    &[r, g, b, a],
                    props,
                )?;

                Some(primitives_bend_moment_r)
            }
            else
            {
                None
            };

        let optional_primitives_bend_moment_s = 
            if let (
                Some(transformed_local_axes_directions), 
                Some([coeff, r, g, b, a]),
            ) = 
                (optional_transformed_local_axes_directions, optional_bend_moment_s_coeff)
            {
                let center = transformed_local_axes_directions[0];
                let axis_s_direction = transformed_local_axes_directions[2];
                let mut primitives_bend_moment_s = Primitives::create();

                let point_2_coordinates = load_axis_point_2_coordinates_handle(
                    coeff, axis_s_direction,
                );

                PlateElement::add_axis_line(
                    &mut primitives_bend_moment_s, &center, &point_2_coordinates, &[r, g, b, a],
                );

                let base_center_point_coordinates = load_axis_base_center_point_coordinates_handle(
                    coeff, axis_s_direction,
                );

                PlateElement::add_axis_cap(
                    &mut primitives_bend_moment_s, 
                    &point_2_coordinates,
                    &base_center_point_coordinates, 
                    &center,
                    &[r, g, b, a],
                    props,
                )?;

                Some(primitives_bend_moment_s)
            }
            else
            {
                None
            };

        let optional_primitives_bend_moment_r_s = 
            if let (
                Some(transformed_local_axes_directions), 
                Some([coeff, r, g, b, a]),
            ) = 
                (optional_transformed_local_axes_directions, optional_bend_moment_r_s_coeff)
            {
                let center = transformed_local_axes_directions[0];
                let axis_r_direction = transformed_local_axes_directions[1];
                let axis_s_direction = transformed_local_axes_directions[2];

                let mut primitives_bend_moment_r_s = Primitives::create();

                let point_2_coordinates_r = load_axis_point_2_coordinates_handle(
                    coeff, axis_r_direction,
                );

                PlateElement::add_axis_line(
                    &mut primitives_bend_moment_r_s, &center, &point_2_coordinates_r, &[r, g, b, a],
                );

                let base_center_point_coordinates_r = load_axis_base_center_point_coordinates_handle(
                    coeff, axis_r_direction,
                );

                PlateElement::add_axis_cap(
                    &mut primitives_bend_moment_r_s, 
                    &point_2_coordinates_r,
                    &base_center_point_coordinates_r, 
                    &center,
                    &[r, g, b, a],
                    props,
                )?;

                let point_2_coordinates_s = load_axis_point_2_coordinates_handle(
                    coeff, axis_s_direction,
                );

                PlateElement::add_axis_line(
                    &mut primitives_bend_moment_r_s, &center, &point_2_coordinates_s, &[r, g, b, a],
                );

                let base_center_point_coordinates_s = load_axis_base_center_point_coordinates_handle(
                    coeff, axis_s_direction,
                );

                PlateElement::add_axis_cap(
                    &mut primitives_bend_moment_r_s, 
                    &point_2_coordinates_s,
                    &base_center_point_coordinates_s, 
                    &center,
                    &[r, g, b, a],
                    props,
                )?;
                
                Some(primitives_bend_moment_r_s)
            }
            else
            {
                None
            };

        let optional_primitives_shear_force_r_t = 
            if let (
                Some(transformed_local_axes_directions), 
                Some([coeff, r, g, b, a]),
            ) = 
                (optional_transformed_local_axes_directions, optional_shear_force_r_t_coeff)
            {
                let center = transformed_local_axes_directions[0];
                let axis_t_direction = transformed_local_axes_directions[3];
                let mut primitives_shear_force_r_t = Primitives::create();

                let point_2_coordinates = load_axis_point_2_coordinates_handle(
                    coeff, axis_t_direction,
                );

                PlateElement::add_axis_line(
                    &mut primitives_shear_force_r_t, &center, &point_2_coordinates, &[r, g, b, a],
                );

                let base_center_point_coordinates = load_axis_base_center_point_coordinates_handle(
                    coeff, axis_t_direction,
                );

                PlateElement::add_axis_cap(
                    &mut primitives_shear_force_r_t, 
                    &point_2_coordinates,
                    &base_center_point_coordinates, 
                    &center,
                    &[r, g, b, a],
                    props,
                )?;

                Some(primitives_shear_force_r_t)
            }
            else
            {
                None
            };

        let optional_primitives_shear_force_s_t = 
            if let (
                Some(transformed_local_axes_directions), 
                Some([coeff, r, g, b, a]),
            ) = 
                (optional_transformed_local_axes_directions, optional_shear_force_s_t_coeff)
            {
                let center = transformed_local_axes_directions[0];
                let axis_t_direction = transformed_local_axes_directions[3];
                let mut primitives_shear_force_s_t = Primitives::create();

                let point_2_coordinates = load_axis_point_2_coordinates_handle(
                    coeff, axis_t_direction,
                );

                PlateElement::add_axis_line(
                    &mut primitives_shear_force_s_t, &center, &point_2_coordinates, &[r, g, b, a],
                );

                let base_center_point_coordinates = load_axis_base_center_point_coordinates_handle(
                    coeff, axis_t_direction,
                );

                PlateElement::add_axis_cap(
                    &mut primitives_shear_force_s_t, 
                    &point_2_coordinates,
                    &base_center_point_coordinates, 
                    &center,
                    &[r, g, b, a],
                    props,
                )?;

                Some(primitives_shear_force_s_t)
            }
            else
            {
                None
            };

        Ok
        (
            PlateElement 
            { 
                primitives_for_selection,
                primitives_visible, 
                primitives_visible_selected,
                optional_primitives_result_u_displacement,
                optional_primitives_local_axes,
                optional_primitives_mem_force_r,
                optional_primitives_mem_force_s,
                optional_primitives_mem_force_r_s,
                optional_primitives_bend_moment_r,
                optional_primitives_bend_moment_s,
                optional_primitives_bend_moment_r_s,
                optional_primitives_shear_force_r_t,
                optional_primitives_shear_force_s_t,
            }
        )
    }


    pub fn get_primitives(&self, gl_mode: &GLMode, is_selected: bool) -> PlateElementPrimitives
    {
        match gl_mode
        {
            GLMode::Selection => 
            {
                PlateElementPrimitives::init(self.primitives_for_selection.clone())
            },
            GLMode::Visible => 
            {
                let mut plate_element_primitives;
                if is_selected
                {
                    plate_element_primitives = PlateElementPrimitives::init(
                        self.primitives_visible_selected.clone(),
                    );
                }
                else
                {
                    plate_element_primitives = PlateElementPrimitives::init(
                        self.primitives_visible.clone(),
                    );
                }

                plate_element_primitives.optional_primitives_result_u_displacement = 
                    self.optional_primitives_result_u_displacement.clone();

                plate_element_primitives.optional_primitives_local_axes = self.optional_primitives_local_axes.clone();

                plate_element_primitives.optional_primitives_mem_force_r = self.optional_primitives_mem_force_r.clone();

                plate_element_primitives.optional_primitives_mem_force_s = self.optional_primitives_mem_force_s.clone();

                plate_element_primitives.optional_primitives_mem_force_r_s = 
                    self.optional_primitives_mem_force_r_s.clone();

                plate_element_primitives.optional_primitives_bend_moment_r = 
                    self.optional_primitives_bend_moment_r.clone();

                plate_element_primitives.optional_primitives_bend_moment_s = 
                    self.optional_primitives_bend_moment_s.clone();

                plate_element_primitives.optional_primitives_bend_moment_r_s = 
                    self.optional_primitives_bend_moment_r_s.clone();

                plate_element_primitives.optional_primitives_shear_force_r_t = 
                    self.optional_primitives_shear_force_r_t.clone();

                plate_element_primitives.optional_primitives_shear_force_s_t = 
                    self.optional_primitives_shear_force_s_t.clone();

                plate_element_primitives
            }
        }
    }
}


impl SelectedObjectTrait for PlateElement {}
