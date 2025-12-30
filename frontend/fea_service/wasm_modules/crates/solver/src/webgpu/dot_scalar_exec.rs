use std::cell::Cell;

use wasm_bindgen::JsValue;
use web_sys::{GpuBuffer, GpuCommandEncoder, GpuComputePassDescriptor, gpu_buffer_usage};

use crate::webgpu::{
    WebGpuCtx,
    buffers::{create_uniform_buffer, write_u32},
    create_readback_buffer, create_storage_buffer_f32,
    dot_partials::{
        DotPartialsPipeline, create_dot_partials_bind_group, create_dot_partials_pipeline,
    },
    dot_reduce::{DotReducePipeline, create_dot_reduce_bind_group, create_dot_reduce_pipeline},
    read_back_f32,
};

/// Computes dot products on the GPU and stores the results into a small "scalar results" buffer.
///
/// Design goals (for your PCG loop):
/// - Allow multiple dot products to be encoded into *one* command buffer.
/// - Avoid mapping/reading large "partials" arrays on CPU.
/// - Read back a small fixed array of scalars per iteration (e.g. pAp, rNorm2, rzNew).
///
/// Typical per-iteration usage:
///   1) call `encode_dot_scalar_into(..., out_index)` several times
///   2) call `encode_copy_scalar_results_to_readback()` once
///   3) submit the command buffer
///   4) await `readback_scalar_results()` once and consume all scalars
pub struct DotScalarExecutor {
    dot_partials_pipeline: DotPartialsPipeline,
    dot_reduce_pipeline: DotReducePipeline,

    // Scratch buffers used by the reduction pipeline.
    // We "ping-pong" between them across reduce passes.
    input_buffer: GpuBuffer,
    output_buffer: GpuBuffer,

    // Max number of partial sums that can be produced for the configured `n`
    // (i.e. ceil(n / 256)).
    // Kept for sanity/debugging; not strictly required at runtime.
    max_partials: usize,

    // GPU-side array of scalar outputs: scalar_results_buffer[i] holds one dot product.
    scalar_results_buffer: GpuBuffer,
    scalar_results_len: usize,

    // MAP_READ staging buffer for reading back scalar_results_buffer (COPY_DST).
    scalar_readback_buffer: GpuBuffer,

    // Uniform parameter buffers (small pool) to avoid allocating new uniform buffers per encode.
    dot_partials_params_buffers: Vec<GpuBuffer>,
    dot_partials_params_buffers_cursor: Cell<usize>,

    dot_reduce_params_buffers: Vec<GpuBuffer>,
    dot_reduce_params_buffers_cursor: Cell<usize>,
}

impl DotScalarExecutor {
    /// Creates the executor.
    ///
    /// `n` controls the maximum vector length you will dot.
    /// `scalar_results_len` controls the number of scalar slots available (per submit).
    /// Example: 7 slots to store pAp, rNorm2, rzNew, rzOld, alpha, -alpha, beta.
    pub fn create(ctx: &WebGpuCtx, n: usize, scalar_results_len: usize) -> Result<Self, JsValue> {
        let device = &ctx.device;

        let dot_partials_pipeline = create_dot_partials_pipeline(ctx)?;
        let dot_reduce_pipeline = create_dot_reduce_pipeline(ctx)?;

        let workgroup_size = 256usize;
        let max_partials = (n + workgroup_size - 1) / workgroup_size;

        // Scratch buffers for reductions:
        // - `input_buffer` receives partial sums from dot_partials pass
        // - reduction passes ping-pong between input_buffer and output_buffer
        //
        // NOTE: only the *final* scalar needs COPY_SRC (for copy to readback),
        // but we keep COPY_SRC on output_buffer to simplify ping-pong.
        let input_buffer = create_storage_buffer_f32(device, max_partials, "dot scratch input", 0)?;
        let output_buffer = create_storage_buffer_f32(
            device,
            max_partials,
            "dot scratch output",
            gpu_buffer_usage::COPY_SRC,
        )?;

        // Scalar results buffer:
        // - COPY_DST: we copy the final scalar into a slot in this buffer
        // - COPY_SRC: we copy the whole array to the readback buffer once per submit
        let scalar_results_buffer = create_storage_buffer_f32(
            device,
            scalar_results_len,
            "dot scalar results buffer",
            gpu_buffer_usage::COPY_SRC | gpu_buffer_usage::COPY_DST,
        )?;

        // Readback buffer for the scalar array.
        let scalar_readback_buffer =
            create_readback_buffer(device, scalar_results_len * 4, "dot scalar readback")?;

        // Params pools (ring buffers).
        // Size should cover the maximum number of dot passes you encode before the next submit.
        let mut dot_partials_params_buffers = Vec::with_capacity(4);
        for i in 0..4 {
            dot_partials_params_buffers.push(create_uniform_buffer(
                device,
                16,
                &format!("dot params {}", i),
            )?);
        }

        let mut dot_reduce_params_buffers = Vec::with_capacity(4);
        for i in 0..4 {
            dot_reduce_params_buffers.push(create_uniform_buffer(
                device,
                16,
                &format!("reduce params {}", i),
            )?);
        }

        Ok(Self {
            dot_partials_pipeline,
            dot_reduce_pipeline,
            input_buffer,
            output_buffer,
            max_partials,
            scalar_results_buffer,
            scalar_results_len,
            scalar_readback_buffer,
            dot_partials_params_buffers,
            dot_partials_params_buffers_cursor: Cell::new(0),
            dot_reduce_params_buffers,
            dot_reduce_params_buffers_cursor: Cell::new(0),
        })
    }

    fn next_dot_partials_params_buffer(&self) -> &GpuBuffer {
        let i = self.dot_partials_params_buffers_cursor.get();
        self.dot_partials_params_buffers_cursor
            .set((i + 1) % self.dot_partials_params_buffers.len());
        &self.dot_partials_params_buffers[i]
    }

    fn next_dot_reduce_params_buffer(&self) -> &GpuBuffer {
        let i = self.dot_reduce_params_buffers_cursor.get();
        self.dot_reduce_params_buffers_cursor
            .set((i + 1) % self.dot_reduce_params_buffers.len());
        &self.dot_reduce_params_buffers[i]
    }

    /// Encodes the GPU work to compute a dot product and store it into:
    ///   scalar_results_buffer[out_index] = dot(a_buffer, b_buffer)
    ///
    /// This method:
    /// - encodes compute passes into `command_encoder`
    /// - encodes one copy into `scalar_results_buffer` (slot `out_index`)
    /// - does NOT submit work
    /// - does NOT map/read back anything
    ///
    /// Required usage: after encoding all dot products for a submit, call
    /// `encode_copy_scalar_results_to_readback()` once, then submit, then call
    /// `readback_scalar_results().await`.
    pub fn encode_dot_scalar_into(
        &self,
        ctx: &WebGpuCtx,
        command_encoder: &GpuCommandEncoder,
        a_buffer: &GpuBuffer,
        b_buffer: &GpuBuffer,
        n: u32,
        out_index: u32,
    ) -> Result<(), JsValue> {
        if (out_index as usize) >= self.scalar_results_len {
            return Err(JsValue::from_str(
                "DotScalarExecutor::encode_dot_scalar_into: out_index out of range",
            ));
        }

        // Degenerate case: dot over an empty vector is 0.0.
        // We write deterministically into the slot using queue.write_buffer.
        // (This is cheap and avoids copying unspecified scratch memory.)
        if n == 0 {
            // Write 0.0f32 into scalar_results_buffer[out_index].
            // NOTE: scalar_results_buffer has COPY_DST usage.
            let zero_bits = 0.0f32.to_bits();
            let offset_bytes = out_index * 4;
            ctx.queue
                .write_buffer_with_u32_and_u8_slice(
                    &self.scalar_results_buffer,
                    offset_bytes,
                    &zero_bits.to_le_bytes(),
                )
                .map_err(|e| {
                    JsValue::from_str(&format!(
                        "DotScalarExecutor::encode_dot_scalar_into: write_buffer failed: {:?}",
                        e
                    ))
                })?;
            return Ok(());
        }

        // ---------------------------------------------------------------------
        // Pass 1: compute partial sums of a[i]*b[i] (one partial per workgroup)
        // ---------------------------------------------------------------------
        let dot_partials_params_buffer = self.next_dot_partials_params_buffer();
        write_u32(&ctx.queue, dot_partials_params_buffer, &[n, 0, 0, 0])?;

        let dot_partials_bind_group = create_dot_partials_bind_group(
            &ctx.device,
            &self.dot_partials_pipeline.dot_partials_bind_group_layout,
            dot_partials_params_buffer,
            a_buffer,
            b_buffer,
            &self.input_buffer, // writes partial sums here
        );

        {
            let compute_pass_desc = GpuComputePassDescriptor::new();
            let compute_pass_encoder =
                command_encoder.begin_compute_pass_with_descriptor(&compute_pass_desc);
            compute_pass_encoder.set_pipeline(&self.dot_partials_pipeline.pipeline);
            compute_pass_encoder.set_bind_group(0, Some(&dot_partials_bind_group));

            let workgroup_size = 256u32;
            let groups_x = (n + workgroup_size - 1) / workgroup_size;
            compute_pass_encoder.dispatch_workgroups(groups_x);
            compute_pass_encoder.end();
        }

        // Number of partials produced by Pass 1.
        let mut current_len: u32 = (n + 256u32 - 1) / 256u32;

        // ---------------------------------------------------------------------
        // Pass 2..k: reduce partials until one scalar remains
        // ---------------------------------------------------------------------
        let mut current_input_buffer = &self.input_buffer;
        let mut current_output_buffer = &self.output_buffer;

        while current_len > 1 {
            let reduce_params = self.next_dot_reduce_params_buffer();
            write_u32(&ctx.queue, reduce_params, &[current_len, 0, 0, 0])?;

            let dot_reduce_bind_group = create_dot_reduce_bind_group(
                &ctx.device,
                &self.dot_reduce_pipeline.dot_reduce_bind_group_layout,
                reduce_params,
                current_input_buffer,
                current_output_buffer,
            );

            let out_len = (current_len + 256u32 - 1) / 256u32;

            let compute_pass_desc = GpuComputePassDescriptor::new();
            let compute_pass_encoder =
                command_encoder.begin_compute_pass_with_descriptor(&compute_pass_desc);
            compute_pass_encoder.set_pipeline(&self.dot_reduce_pipeline.pipeline);
            compute_pass_encoder.set_bind_group(0, Some(&dot_reduce_bind_group));
            compute_pass_encoder.dispatch_workgroups(out_len);
            compute_pass_encoder.end();

            current_len = out_len;
            std::mem::swap(&mut current_input_buffer, &mut current_output_buffer);
        }

        // ---------------------------------------------------------------------
        // Final: copy the scalar into scalar_results_buffer[out_index]
        // ---------------------------------------------------------------------
        command_encoder.copy_buffer_to_buffer_with_u32_and_u32_and_u32(
            current_input_buffer, // holds the scalar at byte offset 0
            0,
            &self.scalar_results_buffer,
            out_index * 4,
            4,
        )?;

        Ok(())
    }

    /// Encodes a single GPU->GPU copy:
    ///   scalar_readback_buffer <- scalar_results_buffer
    ///
    /// Call this once per submit (after you filled all slots you need).
    pub fn encode_copy_scalar_results_to_readback(
        &self,
        command_encoder: &GpuCommandEncoder,
    ) -> Result<(), JsValue> {
        let bytes = (self.scalar_results_len as u32) * 4;
        command_encoder.copy_buffer_to_buffer_with_u32_and_u32_and_u32(
            &self.scalar_results_buffer,
            0,
            &self.scalar_readback_buffer,
            0,
            bytes,
        )?;
        Ok(())
    }

    /// Maps and reads back the scalar results array.
    ///
    /// CONTRACT:
    /// - You must have encoded `encode_copy_scalar_results_to_readback()` and submitted it
    ///   before calling this.
    pub async fn readback_scalar_results(&self) -> Result<Vec<f32>, JsValue> {
        read_back_f32(&self.scalar_readback_buffer, self.scalar_results_len).await
    }

    /// Exposes the GPU scalar array buffer so other kernels (e.g. pcg_update_scalars)
    /// can read/write scalar slots directly.
    pub fn scalar_results_buffer(&self) -> &GpuBuffer {
        &self.scalar_results_buffer
    }
}
