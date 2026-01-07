use std::cell::Cell;

use wasm_bindgen::JsValue;
use web_sys::{GpuBindGroup, GpuBuffer, GpuCommandEncoder, GpuComputePassDescriptor};

use crate::webgpu::buffers::{create_uniform_buffer, write_u32};
use crate::webgpu::ctx::WebGpuCtx;
use crate::webgpu::vec_ops::{
    AxpyFromScalarResultsPipeline, AxpyPipeline, ScaleFromScalarResultsPipeline,
    create_axpy_bind_group, create_axpy_from_scalar_results_bind_group,
    create_axpy_from_scalar_results_pipeline, create_axpy_pipeline,
    create_scale_from_scalar_results_bind_group, create_scale_from_scalar_results_pipeline,
};

/// Vector-ops executor used by PCG:
///   - AXPY with an *immediate* scalar alpha (uniform contains alpha bits)
///   - AXPY with alpha read from scalar_results_buffer[scalar_index]
///   - SCALE with beta read from scalar_results_buffer[scalar_index]
///
/// This executor owns:
///   - pipelines + bind group layouts
///   - a small pool of uniform buffers so we can encode many vec-ops in one command buffer
///     without clobbering params that are still in use by the GPU.
pub struct VecOpsExecutor {
    // y = y + alpha * x  (alpha comes from uniform)
    axpy_pipeline: AxpyPipeline,

    // y = y + scalar_results[scalar_index] * x
    axpy_from_scalar_results_pipeline: AxpyFromScalarResultsPipeline,

    // x = x * scalar_results[scalar_index]
    scale_from_scalar_results_pipeline: ScaleFromScalarResultsPipeline,

    // Uniform params pool shared across all vec-op kernels.
    // Each kernel reads only the subset of fields it needs.
    params_buffers: Vec<GpuBuffer>,
    params_cursor: Cell<usize>,
}

impl VecOpsExecutor {
    pub fn create(ctx: &WebGpuCtx) -> Result<Self, JsValue> {
        let device = &ctx.device;

        let axpy_pipeline = create_axpy_pipeline(ctx)?;
        let axpy_from_scalar_results_pipeline = create_axpy_from_scalar_results_pipeline(ctx)?;
        let scale_from_scalar_results_pipeline = create_scale_from_scalar_results_pipeline(ctx)?;

        // Pool size:
        // must cover the maximum number of vec-ops encoded between submits.
        // PCG "one submit per iteration" tends to encode several ops; 16 is safe.
        let params_buffers_pool_size = 16usize;

        let mut params_buffers = Vec::with_capacity(params_buffers_pool_size);
        for i in 0..params_buffers_pool_size {
            // 32 bytes is enough for both uniform layouts used here:
            //
            // 1) Immediate scalar layout (axpy.wgsl / scale.wgsl style):
            //    [n, 0, 0, 0, alpha_bits, 0, 0, 0]  (8 u32 = 32 bytes)
            //
            // 2) Scalar-results index layout (*_from_scalar_results.wgsl):
            //    [n, scalar_index, 0, 0]            (4 u32 = 16 bytes)
            //
            // We just standardize on 32 bytes for everything.
            params_buffers.push(create_uniform_buffer(
                device,
                32,
                &format!("vec_ops params {}", i),
            )?);
        }

        Ok(Self {
            axpy_pipeline,
            axpy_from_scalar_results_pipeline,
            scale_from_scalar_results_pipeline,
            params_buffers,
            params_cursor: Cell::new(0),
        })
    }

    fn next_params_buffer(&self) -> &GpuBuffer {
        let i = self.params_cursor.get();
        self.params_cursor.set((i + 1) % self.params_buffers.len());
        &self.params_buffers[i]
    }

    /// Write uniform params for kernels that take an immediate scalar (alpha/beta) in the uniform.
    ///
    /// Layout (32 bytes, 8 u32s):
    ///   u32 n            @ offset  0
    ///   u32 _pad0        @ offset  4
    ///   u32 _pad1        @ offset  8
    ///   u32 _pad2        @ offset 12
    ///   u32 alpha_bits   @ offset 16   (f32::to_bits)
    ///   u32 _pad3        @ offset 20
    ///   u32 _pad4        @ offset 24
    ///   u32 _pad5        @ offset 28
    fn write_params_into_for_immediate_scalar(
        &self,
        ctx: &WebGpuCtx,
        params_buffer: &GpuBuffer,
        n: u32,
        alpha: f32,
    ) -> Result<(), JsValue> {
        let alpha_u32 = alpha.to_bits();
        write_u32(&ctx.queue, params_buffer, &[n, 0, 0, 0, alpha_u32, 0, 0, 0])?;
        Ok(())
    }

    /// Write uniform params for kernels that read the scalar from
    /// `scalar_results_buffer[scalar_index]`.
    ///
    /// Layout (16 bytes, 4 u32s):
    ///   u32 n            @ offset  0
    ///   u32 scalar_index @ offset  4
    ///   u32 _pad0        @ offset  8
    ///   u32 _pad1        @ offset 12
    fn write_params_into_for_scalar_results_index(
        &self,
        ctx: &WebGpuCtx,
        params_buffer: &GpuBuffer,
        n: u32,
        scalar_index: u32,
    ) -> Result<(), JsValue> {
        write_u32(&ctx.queue, params_buffer, &[n, scalar_index, 0, 0])?;
        Ok(())
    }

    /// Encode: y = y + alpha * x, where alpha is provided immediately (uniform).
    pub fn encode_axpy_inplace(
        &self,
        ctx: &WebGpuCtx,
        encoder: &GpuCommandEncoder,
        x_buffer: &GpuBuffer,
        y_buffer: &GpuBuffer,
        n: u32,
        alpha: f32,
    ) -> Result<(), JsValue> {
        let params_buffer = self.next_params_buffer();
        self.write_params_into_for_immediate_scalar(ctx, params_buffer, n, alpha)?;

        let bind_group: GpuBindGroup = create_axpy_bind_group(
            &ctx.device,
            &self.axpy_pipeline.axpy_bind_group_layout,
            params_buffer,
            x_buffer,
            y_buffer,
        );

        let compute_pass_desc = GpuComputePassDescriptor::new();
        let compute_pass = encoder.begin_compute_pass_with_descriptor(&compute_pass_desc);
        compute_pass.set_pipeline(&self.axpy_pipeline.pipeline);
        compute_pass.set_bind_group(0, Some(&bind_group));

        let workgroup_size = 256u32;
        compute_pass.dispatch_workgroups(n.div_ceil(workgroup_size));
        compute_pass.end();

        Ok(())
    }

    /// Encode: y = y + scalar_results[scalar_index] * x.
    pub fn encode_axpy_inplace_from_scalar_results(
        &self,
        ctx: &WebGpuCtx,
        encoder: &GpuCommandEncoder,
        x_buffer: &GpuBuffer,
        y_buffer: &GpuBuffer,
        n: u32,
        scalar_results_buffer: &GpuBuffer,
        scalar_index: u32,
    ) -> Result<(), JsValue> {
        let params_buffer = self.next_params_buffer();
        self.write_params_into_for_scalar_results_index(ctx, params_buffer, n, scalar_index)?;

        let bind_group: GpuBindGroup = create_axpy_from_scalar_results_bind_group(
            &ctx.device,
            &self
                .axpy_from_scalar_results_pipeline
                .axpy_from_scalar_results_bind_group_layout,
            params_buffer,
            x_buffer,
            y_buffer,
            scalar_results_buffer,
        );

        let compute_pass_desc = GpuComputePassDescriptor::new();
        let compute_pass = encoder.begin_compute_pass_with_descriptor(&compute_pass_desc);
        compute_pass.set_pipeline(&self.axpy_from_scalar_results_pipeline.pipeline);
        compute_pass.set_bind_group(0, Some(&bind_group));

        let workgroup_size = 256u32;
        compute_pass.dispatch_workgroups(n.div_ceil(workgroup_size));
        compute_pass.end();

        Ok(())
    }

    /// Encode: x = x * scalar_results[scalar_index].
    pub fn encode_scale_inplace_from_scalar_results(
        &self,
        ctx: &WebGpuCtx,
        encoder: &GpuCommandEncoder,
        x_buffer: &GpuBuffer,
        n: u32,
        scalar_results_buffer: &GpuBuffer,
        scalar_index: u32,
    ) -> Result<(), JsValue> {
        let params_buffer = self.next_params_buffer();
        self.write_params_into_for_scalar_results_index(ctx, params_buffer, n, scalar_index)?;

        let bind_group: GpuBindGroup = create_scale_from_scalar_results_bind_group(
            &ctx.device,
            &self
                .scale_from_scalar_results_pipeline
                .scale_from_scalar_results_bind_group_layout,
            params_buffer,
            x_buffer,
            scalar_results_buffer,
        );

        let compute_pass_desc = GpuComputePassDescriptor::new();
        let compute_pass = encoder.begin_compute_pass_with_descriptor(&compute_pass_desc);
        compute_pass.set_pipeline(&self.scale_from_scalar_results_pipeline.pipeline);
        compute_pass.set_bind_group(0, Some(&bind_group));

        let workgroup_size = 256u32;
        compute_pass.dispatch_workgroups(n.div_ceil(workgroup_size));
        compute_pass.end();

        Ok(())
    }
}
