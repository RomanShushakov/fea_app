use std::cell::Cell;

use wasm_bindgen::JsValue;
use web_sys::{GpuBindGroup, GpuBuffer, GpuCommandEncoder, GpuComputePassDescriptor};

use crate::webgpu::buffers::{create_uniform_buffer, write_u32};
use crate::webgpu::ctx::WebGpuCtx;
use crate::webgpu::pcg_update_scalars::{
    PcgUpdateScalarsPipeline, create_pcg_update_scalars_bind_group,
    create_pcg_update_scalars_pipeline,
};

/// Encodes the tiny "scalar update" pass used by PCG.
///
/// This pass reads dot-products from `scalar_results_buffer` and writes
/// derived PCG scalars back into the same buffer:
///   alpha      = rz_old / p_ap
///   minusAlpha = -alpha
///   beta       = rz_new / rz_old
///
/// The shader (`pcg_update_scalars.wgsl`) is single-workgroup and uses only indices,
/// so the executor provides a small uniform `Params` buffer containing:
///   [p_ap_index, rz_new_index, rz_old_index, alpha_index, minus_alpha_index, beta_index, 0, 0]
///
/// IMPORTANT: We keep a *pool* of params uniform buffers so multiple update passes can be
/// encoded into the same command buffer safely (and across multiple in-flight submits).
pub struct PcgUpdateScalarsExecutor {
    pcg_update_scalars_pipeline: PcgUpdateScalarsPipeline,

    // Pool of uniform buffers (32 bytes each) for WGSL Params.
    // This avoids "write-after-submit" hazards when the CPU updates params while
    // the GPU is still reading from an older submit.
    params_buffers: Vec<GpuBuffer>,
    params_buffers_cursor: Cell<usize>,
}

impl PcgUpdateScalarsExecutor {
    /// Create the executor + pipeline + params uniform buffer pool.
    pub fn create(ctx: &WebGpuCtx) -> Result<Self, JsValue> {
        let device = &ctx.device;

        let pcg_update_scalars_pipeline = create_pcg_update_scalars_pipeline(ctx)?;

        // Pool size:
        // - In your PCG iteration you encode this pass 2 times per iteration
        //   (alpha/-alpha early, then beta after rz_new is available).
        // - 4 gives comfortable headroom if you ever batch or overlap work.
        let params_buffers_pool_size = 4usize;

        let mut params_buffers = Vec::with_capacity(params_buffers_pool_size);
        for i in 0..params_buffers_pool_size {
            // 32 bytes = 8 u32 fields (matches WGSL Params struct layout).
            params_buffers.push(create_uniform_buffer(
                device,
                32,
                &format!("pcg update scalars params {}", i),
            )?);
        }

        Ok(Self {
            pcg_update_scalars_pipeline,
            params_buffers,
            params_buffers_cursor: Cell::new(0),
        })
    }

    /// Grab the next params uniform buffer from the pool in round-robin order.
    fn next_params_buffer(&self) -> &GpuBuffer {
        let i = self.params_buffers_cursor.get();
        self.params_buffers_cursor
            .set((i + 1) % self.params_buffers.len());
        &self.params_buffers[i]
    }

    /// Encode one `pcg_update_scalars.wgsl` dispatch.
    ///
    /// Uniform Params layout (matches WGSL exactly):
    ///   u32 p_ap_index         @ offset  0
    ///   u32 rz_new_index       @ offset  4
    ///   u32 rz_old_index       @ offset  8
    ///   u32 alpha_index        @ offset 12
    ///   u32 minus_alpha_index  @ offset 16
    ///   u32 beta_index         @ offset 20
    ///   u32 _pad0              @ offset 24
    ///   u32 _pad1              @ offset 28
    pub fn encode_update_scalars(
        &self,
        ctx: &WebGpuCtx,
        command_encoder: &GpuCommandEncoder,
        scalar_results_buffer: &GpuBuffer,
        p_ap_index: u32,
        rz_new_index: u32,
        rz_old_index: u32,
        alpha_index: u32,
        minus_alpha_index: u32,
        beta_index: u32,
    ) -> Result<(), JsValue> {
        // Allocate params from pool and upload indices.
        let params_buffer = self.next_params_buffer();

        // Params are "indices into scalar_results_buffer".
        // The shader reads:
        //   p_ap  = scalar_results[p_ap_index]
        //   rz_new= scalar_results[rz_new_index]
        //   rz_old= scalar_results[rz_old_index]
        //
        // and writes:
        //   scalar_results[alpha_index]       = rz_old / p_ap
        //   scalar_results[minus_alpha_index] = -alpha
        //   scalar_results[beta_index]        = rz_new / rz_old
        write_u32(
            &ctx.queue,
            params_buffer,
            &[
                p_ap_index,        // offset 0
                rz_new_index,      // offset 4
                rz_old_index,      // offset 8
                alpha_index,       // offset 12
                minus_alpha_index, // offset 16
                beta_index,        // offset 20
                0,                 // pad (offset 24)
                0,                 // pad (offset 28)
            ],
        )?;

        // Bind params + scalar_results.
        let bind_group: GpuBindGroup = create_pcg_update_scalars_bind_group(
            &ctx.device,
            &self
                .pcg_update_scalars_pipeline
                .pcg_update_scalars_bind_group_layout,
            params_buffer,
            scalar_results_buffer,
        );

        // Dispatch 1 workgroup (WGSL has @workgroup_size(1)).
        let compute_pass_desc = GpuComputePassDescriptor::new();
        let compute_pass = command_encoder.begin_compute_pass_with_descriptor(&compute_pass_desc);
        compute_pass.set_pipeline(&self.pcg_update_scalars_pipeline.pipeline);
        compute_pass.set_bind_group(0, Some(&bind_group));
        compute_pass.dispatch_workgroups(1);
        compute_pass.end();

        Ok(())
    }
}
