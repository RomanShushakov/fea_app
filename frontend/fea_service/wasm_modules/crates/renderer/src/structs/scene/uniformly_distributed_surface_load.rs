use wasm_bindgen::JsValue;

use crate::enums::GLMode;
use crate::structs::{Primitives, FloatNumbers};
use crate::functions::{monochrome_cone, find_grid_points_coordinates};
use crate::Props;
use crate::traits::SelectedObjectTrait;


#[derive(Debug, Clone)]
pub struct UniformlyDistributedSurfaceLoad
{
    transformed_uid: [u8; 4],
    primitives_for_selection: Primitives,
    primitives_visible: Primitives,
    primitives_visible_selected: Primitives,
}


impl UniformlyDistributedSurfaceLoad
{
    fn add_load_direction_line(color: &[f32; 4], primitives: &mut Primitives, reference_point: &[f32; 3], 
        direction: &[f32; 3])
    {
        primitives.extend_lines_endpoints_coordinates(&[0.0, 0.0, 0.0]);
        primitives.extend_lines_endpoints_is_to_scale(&[0.0]);
        primitives.extend_lines_endpoints_reference_points(reference_point);
        primitives.extend_lines_endpoints_displacements(&[0.0, 0.0, 0.0]);
        
        primitives.extend_lines_endpoints_coordinates(direction);
        primitives.extend_lines_endpoints_is_to_scale(&[0.0]);
        primitives.extend_lines_endpoints_reference_points(reference_point);
        primitives.extend_lines_endpoints_displacements(&[0.0, 0.0, 0.0]);

        primitives.extend_lines_endpoints_colors(color);
        primitives.extend_lines_endpoints_colors(color);
    }


    fn add_load_direction_cap(props: &Props, primitives: &mut Primitives, vertex_coordinates: &[f32; 3], 
        base_center_point_coordinates: &[f32; 3], vertex_reference_point: &[f32; 3], color: &[f32; 4])
        -> Result<(), JsValue>
    {
        let (axes_cap_vertices_coordinates, axis_cap_vertices_colors_values) = 
            monochrome_cone(
                vertex_coordinates, 
                base_center_point_coordinates,
                props.uniformly_distributed_surface_load_cap_height, 
                props.uniformly_distributed_surface_load_cap_width, 
                props.uniformly_distributed_surface_load_cap_base_points_number, 
                color, 
                props.abs_tol,
                props.rel_tol
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


    fn add_force_arrow(color: &[f32; 4], primitives: &mut Primitives, reference_point: &[f32; 3], 
        direction: &[f32; 3], props: &Props) -> Result<(), JsValue>
    {
        UniformlyDistributedSurfaceLoad::add_load_direction_line(color, primitives, reference_point, direction);
        let norm = (props.uniformly_distributed_surface_load_line_length - 
            props.uniformly_distributed_surface_load_cap_height) / 
            direction.iter().fold(0.0, |acc, x| acc + x * x).sqrt();
        let base_center_point_coordinates = [direction[0] * norm, direction[1] * norm, 
            direction[2] * norm];
        UniformlyDistributedSurfaceLoad::add_load_direction_cap(props, primitives, direction, 
            &base_center_point_coordinates, reference_point, color)?;
        Ok(())
    }


    fn get_primitives_for_selection(transformed_uid: [u8; 4], point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3], point_3_coordinates: [f32; 3], point_4_coordinates: [f32; 3], px: f32, py: f32, 
        pz: f32, props: &Props) -> Result<Primitives, JsValue>
    {
        let mut primitives_for_selection = Primitives::create();
        let color_for_selection = transformed_uid.map(|v| v as f32 / 255.0);

        let grid_points_coordinates = find_grid_points_coordinates(
            props.number_of_uniformly_distributed_surface_load_arrows, &point_1_coordinates, &point_2_coordinates, 
            &point_3_coordinates, &point_4_coordinates);

        let optional_px_direction = 
            if px > 0.0 { Some([props.uniformly_distributed_surface_load_line_length, 0.0, 0.0]) } 
            else if px < 0.0 { Some([-props.uniformly_distributed_surface_load_line_length, 0.0, 0.0]) }
            else { None };
        if let Some(px_direction) = optional_px_direction
        {
            for point_coordinates in grid_points_coordinates.iter()
            {
                UniformlyDistributedSurfaceLoad::add_force_arrow(&color_for_selection, &mut primitives_for_selection, 
                    point_coordinates, &px_direction, props)?;
            }
        }

        let optional_py_direction = 
            if py > 0.0 { Some([0.0, props.uniformly_distributed_surface_load_line_length, 0.0]) } 
            else if py < 0.0 { Some([0.0, -props.uniformly_distributed_surface_load_line_length, 0.0]) }
            else { None };
        if let Some(py_direction) = optional_py_direction
        {
            for point_coordinates in grid_points_coordinates.iter()
            {
                UniformlyDistributedSurfaceLoad::add_force_arrow(&color_for_selection, &mut primitives_for_selection, 
                    point_coordinates, &py_direction, props)?;
            }
        }

        let optional_pz_direction = 
            if pz > 0.0 { Some([0.0, 0.0, props.uniformly_distributed_surface_load_line_length]) } 
            else if pz < 0.0 { Some([0.0, 0.0, -props.uniformly_distributed_surface_load_line_length]) }
            else { None };
        if let Some(pz_direction) = optional_pz_direction
        {
            for point_coordinates in grid_points_coordinates.iter()
            {
                UniformlyDistributedSurfaceLoad::add_force_arrow(&color_for_selection, &mut primitives_for_selection, 
                    point_coordinates, &pz_direction, props)?;
            }
        }

        Ok(primitives_for_selection)
    }


    pub fn create(
        transformed_uid: [u8; 4], 
        point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3], 
        point_3_coordinates: [f32; 3], 
        point_4_coordinates: [f32; 3],
        px: f32,
        py: f32,
        pz: f32,
        props: &Props
    ) 
        -> Result<Self, JsValue>
    {
        let primitives_for_selection = UniformlyDistributedSurfaceLoad::get_primitives_for_selection(
            transformed_uid, point_1_coordinates, point_2_coordinates, point_3_coordinates, point_4_coordinates, 
            px, py, pz, props)?;

        let color_visible = props.uniformly_distributed_surface_load_color;
        let mut primitives_visible = primitives_for_selection.clone();
        let mut lines_endpoints_colors = FloatNumbers::create();
        let lines_endpoints_colors_number = primitives_for_selection.get_ref_lines_endpoints_colors().len() / 4;
        for _ in 0..lines_endpoints_colors_number
        {
            lines_endpoints_colors.extend(&color_visible);    
        }
        primitives_visible.set_lines_endpoints_colors(lines_endpoints_colors);
        let mut caps_colors = FloatNumbers::create();
        let caps_colors_number = primitives_for_selection.get_ref_triangles_vertices_colors().len() / 4;
        for _ in 0..caps_colors_number
        {
            caps_colors.extend(&color_visible);    
        }
        primitives_visible.set_triangles_vertices_colors(caps_colors);

        let mut primitives_visible_selected = primitives_for_selection.clone();
        let mut selected_lines_endpoints_colors = FloatNumbers::create();
        let lines_endpoints_colors_number = primitives_for_selection.get_ref_lines_endpoints_colors().len() / 4;
        for _ in 0..lines_endpoints_colors_number
        {
            selected_lines_endpoints_colors.extend(&props.drawn_object_selected_color);
        }
        primitives_visible_selected.set_lines_endpoints_colors(selected_lines_endpoints_colors);
        let mut selected_caps_colors = FloatNumbers::create();
        let caps_colors_number = primitives_for_selection.get_ref_triangles_vertices_colors().len() / 4;
        for _ in 0..caps_colors_number
        {
            selected_caps_colors.extend(&props.drawn_object_selected_color);
        }
        primitives_visible_selected.set_triangles_vertices_colors(selected_caps_colors);

        Ok(UniformlyDistributedSurfaceLoad { transformed_uid, primitives_for_selection, primitives_visible, 
            primitives_visible_selected })
    }


    pub fn get_primitives(&self, gl_mode: &GLMode, is_selected: bool) -> Primitives
    {
        match gl_mode
        {
            GLMode::Selection => self.primitives_for_selection.clone(),
            GLMode::Visible => 
            {
                if is_selected
                {
                    self.primitives_visible_selected.clone()
                }
                else
                {
                    self.primitives_visible.clone()
                }
            }
        }
    }


    pub fn update_point_coordinates(&mut self, point_1_coordinates: &[f32; 3], point_2_coordinates: &[f32; 3], 
        point_3_coordinates: &[f32; 3], point_4_coordinates: &[f32; 3], props: &Props)
    {
        let grid_points_coordinates = find_grid_points_coordinates(
            props.number_of_uniformly_distributed_surface_load_arrows, point_1_coordinates, point_2_coordinates, 
            point_3_coordinates, point_4_coordinates);

        let mut arrows_lines_reference_points = FloatNumbers::create();
        let mut arrows_caps_reference_points = FloatNumbers::create();

        let number_of_components = self.primitives_for_selection.get_ref_lines_endpoints_coordinates().len() / 
            (6 * props.number_of_uniformly_distributed_surface_load_arrows * 
                props.number_of_uniformly_distributed_surface_load_arrows) as usize;

        for _ in 0..number_of_components
        {
            for grid_point_coordinates in grid_points_coordinates.iter()
            {
                arrows_lines_reference_points.extend(grid_point_coordinates);
                arrows_lines_reference_points.extend(grid_point_coordinates);

                for _ in 0..self.primitives_for_selection.get_ref_triangles_vertices_coordinates().len() / 
                    (3 * props.number_of_uniformly_distributed_surface_load_arrows * 
                        props.number_of_uniformly_distributed_surface_load_arrows * number_of_components as u32) as usize
                {
                    arrows_caps_reference_points.extend(grid_point_coordinates);
                }
            }
        }        

        self.primitives_for_selection.set_lines_endpoints_reference_points(arrows_lines_reference_points.clone());
        self.primitives_visible.set_lines_endpoints_reference_points(arrows_lines_reference_points.clone());
        self.primitives_visible_selected.set_lines_endpoints_reference_points(arrows_lines_reference_points);
        self.primitives_for_selection.set_triangles_vertices_reference_points(arrows_caps_reference_points.clone());
        self.primitives_visible.set_triangles_vertices_reference_points(arrows_caps_reference_points.clone());
        self.primitives_visible_selected.set_triangles_vertices_reference_points(arrows_caps_reference_points);
    }


    pub fn get_transformed_uid(&self) -> [u8; 4]
    {
        self.transformed_uid
    }
}


impl SelectedObjectTrait for UniformlyDistributedSurfaceLoad {}
