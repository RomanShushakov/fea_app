use wasm_bindgen::JsValue;

use extended_matrix::{SquareMatrix, Vector3, BasicOperationsTrait, Position};

use crate::enums::GLMode;
use crate::structs::{Primitives, FloatNumbers, TrussElementPrimitives};
use crate::functions::monochrome_cone;
use crate::Props;
use crate::traits::SelectedObjectTrait;


#[derive(Debug, Clone)]
pub struct TrussElement
{
    primitives_for_selection: Primitives,
    primitives_visible: Primitives,
    primitives_visible_selected: Primitives,
    optional_primitives_result_u_displacement: Option<Primitives>,
    optional_primitives_local_axes: Option<Primitives>,
    optional_primitives_force_r: Option<Primitives>,
}


impl TrussElement
{
    fn get_primitives_for_selection(
        transformed_uid: [u8; 4], 
        point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3], 
        point_1_displacement: &[f32], 
        point_2_displacement: &[f32],
    ) 
        -> Primitives
    {
        let mut primitives_for_selection = Primitives::create();
        primitives_for_selection.extend_lines_endpoints_coordinates(&point_1_coordinates);
        primitives_for_selection.extend_lines_endpoints_is_to_scale(&[1.0]);
        primitives_for_selection.extend_lines_endpoints_reference_points(&point_1_coordinates);
        primitives_for_selection.extend_lines_endpoints_displacements(point_1_displacement);

        primitives_for_selection.extend_lines_endpoints_coordinates(&point_2_coordinates);
        primitives_for_selection.extend_lines_endpoints_is_to_scale(&[1.0]);
        primitives_for_selection.extend_lines_endpoints_reference_points(&point_2_coordinates);
        primitives_for_selection.extend_lines_endpoints_displacements(point_2_displacement);

        let color_for_selection = transformed_uid.map(|v| v as f32 / 255.0);
        primitives_for_selection.extend_lines_endpoints_colors(&color_for_selection);
        primitives_for_selection.extend_lines_endpoints_colors(&color_for_selection);
        primitives_for_selection
    }


    fn transform_to_local_axis_direction(
        axis_direction: [f32; 3], rotation_matrix: &SquareMatrix<f32>,
    ) 
        -> Result<[f32; 3], JsValue>
    {
        let local_axis_direction_matrix = rotation_matrix
            .transpose()
            .multiply(&Vector3::create(&axis_direction))
            .map_err(|e| JsValue::from(e))?;
        let local_axis_direction = [
            *local_axis_direction_matrix
                .get_element_value(&Position(0, 0))
                .map_err(|e| JsValue::from(e))?,
            *local_axis_direction_matrix
                .get_element_value(&Position(1, 0))
                .map_err(|e| JsValue::from(e))?,
            *local_axis_direction_matrix
                .get_element_value(&Position(2, 0))
                .map_err(|e| JsValue::from(e))?,
        ];
        Ok(local_axis_direction)
    }


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
        point_1_displacement: &[f32], 
        point_2_displacement: &[f32],
        optional_point_1_u_result_coeff: Option<[f32; 5]>,
        optional_point_2_u_result_coeff: Option<[f32; 5]>,
        optional_rotation_matrix: &Option<SquareMatrix<f32>>,
        optional_force_r_coeff: Option<[f32; 5]>,
        props: &Props
    ) 
        -> Result<Self, JsValue>
    {
        let primitives_for_selection = TrussElement::get_primitives_for_selection(
            transformed_uid, 
            point_1_coordinates, 
            point_2_coordinates, 
            point_1_displacement, 
            point_2_displacement
        );

        let mut primitives_visible = primitives_for_selection.clone();
        let mut lines_endpoints_colors = FloatNumbers::create();
        lines_endpoints_colors.extend(&props.element_color);
        lines_endpoints_colors.extend(&props.element_color);
        primitives_visible.set_lines_endpoints_colors(lines_endpoints_colors);

        let mut primitives_visible_selected = primitives_for_selection.clone();
        let mut selected_lines_endpoints_colors = FloatNumbers::create();
        selected_lines_endpoints_colors.extend(&props.drawn_object_selected_color);
        selected_lines_endpoints_colors.extend(&props.drawn_object_selected_color);
        primitives_visible_selected.set_lines_endpoints_colors(selected_lines_endpoints_colors);

        let mut primitives_result_u_displacement = primitives_for_selection.clone();
        let mut primitives_result_u_displacement_points_colors = FloatNumbers::create();
        if let (Some(point_1_u_result_coeff), Some(point_2_u_result_coeff)) = 
            (optional_point_1_u_result_coeff, optional_point_2_u_result_coeff)
        {
            primitives_result_u_displacement_points_colors.extend(&[
                point_1_u_result_coeff[1], 
                point_1_u_result_coeff[2], 
                point_1_u_result_coeff[3], 
                point_1_u_result_coeff[4],
            ]);

            primitives_result_u_displacement_points_colors.extend(&[
                point_2_u_result_coeff[1], 
                point_2_u_result_coeff[2], 
                point_2_u_result_coeff[3], 
                point_2_u_result_coeff[4],
            ]);
        }
        else
        {
            primitives_result_u_displacement_points_colors.extend(&props.color_bar_min_color);
            primitives_result_u_displacement_points_colors.extend(&props.color_bar_min_color);
        }
        primitives_result_u_displacement.set_lines_endpoints_colors(primitives_result_u_displacement_points_colors);
        let optional_primitives_result_u_displacement = Some(primitives_result_u_displacement);

        let (optional_primitives_local_axes, optional_primitives_force_r) = 
            if let Some(rotation_matrix) = optional_rotation_matrix
            {
                let axis_r_direction = TrussElement::transform_to_local_axis_direction(
                    [1.0, 0.0, 0.0], &rotation_matrix,
                )?;

                let norm = props.local_axis_line_length / 
                    axis_r_direction.iter().fold(0.0, |acc, x| acc + x * x).sqrt();
                let center = [
                    (point_1_coordinates[0] + point_2_coordinates[0]) / 2.0,
                    (point_1_coordinates[1] + point_2_coordinates[1]) / 2.0,
                    (point_1_coordinates[2] + point_2_coordinates[2]) / 2.0,
                ];
                let point_2_coordinates = [
                    axis_r_direction[0] * norm, axis_r_direction[1] * norm, axis_r_direction[2] * norm,
                ];

                let mut primitives_local_axes = Primitives::create();

                TrussElement::add_axis_line(
                    &mut primitives_local_axes, &center, &point_2_coordinates, &props.local_axis_r_color,
                );

                let norm_2 = (props.local_axis_line_length - 
                    props.local_axis_cap_height) / 
                    axis_r_direction.iter().fold(0.0, |acc, x| acc + x * x).sqrt();
                let base_center_point_coordinates = [
                    axis_r_direction[0] * norm_2, axis_r_direction[1] * norm_2, axis_r_direction[2] * norm_2,
                ];

                TrussElement::add_axis_cap(
                    &mut primitives_local_axes, 
                    &point_2_coordinates,
                    &base_center_point_coordinates, 
                    &center,
                    &props.local_axis_r_color,
                    props,
                )?;

                if let Some([coeff, r, g, b, a]) = optional_force_r_coeff
                {
                    let norm = 
                        (
                            props.symbols_min_line_length * coeff / coeff.abs() + 
                            (props.symbols_max_line_length - props.symbols_min_line_length) * coeff
                        ) / 
                        axis_r_direction.iter().fold(0.0, |acc, x| acc + x * x).sqrt();
                    let point_2_coordinates = [
                        axis_r_direction[0] * norm, axis_r_direction[1] * norm, axis_r_direction[2] * norm,
                    ];

                    let mut primitives_force_r = Primitives::create();

                    TrussElement::add_axis_line(
                        &mut primitives_force_r, &center, &point_2_coordinates, &[r, g, b, a],
                    );

                    let norm_2 = 
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
                        axis_r_direction.iter().fold(0.0, |acc, x| acc + x * x).sqrt();
                    let base_center_point_coordinates = [
                        axis_r_direction[0] * norm_2, axis_r_direction[1] * norm_2, axis_r_direction[2] * norm_2,
                    ];
    
                    TrussElement::add_axis_cap(
                        &mut primitives_force_r, 
                        &point_2_coordinates,
                        &base_center_point_coordinates, 
                        &center,
                        &[r, g, b, a],
                        props,
                    )?;
                    
                    (Some(primitives_local_axes), Some(primitives_force_r))
                }
                else
                {
                    (Some(primitives_local_axes), None)
                }
            }
            else
            {
                (None, None)
            };

        Ok
        (
            TrussElement 
            { 
                primitives_for_selection,
                primitives_visible, 
                primitives_visible_selected,
                optional_primitives_result_u_displacement,
                optional_primitives_local_axes,
                optional_primitives_force_r,
            }
        )
    }


    pub fn get_primitives(&self, gl_mode: &GLMode, is_selected: bool) -> TrussElementPrimitives
    {
        match gl_mode
        {
            GLMode::Selection => 
            {
                TrussElementPrimitives::init(self.primitives_for_selection.clone())
            },
            GLMode::Visible => 
            {
                let mut truss_element_primitives;
                if is_selected
                {
                    truss_element_primitives = TrussElementPrimitives::init(
                        self.primitives_visible_selected.clone(),
                    );
                }
                else
                {
                    truss_element_primitives = TrussElementPrimitives::init(
                        self.primitives_visible.clone(),
                    );
                }

                truss_element_primitives.optional_primitives_result_u_displacement = 
                    self.optional_primitives_result_u_displacement.clone();

                truss_element_primitives.optional_primitives_local_axes = self.optional_primitives_local_axes.clone();
                
                truss_element_primitives.optional_primitives_force_r = self.optional_primitives_force_r.clone();
                
                truss_element_primitives
            }
        }
    }
}


impl SelectedObjectTrait for TrussElement {}
