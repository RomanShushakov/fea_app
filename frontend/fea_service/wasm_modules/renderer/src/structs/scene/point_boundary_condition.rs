use wasm_bindgen::JsValue;

use crate::enums::GLMode;
use crate::structs::{Primitives, FloatNumbers};
use crate::functions::{convert_slice_to_array, monochrome_cone};
use crate::Props;
use crate::traits::SelectedObjectTrait;


#[derive(Debug, Clone)]
pub struct PointBoundaryCondition
{
    point_number: u32,
    primitives_for_selection: Primitives,
    primitives_visible: Primitives,
    primitives_visible_selected: Primitives,
}


impl PointBoundaryCondition
{
    fn add_direction_cap(props: &Props, primitives: &mut Primitives, vertex_coordinates: &[f32; 3], 
        base_center_point_coordinates: &[f32; 3], vertex_reference_point: &[f32; 3], color: &[f32; 4])
        -> Result<(), JsValue>
    {
        let (axes_cap_vertices_coordinates, axis_cap_vertices_colors_values) = 
            monochrome_cone(
                vertex_coordinates, 
                base_center_point_coordinates,
                props.point_boundary_condition_cap_height, 
                props.point_boundary_condition_cap_width, 
                props.point_boundary_condition_cap_base_point_number, 
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
        let norm = (props.point_boundary_condition_cap_height) / direction.iter().fold(0.0, 
            |acc, x| acc + x * x).sqrt();
        let base_center_point_coordinates = [direction[0] * norm, direction[1] * norm, 
            direction[2] * norm];
        PointBoundaryCondition::add_direction_cap(props, primitives, &[0.0, 0.0, 0.0], 
            &base_center_point_coordinates, reference_point, color)?;
        Ok(())
    }


    fn add_moment_arrows(color: &[f32; 4], primitives: &mut Primitives, reference_point: &[f32; 3], 
        direction: &[f32; 3], props: &Props) -> Result<(), JsValue>
    {
        let norm = (props.point_boundary_condition_cap_height) / direction.iter().fold(0.0, 
            |acc, x| acc + x * x).sqrt();
        let base_center_point_coordinates = [direction[0] * norm, direction[1] * norm, 
            direction[2] * norm];
        PointBoundaryCondition::add_direction_cap(props, primitives, &[0.0, 0.0, 0.0], 
            &base_center_point_coordinates, reference_point, color)?;
        PointBoundaryCondition::add_direction_cap(props, primitives, 
            &base_center_point_coordinates, 
            &[
                base_center_point_coordinates[0] * 2.0, base_center_point_coordinates[1] * 2.0,
                base_center_point_coordinates[2] * 2.0
            ], 
            reference_point, color)?;
        Ok(())
    }


    fn get_primitives_for_selection(transformed_uid: [u8; 4], point_coordinates: [f32; 3], optional_ux: Option<f32>, 
        optional_uy: Option<f32>, optional_uz: Option<f32>, optional_rx: Option<f32>, optional_ry: Option<f32>,
        optional_rz: Option<f32>, props: &Props) -> Result<Primitives, JsValue>
    {
        let mut primitives_for_selection = Primitives::create();
        let color_for_selection = transformed_uid.map(|v| v as f32 / 255.0);

        let optional_ux_direction = 
            if optional_ux.is_some() { Some([1.0, 0.0, 0.0]) } else { None };
        if let Some(ux_direction) = optional_ux_direction
        {
            PointBoundaryCondition::add_force_arrow(&color_for_selection, &mut primitives_for_selection, 
                &point_coordinates, &ux_direction, props)?;
        }

        let optional_uy_direction = 
            if optional_uy.is_some() { Some([0.0, 1.0, 0.0]) } else { None };
        if let Some(uy_direction) = optional_uy_direction
        {
            PointBoundaryCondition::add_force_arrow(&color_for_selection, &mut primitives_for_selection, 
                &point_coordinates, &uy_direction, props)?;
        }

        let optional_uz_direction = 
            if optional_uz.is_some() { Some([0.0, 0.0, 1.0]) } else { None };
        if let Some(uz_direction) = optional_uz_direction
        {
            PointBoundaryCondition::add_force_arrow(&color_for_selection, &mut primitives_for_selection, 
                &point_coordinates, &uz_direction, props)?;
        }

        let optional_rx_direction = 
            if optional_rx.is_some() { Some([-1.0, 0.0, 0.0]) } else { None };
        if let Some(rx_direction) = optional_rx_direction
        {
            PointBoundaryCondition::add_moment_arrows(&color_for_selection, &mut primitives_for_selection, 
                &point_coordinates, &rx_direction, props)?;
        }

        let optional_ry_direction = 
            if optional_ry.is_some() { Some([0.0, -1.0, 0.0]) } else { None };
        if let Some(ry_direction) = optional_ry_direction
        {
            PointBoundaryCondition::add_moment_arrows(&color_for_selection, &mut primitives_for_selection, 
                &point_coordinates, &ry_direction, props)?;
        }

        let optional_rz_direction = 
            if optional_rz.is_some() { Some([0.0, 0.0, -1.0]) } else { None };
        if let Some(rz_direction) = optional_rz_direction
        {
            PointBoundaryCondition::add_moment_arrows(&color_for_selection, &mut primitives_for_selection, 
                &point_coordinates, &rz_direction, props)?;
        }

        Ok(primitives_for_selection)
    }


    pub fn create(
        transformed_uid: [u8; 4], 
        point_number: u32,
        point_coordinates: [f32; 3],
        optional_ux: Option<f32>, 
        optional_uy: Option<f32>, 
        optional_uz: Option<f32>, 
        optional_rx: Option<f32>, 
        optional_ry: Option<f32>,
        optional_rz: Option<f32>,
        props: &Props
    ) 
        -> Result<Self, JsValue>
    {
        let primitives_for_selection = PointBoundaryCondition::get_primitives_for_selection(transformed_uid, 
            point_coordinates, optional_ux, optional_uy, optional_uz, optional_rx, optional_ry, optional_rz, props)?;

        let color_visible = props.point_boundary_condition_color;
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

        Ok(PointBoundaryCondition { point_number, primitives_for_selection, primitives_visible, 
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


    pub fn update_displacement_components(&mut self, transformed_uid: [u8; 4], optional_ux: Option<f32>, 
        optional_uy: Option<f32>, optional_uz: Option<f32>, optional_rx: Option<f32>, optional_ry: Option<f32>,
        optional_rz: Option<f32>, props: &Props) -> Result<(), JsValue>
    {
        let point_number = self.point_number;
        let point_coordinates: [f32; 3] = convert_slice_to_array(
            &self.primitives_for_selection.get_ref_triangles_vertices_reference_points().to_vec()[..3]);
        let PointBoundaryCondition { point_number: _, primitives_for_selection, primitives_visible, 
            primitives_visible_selected } = PointBoundaryCondition::create(transformed_uid, point_number, 
            point_coordinates, optional_ux, optional_uy, optional_uz, optional_rx, optional_ry, optional_rz, props)?;

        self.primitives_for_selection = primitives_for_selection;
        self.primitives_visible = primitives_visible;
        self.primitives_visible_selected = primitives_visible_selected;
        
        Ok(())
    }


    pub fn update_point_coordinates(&mut self, point_number: u32, point_coordinates: [f32; 3])
    {
        if point_number == self.point_number
        {
            let mut arrows_caps_reference_points = FloatNumbers::create();
            for _ in 0..self.primitives_for_selection.get_ref_triangles_vertices_coordinates().len() / 3
            {
                arrows_caps_reference_points.extend(&point_coordinates);
            }
            self.primitives_for_selection.set_triangles_vertices_reference_points(arrows_caps_reference_points.clone());
            self.primitives_visible.set_triangles_vertices_reference_points(arrows_caps_reference_points.clone());
            self.primitives_visible_selected.set_triangles_vertices_reference_points(arrows_caps_reference_points);
        }
    }
}


impl SelectedObjectTrait for PointBoundaryCondition {}
