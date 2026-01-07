use extended_matrix::{BasicOperationsTrait, CsrMatrix, Position, Vector};
use finite_element_method::SeparatedStiffnessMatrixSparse;
use iterative_solvers_smpl::{block_jacobi::BlockJacobiPreconditioner, linalg::dot};
use js_sys::Array;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use web_sys::gpu_buffer_usage;

mod ctx;
pub(crate) use ctx::{WebGpuCtx, init_webgpu};

mod buffers;
pub(crate) use buffers::{
    create_readback_buffer, create_storage_buffer_f32, create_storage_buffer_f32_with_data,
    read_back_f32,
};

mod spmv;

mod spmv_exec;
pub(crate) use spmv_exec::SpmvExecutor;

mod vec_ops;

mod vec_ops_exec;
pub(crate) use vec_ops_exec::VecOpsExecutor;

mod dot_partials;

mod block_jacobi;
mod block_jacobi_exec;

mod dot_reduce;
mod dot_scalar_exec;

mod pcg_update_scalars;
mod pcg_update_scalars_exec;

use crate::webgpu::{
    block_jacobi_exec::BlockJacobiExecutor,
    buffers::encode_write_f32_into_storage_buffer_at_index,
    dot_scalar_exec::DotScalarExecutor,
    export::{make_pcg_block_jacobi_csr_input_bundle_text, send_export_to_main_thread},
    pcg_update_scalars_exec::PcgUpdateScalarsExecutor,
};

mod export;

const NODE_DOF: usize = 6;
const REL_TOL: f32 = 1e-4;
const ABS_TOL: f32 = 1e-7;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(value: &str);
}

fn find_b_sparse_webgpu(
    r_a_vector: &Vector<f32>,
    k_ab_triplets: &[(usize, usize, f32)],
    u_b_vector: &Vector<f32>,
    n_aa: usize,
) -> Result<Vec<f32>, String> {
    let mut b = vec![0.0f32; n_aa];
    for i in 0..n_aa {
        b[i] = *r_a_vector.get_element_value(&Position(i, 0))?;
    }

    for &(i, j, a_ij) in k_ab_triplets.iter() {
        let u_j = *u_b_vector.get_element_value(&Position(j, 0))?;
        b[i] -= a_ij * u_j;
    }

    Ok(b)
}

fn build_block_starts_from_k_aa_indexes_webgpu(k_aa_indexes: &[usize], n: usize) -> Vec<usize> {
    let mut starts = Vec::new();
    if k_aa_indexes.is_empty() {
        return starts;
    }

    starts.push(0);

    let mut current_node = k_aa_indexes[0] / NODE_DOF;

    for (i, &global_dof) in k_aa_indexes.iter().enumerate() {
        let node_index = global_dof / NODE_DOF;
        if node_index != current_node {
            starts.push(i);
            current_node = node_index;
        }
    }

    if *starts.last().unwrap() != n {
        starts.push(n);
    }

    starts
}

/// In-place LU factorization (no pivoting) for a small dense matrix stored in a fixed 6x6 buffer.
///
/// Storage / layout:
/// - `mat` is a row-major 6x6 buffer (length must be >= 36).
/// - `n` is the active dimension (we factor only the leading n×n block, where n <= 6).
///
/// Output convention (IMPORTANT: must match block_jacobi.wgsl):
/// - The result is stored in the same `mat` buffer, as a combined LU matrix:
///     - Strict lower triangle (i > j): stores L(i,j) multipliers.
///     - Diagonal and upper triangle (i <= j): stores U(i,j).
/// - L has an implicit unit diagonal: L(i,i) == 1.0 and is NOT stored explicitly.
///   This is why the WGSL forward solve does not divide by L(i,i).
///
/// Math:
/// - No pivoting is performed. This can fail if a pivot is zero (or numerically tiny).
/// - This is suitable if your blocks are well-conditioned and you accept no pivoting.
fn lu_factor_inplace_6(mat: &mut [f32], n: usize) -> Result<(), String> {
    let zero = 0.0f32;
    let stride = 6usize;

    // We factor the leading n×n submatrix in-place.
    // For each k:
    //   - pivot = U(k,k) is mat[k,k]
    //   - compute multipliers L(i,k) = A(i,k)/pivot for i>k and store them into mat[i,k]
    //   - update trailing submatrix:
    //       A(i,j) -= L(i,k) * U(k,j)
    for k in 0..n {
        let a_kk = mat[k * stride + k];
        if a_kk == zero {
            return Err(format!("LU zero pivot at k = {}", k));
        }

        // Compute L(i,k) multipliers for rows below the pivot row.
        // These are stored in the strict lower triangle.
        for i in (k + 1)..n {
            mat[i * stride + k] /= a_kk;
        }

        // Trailing update for the submatrix (k+1..n, k+1..n).
        // Uses U(k,j) from the pivot row and L(i,k) from the column below diagonal.
        for i in (k + 1)..n {
            let l_ik = mat[i * stride + k];
            if l_ik != zero {
                for j in (k + 1)..n {
                    mat[i * stride + j] -= l_ik * mat[k * stride + j];
                }
            }
        }
    }

    Ok(())
}

fn build_lu_blocks_from_csr_block_starts_6(
    a: &CsrMatrix<f32>,
    block_starts: &[usize],
) -> Result<Vec<f32>, String> {
    let n = a.get_n_rows();

    // This builder is specialized for the Block-Jacobi WGSL kernel:
    //   BLOCK_SIZE = 6
    //   LU_STRIDE  = 36 (= 6 * 6)
    //
    // If you ever change BLOCK_SIZE in WGSL, this function must be updated
    // consistently (including LU_STRIDE and the factorization routine).
    let block_size = 6usize;

    if block_starts.len() < 2 {
        return Ok(vec![]);
    }

    let num_blocks = block_starts.len() - 1;

    // Output layout:
    //   out[block_id * 36 .. block_id * 36 + 36] =
    //       row-major 6x6 LU matrix for that block.
    //
    // Each block stores a full 6x6 matrix even if its effective size m < 6.
    let mut out = vec![0.0f32; num_blocks * 36];

    for block in 0..num_blocks {
        let offset = block_starts[block];
        let end = block_starts[block + 1];

        // Invalid or empty block range:
        // leave this block as all zeros / identity.
        if end > n || offset >= end {
            continue;
        }

        // m is the actual size of this block (m <= 6).
        // Only the leading m×m submatrix is meaningful.
        let m = (end - offset).min(block_size);

        // Local dense 6x6 buffer (row-major).
        let mut mat = [0.0f32; 36];

        // Initialize to identity.
        //
        // This is important:
        // - Missing entries in the CSR pattern should not make the block singular.
        // - If the diagonal is absent in CSR, we still get a safe pivot (=1.0).
        for i in 0..block_size {
            mat[i * block_size + i] = 1.0;
        }

        // Fill the leading m×m block from CSR.
        //
        // We only copy entries fully inside the block:
        //   global row i in [offset, offset + m)
        //   global col j in [offset, offset + m)
        for i_local in 0..m {
            let i = offset + i_local;

            let row_start = a.get_row_ptr()[i];
            let row_end = a.get_row_ptr()[i + 1];

            for idx in row_start..row_end {
                let j = a.get_col_index()[idx];
                if j >= offset && j < offset + m {
                    let j_local = j - offset;
                    mat[i_local * block_size + j_local] = a.get_values()[idx];
                }
            }
        }

        // In-place LU factorization of the leading m×m submatrix.
        //
        // CRITICAL STORAGE CONTRACT (must match block_jacobi.wgsl):
        //   - Strict lower triangle stores L(i,j), i > j
        //   - Diagonal and upper triangle store U(i,j), i <= j
        //   - L has an implicit unit diagonal (L(i,i) = 1.0, not stored)
        //
        // The WGSL kernel relies on this exact layout for:
        //   - forward solve: assumes unit-diagonal L
        //   - backward solve: divides by U(i,i)
        lu_factor_inplace_6(&mut mat, m)?;

        // Copy the full 6x6 slab into the output buffer.
        out[block * 36..block * 36 + 36].copy_from_slice(&mat);
    }

    Ok(out)
}

async fn pcg_block_jacobi_csr_webgpu(
    a: &CsrMatrix<f32>,
    b: &[f32],
    x: &mut [f32],
    max_iter: usize,
    rel_tol: f32,
    abs_tol: f32,
    block_starts: &[usize],
    ctx: &WebGpuCtx,
    spmv_exec: &SpmvExecutor,
    vec_ops_exec: &VecOpsExecutor,
    dot_scalar_exec: &DotScalarExecutor,
) -> Result<usize, JsValue> {
    // -------------------------------------------------------------------------
    // 0) Validate dimensions and precompute common constants
    // -------------------------------------------------------------------------
    let n = a.get_n_rows();
    if a.get_n_cols() != n {
        return Err(JsValue::from_str(
            "PCG(BlockJacobiGpu): matrix A is not square",
        ));
    }
    if b.len() != n || x.len() != n {
        return Err(JsValue::from_str(&format!(
            "PCG(BlockJacobiGpu): dimension mismatch: A is {}x{}, b len {}, x len {}",
            a.get_n_rows(),
            a.get_n_cols(),
            b.len(),
            x.len()
        )));
    }

    let zero: f32 = 0.0;
    let n_u32: u32 = n as u32;
    let n_bytes: u32 = (n * 4) as u32;

    // -------------------------------------------------------------------------
    // 1) CPU initialization (reference / cheap correctness path)
    //
    //    r0 = b - A*x0
    //    z0 = M^{-1} r0   (CPU Block-Jacobi)
    //    p0 = z0
    //    rz_old = r0^T z0
    // -------------------------------------------------------------------------
    let block_jacobi_preconditioner =
        BlockJacobiPreconditioner::create_from_csr_with_blocks(a, block_starts).map_err(|e| {
            JsValue::from_str(&format!(
                "PCG(BlockJacobiGpu): building preconditioner failed: {}",
                e
            ))
        })?;

    let ax0 = a
        .spmv(x)
        .map_err(|e| format!("PCG(BlockJacobiGpu): A*x failed: {}", e))?;

    let mut r_host = vec![zero; n];
    for i in 0..n {
        r_host[i] = b[i] - ax0[i];
    }

    let mut z_host = vec![zero; n];
    block_jacobi_preconditioner.apply(&r_host, &mut z_host)?;

    let p_host = z_host.clone();

    let mut rz_old: f32 = dot(&r_host, &z_host)?;

    // -------------------------------------------------------------------------
    // 2) Compute ||b||^2 once (GPU)
    //
    // Stopping condition uses squared norms:
    //   r_norm2 <= abs_tol^2  OR  r_norm2 <= rel_tol^2 * b_norm2
    // -------------------------------------------------------------------------
    let b_gpu = create_storage_buffer_f32_with_data(&ctx.device, &ctx.queue, b, "pcg b", 0)?;
    let b_norm2: f32 = {
        let command_encoder = ctx.device.create_command_encoder();

        // We can reuse any scalar slot for this one-time computation.
        dot_scalar_exec.encode_dot_scalar_into(ctx, &command_encoder, &b_gpu, &b_gpu, n_u32, 0)?;
        dot_scalar_exec.encode_copy_scalar_results_to_readback(&command_encoder)?;

        let command_buffer = command_encoder.finish();
        ctx.queue
            .submit(&[command_buffer].iter().collect::<Array>());

        let scalar_results = dot_scalar_exec.readback_scalar_results().await?;
        scalar_results[0]
    };

    if b_norm2 == zero {
        return Ok(0);
    }

    let rel_tol2: f32 = rel_tol * rel_tol;
    let abs_tol2: f32 = abs_tol * abs_tol;

    // -------------------------------------------------------------------------
    // 3) Upload initial vectors to GPU
    // -------------------------------------------------------------------------
    let x_gpu = create_storage_buffer_f32_with_data(
        &ctx.device,
        &ctx.queue,
        x,
        "pcg x",
        gpu_buffer_usage::COPY_SRC,
    )?;
    let r_gpu = create_storage_buffer_f32_with_data(
        &ctx.device,
        &ctx.queue,
        &r_host,
        "pcg r",
        gpu_buffer_usage::COPY_SRC,
    )?;
    let p_gpu = create_storage_buffer_f32_with_data(
        &ctx.device,
        &ctx.queue,
        &p_host,
        "pcg p",
        gpu_buffer_usage::COPY_SRC,
    )?;
    let z_gpu = create_storage_buffer_f32(&ctx.device, n, "pcg z", 0)?;

    let x_readback = create_readback_buffer(&ctx.device, n_bytes as usize, "pcg x readback")?;

    // -------------------------------------------------------------------------
    // 4) Build GPU Block-Jacobi preconditioner (persistent buffers) once
    // -------------------------------------------------------------------------
    let lu_blocks = build_lu_blocks_from_csr_block_starts_6(a, block_starts)?;
    let block_starts_u32: Vec<u32> = block_starts.iter().map(|&v| v as u32).collect();

    let block_jacobi_exec = BlockJacobiExecutor::create(ctx, n_u32, &lu_blocks, &block_starts_u32)?;

    // -------------------------------------------------------------------------
    // 5) Scalar slot layout (shared scalar_results buffer)
    //
    // IMPORTANT:
    //   DotScalarExecutor must be created with scalar_results_len > max index used here.
    //   With these indices, you need at least 7 slots (0..=6).
    // -------------------------------------------------------------------------
    let scalar_results_index_for_p_ap: u32 = 0; // p^T (A p)
    let scalar_results_index_for_r_norm2: u32 = 1; // r^T r
    let scalar_results_index_for_rz_new: u32 = 2; // r^T z (new)
    let scalar_results_index_for_rz_old: u32 = 3; // r^T z (old) written from CPU
    let scalar_results_index_for_alpha: u32 = 4; // alpha (GPU)
    let scalar_results_index_for_minus_alpha: u32 = 5; // -alpha (GPU)
    let scalar_results_index_for_beta: u32 = 6; // beta (GPU)

    // -------------------------------------------------------------------------
    // 6) Small GPU scalar kernel (computes alpha/-alpha/beta from dot products)
    // -------------------------------------------------------------------------
    let pcg_update_scalars_exec = PcgUpdateScalarsExecutor::create(ctx)?;

    // -------------------------------------------------------------------------
    // 7) Main PCG loop
    //
    // Design goal:
    //   - Encode all GPU work for the iteration into ONE command buffer.
    //   - Do ONE submit + ONE scalar readback per iteration.
    // -------------------------------------------------------------------------
    for k in 0..max_iter {
        let iterations = k + 1;

        let command_encoder = ctx.device.create_command_encoder();
        command_encoder.set_label("pcg single-submit iteration encoder");

        // ---------------------------------------------------------------------
        // A) Ap = A * p   (SpMV executor writes into spmv_exec.y_buffer()).
        // ---------------------------------------------------------------------
        spmv_exec.encode_copy_x_from(&command_encoder, &p_gpu, n_bytes)?;
        spmv_exec.encode_spmv(&command_encoder)?;

        // ---------------------------------------------------------------------
        // B) pAp = dot(p, Ap) -> scalar slot [p_ap]
        // ---------------------------------------------------------------------
        dot_scalar_exec.encode_dot_scalar_into(
            ctx,
            &command_encoder,
            &p_gpu,
            spmv_exec.y_buffer(),
            n_u32,
            scalar_results_index_for_p_ap,
        )?;

        // ---------------------------------------------------------------------
        // C) Write rz_old into scalar slot [rz_old] (needed for alpha/beta formulas)
        // ---------------------------------------------------------------------
        encode_write_f32_into_storage_buffer_at_index(
            &ctx.device,
            &ctx.queue,
            &command_encoder,
            dot_scalar_exec.scalar_results_buffer(),
            scalar_results_index_for_rz_old,
            rz_old,
            "pcg rz_old staging",
        )?;

        // ---------------------------------------------------------------------
        // D) Compute alpha and -alpha (beta not meaningful yet; rz_new not computed)
        // ---------------------------------------------------------------------
        pcg_update_scalars_exec.encode_update_scalars(
            ctx,
            &command_encoder,
            dot_scalar_exec.scalar_results_buffer(),
            scalar_results_index_for_p_ap,
            scalar_results_index_for_rz_new, // placeholder for this first pass
            scalar_results_index_for_rz_old,
            scalar_results_index_for_alpha,
            scalar_results_index_for_minus_alpha,
            scalar_results_index_for_beta,
        )?;

        // ---------------------------------------------------------------------
        // E) x = x + alpha * p
        //    r = r + (-alpha) * Ap
        // ---------------------------------------------------------------------
        vec_ops_exec.encode_axpy_inplace_from_scalar_results(
            ctx,
            &command_encoder,
            &p_gpu,
            &x_gpu,
            n_u32,
            dot_scalar_exec.scalar_results_buffer(),
            scalar_results_index_for_alpha,
        )?;
        vec_ops_exec.encode_axpy_inplace_from_scalar_results(
            ctx,
            &command_encoder,
            spmv_exec.y_buffer(),
            &r_gpu,
            n_u32,
            dot_scalar_exec.scalar_results_buffer(),
            scalar_results_index_for_minus_alpha,
        )?;

        // ---------------------------------------------------------------------
        // F) r_norm2 = dot(r, r) -> scalar slot [r_norm2]
        // ---------------------------------------------------------------------
        dot_scalar_exec.encode_dot_scalar_into(
            ctx,
            &command_encoder,
            &r_gpu,
            &r_gpu,
            n_u32,
            scalar_results_index_for_r_norm2,
        )?;

        // ---------------------------------------------------------------------
        // G) z = M^{-1} r  (Block-Jacobi apply)
        // ---------------------------------------------------------------------
        block_jacobi_exec.encode_apply(ctx, &command_encoder, &r_gpu, &z_gpu)?;

        // ---------------------------------------------------------------------
        // H) rz_new = dot(r, z) -> scalar slot [rz_new]
        // ---------------------------------------------------------------------
        dot_scalar_exec.encode_dot_scalar_into(
            ctx,
            &command_encoder,
            &r_gpu,
            &z_gpu,
            n_u32,
            scalar_results_index_for_rz_new,
        )?;

        // ---------------------------------------------------------------------
        // I) Now rz_new exists -> compute beta into scalar slot [beta]
        // ---------------------------------------------------------------------
        pcg_update_scalars_exec.encode_update_scalars(
            ctx,
            &command_encoder,
            dot_scalar_exec.scalar_results_buffer(),
            scalar_results_index_for_p_ap,
            scalar_results_index_for_rz_new,
            scalar_results_index_for_rz_old,
            scalar_results_index_for_alpha,
            scalar_results_index_for_minus_alpha,
            scalar_results_index_for_beta,
        )?;

        // ---------------------------------------------------------------------
        // J) p = z + beta * p
        // ---------------------------------------------------------------------
        vec_ops_exec.encode_scale_inplace_from_scalar_results(
            ctx,
            &command_encoder,
            &p_gpu,
            n_u32,
            dot_scalar_exec.scalar_results_buffer(),
            scalar_results_index_for_beta,
        )?;
        vec_ops_exec.encode_axpy_inplace(ctx, &command_encoder, &z_gpu, &p_gpu, n_u32, 1.0)?;

        // ---------------------------------------------------------------------
        // K) Copy scalar results buffer -> CPU readback buffer (once per iteration)
        // ---------------------------------------------------------------------
        dot_scalar_exec.encode_copy_scalar_results_to_readback(&command_encoder)?;

        // Submit once.
        let command_buffer = command_encoder.finish();
        ctx.queue
            .submit(&[command_buffer].iter().collect::<Array>());

        // Read all scalar slots back once.
        let scalar_results = dot_scalar_exec.readback_scalar_results().await?;

        let p_ap = scalar_results[scalar_results_index_for_p_ap as usize];
        let r_norm2 = scalar_results[scalar_results_index_for_r_norm2 as usize];
        let rz_new = scalar_results[scalar_results_index_for_rz_new as usize];

        // Cheap explicit breakdown checks.
        if p_ap == zero {
            return Err(JsValue::from_str(
                "PCG(BlockJacobiGpu): dot(p,Ap) is zero (breakdown)",
            ));
        }
        if rz_old == zero {
            return Err(JsValue::from_str(
                "PCG(BlockJacobiGpu): rz_old is zero (breakdown)",
            ));
        }

        // Stopping condition (squared norms, CPU).
        if r_norm2 <= abs_tol2 || r_norm2 <= rel_tol2 * b_norm2 {
            // Read x back once on convergence.
            let x_host = {
                let command_encoder = ctx.device.create_command_encoder();
                command_encoder.copy_buffer_to_buffer_with_u32_and_u32_and_u32(
                    &x_gpu,
                    0,
                    &x_readback,
                    0,
                    n_bytes,
                )?;
                let command_buffer = command_encoder.finish();
                ctx.queue
                    .submit(&[command_buffer].iter().collect::<Array>());
                read_back_f32(&x_readback, n).await?
            };

            x.copy_from_slice(&x_host);
            return Ok(iterations);
        }

        rz_old = rz_new;
    }

    Err(JsValue::from_str(&format!(
        "PCG(BlockJacobiGpu): did not converge in {} iterations",
        max_iter
    )))
}

pub(crate) async fn find_ua_vector_iterative_pcg_block_jacobi_sparse_webgpu(
    separated_stiffness_matrix_sparse: &SeparatedStiffnessMatrixSparse<f32>,
    r_a_vector: &Vector<f32>,
    u_b_vector: &Vector<f32>,
    max_iter: usize,
    ctx: &WebGpuCtx,
) -> Result<(Vector<f32>, usize), JsValue> {
    log("block jacobi sparse webgpu");

    // 1) build b (CPU)
    let b_values = find_b_sparse_webgpu(
        r_a_vector,
        separated_stiffness_matrix_sparse.get_k_ab_triplets(),
        u_b_vector,
        separated_stiffness_matrix_sparse.get_n_aa(),
    )?;

    // 2) build CSR on CPU
    let csr = CsrMatrix::from_coo(
        separated_stiffness_matrix_sparse.get_n_aa(),
        separated_stiffness_matrix_sparse.get_n_aa(),
        separated_stiffness_matrix_sparse.get_k_aa_triplets(),
    )
    .map_err(|e| JsValue::from_str(&format!("CSR from COO failed: {}", e)))?;

    // 3) extract CSR arrays for GPU
    let row_ptr_u32: Vec<u32> = csr.get_row_ptr().iter().map(|&v| v as u32).collect();
    let col_idx_u32: Vec<u32> = csr.get_col_index().iter().map(|&v| v as u32).collect();
    let values_f32: Vec<f32> = csr.get_values().to_vec();

    // 4) create SpmvExecutor once
    let spmv_exec = SpmvExecutor::create(
        ctx,
        csr.get_n_rows() as u32,
        &row_ptr_u32,
        &col_idx_u32,
        &values_f32,
    )?;

    // 5) create VecOpsExecutor once
    let vec_ops_exec = VecOpsExecutor::create(ctx)?;

    let n = b_values.len();

    // 6) create DotScalarExecutor once
    let dot_scalar_exec = DotScalarExecutor::create(ctx, n, 7)?;

    let block_starts = build_block_starts_from_k_aa_indexes_webgpu(
        separated_stiffness_matrix_sparse.get_k_aa_indexes(),
        n,
    );
    let mut u_a_values = vec![0.0f32; n];

    let x0_f32 = u_a_values.clone();

    let iterations = pcg_block_jacobi_csr_webgpu(
        &csr,
        &b_values,
        &mut u_a_values,
        max_iter,
        REL_TOL,
        ABS_TOL,
        &block_starts,
        ctx,
        &spmv_exec,
        &vec_ops_exec,
        &dot_scalar_exec,
    )
    .await
    .map_err(|e| JsValue::from_str(&format!("PCG(BlockJacobiGpu) failed: {:?}", e)))?;

    let export_text = make_pcg_block_jacobi_csr_input_bundle_text(
        csr.get_n_rows(),
        csr.get_values().len(),
        &row_ptr_u32,
        &col_idx_u32,
        &values_f32,
        &b_values,
        &x0_f32,
        Some(&u_a_values),
        &block_starts,
    );

    // Send to main thread
    send_export_to_main_thread("pcg_block_jacobi_dataset.txt", &export_text)?;

    Ok((Vector::create(&u_a_values), iterations))
}
