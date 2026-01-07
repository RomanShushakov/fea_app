use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext as GL, CanvasRenderingContext2d as CTX};

use crate::Renderer;
use crate::traits::BufferDataTrait;
use crate::enums::{PrimitiveType, GLMode, SceneState, ResultPlot};
use crate::structs::Denotation;
use crate::functions::{add_denotation, add_color_bar, add_color_bar_caption};


#[wasm_bindgen]
impl Renderer
{
    fn compose_projection_matrix(&self, aspect: f32, z_far: f32, z_near: f32) -> [f32; 16]
    {
        let mut projection_matrix = mat4::new_zero();
        mat4::orthographic(&mut projection_matrix,
            &(1.0 / aspect), &1.0, &(-1.0 / aspect), &-1.0,
            &z_near, &z_far);
        projection_matrix
    }


    fn compose_model_view_matrix(&self, z_far: f32, z_near: f32) -> [f32; 16]
    {
        let mut model_view_matrix = mat4::new_identity();
        mat4::translate(&mut model_view_matrix, &mat4::new_identity(),
            &[self.manipulation.dx, self.manipulation.dy, - (z_far + z_near) / 2.0]);
        let mat_to_scale = model_view_matrix;
        mat4::scale(&mut model_view_matrix, &mat_to_scale,
            &[1.0 + self.manipulation.d_scale, 1.0 + self.manipulation.d_scale, 1.0 + self.manipulation.d_scale]);
        let mat_to_rotate = model_view_matrix;
        mat4::rotate_x(&mut model_view_matrix, &mat_to_rotate, &self.manipulation.phi);
        let mat_to_rotate = model_view_matrix;
        mat4::rotate_y(&mut model_view_matrix, &mat_to_rotate, &self.manipulation.theta);
        model_view_matrix
    }


    fn draw_scene_notations(&self, projection_matrix: &[f32; 16], model_view_matrix: &[f32; 16], ctx: &CTX)
    {
        let width = self.canvas_gl.width();
        let height = self.canvas_gl.height();
        let aspect: f32 = width as f32 / height as f32;

        let scale = if aspect > 1.0 { self.scene.get_scale() / aspect } else { self.scene.get_scale() * aspect };
        let center = self.scene.get_center();
        let shift = center.into_iter().map(|coord| coord * scale).collect::<Vec<f32>>();

        let mut matrix = mat4::new_identity();
        mat4::mul(&mut matrix, projection_matrix, model_view_matrix);

        let denotations_data = self.scene.get_denotations_data(&self.manipulation);
        for point_object_denotation in denotations_data.get_ref_point_objects_denotations()
        {
            let Denotation { notation, center, color } = point_object_denotation;
            ctx.set_fill_style(&color.into());
            add_denotation(ctx,
                &[
                    center[0] * scale - shift[0] - 
                    self.props.drawn_point_object_denotation_shift / (1.0 + self.manipulation.d_scale),
                    center[1] * scale - shift[1] -
                    self.props.drawn_point_object_denotation_shift / (1.0 + self.manipulation.d_scale),
                    center[2] * scale - shift[2],
                    1.0
                ],
                &matrix,
                width as f32, height as f32,
                notation);
            ctx.stroke();
        }
        for line_object_denotation in denotations_data.get_ref_line_objects_denotations()
        {
            let Denotation { notation, center, color } = line_object_denotation;
            ctx.set_fill_style(&color.into());
            add_denotation(ctx,
                &[
                    center[0] * scale - shift[0],
                    center[1] * scale - shift[1] +
                    self.props.drawn_line_object_denotation_shift / (1.0 + self.manipulation.d_scale),
                    center[2] * scale - shift[2],
                    1.0
                ],
                &matrix,
                width as f32, height as f32,
                notation);
            ctx.stroke();

        }
        for surface_object_denotation in denotations_data.get_ref_surface_objects_denotations()
        {
            let Denotation { notation, center, color } = surface_object_denotation;
            ctx.set_fill_style(&color.into());
            add_denotation(ctx,
                &[
                    center[0] * scale - shift[0],
                    center[1] * scale - shift[1],
                    center[2] * scale - shift[2],
                    1.0
                ],
                &matrix,
                width as f32, height as f32,
                notation);
            ctx.stroke();
        }
    }


    pub(super) fn draw_scene(&self, gl: &GL, gl_mode: GLMode, z_near: f32, z_far: f32, ctx: &CTX) -> Result<(), JsValue>
    {
        let width = self.canvas_gl.width();
        let height = self.canvas_gl.height();
        let aspect: f32 = width as f32 / height as f32;

        let scale = if aspect > 1.0 { self.scene.get_scale() / aspect } else { self.scene.get_scale() * aspect };
        let center = self.scene.get_center();
        let shift = center.into_iter().map(|coord| coord * scale).collect::<Vec<f32>>();

        let projection_matrix = self.compose_projection_matrix(aspect, z_far, z_near);
        let model_view_matrix = self.compose_model_view_matrix(z_far, z_near);

        let point_size = match gl_mode { GLMode::Selection => 12.0, GLMode::Visible => 5.0 };

        let displacement_magnitude = match self.scene.get_scene_state()
        {
            SceneState::Postprocessor((optional_result_plot, _, _, _)) => 
            {
                match optional_result_plot
                {
                    Some(ResultPlot::Displacements(magnitude)) => magnitude,
                    Some(_) | None => 0.0,
                }
            },
            SceneState::Preprocessor => 0.0,
        };

        self.associate_vertex_buffer_with_shader_programs()?;
        self.associate_is_to_scale_buffer_with_shader_programs()?;
        self.associate_reference_point_buffer_with_shader_programs()?;
        self.associate_vertex_displacement_buffer_with_shader_programs()?;
        self.associate_color_buffer_with_shader_programs()?;
        gl.uniform1f(Some(self.shader_programs.get_ref_displacement_magnitude()), displacement_magnitude);
        gl.uniform1f(Some(self.shader_programs.get_ref_point_size()), point_size);
        gl.uniform3f(Some(self.shader_programs.get_ref_shift()), shift[0], shift[1], shift[2]);
        gl.uniform1f(Some(self.shader_programs.get_ref_scale()), scale);
        gl.uniform1f(Some(self.shader_programs.get_ref_d_scale()), 1.0 + self.manipulation.d_scale);
        gl.uniform2f(Some(self.shader_programs.get_ref_screen_size()), width as f32, height as f32);
        gl.uniform_matrix4fv_with_f32_array(
            Some(self.shader_programs.get_ref_projection_matrix()), false, &projection_matrix,
        );
        gl.uniform_matrix4fv_with_f32_array(
            Some(self.shader_programs.get_ref_model_view_matrix()), false, &model_view_matrix,
        );

        let primitives = self.scene.get_primitives(gl_mode, &self.manipulation);
        for primitive_type in [PrimitiveType::Point, PrimitiveType::Line, PrimitiveType::Triangle]
        {
            let (
                vertices_coordinates, 
                vertices_is_to_scale,
                vertices_reference_points,
                vertices_displacements, 
                vertices_colors, 
                count, 
                mode,
            ) = match primitive_type
            {
                PrimitiveType::Point => 
                {
                    (
                        primitives.get_ref_points_coordinates(),
                        primitives.get_ref_points_is_to_scale(),
                        primitives.get_ref_points_reference_points(),
                        primitives.get_ref_points_displacements(),
                        primitives.get_ref_points_colors(),
                        primitives.get_points_count(),
                        GL::POINTS,
                    )
                },
                PrimitiveType::Line => 
                {
                    (
                        primitives.get_ref_lines_endpoints_coordinates(),
                        primitives.get_ref_lines_endpoints_is_to_scale(),
                        primitives.get_ref_lines_endpoints_reference_points(),
                        primitives.get_ref_lines_endpoints_displacements(),
                        primitives.get_ref_lines_endpoints_colors(),
                        primitives.get_lines_count(),
                        GL::LINES,
                    )
                },
                PrimitiveType::Triangle => 
                {
                    (
                        primitives.get_ref_triangles_vertices_coordinates(),
                        primitives.get_ref_triangles_vertices_is_to_scale(),
                        primitives.get_ref_triangles_vertices_reference_points(),
                        primitives.get_ref_triangles_vertices_displacements(),
                        primitives.get_ref_triangles_vertices_colors(),
                        primitives.get_triangles_count(),
                        GL::TRIANGLES,
                    )
                },
            };
            vertices_coordinates.store(gl, self.optional_vertex_buffer.as_ref()); 
            vertices_is_to_scale.store(gl, self.optional_is_to_scale_buffer.as_ref()); 
            vertices_reference_points.store(gl, self.optional_reference_point_buffer.as_ref()); 
            vertices_displacements.store(gl, self.optional_vertex_displacement_buffer.as_ref()); 
            vertices_colors.store(gl, self.optional_color_buffer.as_ref()); 
            gl.draw_arrays(mode, 0, count);  
        }

        if let GLMode::Visible = gl_mode {
            self.draw_scene_notations(&projection_matrix, &model_view_matrix, ctx);

            if let SceneState::Postprocessor((
                    optional_result_plot,
                    extreme_global_displacements_data,
                    extreme_global_loads_data,
                    extreme_elements_loads_data,
                    
                )) = self.scene.get_scene_state() {
                match optional_result_plot
                {
                    Some(ResultPlot::Displacements(_)) =>
                    {
                        if let Some([min_u_result, max_u_result]) = 
                            extreme_global_displacements_data.get_optional_extreme_u_f_result()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "DISPLACEMENT",
                                &["U_RESULT"],
                                min_u_result, 
                                max_u_result, 
                                &self.props,
                            );
                        }
                        else
                        {
                            return Err(JsValue::from("Renderer: There are no extreme global U displacement!"));
                        }
                    },
                    Some(ResultPlot::GlobalForces) => 
                    {
                        if let Some([min_f, max_f]) = 
                            extreme_global_loads_data.get_optional_extreme_u_f()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "GLOBAL FORCE",
                                &["F_X, F_Y, F_Z"],
                                min_f, 
                                max_f, 
                                &self.props,
                            );
                        }
                    },
                    Some(ResultPlot::GlobalMoments) => 
                    {
                        if let Some([min_m, max_m]) = 
                            extreme_global_loads_data.get_optional_extreme_r_m()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "GLOBAL MOMENT",
                                &["M_X, M_Y, M_Z"],
                                min_m, 
                                max_m, 
                                &self.props,
                            );
                        }
                    },
                    Some(ResultPlot::ForceR) => 
                    {
                        if let Some([min_f, max_f]) = 
                            extreme_elements_loads_data.get_optional_extreme_f_r()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "ELEMENT FORCE",
                                &["FORCE_R"],
                                min_f, 
                                max_f, 
                                &self.props,
                            );
                        }
                    },
                    Some(ResultPlot::ForceS) => 
                    {
                        if let Some([min_f, max_f]) = 
                            extreme_elements_loads_data.get_optional_extreme_f_s()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "ELEMENT FORCE",
                                &["FORCE_S"],
                                min_f, 
                                max_f, 
                                &self.props,
                            );
                        }
                    },
                    Some(ResultPlot::ForceT) => 
                    {
                        if let Some([min_f, max_f]) = 
                            extreme_elements_loads_data.get_optional_extreme_f_t()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "ELEMENT FORCE",
                                &["FORCE_T"],
                                min_f, 
                                max_f, 
                                &self.props,
                            );
                        }
                    },
                    Some(ResultPlot::MomentR) => 
                    {
                        if let Some([min_m, max_m]) = 
                            extreme_elements_loads_data.get_optional_extreme_m_r()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "ELEMENT FORCE",
                                &["MOMENT_R"],
                                min_m, 
                                max_m, 
                                &self.props,
                            );
                        }
                    },
                    Some(ResultPlot::MomentS) => 
                    {
                        if let Some([min_m, max_m]) = 
                            extreme_elements_loads_data.get_optional_extreme_m_s()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "ELEMENT FORCE",
                                &["MOMENT_S"],
                                min_m, 
                                max_m, 
                                &self.props,
                            );
                        }
                    },
                    Some(ResultPlot::MomentT) => 
                    {
                        if let Some([min_m, max_m]) = 
                            extreme_elements_loads_data.get_optional_extreme_m_t()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "ELEMENT FORCE",
                                &["MOMENT_T"],
                                min_m, 
                                max_m, 
                                &self.props,
                            );
                        }
                    },
                    Some(ResultPlot::MemForceR) => 
                    {
                        if let Some([min_f, max_f]) = 
                            extreme_elements_loads_data.get_optional_extreme_mem_f_r()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "ELEMENT FORCE",
                                &["MEMBRANE_FORCE_R"],
                                min_f, 
                                max_f, 
                                &self.props,
                            );
                        }
                    },
                    Some(ResultPlot::MemForceS) => 
                    {
                        if let Some([min_f, max_f]) = 
                            extreme_elements_loads_data.get_optional_extreme_mem_f_s()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "ELEMENT FORCE",
                                &["MEMBRANE_FORCE_S"],
                                min_f, 
                                max_f, 
                                &self.props,
                            );
                        }
                    },
                    Some(ResultPlot::MemForceRS) => 
                    {
                        if let Some([min_f, max_f]) = 
                            extreme_elements_loads_data.get_optional_extreme_mem_f_r_s()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "ELEMENT FORCE",
                                &["MEMBRANE_FORCE_R_S"],
                                min_f, 
                                max_f, 
                                &self.props,
                            );
                        }
                    },
                    Some(ResultPlot::BendMomentR) => 
                    {
                        if let Some([min_m, max_m]) = 
                            extreme_elements_loads_data.get_optional_extreme_bend_m_r()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "ELEMENT FORCE",
                                &["BENDING_MOMENT_R"],
                                min_m, 
                                max_m, 
                                &self.props,
                            );
                        }
                    },
                    Some(ResultPlot::BendMomentS) => 
                    {
                        if let Some([min_m, max_m]) = 
                            extreme_elements_loads_data.get_optional_extreme_bend_m_s()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "ELEMENT FORCE",
                                &["BENDING_MOMENT_S"],
                                min_m, 
                                max_m, 
                                &self.props,
                            );
                        }
                    },
                    Some(ResultPlot::BendMomentRS) => 
                    {
                        if let Some([min_m, max_m]) = 
                            extreme_elements_loads_data.get_optional_extreme_bend_m_r_s()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "ELEMENT FORCE",
                                &["BENDING_MOMENT_R_S"],
                                min_m, 
                                max_m, 
                                &self.props,
                            );
                        }
                    },
                    Some(ResultPlot::ShearForceRT) => 
                    {
                        if let Some([min_f, max_f]) = 
                            extreme_elements_loads_data.get_optional_extreme_shear_f_r_t()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "ELEMENT FORCE",
                                &["SHEAR_FORCE_R_T"],
                                min_f, 
                                max_f, 
                                &self.props,
                            );
                        }
                    },
                    Some(ResultPlot::ShearForceST) => 
                    {
                        if let Some([min_f, max_f]) = 
                            extreme_elements_loads_data.get_optional_extreme_shear_f_s_t()
                        {
                            add_color_bar(ctx, width as f32, height as f32, &self.props);
                            add_color_bar_caption(
                                ctx, 
                                width as f32, 
                                height as f32,
                                "ELEMENT FORCE",
                                &["SHEAR_FORCE_S_T"],
                                min_f, 
                                max_f, 
                                &self.props,
                            );
                        }
                    },
                    _ => ()
                }
            }
        }

        Ok(())
    }
}
