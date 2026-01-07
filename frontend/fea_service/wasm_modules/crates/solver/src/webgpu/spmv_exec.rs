use wasm_bindgen::JsValue;
use web_sys::gpu_buffer_usage;
use web_sys::{GpuBindGroup, GpuBuffer, GpuCommandEncoder, GpuComputePassDescriptor};

use crate::webgpu::buffers::{
    create_storage_buffer_f32, create_storage_buffer_f32_with_data,
    create_storage_buffer_u32_with_data, create_uniform_buffer, write_u32,
};
use crate::webgpu::ctx::WebGpuCtx;
use crate::webgpu::spmv::{SpmvPipeline, create_spmv_bind_group, create_spmv_pipeline};

// SpmvExecutor
//
// Owns persistent GPU resources for CSR SpMV:
//   - CSR structure buffers: row_ptr, col_idx, values (uploaded once)
//   - params uniform buffer: n_rows (written once)
//   - internal x/y vectors used by the shader:
//       x_buffer: input vector for A*x (you copy your current vector into it)
//       y_buffer: output vector (SpMV result), reused every call
//
// Typical usage per iteration:
//   1) encode_copy_x_from(..., p_gpu, n_bytes)
//   2) encode_spmv(...)
//   3) read y_buffer() as "Ap" (used by later kernels)

pub struct SpmvExecutor {
    n_rows: u32,

    // Pipeline + bind group
    spmv_pipeline: SpmvPipeline,
    spmv_bind_group: GpuBindGroup,

    // Persistent buffers (created once)
    params_buffer: GpuBuffer,
    row_ptr_buffer: GpuBuffer,
    col_idx_buffer: GpuBuffer,
    values_buffer: GpuBuffer,
    x_buffer: GpuBuffer,
    y_buffer: GpuBuffer,
}

impl SpmvExecutor {
    pub fn create(
        ctx: &WebGpuCtx,
        n_rows: u32,
        row_ptr_u32: &[u32],
        col_idx_u32: &[u32],
        values_f32: &[f32],
    ) -> Result<Self, JsValue> {
        let device = &ctx.device;
        let queue = &ctx.queue;

        // 1) Create pipeline (once).
        let spmv_pipeline = create_spmv_pipeline(ctx)?;

        // 2) Params uniform (once): n_rows is enough for the current shader.
        let params_buffer = create_uniform_buffer(device, 16, "spmv params")?;
        write_u32(queue, &params_buffer, &[n_rows, 0, 0, 0])?;

        // 3) CSR buffers (once).
        // row_ptr length must be n_rows + 1; col_idx/values length must be nnz.
        let row_ptr_buffer =
            create_storage_buffer_u32_with_data(device, queue, row_ptr_u32, "spmv row_ptr", 0)?;
        let col_idx_buffer =
            create_storage_buffer_u32_with_data(device, queue, col_idx_u32, "spmv col_idx", 0)?;
        let values_buffer =
            create_storage_buffer_f32_with_data(device, queue, values_f32, "spmv values", 0)?;

        // 4) Internal x/y buffers (reused every encode_spmv call).
        //
        // x is read-only in WGSL => STORAGE is enough. We also need COPY_DST because
        // we fill x via GPU->GPU copy from some external buffer.
        let x_buffer = create_storage_buffer_f32(device, n_rows as usize, "spmv x", 0)?;

        // y is read_write in WGSL and is consumed by other kernels. COPY_SRC is useful
        // if we ever want to copy y into a readback buffer for debugging/validation.
        let y_buffer = create_storage_buffer_f32(
            device,
            n_rows as usize,
            "spmv y",
            gpu_buffer_usage::COPY_SRC,
        )?;

        // 5) Bind group (once).
        let spmv_bind_group = create_spmv_bind_group(
            device,
            &spmv_pipeline.spmv_bind_group_layout,
            &params_buffer,
            &row_ptr_buffer,
            &col_idx_buffer,
            &values_buffer,
            &x_buffer,
            &y_buffer,
        );

        Ok(Self {
            n_rows,
            spmv_pipeline,
            spmv_bind_group,
            params_buffer,
            row_ptr_buffer,
            col_idx_buffer,
            values_buffer,
            x_buffer,
            y_buffer,
        })
    }

    /// Encode the CSR SpMV compute pass:
    ///   y_buffer = A * x_buffer
    pub fn encode_spmv(&self, command_encoder: &GpuCommandEncoder) -> Result<(), JsValue> {
        let compute_pass_desc = GpuComputePassDescriptor::new();
        compute_pass_desc.set_label("spmv pass");

        let compute_pass_encoder =
            command_encoder.begin_compute_pass_with_descriptor(&compute_pass_desc);

        compute_pass_encoder.set_pipeline(&self.spmv_pipeline.pipeline);
        compute_pass_encoder.set_bind_group(0, Some(&self.spmv_bind_group));

        // One invocation computes one row.
        let workgroup_size = 256u32;
        let groups_x = self.n_rows.div_ceil(workgroup_size);
        compute_pass_encoder.dispatch_workgroups(groups_x);

        compute_pass_encoder.end();
        Ok(())
    }

    /// Output buffer produced by encode_spmv(): y = A*x
    pub fn y_buffer(&self) -> &GpuBuffer {
        &self.y_buffer
    }

    /// Encode a GPU->GPU copy into the executor's internal x_buffer:
    ///   x_buffer <- src_gpu
    ///
    /// `n_bytes` should be `n_rows * sizeof(f32)`.
    pub fn encode_copy_x_from(
        &self,
        command_encoder: &GpuCommandEncoder,
        src_gpu: &GpuBuffer,
        n_bytes: u32,
    ) -> Result<(), JsValue> {
        command_encoder.copy_buffer_to_buffer_with_u32_and_u32_and_u32(
            src_gpu,
            0,
            &self.x_buffer,
            0,
            n_bytes,
        )?;
        Ok(())
    }
}
