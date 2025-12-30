use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext as GL, CanvasRenderingContext2d as CTX};

use crate::Renderer;
use crate::traits::BufferDataTrait;
use crate::enums::PrimitiveType;
use crate::functions::{add_hints, add_denotation};


#[wasm_bindgen]
impl Renderer
{
    fn compose_cs_axes_projection_matrix(&self, z_far: f32, z_near: f32) -> [f32; 16]
    {
        let mut projection_matrix = mat4::new_zero();
        mat4::orthographic(&mut projection_matrix, &1.0, &1.0, &-1.0, &-1.0, &z_near, &z_far);
        projection_matrix
    }


    fn compose_cs_axes_model_view_matrix(&self, aspect: f32) -> [f32; 16]
    {
        let mut model_view_matrix = mat4::new_identity();
        let y_shift =
            if self.props.cs_axes_y_shift > 0.0 
            { 
                1.0 - (1.0 - self.props.cs_axes_y_shift) * aspect 
            }
            else 
            { 
                - 1.0 + (1.0 + self.props.cs_axes_y_shift) * aspect 
            };
        mat4::translate(
            &mut model_view_matrix, &mat4::new_identity(),
            &[self.props.cs_axes_x_shift, y_shift, self.props.cs_axes_z_shift],
        );
        let mat_to_scale = model_view_matrix;
        mat4::scale(
            &mut model_view_matrix, &mat_to_scale,
            &[self.props.cs_axes_scale, 
            self.props.cs_axes_scale * aspect, self.props.cs_axes_scale * aspect],
        );
        let mat_to_rotate = model_view_matrix;
        mat4::rotate_x(&mut model_view_matrix,&mat_to_rotate, &self.manipulation.phi);
        let mat_to_rotate = model_view_matrix;
        mat4::rotate_y(&mut model_view_matrix, &mat_to_rotate, &self.manipulation.theta);
        model_view_matrix
    }


    fn draw_cs_axes_notations(&self, model_view_matrix: &[f32; 16], ctx: &CTX) -> Result<(), JsValue>
    {
        let width = self.canvas_gl.width();
        let height = self.canvas_gl.height();

        ctx.set_fill_style(&self.props.canvas_axes_denotation_color.clone().into());

        add_denotation(&ctx,
            &[
                1.0 + self.props.axis_x_denotation_shift_x, 
                0.0 + self.props.axis_x_denotation_shift_y,
                0.0, 
                1.0,
            ],
            model_view_matrix, 
            width as f32,
            height as f32,
            "X",
        );
        add_denotation(&ctx,
            &[
                0.0 + self.props.axis_y_denotation_shift_x, 
                1.0 + self.props.axis_y_denotation_shift_y, 
                0.0, 
                1.0,
            ],
            model_view_matrix, 
            width as f32, 
            height as f32, 
            "Y",
        );
        add_denotation(&ctx,
            &[
                0.0 + self.props.axis_z_denotation_shift_x, 
                0.0 + self.props.axis_z_denotation_shift_y,
                1.0 + self.props.axis_z_denotation_shift_z, 
                1.0
            ],
            model_view_matrix,
            width as f32,
            height as f32,
            "Z",
        );
        ctx.stroke();

        ctx.set_fill_style(&self.props.hints_color.clone().into());
        add_hints(&ctx, width as f32, height as f32, &self.props);
        ctx.stroke();

        Ok(())
    }


    pub(super) fn draw_cs_axes(&self, gl: &GL, z_near: f32, z_far: f32, ctx: &CTX) -> Result<(), JsValue>
    {
        let width = self.canvas_gl.width();
        let height = self.canvas_gl.height();
        let aspect: f32 = width as f32 / height as f32;

        let projection_matrix = self.compose_cs_axes_projection_matrix(z_far, z_near);
        let model_view_matrix = self.compose_cs_axes_model_view_matrix(aspect);
        
        self.associate_vertex_buffer_with_shader_programs()?;
        self.associate_is_to_scale_buffer_with_shader_programs()?;
        self.associate_reference_point_buffer_with_shader_programs()?;
        self.associate_vertex_displacement_buffer_with_shader_programs()?;
        self.associate_color_buffer_with_shader_programs()?;
        gl.uniform1f(Some(self.shader_programs.get_ref_displacement_magnitude()), 1.0);
        gl.uniform1f(Some(self.shader_programs.get_ref_point_size()), 5.0);
        gl.uniform3f(Some(self.shader_programs.get_ref_shift()), 0.0, 0.0, 0.0);
        gl.uniform1f(Some(self.shader_programs.get_ref_scale()), 1.0);
        gl.uniform1f(Some(self.shader_programs.get_ref_d_scale()), 1.0);
        gl.uniform2f(Some(self.shader_programs.get_ref_screen_size()), width as f32, height as f32);
        gl.uniform_matrix4fv_with_f32_array(
            Some(self.shader_programs.get_ref_projection_matrix()), false, &projection_matrix,
        );
        gl.uniform_matrix4fv_with_f32_array(
            Some(self.shader_programs.get_ref_model_view_matrix()), false, &model_view_matrix,
        );

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
            ) = 
                match primitive_type
                {
                    PrimitiveType::Point => 
                    {
                        (
                            self.cs_axes_primitives.get_ref_points_coordinates(),
                            self.cs_axes_primitives.get_ref_points_is_to_scale(),
                            self.cs_axes_primitives.get_ref_points_reference_points(),
                            self.cs_axes_primitives.get_ref_points_displacements(),
                            self.cs_axes_primitives.get_ref_points_colors(),
                            self.cs_axes_primitives.get_points_count(),
                            GL::POINTS,
                        )
                    },
                    PrimitiveType::Line => 
                    {
                        (
                            self.cs_axes_primitives.get_ref_lines_endpoints_coordinates(),
                            self.cs_axes_primitives.get_ref_lines_endpoints_is_to_scale(),
                            self.cs_axes_primitives.get_ref_lines_endpoints_reference_points(),
                            self.cs_axes_primitives.get_ref_lines_endpoints_displacements(),
                            self.cs_axes_primitives.get_ref_lines_endpoints_colors(),
                            self.cs_axes_primitives.get_lines_count(),
                            GL::LINES,
                        )
                    },
                    PrimitiveType::Triangle => 
                    {
                        (
                            self.cs_axes_primitives.get_ref_triangles_vertices_coordinates(),
                            self.cs_axes_primitives.get_ref_triangles_vertices_is_to_scale(),
                            self.cs_axes_primitives.get_ref_triangles_vertices_reference_points(),
                            self.cs_axes_primitives.get_ref_triangles_vertices_displacements(),
                            self.cs_axes_primitives.get_ref_triangles_vertices_colors(),
                            self.cs_axes_primitives.get_triangles_count(),
                            GL::TRIANGLES,
                        )
                    },
                };
            vertices_coordinates.store(&gl, self.optional_vertex_buffer.as_ref()); 
            vertices_is_to_scale.store(&gl, self.optional_is_to_scale_buffer.as_ref()); 
            vertices_reference_points.store(&gl, self.optional_reference_point_buffer.as_ref()); 
            vertices_displacements.store(&gl, self.optional_vertex_displacement_buffer.as_ref()); 
            vertices_colors.store(&gl, self.optional_color_buffer.as_ref()); 
            gl.draw_arrays(mode, 0, count);  
        }

        self.draw_cs_axes_notations(&model_view_matrix, ctx)?;

        Ok(())
    }
}
