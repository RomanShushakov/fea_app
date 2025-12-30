use wasm_bindgen::JsValue;
use web_sys::{GpuBindGroup, GpuBuffer, GpuCommandEncoder, GpuComputePassDescriptor};

use crate::webgpu::block_jacobi::{
    BlockJacobiPipeline, create_block_jacobi_bind_group, create_block_jacobi_pipeline,
};
use crate::webgpu::buffers::{
    create_storage_buffer_f32_with_data, create_storage_buffer_u32_with_data,
    create_uniform_buffer, write_u32,
};
use crate::webgpu::ctx::WebGpuCtx;

/// BlockJacobiExecutor
///
/// Owns the immutable GPU resources for the Block-Jacobi preconditioner:
///   - `lu_blocks_buffer`: packed LU factors (one dense 6x6 per block, row-major)
///   - `block_starts_buffer`: block boundaries (length = num_blocks + 1)
///   - `params_buffer`: uniform constants (n, num_blocks)
///
/// Per apply call you provide:
///   - `r_gpu`: input vector r (typically residual)
///   - `z_gpu`: output vector z = M^{-1} r
///
/// Dispatch:
///   - 1 workgroup per block (workgroup_size = 1 in WGSL),
///     so we dispatch `num_blocks` workgroups on X.
pub struct BlockJacobiExecutor {
    n: u32,
    num_blocks: u32,

    // Pipeline + layout (immutable)
    block_jacobi_pipeline: BlockJacobiPipeline,

    // Persistent GPU buffers (immutable once created)
    params_buffer: GpuBuffer,       // 16 bytes: [n, num_blocks, 0, 0]
    lu_blocks_buffer: GpuBuffer,    // num_blocks * 36 f32 (6*6 per block)
    block_starts_buffer: GpuBuffer, // (num_blocks + 1) u32
}

impl BlockJacobiExecutor {
    /// Create GPU resources for the Block-Jacobi preconditioner.
    ///
    /// Contract with `block_jacobi.wgsl`:
    ///   - `lu_blocks_host` is a packed array of dense 6x6 LU blocks:
    ///       block_id * 36 .. block_id * 36 + 36
    ///   - `block_starts_u32` has length (num_blocks + 1) and defines the block ranges.
    ///
    /// NOTE:
    ///   This executor assumes the WGSL kernel is compiled with BLOCK_SIZE = 6
    ///   and LU_STRIDE = 36. If you generalize block size later, this constructor
    ///   is the right place to encode the stride/size contract explicitly.
    pub fn create(
        ctx: &WebGpuCtx,
        n: u32,
        lu_blocks_host: &[f32],
        block_starts_u32: &[u32],
    ) -> Result<Self, JsValue> {
        let device = &ctx.device;
        let queue = &ctx.queue;

        let block_jacobi_pipeline = create_block_jacobi_pipeline(ctx)?;

        // Derive num_blocks from block_starts length.
        if block_starts_u32.len() < 2 {
            return Err(JsValue::from_str(
                "BlockJacobiExecutor::create: block_starts must have at least 2 entries",
            ));
        }
        let num_blocks: u32 = (block_starts_u32.len() - 1) as u32;

        // Validate LU packing matches 6x6 per block (36 floats).
        let expected_lu_len: usize = (num_blocks as usize) * 36;
        if lu_blocks_host.len() != expected_lu_len {
            return Err(JsValue::from_str(&format!(
                "BlockJacobiExecutor::create: lu_blocks len {} != expected {} (num_blocks={} * 36)",
                lu_blocks_host.len(),
                expected_lu_len,
                num_blocks
            )));
        }

        // // Cheap sanity checks (debug-only):
        // // - last block start should be <= n (common invariant)
        // // - block_starts should be non-decreasing
        // debug_assert!(
        //     *block_starts_u32.last().unwrap() <= n,
        //     "block_starts last entry must be <= n"
        // );
        // debug_assert!(
        //     block_starts_u32.windows(2).all(|w| w[0] <= w[1]),
        //     "block_starts must be non-decreasing"
        // );

        // Uniform params: [n, num_blocks, 0, 0]
        let params_buffer = create_uniform_buffer(device, 16, "block_jacobi params")?;
        write_u32(queue, &params_buffer, &[n, num_blocks, 0, 0])?;

        // Immutable preconditioner data buffers.
        let lu_blocks_buffer = create_storage_buffer_f32_with_data(
            device,
            queue,
            lu_blocks_host,
            "block_jacobi lu_blocks",
            0,
        )?;

        let block_starts_buffer = create_storage_buffer_u32_with_data(
            device,
            queue,
            block_starts_u32,
            "block_jacobi block_starts",
            0,
        )?;

        Ok(Self {
            n,
            num_blocks,
            block_jacobi_pipeline,
            params_buffer,
            lu_blocks_buffer,
            block_starts_buffer,
        })
    }

    /// Encode: z = M^{-1} r
    ///
    /// Notes:
    ///   - This only encodes GPU work into `command_encoder`.
    ///   - The caller controls submit order and synchronization.
    ///   - `r_gpu` and `z_gpu` must be storage buffers of length >= n floats.
    pub fn encode_apply(
        &self,
        ctx: &WebGpuCtx,
        command_encoder: &GpuCommandEncoder,
        r_gpu: &GpuBuffer,
        z_gpu: &GpuBuffer,
    ) -> Result<(), JsValue> {
        // Bind group depends on per-call buffers r/z, so build it on demand.
        let bind_group: GpuBindGroup = create_block_jacobi_bind_group(
            &ctx.device,
            &self.block_jacobi_pipeline.block_jacobi_bind_group_layout,
            &self.params_buffer,
            &self.lu_blocks_buffer,
            &self.block_starts_buffer,
            r_gpu,
            z_gpu,
        );

        let pass_desc = GpuComputePassDescriptor::new();
        pass_desc.set_label("block_jacobi apply pass");

        let pass = command_encoder.begin_compute_pass_with_descriptor(&pass_desc);
        pass.set_pipeline(&self.block_jacobi_pipeline.pipeline);
        pass.set_bind_group(0, Some(&bind_group));

        // One workgroup per block (workgroup_size = 1 in WGSL).
        pass.dispatch_workgroups(self.num_blocks);
        pass.end();

        Ok(())
    }

    // /// Expose for diagnostics / logging.
    // pub fn n(&self) -> u32 {
    //     self.n
    // }

    // /// Expose for diagnostics / logging.
    // pub fn num_blocks(&self) -> u32 {
    //     self.num_blocks
    // }
}
