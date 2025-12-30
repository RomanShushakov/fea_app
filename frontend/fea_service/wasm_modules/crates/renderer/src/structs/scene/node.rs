use wasm_bindgen::JsValue;

use crate::enums::GLMode;
use crate::structs::{Primitives, FloatNumbers, NodePrimitives};
use crate::functions::monochrome_cone;
use crate::Props;
use crate::traits::SelectedObjectTrait;


#[derive(Debug, Clone)]
pub struct Node
{
    primitives_for_selection: Primitives,
    primitives_visible: Primitives,
    primitives_visible_selected: Primitives,
    optional_primitives_result_u_displacement: Option<Primitives>,
    optional_primitives_global_forces: Option<Primitives>,
    optional_primitives_global_moments: Option<Primitives>,
}


impl Node
{
    fn get_primitives_for_selection(
        transformed_uid: [u8; 4], x: f32, y: f32, z: f32, displacement: &[f32],
    ) 
        -> Primitives
    {
        let mut primitives_for_selection = Primitives::create();
        primitives_for_selection.extend_points_coordinates(&[x, y, z]);
        primitives_for_selection.extend_points_is_to_scale(&[1.0]);
        primitives_for_selection.extend_points_reference_points(&[x, y, z]);
        primitives_for_selection.extend_points_displacements(displacement);
        let color_for_selection = transformed_uid.map(|v| v as f32 / 255.0);
        primitives_for_selection.extend_points_colors(&color_for_selection);
        primitives_for_selection
    }


    fn add_global_force_moment_line(
        primitives: &mut Primitives,
        center: &[f32; 3],
        point_2_coordinates: &[f32; 3],
        endpoints_displacements: &[f32],
        endpoints_colors: &[f32; 4],
    )
    {
        primitives.extend_lines_endpoints_coordinates(&[0.0, 0.0, 0.0]);
        primitives.extend_lines_endpoints_is_to_scale(&[0.0]);
        primitives.extend_lines_endpoints_reference_points(center);
        primitives.extend_lines_endpoints_displacements(endpoints_displacements);
        
        primitives.extend_lines_endpoints_coordinates(point_2_coordinates);
        primitives.extend_lines_endpoints_is_to_scale(&[0.0]);
        primitives.extend_lines_endpoints_reference_points(center);
        primitives.extend_lines_endpoints_displacements(endpoints_displacements);

        primitives.extend_lines_endpoints_colors(endpoints_colors);
        primitives.extend_lines_endpoints_colors(endpoints_colors);
    }


    fn add_global_force_moment_cap(
        props: &Props, 
        primitives: &mut Primitives, 
        vertex_coordinates: &[f32; 3], 
        base_center_point_coordinates: &[f32; 3],
        vertex_reference_point: &[f32; 3],
        endpoints_displacements: &[f32],
        endpoints_colors: &[f32; 4],
        coeff: f32,
    ) 
        -> Result<(), JsValue>
    {
        let (axes_cap_vertices_coordinates, axis_cap_vertices_colors_values) = monochrome_cone(
            vertex_coordinates, 
            base_center_point_coordinates,
            props.symbols_min_cap_height + (props.symbols_max_cap_height - props.symbols_min_cap_height) * coeff.abs(), 
            props.symbols_min_cap_width + (props.symbols_max_cap_width - props.symbols_min_cap_width) * coeff.abs(), 
            props.symbols_cap_base_points_number, 
            endpoints_colors, 
            props.abs_tol,
            props.rel_tol,
        )?;
        
        primitives.extend_triangles_vertices_coordinates(&axes_cap_vertices_coordinates);
        primitives.extend_triangles_vertices_is_to_scale(
            &vec![0.0; axes_cap_vertices_coordinates.len() / 3],
        );
        for _ in 0..axes_cap_vertices_coordinates.len() / 3
        {
            primitives.extend_triangles_vertices_reference_points(vertex_reference_point);
            primitives.extend_triangles_vertices_displacements(endpoints_displacements);
        }
        primitives.extend_triangles_vertices_colors(&axis_cap_vertices_colors_values);

        Ok(())
    }


    pub fn create(
        transformed_uid: [u8; 4], 
        x: f32, 
        y: f32, 
        z: f32,
        displacement: &[f32],
        optional_u_result_coeff: Option<[f32; 5]>,
        optional_fx_coeff: Option<[f32; 5]>,
        optional_fy_coeff: Option<[f32; 5]>,
        optional_fz_coeff: Option<[f32; 5]>,
        optional_mx_coeff: Option<[f32; 5]>,
        optional_my_coeff: Option<[f32; 5]>,
        optional_mz_coeff: Option<[f32; 5]>,
        props: &Props
    ) 
        -> Result<Self, JsValue>
    {
        let primitives_for_selection = Node::get_primitives_for_selection(
            transformed_uid, x, y, z, displacement,
        );

        let mut primitives_visible = primitives_for_selection.clone();
        let mut points_colors = FloatNumbers::create();
        points_colors.extend(&props.node_color);
        primitives_visible.set_points_colors(points_colors);
        let mut primitives_visible_selected = primitives_for_selection.clone();
        
        let mut selected_points_colors = FloatNumbers::create();
        selected_points_colors.extend(&props.drawn_object_selected_color);
        primitives_visible_selected.set_points_colors(selected_points_colors);

        let mut primitives_result_u_displacement = primitives_for_selection.clone();
        let mut primitives_result_u_displacement_points_colors = FloatNumbers::create();
        if let Some(u_result_coeff) = optional_u_result_coeff
        {
            primitives_result_u_displacement_points_colors.extend(&[
                u_result_coeff[1], u_result_coeff[2], u_result_coeff[3], u_result_coeff[4],
            ]);
        }
        else
        {
            primitives_result_u_displacement_points_colors.extend(&props.color_bar_min_color);
        }
        primitives_result_u_displacement.set_points_colors(primitives_result_u_displacement_points_colors);
        let optional_primitives_result_u_displacement = Some(primitives_result_u_displacement);

        let mut primitives_global_forces = Primitives::create();
        let mut is_global_forces_primitives_extended = false;

        let mut primitives_global_moments = Primitives::create();
        let mut is_global_moments_primitives_extended = false;

        for (i, optional_force_moment_coeff) in [
            optional_fx_coeff,
            optional_fy_coeff,
            optional_fz_coeff,
            optional_mx_coeff,
            optional_my_coeff,
            optional_mz_coeff,
        ].iter().enumerate()
        {
            if let Some([coeff, r, g, b, a]) = optional_force_moment_coeff
            {
                let index = if i <= 2 { i } else { i - 3 }; 

                let center = [x, y, z];

                let mut point_2_coordinates = [0.0; 3];
                point_2_coordinates[index] = props.symbols_min_line_length * coeff / coeff.abs() + 
                    (props.symbols_max_line_length - props.symbols_min_line_length) * coeff;

                let mut base_center_point_coordinates = [0.0; 3];
                base_center_point_coordinates[index] = 
                    (
                        props.symbols_min_line_length * coeff / coeff.abs() + 
                        (props.symbols_max_line_length - props.symbols_min_line_length) * coeff
                    ) - 
                    (
                        props.symbols_min_cap_height * coeff / coeff.abs() + 
                        (props.symbols_max_cap_height - props.symbols_min_cap_height) * coeff
                    );

                Node::add_global_force_moment_line(
                    if i <= 2 { &mut primitives_global_forces } else { &mut primitives_global_moments }, 
                    &center, 
                    &point_2_coordinates, 
                    displacement, 
                    &[*r, *g, *b, *a],
                );
    
                Node::add_global_force_moment_cap(
                    props, 
                    if i <= 2 { &mut primitives_global_forces } else { &mut primitives_global_moments }, 
                    &point_2_coordinates,
                    &base_center_point_coordinates, 
                    &center,
                    displacement,
                    &[*r, *g, *b, *a],
                    *coeff,
                )?;
    
                if i <= 2
                {
                    is_global_forces_primitives_extended = true;
                }
                else
                {
                    is_global_moments_primitives_extended = true;
                }
                
            }
        }

        let optional_primitives_global_forces = if is_global_forces_primitives_extended
        {
            Some(primitives_global_forces)
        }
        else
        {
            None
        };

        let optional_primitives_global_moments = if is_global_moments_primitives_extended
        {
            Some(primitives_global_moments)
        }
        else
        {
            None
        };

        Ok
        (
            Node 
            { 
                primitives_for_selection,
                primitives_visible,
                primitives_visible_selected,
                optional_primitives_result_u_displacement,
                optional_primitives_global_forces,
                optional_primitives_global_moments,
            }
        )
    }


    pub fn get_primitives(&self, gl_mode: &GLMode, is_selected: bool) -> NodePrimitives
    {
        match gl_mode
        {
            GLMode::Selection => 
            {
                NodePrimitives::init(self.primitives_for_selection.clone())
            },
            GLMode::Visible => 
            {
                let mut node_primitives;
                if is_selected
                {
                    node_primitives = NodePrimitives::init(self.primitives_visible_selected.clone());
                }
                else
                {
                    node_primitives = NodePrimitives::init(self.primitives_visible.clone());
                }

                node_primitives.optional_primitives_result_u_displacement = 
                    self.optional_primitives_result_u_displacement.clone();
                
                node_primitives.optional_primitives_global_forces = self.optional_primitives_global_forces.clone();
                
                node_primitives.optional_primitives_global_moments = self.optional_primitives_global_moments.clone();
                
                node_primitives
            }
        }
    }
}


impl SelectedObjectTrait for Node {}
