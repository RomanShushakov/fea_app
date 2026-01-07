use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::JsValue;

use crate::enums::{SurfaceObjectType, GLMode, MeshSeedType};
use crate::structs::{Primitives, FloatNumbers, UniformlyDistributedSurfaceLoad, SurfacePrimitives};
use crate::traits::{DenotationTrait, SelectedObjectTrait};
use crate::functions::{convert_vec_to_array, convert_slice_to_array, monochrome_cone};
use crate::Props;


#[derive(Debug, Clone)]
pub struct Surface
{
    number: u32,
    point_1_number: u32,
    point_2_number: u32,
    point_3_number: u32,
    point_4_number: u32,
    primitives_for_selection: Primitives,
    primitives_visible_edges_1_3: Primitives,
    primitives_visible_selected_edges_1_3: Primitives,
    primitives_visible_edges_2_4: Primitives,
    primitives_visible_selected_edges_2_4: Primitives,
    primitives_normal: Primitives,
    optional_primitives_mesh_seed: Option<[Primitives; 2]>,
    optional_uniformly_distributed_surface_load: Option<Rc<RefCell<UniformlyDistributedSurfaceLoad>>>,
}


impl Surface
{
    fn get_primitives_for_selection(
        transformed_uid: [u8; 4], 
        point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3], 
        point_3_coordinates: [f32; 3], 
        point_4_coordinates: [f32; 3],
        optional_point_1_displacement: Option<&[f32]>, 
        optional_point_2_displacement: Option<&[f32]>, 
        optional_point_3_displacement: Option<&[f32]>, 
        optional_point_4_displacement: Option<&[f32]>, 
    ) 
        -> Primitives
    {
        let mut primitives_for_selection = Primitives::create();
        for (point_coordinates, optional_point_displacements) in 
            [
                point_1_coordinates, point_2_coordinates, point_3_coordinates, 
                point_3_coordinates, point_4_coordinates, point_1_coordinates
            ].iter().zip(
                [
                    optional_point_1_displacement, optional_point_2_displacement, optional_point_3_displacement,
                    optional_point_3_displacement, optional_point_4_displacement, optional_point_1_displacement
                ]
            )
        {
            primitives_for_selection.extend_triangles_vertices_coordinates(point_coordinates);
            primitives_for_selection.extend_triangles_vertices_is_to_scale(&[1.0]);
            primitives_for_selection.extend_triangles_vertices_reference_points(point_coordinates);
            if let Some(displacement) = optional_point_displacements
            {
                primitives_for_selection.extend_triangles_vertices_displacements(displacement);
            }
            else
            {
                primitives_for_selection.extend_triangles_vertices_displacements(&[0.0, 0.0, 0.0]);
            }
        }
        let color_for_selection = transformed_uid.map(|v| v as f32 / 255.0);
        for _ in 0..6
        {
            primitives_for_selection.extend_triangles_vertices_colors(&color_for_selection);    
        }
        primitives_for_selection
    }


    fn add_normal_direction_line(props: &Props, primitives: &mut Primitives, center: &[f32; 3], 
        point_2_coordinates: &[f32; 3])
    {
        primitives.extend_lines_endpoints_coordinates(&[0.0, 0.0, 0.0]);
        primitives.extend_lines_endpoints_is_to_scale(&[0.0]);
        primitives.extend_lines_endpoints_reference_points(center);
        primitives.extend_lines_endpoints_displacements(&[0.0, 0.0, 0.0]);
        
        primitives.extend_lines_endpoints_coordinates(point_2_coordinates);
        primitives.extend_lines_endpoints_is_to_scale(&[0.0]);
        primitives.extend_lines_endpoints_reference_points(center);
        primitives.extend_lines_endpoints_displacements(&[0.0, 0.0, 0.0]);

        primitives.extend_lines_endpoints_colors(&props.surface_normal_color);
        primitives.extend_lines_endpoints_colors(&props.surface_normal_color);
    }


    fn add_normal_direction_cap(props: &Props, primitives: &mut Primitives, vertex_coordinates: &[f32; 3], 
        base_center_point_coordinates: &[f32; 3], vertex_reference_point: &[f32; 3]) -> Result<(), JsValue>
    {
        let (axes_cap_vertices_coordinates, axis_cap_vertices_colors_values) = 
            monochrome_cone(
                vertex_coordinates, 
                base_center_point_coordinates,
                props.surface_normal_cap_height, 
                props.surface_normal_cap_width, 
                props.surface_normal_cap_base_points_number, 
                &props.surface_normal_color, 
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


    fn get_primitives_visible_edges_1_3(
        point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3], 
        point_3_coordinates: [f32; 3], 
        point_4_coordinates: [f32; 3],
        optional_point_1_displacement: Option<&[f32]>, 
        optional_point_2_displacement: Option<&[f32]>, 
        optional_point_3_displacement: Option<&[f32]>, 
        optional_point_4_displacement: Option<&[f32]>, 
        color_visible: [f32; 4]
    ) 
        -> Primitives
    {
        let mut primitives_visible_edges_1_3 = Primitives::create();
        for ((point_1_coordinates, point_2_coordinates), 
            (point_1_displacements, point_2_displacements)) in
            [
                (point_1_coordinates, point_2_coordinates), (point_3_coordinates, point_4_coordinates)
            ].iter().zip(
                [
                    (optional_point_1_displacement, optional_point_2_displacement),
                    (optional_point_3_displacement, optional_point_4_displacement),
                ]
            )
        {   
            primitives_visible_edges_1_3.extend_lines_endpoints_coordinates(point_1_coordinates);
            primitives_visible_edges_1_3.extend_lines_endpoints_is_to_scale(&[1.0]);
            primitives_visible_edges_1_3.extend_lines_endpoints_reference_points(point_1_coordinates);
            if let Some(displacement) = point_1_displacements
            {
                primitives_visible_edges_1_3.extend_lines_endpoints_displacements(displacement);
            }
            else
            {
                primitives_visible_edges_1_3.extend_lines_endpoints_displacements(&[0.0, 0.0, 0.0]);
            }
            primitives_visible_edges_1_3.extend_lines_endpoints_coordinates(point_2_coordinates);
            primitives_visible_edges_1_3.extend_lines_endpoints_is_to_scale(&[1.0]);
            primitives_visible_edges_1_3.extend_lines_endpoints_reference_points(point_2_coordinates);
            if let Some(displacement) = point_2_displacements
            {
                primitives_visible_edges_1_3.extend_lines_endpoints_displacements(displacement);
            }
            else
            {
                primitives_visible_edges_1_3.extend_lines_endpoints_displacements(&[0.0, 0.0, 0.0]);
            }
        }
        let mut lines_endpoints_colors = FloatNumbers::create();
        for _ in 0..4
        {
            lines_endpoints_colors.extend(&color_visible);    
        }
        primitives_visible_edges_1_3.set_lines_endpoints_colors(lines_endpoints_colors);
        primitives_visible_edges_1_3
    }


    fn get_primitives_visible_edges_2_4(
        point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3], 
        point_3_coordinates: [f32; 3], 
        point_4_coordinates: [f32; 3],
        optional_point_1_displacement: Option<&[f32]>, 
        optional_point_2_displacement: Option<&[f32]>, 
        optional_point_3_displacement: Option<&[f32]>, 
        optional_point_4_displacement: Option<&[f32]>, 
        color_visible: [f32; 4]
    ) 
        -> Primitives
    {
        let mut primitives_visible_edges_2_4 = Primitives::create();
        for ((point_1_coordinates, point_2_coordinates), 
            (point_1_displacements, point_2_displacements)) in
            [
                (point_2_coordinates, point_3_coordinates), (point_4_coordinates, point_1_coordinates)
            ].iter().zip(
                [
                    (optional_point_2_displacement, optional_point_3_displacement),
                    (optional_point_4_displacement, optional_point_1_displacement)
                ]
            )
        {   
            primitives_visible_edges_2_4.extend_lines_endpoints_coordinates(point_1_coordinates);
            primitives_visible_edges_2_4.extend_lines_endpoints_is_to_scale(&[1.0]);
            primitives_visible_edges_2_4.extend_lines_endpoints_reference_points(point_1_coordinates);
            if let Some(displacement) = point_1_displacements
            {
                primitives_visible_edges_2_4.extend_lines_endpoints_displacements(displacement);
            }
            else
            {
                primitives_visible_edges_2_4.extend_lines_endpoints_displacements(&[0.0, 0.0, 0.0]);
            }
            primitives_visible_edges_2_4.extend_lines_endpoints_coordinates(point_2_coordinates);
            primitives_visible_edges_2_4.extend_lines_endpoints_is_to_scale(&[1.0]);
            primitives_visible_edges_2_4.extend_lines_endpoints_reference_points(point_2_coordinates);
            if let Some(displacement) = point_2_displacements
            {
                primitives_visible_edges_2_4.extend_lines_endpoints_displacements(displacement);
            }
            else
            {
                primitives_visible_edges_2_4.extend_lines_endpoints_displacements(&[0.0, 0.0, 0.0]);
            }
        }
        let mut lines_endpoints_colors = FloatNumbers::create();
        for _ in 0..4
        {
            lines_endpoints_colors.extend(&color_visible);    
        }
        primitives_visible_edges_2_4.set_lines_endpoints_colors(lines_endpoints_colors);
        primitives_visible_edges_2_4
    }


    pub fn create(
        transformed_uid: [u8; 4], 
        number: u32, 
        point_1_number: u32, 
        point_2_number: u32, 
        point_3_number: u32, 
        point_4_number: u32,
        point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3], 
        point_3_coordinates: [f32; 3], 
        point_4_coordinates: [f32; 3], 
        surface_object_type: SurfaceObjectType, 
        optional_point_1_displacement: Option<&[f32]>, 
        optional_point_2_displacement: Option<&[f32]>, 
        optional_point_3_displacement: Option<&[f32]>, 
        optional_point_4_displacement: Option<&[f32]>, 
        props: &Props
    ) 
        -> Self
    {
        let primitives_for_selection = Surface::get_primitives_for_selection(transformed_uid, 
            point_1_coordinates, point_2_coordinates, point_3_coordinates, point_4_coordinates, 
            optional_point_1_displacement, optional_point_2_displacement, optional_point_3_displacement, 
            optional_point_4_displacement);
        
        let color_visible = match surface_object_type
        {
            SurfaceObjectType::SurfaceDefault => props.surface_default_color,
            SurfaceObjectType::SurfacePlate => props.surface_plate_props_color,
        };

        let primitives_visible_edges_1_3 = Surface::get_primitives_visible_edges_1_3(point_1_coordinates, 
            point_2_coordinates, point_3_coordinates, point_4_coordinates, optional_point_1_displacement, 
            optional_point_2_displacement, optional_point_3_displacement, optional_point_4_displacement, color_visible);
        let mut primitives_visible_selected_edges_1_3 = primitives_visible_edges_1_3.clone();
        let mut selected_lines_endpoints_colors = FloatNumbers::create();
        for _ in 0..4
        {
            selected_lines_endpoints_colors.extend(&props.drawn_object_selected_color);    
        }
        primitives_visible_selected_edges_1_3.set_lines_endpoints_colors(selected_lines_endpoints_colors);

        let primitives_visible_edges_2_4 = Surface::get_primitives_visible_edges_2_4(point_1_coordinates, 
            point_2_coordinates, point_3_coordinates, point_4_coordinates, optional_point_1_displacement, 
            optional_point_2_displacement, optional_point_3_displacement, optional_point_4_displacement, color_visible);
        let mut primitives_visible_selected_edges_2_4 = primitives_visible_edges_2_4.clone();
        let mut selected_lines_endpoints_colors = FloatNumbers::create();
        for _ in 0..4
        {
            selected_lines_endpoints_colors.extend(&props.drawn_object_selected_color);    
        }
        primitives_visible_selected_edges_2_4.set_lines_endpoints_colors(selected_lines_endpoints_colors);

        Surface 
        { 
            number, point_1_number, point_2_number, point_3_number, point_4_number, primitives_for_selection, 
            primitives_visible_edges_1_3, primitives_visible_selected_edges_1_3, primitives_visible_edges_2_4, 
            primitives_visible_selected_edges_2_4, primitives_normal: Primitives::create(), 
            optional_primitives_mesh_seed: None, optional_uniformly_distributed_surface_load: None,
        }
    }


    pub fn get_primitives(&self, gl_mode: &GLMode, is_selected: bool) -> SurfacePrimitives
    {
        match gl_mode
        {
            GLMode::Selection => 
            {
                let mut surface_primitives = SurfacePrimitives::init();
                surface_primitives.optional_primitives_for_selection = Some(self.primitives_for_selection.clone());
                surface_primitives
            },
            GLMode::Visible => 
            {
                let mut surface_primitives = SurfacePrimitives::init();
                surface_primitives.optional_primitives_normal = Some(self.primitives_normal.clone());
                if is_selected
                {
                    surface_primitives.optional_primitives_edges_1_3 = 
                        Some(self.primitives_visible_selected_edges_1_3.clone());
                    surface_primitives.optional_primitives_edges_2_4 = 
                        Some(self.primitives_visible_selected_edges_2_4.clone());
                }
                else
                {
                    surface_primitives.optional_primitives_edges_1_3 = Some(self.primitives_visible_edges_1_3.clone());
                    surface_primitives.optional_primitives_edges_2_4 = Some(self.primitives_visible_edges_2_4.clone());
                }

                if let Some(primitives_mesh_seed) = self.optional_primitives_mesh_seed.as_ref()
                {
                    surface_primitives.optional_primitives_mesh_seed_edges_1_3 = Some(primitives_mesh_seed[0].clone());
                    surface_primitives.optional_primitives_mesh_seed_edges_2_4 = Some(primitives_mesh_seed[1].clone());
                }
                surface_primitives
            }
        }
    }


    fn get_primitives_for_selection_start_indexes(&self, point_number: u32) -> Vec<usize>
    {
        let mut primitives_for_selection_start_indexes = Vec::new();
        if point_number == self.point_1_number
        {
            primitives_for_selection_start_indexes.extend([0, 15]);
        }
        if point_number == self.point_2_number
        {
            primitives_for_selection_start_indexes.extend([3]);
        }
        if point_number == self.point_3_number
        {
            primitives_for_selection_start_indexes.extend([6, 9]);
        }
        if point_number == self.point_4_number
        {
            primitives_for_selection_start_indexes.extend([12]);
        }
        primitives_for_selection_start_indexes
    } 


    fn get_primitives_visible_edges_1_3_start_indexes(&self, point_number: u32) -> Vec<usize>
    {
        let mut primitives_visible_edges_1_3_start_indexes = Vec::new();
        if point_number == self.point_1_number
        {
            primitives_visible_edges_1_3_start_indexes.extend([0]);
        }
        if point_number == self.point_2_number
        {
            primitives_visible_edges_1_3_start_indexes.extend([3]);
        }
        if point_number == self.point_3_number
        {
            primitives_visible_edges_1_3_start_indexes.extend([6]);
        }
        if point_number == self.point_4_number
        {
            primitives_visible_edges_1_3_start_indexes.extend([9]);
        }
        primitives_visible_edges_1_3_start_indexes
    } 


    fn get_primitives_visible_edges_2_4_start_indexes(&self, point_number: u32) -> Vec<usize>
    {
        let mut primitives_visible_edges_2_4_start_indexes = Vec::new();
        if point_number == self.point_1_number
        {
            primitives_visible_edges_2_4_start_indexes.extend([9]);
        }
        if point_number == self.point_2_number
        {
            primitives_visible_edges_2_4_start_indexes.extend([0]);
        }
        if point_number == self.point_3_number
        {
            primitives_visible_edges_2_4_start_indexes.extend([3]);
        }
        if point_number == self.point_4_number
        {
            primitives_visible_edges_2_4_start_indexes.extend([6]);
        }
        primitives_visible_edges_2_4_start_indexes
    }


    pub fn update_vertex_point_coordinates(&mut self, point_number: u32, point_coordinates: [f32; 3], props: &Props)
    {
        let primitives_for_selection_start_indexes = 
            self.get_primitives_for_selection_start_indexes(point_number);
        let mut vertices_coordinates_for_selection = self.primitives_for_selection
            .get_ref_triangles_vertices_coordinates().to_vec();
        for start_index in primitives_for_selection_start_indexes
        {
            for i in 0..3
            {
                vertices_coordinates_for_selection[start_index + i] = point_coordinates[i];
            }
        }
        let mut new_vertices_coordinates_for_selection = FloatNumbers::create();
        new_vertices_coordinates_for_selection.extend(&vertices_coordinates_for_selection);
        self.primitives_for_selection.set_triangles_vertices_coordinates(new_vertices_coordinates_for_selection);

        let primitives_visible_edges_1_3_start_indexes = 
            self.get_primitives_visible_edges_1_3_start_indexes(point_number);
        let mut vertices_coordinates_edges_1_3_visible = self.primitives_visible_edges_1_3
            .get_ref_lines_endpoints_coordinates().to_vec();
        for start_index in primitives_visible_edges_1_3_start_indexes
        {
            for i in 0..3
            {
                vertices_coordinates_edges_1_3_visible[start_index + i] = point_coordinates[i];
            }
        }
        let mut new_vertices_coordinates_edges_1_3_visible = FloatNumbers::create();
        new_vertices_coordinates_edges_1_3_visible.extend(&vertices_coordinates_edges_1_3_visible);
        self.primitives_visible_edges_1_3.set_lines_endpoints_coordinates(
            new_vertices_coordinates_edges_1_3_visible.clone());
        self.primitives_visible_selected_edges_1_3.set_lines_endpoints_coordinates(
            new_vertices_coordinates_edges_1_3_visible);

        let primitives_visible_edges_2_4_start_indexes = 
            self.get_primitives_visible_edges_2_4_start_indexes(point_number);
        let mut vertices_coordinates_edges_2_4_visible = self.primitives_visible_edges_2_4
            .get_ref_lines_endpoints_coordinates().to_vec();
        for start_index in primitives_visible_edges_2_4_start_indexes
        {
            for i in 0..3
            {
                vertices_coordinates_edges_2_4_visible[start_index + i] = point_coordinates[i];
            }
        }
        let mut new_vertices_coordinates_edges_2_4_visible = FloatNumbers::create();
        new_vertices_coordinates_edges_2_4_visible.extend(&vertices_coordinates_edges_2_4_visible);
        self.primitives_visible_edges_2_4.set_lines_endpoints_coordinates(
            new_vertices_coordinates_edges_2_4_visible.clone());
        self.primitives_visible_selected_edges_2_4.set_lines_endpoints_coordinates(
            new_vertices_coordinates_edges_2_4_visible);

        let center = self.get_center();
        let mut normal_line_reference_points = FloatNumbers::create();
        for _ in 0..self.primitives_normal.get_ref_lines_endpoints_coordinates().len() / 3
        {
            normal_line_reference_points.extend(&center);
        }
        self.primitives_normal.set_lines_endpoints_reference_points(normal_line_reference_points);
        let mut normal_cap_reference_points = FloatNumbers::create();
        for _ in 0..self.primitives_normal.get_ref_triangles_vertices_coordinates().len() / 3
        {
            normal_cap_reference_points.extend(&center);
        }
        self.primitives_normal.set_triangles_vertices_reference_points(normal_cap_reference_points);

        let [point_1_coordinates, point_2_coordinates, point_3_coordinates, 
                point_4_coordinates] = self.get_vertex_points_coordinates();
        if let Some(primitives_mesh_seed) = self.optional_primitives_mesh_seed.as_mut()
        {
            let edge_1_3_mesh_seed_value = (primitives_mesh_seed[0].get_points_count() / 2 + 1) as u8;
            Surface::update_edge_mesh_seed_point_coordinates(&mut primitives_mesh_seed[0], 
                &point_1_coordinates, &point_2_coordinates, 
                &point_4_coordinates, &point_3_coordinates, 
                edge_1_3_mesh_seed_value);

            let edge_2_4_mesh_seed_value = (primitives_mesh_seed[1].get_points_count() / 2 + 1) as u8;
            Surface::update_edge_mesh_seed_point_coordinates(&mut primitives_mesh_seed[1], 
                &point_2_coordinates, &point_3_coordinates, 
                &point_1_coordinates, &point_4_coordinates, 
                edge_2_4_mesh_seed_value);
        }
        if let Some(uniformly_distributed_surface_load) = 
            self.optional_uniformly_distributed_surface_load.as_ref()
        {
            uniformly_distributed_surface_load.borrow_mut().update_point_coordinates(&point_1_coordinates, 
                &point_2_coordinates, &point_3_coordinates, &point_4_coordinates, props);
        }
    }


    fn get_vertices_coordinates_in_primitives_for_selection(point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3], point_3_coordinates: [f32; 3], point_4_coordinates: [f32; 3]) -> FloatNumbers
    {
        let mut triangles_vertices_coordinates = FloatNumbers::create();
        for point_coordinates in 
            [
                point_1_coordinates, point_2_coordinates, point_3_coordinates, 
                point_3_coordinates, point_4_coordinates, point_1_coordinates
            ].iter()
        {
            triangles_vertices_coordinates.extend(point_coordinates);
        }
        triangles_vertices_coordinates
    }


    fn get_vertices_coordinates_in_primitives_visible_edges_1_3(point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3], point_3_coordinates: [f32; 3], point_4_coordinates: [f32; 3]) -> FloatNumbers
    {
        let mut triangles_vertices_coordinates = FloatNumbers::create();
        for (point_1_coordinates, point_2_coordinates) in
            [
                (point_1_coordinates, point_2_coordinates), (point_3_coordinates, point_4_coordinates)
            ].iter()
        {   
            triangles_vertices_coordinates.extend(point_1_coordinates);
            triangles_vertices_coordinates.extend(point_2_coordinates);
        }
        triangles_vertices_coordinates
    }


    fn get_vertices_coordinates_in_primitives_visible_edges_2_4(point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3], point_3_coordinates: [f32; 3], point_4_coordinates: [f32; 3]) -> FloatNumbers
    {
        let mut triangles_vertices_coordinates = FloatNumbers::create();
        for (point_1_coordinates, point_2_coordinates) in
            [
                (point_2_coordinates, point_3_coordinates), (point_4_coordinates, point_1_coordinates)
            ].iter()
        {   
            triangles_vertices_coordinates.extend(point_1_coordinates);
            triangles_vertices_coordinates.extend(point_2_coordinates);
        }
        triangles_vertices_coordinates
    }


    pub fn update_vertices(&mut self, point_1_number: u32, point_2_number: u32, point_3_number: u32, 
        point_4_number: u32, point_1_coordinates: [f32; 3], point_2_coordinates: [f32; 3], 
        point_3_coordinates: [f32; 3], point_4_coordinates: [f32; 3])
    {
        self.point_1_number = point_1_number;
        self.point_2_number = point_2_number;
        self.point_3_number = point_3_number;
        self.point_4_number = point_4_number;

        let vertices_coordinates_in_primitives_for_selection = 
            Surface::get_vertices_coordinates_in_primitives_for_selection(point_1_coordinates, point_2_coordinates, 
                point_3_coordinates, point_4_coordinates);
        self.primitives_for_selection.set_triangles_vertices_coordinates(
            vertices_coordinates_in_primitives_for_selection);

        let vertices_coordinates_in_primitives_visible_edges_1_3 = 
            Surface::get_vertices_coordinates_in_primitives_visible_edges_1_3(point_1_coordinates, point_2_coordinates, 
                point_3_coordinates, point_4_coordinates);
        self.primitives_visible_edges_1_3.set_lines_endpoints_coordinates(
            vertices_coordinates_in_primitives_visible_edges_1_3.clone());
        self.primitives_visible_selected_edges_1_3.set_lines_endpoints_coordinates(
            vertices_coordinates_in_primitives_visible_edges_1_3);

        let vertices_coordinates_in_primitives_visible_edges_2_4 = 
            Surface::get_vertices_coordinates_in_primitives_visible_edges_2_4(point_1_coordinates, point_2_coordinates, 
                point_3_coordinates, point_4_coordinates);
        self.primitives_visible_edges_2_4.set_lines_endpoints_coordinates(
            vertices_coordinates_in_primitives_visible_edges_2_4.clone());
        self.primitives_visible_selected_edges_2_4.set_lines_endpoints_coordinates(
            vertices_coordinates_in_primitives_visible_edges_2_4);
    }


    pub fn update_surface_object_type(&mut self, surface_object_type: SurfaceObjectType, props: &Props)
    {
        let color_visible = match surface_object_type
        {
            SurfaceObjectType::SurfaceDefault => props.surface_default_color,
            SurfaceObjectType::SurfacePlate => props.surface_plate_props_color,
        };
        let mut vertices_colors = FloatNumbers::create();
        for _ in 0..4
        {
            vertices_colors.extend(&color_visible);
        }
        self.primitives_visible_edges_1_3.set_lines_endpoints_colors(vertices_colors.clone());
        self.primitives_visible_edges_2_4.set_lines_endpoints_colors(vertices_colors);
    }


    pub fn update_surface_normal(&mut self, normal: &[f32], props: &Props) -> Result<(), JsValue>
    {
        let direction: [f32; 3] = convert_slice_to_array(normal);
        let norm = props.surface_normal_line_length / 
            direction.iter().fold(0.0, |acc, x| acc + x * x).sqrt();
        let center = self.get_center();
        let point_2_coordinates = [direction[0] * norm, direction[1] * norm, direction[2] * norm];

        let mut primitives_normal = Primitives::create();

        Surface::add_normal_direction_line(props, &mut primitives_normal, &center, &point_2_coordinates);

        let norm_2 = (props.surface_normal_line_length - 
            props.surface_normal_cap_height) / 
            direction.iter().fold(0.0, |acc, x| acc + x * x).sqrt();
        let base_center_point_coordinates = [direction[0] * norm_2, direction[1] * norm_2, 
            direction[2] * norm_2];

        Surface::add_normal_direction_cap(props, &mut primitives_normal, 
            &point_2_coordinates, &base_center_point_coordinates, 
            &center)?;

        self.primitives_normal = primitives_normal;

        Ok(())
    }


    pub fn get_vertex_points_coordinates(&self) -> [[f32; 3]; 4]
    {
        let endpoints_coordinates: [f32; 12] = convert_vec_to_array(
            self.primitives_visible_edges_1_3.get_ref_lines_endpoints_coordinates().to_vec());
        [
            [endpoints_coordinates[0], endpoints_coordinates[1], endpoints_coordinates[2]], 
            [endpoints_coordinates[3], endpoints_coordinates[4], endpoints_coordinates[5]],
            [endpoints_coordinates[6], endpoints_coordinates[7], endpoints_coordinates[8]],
            [endpoints_coordinates[9], endpoints_coordinates[10], endpoints_coordinates[11]],
        ]
    }


    fn update_edge_mesh_seed_point_coordinates(primitives: &mut Primitives, 
        edge_1_point_1_coordinates: &[f32; 3], edge_1_point_2_coordinates: &[f32; 3], 
        edge_2_point_1_coordinates: &[f32; 3], edge_2_point_2_coordinates: &[f32; 3],
        mesh_seed_value: u8)
    {
        let mut mesh_seeds_coordinates = FloatNumbers::create();
        let mut mesh_seeds_reference_points = FloatNumbers::create();

        let edge_1_step = [
            (edge_1_point_2_coordinates[0] - edge_1_point_1_coordinates[0]) / mesh_seed_value as f32,
            (edge_1_point_2_coordinates[1] - edge_1_point_1_coordinates[1]) / mesh_seed_value as f32,
            (edge_1_point_2_coordinates[2] - edge_1_point_1_coordinates[2]) / mesh_seed_value as f32,
        ];

        let edge_2_step = [
            (edge_2_point_2_coordinates[0] - edge_2_point_1_coordinates[0]) / mesh_seed_value as f32,
            (edge_2_point_2_coordinates[1] - edge_2_point_1_coordinates[1]) / mesh_seed_value as f32,
            (edge_2_point_2_coordinates[2] - edge_2_point_1_coordinates[2]) / mesh_seed_value as f32,
        ];

        for i in 1..mesh_seed_value
        {
            let edge_1_mesh_seed_coordinates = [
                edge_1_point_1_coordinates[0] + edge_1_step[0] * i as f32,
                edge_1_point_1_coordinates[1] + edge_1_step[1] * i as f32, 
                edge_1_point_1_coordinates[2] + edge_1_step[2] * i as f32,
            ];
            mesh_seeds_coordinates.extend(&edge_1_mesh_seed_coordinates);
            mesh_seeds_reference_points.extend(&edge_1_mesh_seed_coordinates);
        }
        for i in 1..mesh_seed_value
        {
            let edge_2_mesh_seed_coordinates = [
                edge_2_point_1_coordinates[0] + edge_2_step[0] * i as f32,
                edge_2_point_1_coordinates[1] + edge_2_step[1] * i as f32, 
                edge_2_point_1_coordinates[2] + edge_2_step[2] * i as f32,
            ];
            mesh_seeds_coordinates.extend(&edge_2_mesh_seed_coordinates);
            mesh_seeds_reference_points.extend(&edge_2_mesh_seed_coordinates);
        }
        primitives.set_points_coordinates(mesh_seeds_coordinates);
        primitives.set_points_reference_points(mesh_seeds_reference_points);
    }


    fn update_edge_mesh_seed(primitives: &mut Primitives, color_visible: &[f32; 4], edge_point_1_coordinates: &[f32; 3], 
        edge_point_2_coordinates: &[f32; 3], mesh_seed_value: u8)
    {
        let step = [
            (edge_point_2_coordinates[0] - edge_point_1_coordinates[0]) / mesh_seed_value as f32,
            (edge_point_2_coordinates[1] - edge_point_1_coordinates[1]) / mesh_seed_value as f32,
            (edge_point_2_coordinates[2] - edge_point_1_coordinates[2]) / mesh_seed_value as f32,
        ];

        for i in 1..mesh_seed_value
        {
            let mesh_seed_coordinates = [
                edge_point_1_coordinates[0] + step[0] * i as f32,
                edge_point_1_coordinates[1] + step[1] * i as f32, 
                edge_point_1_coordinates[2] + step[2] * i as f32,
            ];
            primitives.extend_points_coordinates(&mesh_seed_coordinates);
            primitives.extend_points_colors(color_visible);
            primitives.extend_points_is_to_scale(&[1.0]);
            primitives.extend_points_displacements(&[0.0, 0.0, 0.0]);
            primitives.extend_points_reference_points(&mesh_seed_coordinates);
        }
    }


    pub fn update_surface_mesh_seed(&mut self, surface_mesh_seed_values: Option<Box<[u8]>>, 
        mesh_seed_type: MeshSeedType, props: &Props)
    {
        let optional_color_visible = match mesh_seed_type
        {
            MeshSeedType::Global => Some(props.global_mesh_seed_color),
            MeshSeedType::Local => Some(props.local_mesh_seed_color),
            MeshSeedType::None => None,
        };
        if let (Some(color_visible), Some(values)) = (optional_color_visible, surface_mesh_seed_values)
        {
            let mut primitives_mesh_seed = [Primitives::create(), Primitives::create()];

            let [point_1_coordinates, point_2_coordinates, point_3_coordinates, 
                point_4_coordinates] = self.get_vertex_points_coordinates();

            Surface::update_edge_mesh_seed(&mut primitives_mesh_seed[0], &color_visible, 
                &point_1_coordinates, &point_2_coordinates, 
                values[0]);

            Surface::update_edge_mesh_seed(&mut primitives_mesh_seed[0], &color_visible, 
                &point_4_coordinates, &point_3_coordinates, 
                values[0]);

            Surface::update_edge_mesh_seed(&mut primitives_mesh_seed[1], &color_visible, 
                &point_2_coordinates, &point_3_coordinates, 
                values[1]);

            Surface::update_edge_mesh_seed(&mut primitives_mesh_seed[1], &color_visible, 
                &point_1_coordinates, &point_4_coordinates, 
                values[1]);

            self.optional_primitives_mesh_seed = Some(primitives_mesh_seed);
        }
        else
        {
            self.optional_primitives_mesh_seed = None;
        }
    }


    pub fn update_uniformly_distributed_surface_load(&mut self, transformed_uid: [u8; 4], point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3], point_3_coordinates: [f32; 3], point_4_coordinates: [f32; 3], px: f32, py: f32, 
        pz: f32, props: &Props) -> Result<(), JsValue>
    {
        let uniformly_distributed_surface_load = 
            Rc::new(RefCell::new(UniformlyDistributedSurfaceLoad::create(transformed_uid, point_1_coordinates, 
                point_2_coordinates, point_3_coordinates, point_4_coordinates, px, py, pz, props)?));
        self.optional_uniformly_distributed_surface_load = Some(uniformly_distributed_surface_load);
        Ok(())
    }


    pub fn get_ref_uniformly_distributed_surface_load(&self) -> Option<Rc<RefCell<UniformlyDistributedSurfaceLoad>>>
    {
        if let Some(uniformly_distributed_surface_load) = 
            self.optional_uniformly_distributed_surface_load.as_ref()
        {
            return Some(Rc::clone(uniformly_distributed_surface_load));
        }
        None
    }


    pub fn get_optional_uniformly_distributed_surface_load_transformed_uid(&self) -> Option<[u8; 4]>
    {
        if let Some(uniformly_distributed_surface_load) = 
            self.optional_uniformly_distributed_surface_load.as_ref()
        {
            let transformed_uid = uniformly_distributed_surface_load.borrow().get_transformed_uid();
            return Some(transformed_uid);
        }
        None
    }
}


impl DenotationTrait for Surface
{
    fn get_notation(&self) -> String 
    {
        self.number.to_string()
    }


    fn get_center(&self) -> [f32; 3] 
    {
        let triangles_vertices_coordinates: [f32; 18] = convert_vec_to_array(self.primitives_for_selection
            .get_ref_triangles_vertices_coordinates().to_vec());
        
        [
            (triangles_vertices_coordinates[0] + triangles_vertices_coordinates[3] + 
                triangles_vertices_coordinates[6] + triangles_vertices_coordinates[12]) / 4.0,
            (triangles_vertices_coordinates[1] + triangles_vertices_coordinates[4] + 
                triangles_vertices_coordinates[7] + triangles_vertices_coordinates[13]) / 4.0,
            (triangles_vertices_coordinates[2] + triangles_vertices_coordinates[5] + 
                triangles_vertices_coordinates[8] + triangles_vertices_coordinates[14]) / 4.0,
        ]
    }


    fn get_color_str(&self, is_selected: bool) -> String 
    {
        let color_array_data = 
            if is_selected
            {
                self.primitives_visible_selected_edges_1_3.get_ref_lines_endpoints_colors()
            }
            else
            {
                self.primitives_visible_edges_1_3.get_ref_lines_endpoints_colors()
            };
        let color_array: [f32; 16] = convert_vec_to_array(color_array_data.to_vec());
        let color_str = 
            format!("rgb({}, {}, {})", color_array[0] * 255.0, color_array[1] * 255.0, color_array[2] * 255.0);
        color_str
    }
}

impl SelectedObjectTrait for Surface {}
