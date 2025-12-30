use wasm_bindgen::JsValue;

use crate::enums::GLMode;
use crate::structs::{Primitives, FloatNumbers};
use crate::functions::{convert_slice_to_array, monochrome_cone};
use crate::Props;
use crate::traits::SelectedObjectTrait;


#[derive(Debug, Clone)]
pub struct ConcentratedLoad
{
    point_number: u32,
    primitives_for_selection: Primitives,
    primitives_visible: Primitives,
    primitives_visible_selected: Primitives,
}


impl ConcentratedLoad
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
                props.concentrated_load_cap_height, 
                props.concentrated_load_cap_width, 
                props.concentrated_load_cap_base_points_number, 
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
            &vec![0.0; axes_cap_vertices_coordinates.len()]);
        primitives.extend_triangles_vertices_colors(&axis_cap_vertices_colors_values);

        Ok(())
    }


    fn add_force_arrow(color: &[f32; 4], primitives: &mut Primitives, reference_point: &[f32; 3], 
        direction: &[f32; 3], props: &Props) -> Result<(), JsValue>
    {
        ConcentratedLoad::add_load_direction_line(color, primitives, reference_point, direction);
        let norm = (props.concentrated_load_line_length - props.concentrated_load_cap_height) / 
            direction.iter().fold(0.0, |acc, x| acc + x * x).sqrt();
        let base_center_point_coordinates = [direction[0] * norm, direction[1] * norm, 
            direction[2] * norm];
        ConcentratedLoad::add_load_direction_cap(props, primitives, direction, 
            &base_center_point_coordinates, reference_point, color)?;
        Ok(())
    }


    fn add_moment_arrows(color: &[f32; 4], primitives: &mut Primitives, reference_point: &[f32; 3], 
        direction: &[f32; 3], props: &Props) -> Result<(), JsValue>
    {
        ConcentratedLoad::add_load_direction_line(color, primitives, reference_point, direction);
        let norm = (props.concentrated_load_line_length * 0.8 - props.concentrated_load_cap_height) / 
            direction.iter().fold(0.0, |acc, x| acc + x * x).sqrt();
        let base_center_point_coordinates = [direction[0] * norm, direction[1] * norm, 
            direction[2] * norm];
        ConcentratedLoad::add_load_direction_cap(props, primitives, direction, 
            &base_center_point_coordinates, reference_point, color)?;
        ConcentratedLoad::add_load_direction_cap(props, primitives, 
            &[
                direction[0] * 0.8, direction[1] * 0.8, direction[2] * 0.8
            ], 
            &[
                base_center_point_coordinates[0] * 0.8, base_center_point_coordinates[1] * 0.8,
                base_center_point_coordinates[2] * 0.8
            ], 
            reference_point, color)?;
        Ok(())
    }


    fn get_primitives_for_selection(transformed_uid: [u8; 4], point_coordinates: [f32; 3], fx: f32, fy: f32, fz: f32,
        mx: f32, my: f32, mz: f32, props: &Props) -> Result<Primitives, JsValue>
    {
        let mut primitives_for_selection = Primitives::create();
        let color_for_selection = transformed_uid.map(|v| v as f32 / 255.0);

        let optional_fx_direction = 
            if fx > 0.0 { Some([props.concentrated_load_line_length, 0.0, 0.0]) } 
            else if fx < 0.0 { Some([-props.concentrated_load_line_length, 0.0, 0.0]) }
            else { None };
        if let Some(fx_direction) = optional_fx_direction
        {
            ConcentratedLoad::add_force_arrow(&color_for_selection, &mut primitives_for_selection, 
                &point_coordinates, &fx_direction, props)?;
        }

        let optional_fy_direction = 
            if fy > 0.0 { Some([0.0, props.concentrated_load_line_length, 0.0]) } 
            else if fy < 0.0 { Some([0.0, -props.concentrated_load_line_length, 0.0]) }
            else { None };
        if let Some(fy_direction) = optional_fy_direction
        {
            ConcentratedLoad::add_force_arrow(&color_for_selection, &mut primitives_for_selection, 
                &point_coordinates, &fy_direction, props)?;
        }

        let optional_fz_direction = 
            if fz > 0.0 { Some([0.0, 0.0, props.concentrated_load_line_length]) } 
            else if fz < 0.0 { Some([0.0, 0.0, -props.concentrated_load_line_length]) }
            else { None };
        if let Some(fz_direction) = optional_fz_direction
        {
            ConcentratedLoad::add_force_arrow(&color_for_selection, &mut primitives_for_selection, 
                &point_coordinates, &fz_direction, props)?;
        }

        let optional_mx_direction = 
            if mx > 0.0 { Some([props.concentrated_load_line_length * 0.8, 0.0, 0.0]) } 
            else if mx < 0.0 { Some([-props.concentrated_load_line_length * 0.8, 0.0, 0.0]) }
            else { None };
        if let Some(mx_direction) = optional_mx_direction
        {
            ConcentratedLoad::add_moment_arrows(&color_for_selection, &mut primitives_for_selection, 
                &point_coordinates, &mx_direction, props)?;
        }

        let optional_my_direction = 
            if my > 0.0 { Some([0.0, props.concentrated_load_line_length * 0.8, 0.0]) } 
            else if my < 0.0 { Some([0.0,  -props.concentrated_load_line_length * 0.8, 0.0]) }
            else { None };
        if let Some(my_direction) = optional_my_direction
        {
            ConcentratedLoad::add_moment_arrows(&color_for_selection, &mut primitives_for_selection, 
                &point_coordinates, &my_direction, props)?;
        }

        let optional_mz_direction = 
            if mz > 0.0 { Some([0.0, 0.0, props.concentrated_load_line_length * 0.8]) } 
            else if mz < 0.0 { Some([0.0, 0.0,  -props.concentrated_load_line_length * 0.8]) }
            else { None };
        if let Some(mz_direction) = optional_mz_direction
        {
            ConcentratedLoad::add_moment_arrows(&color_for_selection, &mut primitives_for_selection, 
                &point_coordinates, &mz_direction, props)?;
        }

        Ok(primitives_for_selection)
    }


    pub fn create(
        transformed_uid: [u8; 4], 
        point_number: u32,
        point_coordinates: [f32; 3],
        fx: f32,
        fy: f32,
        fz: f32,
        mx: f32,
        my: f32,
        mz: f32,
        props: &Props
    ) 
        -> Result<Self, JsValue>
    {
        let primitives_for_selection = ConcentratedLoad::get_primitives_for_selection(transformed_uid, 
            point_coordinates, fx, fy, fz, mx, my, mz, props)?;

        let color_visible = props.concentrated_load_color;
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

        Ok(ConcentratedLoad { point_number, primitives_for_selection, primitives_visible, 
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


    pub fn update_load_components(&mut self, transformed_uid: [u8; 4], fx: f32, fy: f32, fz: f32, mx: f32, my: f32, 
        mz: f32, props: &Props) -> Result<(), JsValue>
    {
        let point_number = self.point_number;
        let point_coordinates: [f32; 3] = convert_slice_to_array(
            &self.primitives_for_selection.get_ref_lines_endpoints_reference_points().to_vec()[..3]);
        let ConcentratedLoad { point_number: _, primitives_for_selection, primitives_visible, 
            primitives_visible_selected } = ConcentratedLoad::create(transformed_uid, point_number, 
            point_coordinates, fx, fy, fz, mx, my, mz, props)?;

        self.primitives_for_selection = primitives_for_selection;
        self.primitives_visible = primitives_visible;
        self.primitives_visible_selected = primitives_visible_selected;
        
        Ok(())
    }


    pub fn update_point_coordinates(&mut self, point_number: u32, point_coordinates: [f32; 3])
    {
        if point_number == self.point_number
        {
            let mut arrows_lines_reference_points = FloatNumbers::create();
            for _ in 0..self.primitives_for_selection.get_ref_lines_endpoints_coordinates().len() / 3
            {
                arrows_lines_reference_points.extend(&point_coordinates);
            }
            self.primitives_for_selection.set_lines_endpoints_reference_points(arrows_lines_reference_points.clone());
            self.primitives_visible.set_lines_endpoints_reference_points(arrows_lines_reference_points.clone());
            self.primitives_visible_selected.set_lines_endpoints_reference_points(arrows_lines_reference_points);
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


impl SelectedObjectTrait for ConcentratedLoad {}
