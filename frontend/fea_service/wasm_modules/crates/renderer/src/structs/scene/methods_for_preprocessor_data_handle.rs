use crate::structs::{Primitives, Scene, DenotationsData, Manipulation};
use crate::enums::GLMode;
use crate::traits::DenotationTrait;


impl Scene
{
    pub fn get_preprocessor_primitives(&self, gl_mode: GLMode, manipulation: &Manipulation) -> Primitives
    {
        let mut primitives = Primitives::create();
        if manipulation.is_point_visible 
        {
            for point in self.points.values()
            {
                let point_primitives = point.get_primitives(&gl_mode, false);
                primitives += point_primitives;
            }
            for selected_point in self.selected_points.values()
            {
                let selected_point_primitives = selected_point.get_primitives(&gl_mode, true);
                primitives += selected_point_primitives;
            }
        }

        if manipulation.is_line_visible
        {
            for line in self.lines.values()
            {
                let line_primitives = line.get_primitives(&gl_mode, false);
                primitives += line_primitives.basic_primitives;
                if let Some(primitives_local_axis_1_direction) = 
                    line_primitives.optional_primitives_local_axis_1_direction
                {
                    if manipulation.is_beam_section_orientation_visible
                    {
                        primitives += primitives_local_axis_1_direction;
                    }
                }

                if let Some(primitives_mesh_seed) = 
                    line_primitives.optional_primitives_mesh_seed
                {
                    if manipulation.is_mesh_seed_visible
                    {
                        primitives += primitives_mesh_seed;
                    }
                }
            }
            for selected_line in self.selected_lines.values()
            {
                let selected_line_primitives = selected_line.get_primitives(&gl_mode, true);
                primitives += selected_line_primitives.basic_primitives;
                if let Some(primitives_local_axis_1_direction) = 
                    selected_line_primitives.optional_primitives_local_axis_1_direction
                {
                    if manipulation.is_beam_section_orientation_visible
                    {
                        primitives += primitives_local_axis_1_direction;
                    }
                }

                if let Some(primitives_mesh_seed) = 
                    selected_line_primitives.optional_primitives_mesh_seed
                {
                    if manipulation.is_mesh_seed_visible
                    {
                        primitives += primitives_mesh_seed;
                    }
                }
            }
        }

        if manipulation.is_surface_visible
        {
            for surface in self.surfaces.values()
            {
                let surface_primitives = surface.get_primitives(&gl_mode, false);
                if let Some(primitives_for_selection) = surface_primitives.optional_primitives_for_selection
                {
                    primitives += primitives_for_selection.clone();
                }
                else
                {
                    if manipulation.is_surface_edges_1_3_visible
                    {
                        if let Some(primitives_edges_1_3) = surface_primitives.optional_primitives_edges_1_3
                        {
                            primitives += primitives_edges_1_3.clone();
                        }
                        if manipulation.is_mesh_seed_visible
                        {
                            if let Some(primitives_mesh_seed_edges_1_3) = 
                                surface_primitives.optional_primitives_mesh_seed_edges_1_3
                            {
                                primitives += primitives_mesh_seed_edges_1_3.clone();
                            }
                        }    
                    }
                    if manipulation.is_surface_edges_2_4_visible
                    {
                        if let Some(primitives_edges_2_4) = surface_primitives.optional_primitives_edges_2_4
                        {
                            primitives += primitives_edges_2_4.clone();
                        }
                        if manipulation.is_mesh_seed_visible
                        {
                            if let Some(primitives_mesh_seed_edges_2_4) = 
                                surface_primitives.optional_primitives_mesh_seed_edges_2_4
                            {
                                primitives += primitives_mesh_seed_edges_2_4.clone();
                            }
                        }   
                    }
                    if manipulation.is_surface_normal_visible
                    {
                        if let Some(primitives_normal) = surface_primitives.optional_primitives_normal
                        {
                            primitives += primitives_normal.clone();
                        }
                    }
                }
            }
            for selected_surface in self.selected_surfaces.values()
            {
                let selected_surface_primitives = 
                    selected_surface.get_primitives(&gl_mode, true);
                if let Some(primitives_for_selection) = 
                    selected_surface_primitives.optional_primitives_for_selection
                {
                    primitives += primitives_for_selection.clone();
                }
                else
                {
                    if manipulation.is_surface_edges_1_3_visible
                    {
                        if let Some(primitives_edges_1_3) = 
                            selected_surface_primitives.optional_primitives_edges_1_3
                        {
                            primitives += primitives_edges_1_3.clone();
                        }
                        if manipulation.is_mesh_seed_visible
                        {
                            if let Some(primitives_mesh_seed_edges_1_3) = 
                                selected_surface_primitives.optional_primitives_mesh_seed_edges_1_3
                            {
                                primitives += primitives_mesh_seed_edges_1_3.clone();
                            }
                        }    
                    }
                    if manipulation.is_surface_edges_2_4_visible
                    {
                        if let Some(primitives_edges_2_4) = 
                            selected_surface_primitives.optional_primitives_edges_2_4
                        {
                            primitives += primitives_edges_2_4.clone();
                        }
                        if manipulation.is_mesh_seed_visible
                        {
                            if let Some(primitives_mesh_seed_edges_2_4) = 
                                selected_surface_primitives.optional_primitives_mesh_seed_edges_2_4
                            {
                                primitives += primitives_mesh_seed_edges_2_4.clone();
                            }
                        }   
                    }
                    if manipulation.is_surface_normal_visible
                    {
                        if let Some(primitives_normal) = selected_surface_primitives.optional_primitives_normal
                        {
                            primitives += primitives_normal.clone();
                        }
                    }
                }
            }
        }

        if manipulation.is_load_visible
        {
            for concentrated_load in self.concentrated_loads.values()
            {
                let concentrated_load_primitives = concentrated_load.get_primitives(&gl_mode, 
                    false);
                primitives += concentrated_load_primitives;
            }
            for selected_concentrated_load in self.selected_concentrated_loads.values()
            {
                let selected_concentrated_load_primitives = selected_concentrated_load.get_primitives(
                    &gl_mode, true);
                primitives += selected_concentrated_load_primitives;
            }

            for uniformly_distributed_line_load in 
                self.uniformly_distributed_line_loads.values()
            {
                let uniformly_distributed_line_load_primitives = 
                    uniformly_distributed_line_load.borrow().get_primitives(&gl_mode, false);
                primitives += uniformly_distributed_line_load_primitives;
            }
            for selected_uniformly_distributed_line_load in 
                self.selected_uniformly_distributed_line_loads.values()
            {
                let selected_uniformly_distributed_line_load_primitives = 
                    selected_uniformly_distributed_line_load.borrow().get_primitives(&gl_mode, true);
                primitives += selected_uniformly_distributed_line_load_primitives;
            }

            for uniformly_distributed_surface_load in 
                self.uniformly_distributed_surface_loads.values()
            {
                let uniformly_distributed_surface_load_primitives = 
                    uniformly_distributed_surface_load.borrow().get_primitives(&gl_mode, false);
                primitives += uniformly_distributed_surface_load_primitives;
            }
            for selected_uniformly_distributed_surface_load in 
                self.selected_uniformly_distributed_surface_loads.values()
            {
                let selected_uniformly_distributed_surface_load_primitives = 
                    selected_uniformly_distributed_surface_load.borrow().get_primitives(&gl_mode, true);
                primitives += selected_uniformly_distributed_surface_load_primitives;
            }
        }

        if manipulation.is_boundary_condition_visible
        {
            for point_boundary_condition in self.point_boundary_conditions.values()
            {
                let point_boundary_condition_primitives = point_boundary_condition.get_primitives(&gl_mode, 
                    false);
                primitives += point_boundary_condition_primitives;
            }
            for selected_point_boundary_condition in 
                self.selected_point_boundary_conditions.values()
            {
                let selected_point_boundary_condition_primitives = 
                    selected_point_boundary_condition.get_primitives(&gl_mode, true);
                primitives += selected_point_boundary_condition_primitives;
            }
        }

        primitives
    }


    pub fn get_preprocessor_objects_data_for_denotation(&self, manipulation: &Manipulation) -> DenotationsData
    {
        let mut denotations_data = DenotationsData::create();
        if manipulation.is_point_visible
        {
            for point in self.points.values()
            {
                let denotation = point.get_denotation(false);
                denotations_data.add_point_object_denotation(denotation);
            }
            for selected_point in self.selected_points.values()
            {
                let denotation = selected_point.get_denotation(true);
                denotations_data.add_point_object_denotation(denotation);
            }
        }

        if manipulation.is_line_visible
        {
            for line in self.lines.values()
            {
                let denotation = line.get_denotation(false);
                denotations_data.add_line_object_denotation(denotation);
            }
            for selected_line in self.selected_lines.values()
            {
                let denotation = selected_line.get_denotation(true);
                denotations_data.add_line_object_denotation(denotation);
            }
        }

        if manipulation.is_surface_visible
        {
            for surface in self.surfaces.values()
            {
                let denotation = surface.get_denotation(false);
                denotations_data.add_surface_object_denotation(denotation);
            }
            for selected_surface in self.selected_surfaces.values()
            {
                let denotation = selected_surface.get_denotation(true);
                denotations_data.add_surface_object_denotation(denotation);
            }
        }
        denotations_data
    }
}
