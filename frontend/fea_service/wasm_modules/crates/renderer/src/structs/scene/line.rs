use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::JsValue;

use crate::enums::{LineObjectType, GLMode, MeshSeedType};
use crate::structs::{Primitives, FloatNumbers, UniformlyDistributedLineLoad, LinePrimitives};
use crate::traits::{DenotationTrait, SelectedObjectTrait};
use crate::functions::{convert_vec_to_array, convert_slice_to_array, monochrome_cone};
use crate::Props;


#[derive(Debug, Clone)]
pub struct Line
{
    number: u32,
    point_1_number: u32,
    point_2_number: u32,
    primitives_for_selection: Primitives,
    primitives_visible: Primitives,
    primitives_visible_selected: Primitives,
    optional_primitives_local_axis_1_direction: Option<Primitives>,
    optional_primitives_mesh_seed: Option<Primitives>,
    optional_uniformly_distributed_line_load: Option<Rc<RefCell<UniformlyDistributedLineLoad>>>,
}


impl Line
{
    fn get_primitives_for_selection(
        transformed_uid: [u8; 4],
        point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3],
        optional_point_1_displacement: Option<&[f32]>, 
        optional_point_2_displacement: Option<&[f32]>,
    ) 
        -> Primitives
    {
        let mut primitives_for_selection = Primitives::create();
        primitives_for_selection.extend_lines_endpoints_coordinates(&point_1_coordinates);
        primitives_for_selection.extend_lines_endpoints_is_to_scale(&[1.0]);
        primitives_for_selection.extend_lines_endpoints_reference_points(&point_1_coordinates);
        if let Some(displacement) = optional_point_1_displacement
        {
            primitives_for_selection.extend_lines_endpoints_displacements(displacement);
        }
        else
        {
            primitives_for_selection.extend_lines_endpoints_displacements(&[0.0, 0.0, 0.0]);
        }
        primitives_for_selection.extend_lines_endpoints_coordinates(&point_2_coordinates);
        primitives_for_selection.extend_lines_endpoints_is_to_scale(&[1.0]);
        primitives_for_selection.extend_lines_endpoints_reference_points(&point_2_coordinates);
        if let Some(displacement) = optional_point_2_displacement
        {
            primitives_for_selection.extend_lines_endpoints_displacements(displacement);
        }
        else
        {
            primitives_for_selection.extend_lines_endpoints_displacements(&[0.0, 0.0, 0.0]);
        }
        let color_for_selection = transformed_uid.map(|v| v as f32 / 255.0);
        primitives_for_selection.extend_lines_endpoints_colors(&color_for_selection);
        primitives_for_selection.extend_lines_endpoints_colors(&color_for_selection);
        primitives_for_selection
    }


    fn add_local_axis_1_direction_line(
        props: &Props, primitives: &mut Primitives, center: &[f32; 3], point_2_coordinates: &[f32; 3],
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

        primitives.extend_lines_endpoints_colors(&props.beam_section_orientation_color);
        primitives.extend_lines_endpoints_colors(&props.beam_section_orientation_color);
    }


    fn add_local_axis_1_direction_cap(
        props: &Props, 
        primitives: &mut Primitives, 
        vertex_coordinates: &[f32; 3], 
        base_center_point_coordinates: &[f32; 3],
        vertex_reference_point: &[f32; 3],
    ) 
        -> Result<(), JsValue>
    {
        let (
            axes_cap_vertices_coordinates, axis_cap_vertices_colors_values,
        ) = monochrome_cone(
            vertex_coordinates, 
            base_center_point_coordinates,
            props.beam_section_orientation_cap_height, 
            props.beam_section_orientation_cap_width, 
            props.beam_section_orientation_cap_base_points_number, 
            &props.beam_section_orientation_color, 
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
        number: u32, 
        point_1_number: u32, 
        point_2_number: u32,
        point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3],
        line_object_type: LineObjectType, 
        optional_point_1_displacement: Option<&[f32]>, 
        optional_point_2_displacement: Option<&[f32]>, 
        props: &Props
    ) 
        -> Self
    {
        let primitives_for_selection = Line::get_primitives_for_selection(transformed_uid, 
            point_1_coordinates, point_2_coordinates, optional_point_1_displacement, optional_point_2_displacement);

        let color_visible = match line_object_type
        {
            LineObjectType::LineDefault => props.line_default_color,
            LineObjectType::LineTruss => props.line_truss_props_color,
            LineObjectType::LineBeam => props.line_beam_props_color,
            LineObjectType::Element => props.element_color,
        };
        let mut primitives_visible = primitives_for_selection.clone();
        let mut lines_endpoints_colors = FloatNumbers::create();
        lines_endpoints_colors.extend(&color_visible);
        lines_endpoints_colors.extend(&color_visible);
        primitives_visible.set_lines_endpoints_colors(lines_endpoints_colors);
        let mut primitives_visible_selected = primitives_for_selection.clone();
        let mut selected_lines_endpoints_colors = FloatNumbers::create();
        selected_lines_endpoints_colors.extend(&props.drawn_object_selected_color);
        selected_lines_endpoints_colors.extend(&props.drawn_object_selected_color);
        primitives_visible_selected.set_lines_endpoints_colors(selected_lines_endpoints_colors);

        Line 
        { 
            number,
            point_1_number,
            point_2_number,
            primitives_for_selection,
            primitives_visible, 
            primitives_visible_selected,
            optional_primitives_local_axis_1_direction: None,
            optional_primitives_mesh_seed: None,
            optional_uniformly_distributed_line_load: None,
        }
    }


    pub fn get_primitives(&self, gl_mode: &GLMode, is_selected: bool) -> LinePrimitives
    {
        match gl_mode
        {
            GLMode::Selection => 
            {
                LinePrimitives::init(self.primitives_for_selection.clone())
            },
            GLMode::Visible => 
            {
                let mut line_primitives;
                if is_selected
                {
                    line_primitives = LinePrimitives::init(self.primitives_visible_selected.clone());
                }
                else
                {
                    line_primitives = LinePrimitives::init(self.primitives_visible.clone());
                }

                if let Some(primitives_local_axis_1_direction) = 
                    self.optional_primitives_local_axis_1_direction.as_ref()
                {
                    line_primitives.optional_primitives_local_axis_1_direction = 
                        Some(primitives_local_axis_1_direction.clone());
                }

                if let Some(primitives_mesh_seed) = self.optional_primitives_mesh_seed.as_ref()
                {
                    line_primitives.optional_primitives_mesh_seed = Some(primitives_mesh_seed.clone());
                }
                line_primitives
            }
        }
    }


    pub fn update_endpoint_coordinates(&mut self, point_number: u32, point_coordinates: [f32; 3], props: &Props)
    {
        let mut endpoints_coordinates = 
            self.primitives_for_selection.get_ref_lines_endpoints_coordinates().to_vec();
        let mut optional_start_index = None;
        if self.point_1_number == point_number
        {
            optional_start_index = Some(0);
        }
        if self.point_2_number == point_number
        {
            optional_start_index = Some(3);
        }
        if let Some(start_index) = optional_start_index
        {
            for i in 0..3
            {
                endpoints_coordinates[start_index + i] = point_coordinates[i];
            }
            let mut new_endpoint_coordinates = FloatNumbers::create();
            new_endpoint_coordinates.extend(&endpoints_coordinates);
            self.primitives_for_selection.set_lines_endpoints_coordinates(new_endpoint_coordinates.clone());
            self.primitives_visible.set_lines_endpoints_coordinates(new_endpoint_coordinates.clone());
            self.primitives_visible_selected.set_lines_endpoints_coordinates(new_endpoint_coordinates.clone());
            if let Some(uniformly_distributed_line_load) = 
                self.optional_uniformly_distributed_line_load.as_ref()
            {
                uniformly_distributed_line_load.borrow_mut().update_point_coordinates(
                    convert_vec_to_array(endpoints_coordinates), props);
            }
        }
    }


    pub fn update_endpoints(
        &mut self, 
        point_1_number: u32, 
        point_2_number: u32, 
        point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3],
    )
    {
        self.point_1_number = point_1_number;
        self.point_2_number = point_2_number;
        let mut lines_endpoints_coordinates = FloatNumbers::create();
        lines_endpoints_coordinates.extend(&point_1_coordinates);
        lines_endpoints_coordinates.extend(&point_2_coordinates);

        self.primitives_for_selection.set_lines_endpoints_coordinates(lines_endpoints_coordinates.clone());
        self.primitives_visible.set_lines_endpoints_coordinates(lines_endpoints_coordinates.clone());
        self.primitives_visible_selected.set_lines_endpoints_coordinates(lines_endpoints_coordinates);
    }


    pub fn update_line_object_type(&mut self, line_object_type: LineObjectType, props: &Props)
    {
        let color_visible = match line_object_type
        {
            LineObjectType::LineDefault => props.line_default_color,
            LineObjectType::LineTruss => props.line_truss_props_color,
            LineObjectType::LineBeam => props.line_beam_props_color,
            LineObjectType::Element => props.element_color,
        };
        let mut lines_endpoints_colors = FloatNumbers::create();
        lines_endpoints_colors.extend(&color_visible);
        lines_endpoints_colors.extend(&color_visible);
        self.primitives_visible.set_lines_endpoints_colors(lines_endpoints_colors);
    }


    pub fn update_line_local_axis_1_direction(&mut self, 
        optional_transformed_local_axis_1_direction: Option<Box<[f32]>>, props: &Props,
    ) 
        -> Result<(), JsValue>
    {
        match optional_transformed_local_axis_1_direction
        {
            Some(local_axis_1_direction) => 
            {
                let direction: [f32; 3] = convert_slice_to_array(&local_axis_1_direction);
                let norm = props.beam_section_orientation_line_length / 
                    direction.iter().fold(0.0, |acc, x| acc + x * x).sqrt();
                let center = self.get_center();
                let point_2_coordinates = [direction[0] * norm, direction[1] * norm, direction[2] * norm];

                let mut primitives_local_axis_1_direction = Primitives::create();

                Line::add_local_axis_1_direction_line(
                    props, &mut primitives_local_axis_1_direction, &center, &point_2_coordinates,
                );

                let norm_2 = (props.beam_section_orientation_line_length - 
                    props.beam_section_orientation_cap_height) / 
                    direction.iter().fold(0.0, |acc, x| acc + x * x).sqrt();
                let base_center_point_coordinates = [direction[0] * norm_2, direction[1] * norm_2, 
                    direction[2] * norm_2];

                Line::add_local_axis_1_direction_cap(
                    props, 
                    &mut primitives_local_axis_1_direction, 
                    &point_2_coordinates,
                    &base_center_point_coordinates, 
                    &center,
                )?;

                self.optional_primitives_local_axis_1_direction = Some(primitives_local_axis_1_direction);
            },
            None => self.optional_primitives_local_axis_1_direction = None,
        }

        Ok(())
    }


    pub fn get_endpoints_coordinates(&self) -> [[f32; 3]; 2]
    {
        let endpoints_coordinates: [f32; 6] = convert_vec_to_array(
            self.primitives_for_selection.get_ref_lines_endpoints_coordinates().to_vec());
        [
            [endpoints_coordinates[0], endpoints_coordinates[1], endpoints_coordinates[2]], 
            [endpoints_coordinates[3], endpoints_coordinates[4], endpoints_coordinates[5]],
        ]
    }


    pub fn update_line_mesh_seed(
        &mut self, line_mesh_seed_value: Option<u8>, mesh_seed_type: MeshSeedType, props: &Props,
    )
    {
        let optional_color_visible = match mesh_seed_type
        {
            MeshSeedType::Global => Some(props.global_mesh_seed_color),
            MeshSeedType::Local => Some(props.local_mesh_seed_color),
            MeshSeedType::None => None,
        };
        if let (Some(color_visible), Some(value)) = (optional_color_visible, line_mesh_seed_value)
        {
            let mut primitives_mesh_seed = Primitives::create();

            let mut mesh_seeds_coordinates = FloatNumbers::create();
            let mut mesh_seeds_colors = FloatNumbers::create();
            let mut mesh_seeds_is_to_scale = FloatNumbers::create();
            let mut mesh_seeds_displacements = FloatNumbers::create();
            let mut mesh_seeds_reference_points = FloatNumbers::create();

            let [point_1_coordinates, point_2_coordinates] = self.get_endpoints_coordinates();
            let step = [
                (point_2_coordinates[0] - point_1_coordinates[0]) / value as f32,
                (point_2_coordinates[1] - point_1_coordinates[1]) / value as f32,
                (point_2_coordinates[2] - point_1_coordinates[2]) / value as f32,
            ];
            for i in 1..value
            {
                let mesh_seed_coordinates = [
                    point_1_coordinates[0] + step[0] * i as f32,
                    point_1_coordinates[1] + step[1] * i as f32, 
                    point_1_coordinates[2] + step[2] * i as f32,
                ];
                mesh_seeds_coordinates.extend(&mesh_seed_coordinates);
                mesh_seeds_colors.extend(&color_visible);
                mesh_seeds_is_to_scale.extend(&[1.0]);
                mesh_seeds_displacements.extend(&[0.0, 0.0, 0.0]);
                mesh_seeds_reference_points.extend(&mesh_seed_coordinates);
            }
            primitives_mesh_seed.set_points_coordinates(mesh_seeds_coordinates);
            primitives_mesh_seed.set_points_colors(mesh_seeds_colors);
            primitives_mesh_seed.set_points_is_to_scale(mesh_seeds_is_to_scale);
            primitives_mesh_seed.set_points_displacements(mesh_seeds_displacements);
            primitives_mesh_seed.set_points_reference_points(mesh_seeds_reference_points);
            self.optional_primitives_mesh_seed = Some(primitives_mesh_seed);
        }
        else
        {
            self.optional_primitives_mesh_seed = None;
        }
    }


    pub fn update_uniformly_distributed_line_load(
        &mut self, 
        transformed_uid: [u8; 4], 
        point_1_coordinates: [f32; 3], 
        point_2_coordinates: [f32; 3], 
        qx: f32, 
        qy: f32, 
        qz: f32, 
        props: &Props,
    ) 
        -> Result<(), JsValue>
    {
        let uniformly_distributed_line_load = 
            Rc::new(RefCell::new(UniformlyDistributedLineLoad::create(transformed_uid, point_1_coordinates, 
                point_2_coordinates, qx, qy, qz, props)?));
        self.optional_uniformly_distributed_line_load = Some(uniformly_distributed_line_load);
        Ok(())
    }


    pub fn get_ref_uniformly_distributed_line_load(&self) -> Option<Rc<RefCell<UniformlyDistributedLineLoad>>>
    {
        if let Some(uniformly_distributed_line_load) = 
            self.optional_uniformly_distributed_line_load.as_ref()
        {
            return Some(Rc::clone(uniformly_distributed_line_load));
        }
        None
    }


    pub fn get_optional_uniformly_distributed_line_load_transformed_uid(&self) -> Option<[u8; 4]>
    {
        if let Some(uniformly_distributed_line_load) = 
            self.optional_uniformly_distributed_line_load.as_ref()
        {
            let transformed_uid = uniformly_distributed_line_load.borrow().get_transformed_uid();
            return Some(transformed_uid);
        }
        None
    }
}


impl DenotationTrait for Line
{
    fn get_notation(&self) -> String 
    {
        self.number.to_string()
    }


    fn get_center(&self) -> [f32; 3] 
    {
        let line_endpoints_coordinates: [f32; 6] = convert_vec_to_array(self.primitives_for_selection
            .get_ref_lines_endpoints_coordinates().to_vec());
        let center = [
            (line_endpoints_coordinates[0] + line_endpoints_coordinates[3]) / 2.0,
            (line_endpoints_coordinates[1] + line_endpoints_coordinates[4]) / 2.0,
            (line_endpoints_coordinates[2] + line_endpoints_coordinates[5]) / 2.0,
        ];
        center
    }


    fn get_color_str(&self, is_selected: bool) -> String 
    {
        let color_array_data = 
            if is_selected
            {
                self.primitives_visible_selected.get_ref_lines_endpoints_colors()
            }
            else
            {
                self.primitives_visible.get_ref_lines_endpoints_colors()
            };
        let color_array: [f32; 8] = convert_vec_to_array(color_array_data.to_vec());
        let color_str = 
            format!("rgb({}, {}, {})", color_array[0] * 255.0, color_array[1] * 255.0, color_array[2] * 255.0);
        color_str
    }
}


impl SelectedObjectTrait for Line {}
