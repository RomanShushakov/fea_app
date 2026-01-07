use crate::structs::{Primitives, Scene, DenotationsData, Manipulation};
use crate::enums::{GLMode, ResultPlot};


impl Scene
{
    pub fn get_postprocessor_primitives(
        &self, gl_mode: GLMode, manipulation: &Manipulation, optional_result_plot: Option<ResultPlot>,
    ) 
        -> Primitives
    {
        let mut primitives = Primitives::create();
        if manipulation.is_node_visible 
        {
            for node in self.nodes.values()
            {
                let node_primitives = node.get_primitives(&gl_mode, false);

                match optional_result_plot
                {
                    Some(ResultPlot::Displacements(_)) => 
                    {
                        match gl_mode
                        {
                            GLMode::Visible => 
                            {
                                if let Some(primitives_result_u_displacement) = 
                                    node_primitives.optional_primitives_result_u_displacement
                                {
                                    primitives += primitives_result_u_displacement;
                                }
                            },
                            GLMode::Selection => primitives += node_primitives.basic_primitives,
                        }
                    },
                    Some(ResultPlot::GlobalForces) => 
                    {
                        primitives += node_primitives.basic_primitives;
                        if let Some(primitives_global_forces) = 
                            node_primitives.optional_primitives_global_forces
                        {
                            primitives += primitives_global_forces;
                        }
                    },
                    Some(ResultPlot::GlobalMoments) => 
                    {
                        primitives += node_primitives.basic_primitives;
                        if let Some(primitives_global_moments) = 
                            node_primitives.optional_primitives_global_moments
                        {
                            primitives += primitives_global_moments;
                        }
                    },
                    _ => primitives += node_primitives.basic_primitives,
                }
            }
            for selected_node in self.selected_nodes.values()
            {
                let selected_node_primitives = selected_node.get_primitives(&gl_mode, true);
                primitives += selected_node_primitives.basic_primitives;

                match optional_result_plot
                {
                    Some(ResultPlot::GlobalForces) => 
                    {
                        if let Some(primitives_global_forces) = 
                            selected_node_primitives.optional_primitives_global_forces
                        {
                            primitives += primitives_global_forces;
                        }
                    },
                    Some(ResultPlot::GlobalMoments) => 
                    {
                        if let Some(primitives_global_moments) = 
                            selected_node_primitives.optional_primitives_global_moments
                        {
                            primitives += primitives_global_moments;
                        }
                    },
                    _ => (),
                }
            }
        }

        if manipulation.is_truss_element_visible
        {
            for truss_element in self.truss_elements.values()
            {
                let truss_element_primitives = truss_element.get_primitives(
                    &gl_mode, false,
                );

                match optional_result_plot
                {
                    Some(ResultPlot::Displacements(_)) => 
                    {
                        match gl_mode
                        {
                            GLMode::Visible => 
                            {
                                if let Some(primitives_result_u_displacement) = 
                                    truss_element_primitives.optional_primitives_result_u_displacement
                                {
                                    primitives += primitives_result_u_displacement;
                                }
                            },
                            GLMode::Selection => primitives += truss_element_primitives.basic_primitives,
                        }
                    },
                    Some(ResultPlot::ForceR) => 
                    {
                        primitives += truss_element_primitives.basic_primitives;

                        if let Some(primitives_local_axes) = 
                            truss_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_force_r) = 
                            truss_element_primitives.optional_primitives_force_r
                        {
                            primitives += primitives_force_r;
                        }
                    },
                    _ =>
                    {
                        primitives += truss_element_primitives.basic_primitives;
                        if let Some(primitives_local_axes) = 
                            truss_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }
                    },
                }
            }
            for selected_truss_element in self.selected_truss_elements.values()
            {
                let selected_truss_element_primitives = selected_truss_element.get_primitives(
                    &gl_mode, true,
                );
                primitives += selected_truss_element_primitives.basic_primitives;

                match optional_result_plot
                {
                    Some(ResultPlot::Displacements(_)) => (),
                    Some(ResultPlot::ForceR) => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_truss_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_force_r) = 
                            selected_truss_element_primitives.optional_primitives_force_r
                        {
                            primitives += primitives_force_r;
                        }
                    }
                    _ => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_truss_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }
                    }
                }
            }
        }

        if manipulation.is_beam_element_visible
        {
            for beam_element in self.beam_elements.values()
            {
                let beam_element_primitives = beam_element.get_primitives(
                    &gl_mode, false,
                );

                match optional_result_plot
                {
                    Some(ResultPlot::Displacements(_)) => 
                    {
                        match gl_mode
                        {
                            GLMode::Visible => 
                            {
                                if let Some(primitives_result_u_displacement) = 
                                    beam_element_primitives.optional_primitives_result_u_displacement
                                {
                                    primitives += primitives_result_u_displacement;
                                }
                            },
                            GLMode::Selection => primitives += beam_element_primitives.basic_primitives,
                        }
                    },
                    Some(ResultPlot::ForceR) => 
                    {
                        primitives += beam_element_primitives.basic_primitives;

                        if let Some(primitives_local_axes) = 
                            beam_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_force_r) = 
                            beam_element_primitives.optional_primitives_force_r
                        {
                            primitives += primitives_force_r;
                        }
                    },
                    Some(ResultPlot::ForceS) => 
                    {
                        primitives += beam_element_primitives.basic_primitives;

                        if let Some(primitives_local_axes) = 
                            beam_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_force_s) = 
                            beam_element_primitives.optional_primitives_force_s
                        {
                            primitives += primitives_force_s;
                        }
                    },
                    Some(ResultPlot::ForceT) => 
                    {
                        primitives += beam_element_primitives.basic_primitives;

                        if let Some(primitives_local_axes) = 
                            beam_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_force_t) = 
                            beam_element_primitives.optional_primitives_force_t
                        {
                            primitives += primitives_force_t;
                        }
                    },
                    Some(ResultPlot::MomentR) => 
                    {
                        primitives += beam_element_primitives.basic_primitives;

                        if let Some(primitives_local_axes) = 
                            beam_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_moment_r) = 
                            beam_element_primitives.optional_primitives_moment_r
                        {
                            primitives += primitives_moment_r;
                        }
                    },
                    Some(ResultPlot::MomentS) => 
                    {
                        primitives += beam_element_primitives.basic_primitives;

                        if let Some(primitives_local_axes) = 
                            beam_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_moment_s) = 
                            beam_element_primitives.optional_primitives_moment_s
                        {
                            primitives += primitives_moment_s;
                        }
                    },
                    Some(ResultPlot::MomentT) => 
                    {
                        primitives += beam_element_primitives.basic_primitives;

                        if let Some(primitives_local_axes) = 
                            beam_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_moment_t) = 
                            beam_element_primitives.optional_primitives_moment_t
                        {
                            primitives += primitives_moment_t;
                        }
                    },
                    _ =>
                    {
                        primitives += beam_element_primitives.basic_primitives;
                        if let Some(primitives_local_axes) = 
                            beam_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }
                    },
                }
            }
            for selected_beam_element in self.selected_beam_elements.values()
            {
                let selected_beam_element_primitives = selected_beam_element.get_primitives(
                    &gl_mode, true,
                );
                primitives += selected_beam_element_primitives.basic_primitives;

                match optional_result_plot
                {
                    Some(ResultPlot::Displacements(_)) => (),
                    Some(ResultPlot::ForceR) => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_beam_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_force_r) = 
                            selected_beam_element_primitives.optional_primitives_force_r
                        {
                            primitives += primitives_force_r;
                        }
                    },
                    Some(ResultPlot::ForceS) => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_beam_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_force_s) = 
                            selected_beam_element_primitives.optional_primitives_force_s
                        {
                            primitives += primitives_force_s;
                        }
                    }
                    Some(ResultPlot::ForceT) => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_beam_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_force_t) = 
                            selected_beam_element_primitives.optional_primitives_force_t
                        {
                            primitives += primitives_force_t;
                        }
                    },
                    Some(ResultPlot::MomentR) => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_beam_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_moment_r) = 
                            selected_beam_element_primitives.optional_primitives_moment_r
                        {
                            primitives += primitives_moment_r;
                        }
                    },
                    Some(ResultPlot::MomentS) => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_beam_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_moment_s) = 
                            selected_beam_element_primitives.optional_primitives_moment_s
                        {
                            primitives += primitives_moment_s;
                        }
                    }
                    Some(ResultPlot::MomentT) => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_beam_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_moment_t) = 
                            selected_beam_element_primitives.optional_primitives_moment_t
                        {
                            primitives += primitives_moment_t;
                        }
                    },
                    _ => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_beam_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }
                    },
                }
            }
        }

        if manipulation.is_plate_element_visible
        {
            for plate_element in self.plate_elements.values()
            {
                let plate_element_primitives = plate_element.get_primitives(
                    &gl_mode, false,
                );

                match optional_result_plot
                {
                    Some(ResultPlot::Displacements(_)) => 
                    {
                        match gl_mode
                        {
                            GLMode::Visible => 
                            {
                                if let Some(primitives_result_u_displacement) = 
                                    plate_element_primitives.optional_primitives_result_u_displacement
                                {
                                    primitives += primitives_result_u_displacement;
                                }
                            },
                            GLMode::Selection => primitives += plate_element_primitives.basic_primitives,
                        }
                    },
                    Some(ResultPlot::MemForceR) => 
                    {
                        primitives += plate_element_primitives.basic_primitives;

                        if let Some(primitives_local_axes) = 
                            plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_mem_force_r) = 
                            plate_element_primitives.optional_primitives_mem_force_r
                        {
                            primitives += primitives_mem_force_r;
                        }
                    },
                    Some(ResultPlot::MemForceS) => 
                    {
                        primitives += plate_element_primitives.basic_primitives;

                        if let Some(primitives_local_axes) = 
                            plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_mem_force_s) = 
                            plate_element_primitives.optional_primitives_mem_force_s
                        {
                            primitives += primitives_mem_force_s;
                        }
                    },
                    Some(ResultPlot::MemForceRS) => 
                    {
                        primitives += plate_element_primitives.basic_primitives;

                        if let Some(primitives_local_axes) = 
                            plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_mem_force_r_s) = 
                            plate_element_primitives.optional_primitives_mem_force_r_s
                        {
                            primitives += primitives_mem_force_r_s;
                        }
                    },
                    Some(ResultPlot::BendMomentR) => 
                    {
                        primitives += plate_element_primitives.basic_primitives;

                        if let Some(primitives_local_axes) = 
                            plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_bend_moment_r) = 
                            plate_element_primitives.optional_primitives_bend_moment_r
                        {
                            primitives += primitives_bend_moment_r;
                        }
                    },
                    Some(ResultPlot::BendMomentS) => 
                    {
                        primitives += plate_element_primitives.basic_primitives;

                        if let Some(primitives_local_axes) = 
                            plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_bend_moment_s) = 
                            plate_element_primitives.optional_primitives_bend_moment_s
                        {
                            primitives += primitives_bend_moment_s;
                        }
                    },
                    Some(ResultPlot::BendMomentRS) => 
                    {
                        primitives += plate_element_primitives.basic_primitives;

                        if let Some(primitives_local_axes) = 
                            plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_bend_moment_r_s) = 
                            plate_element_primitives.optional_primitives_bend_moment_r_s
                        {
                            primitives += primitives_bend_moment_r_s;
                        }
                    },
                    Some(ResultPlot::ShearForceRT) => 
                    {
                        primitives += plate_element_primitives.basic_primitives;

                        if let Some(primitives_local_axes) = 
                            plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_shear_force_r_t) = 
                            plate_element_primitives.optional_primitives_shear_force_r_t
                        {
                            primitives += primitives_shear_force_r_t;
                        }
                    },
                    Some(ResultPlot::ShearForceST) => 
                    {
                        primitives += plate_element_primitives.basic_primitives;

                        if let Some(primitives_local_axes) = 
                            plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_shear_force_s_t) = 
                            plate_element_primitives.optional_primitives_shear_force_s_t
                        {
                            primitives += primitives_shear_force_s_t;
                        }
                    },
                    _ =>
                    {
                        primitives += plate_element_primitives.basic_primitives;
                        if let Some(primitives_local_axes) = 
                            plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }
                    },
                }
            }
            for selected_plate_element in self.selected_plate_elements.values()
            {
                let selected_plate_element_primitives = selected_plate_element.get_primitives(
                    &gl_mode, true,
                );
                primitives += selected_plate_element_primitives.basic_primitives;

                match optional_result_plot
                {
                    Some(ResultPlot::Displacements(_)) => (),
                    Some(ResultPlot::MemForceR) => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_mem_force_r) = 
                            selected_plate_element_primitives.optional_primitives_mem_force_r
                        {
                            primitives += primitives_mem_force_r;
                        }
                    },
                    Some(ResultPlot::MemForceS) => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_mem_force_s) = 
                            selected_plate_element_primitives.optional_primitives_mem_force_s
                        {
                            primitives += primitives_mem_force_s;
                        }
                    },
                    Some(ResultPlot::MemForceRS) => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_mem_force_r_s) = 
                            selected_plate_element_primitives.optional_primitives_mem_force_r_s
                        {
                            primitives += primitives_mem_force_r_s;
                        }
                    },
                    Some(ResultPlot::BendMomentR) => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_bend_moment_r) = 
                            selected_plate_element_primitives.optional_primitives_bend_moment_r
                        {
                            primitives += primitives_bend_moment_r;
                        }
                    },
                    Some(ResultPlot::BendMomentS) => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_bend_moment_s) = 
                            selected_plate_element_primitives.optional_primitives_bend_moment_s
                        {
                            primitives += primitives_bend_moment_s;
                        }
                    },
                    Some(ResultPlot::BendMomentRS) => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_bend_moment_r_s) = 
                            selected_plate_element_primitives.optional_primitives_bend_moment_r_s
                        {
                            primitives += primitives_bend_moment_r_s;
                        }
                    },
                    Some(ResultPlot::ShearForceRT) => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_shear_force_r_t) = 
                            selected_plate_element_primitives.optional_primitives_shear_force_r_t
                        {
                            primitives += primitives_shear_force_r_t;
                        }
                    },
                    Some(ResultPlot::ShearForceST) => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }

                        if let Some(primitives_shear_force_s_t) = 
                            selected_plate_element_primitives.optional_primitives_shear_force_s_t
                        {
                            primitives += primitives_shear_force_s_t;
                        }
                    },
                    _ => 
                    {
                        if let Some(primitives_local_axes) = 
                            selected_plate_element_primitives.optional_primitives_local_axes
                            && manipulation.is_local_axes_visible
                            {
                                primitives += primitives_local_axes;
                            }
                    },
                }
            }
        }

        primitives
    }


    pub fn get_postprocessor_objects_data_for_denotation(&self, _manipulation: &Manipulation) -> DenotationsData
    {
        DenotationsData::create()
    }
}
