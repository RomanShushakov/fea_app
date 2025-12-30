// This pass computes the PCG scalar coefficients (alpha, -alpha, beta)
// using dot-products that were already produced earlier in the iteration.
//
// It reads values from `scalar_results` using indices provided via `Params`,
// then writes the computed scalars back into `scalar_results` (also via indices).
//
// Typical PCG meaning (for iteration k):
//   p_ap  = p_k^T (A p_k)
//   rz_old = r_k^T z_k
//   rz_new = r_{k+1}^T z_{k+1}
//
// Scalars computed:
//   alpha = rz_old / p_ap
//   beta  = rz_new / rz_old
//
// Important note about "single-submit PCG":
//   - In the iteration encoder, p_ap is available before r/z are updated.
//   - rz_new becomes available only after the preconditioner + dot(r,z).
//   Therefore this shader is often executed twice per iteration:
//     1) early: compute alpha and -alpha (beta may be 0/unused at that moment)
//     2) late : compute beta (alpha may be recomputed identically; harmless)
//
// This is a 1-thread compute pass: the work is tiny (just a few loads/divides/stores).

struct Params {
    // Indices into scalar_results[] for INPUT values
    p_ap_index: u32,           // scalar_results[p_ap_index]  = pAp
    rz_new_index: u32,         // scalar_results[rz_new_index] = rzNew
    rz_old_index: u32,         // scalar_results[rz_old_index] = rzOld

    // Indices into scalar_results[] for OUTPUT values
    alpha_index: u32,          // scalar_results[alpha_index] = alpha
    minus_alpha_index: u32,    // scalar_results[minus_alpha_index] = -alpha
    beta_index: u32,           // scalar_results[beta_index] = beta

    // Padding to keep the uniform buffer 16-byte aligned (matches Rust-side u32[8])
    _pad0: u32,
    _pad1: u32,
};

@group(0) @binding(0) var<uniform> params: Params;

// One shared scalar "mailbox" buffer for all scalar values in the solver.
// We both read and write into it in this shader.
@group(0) @binding(1) var<storage, read_write> scalar_results: array<f32>;

@compute @workgroup_size(1)
fn compute_main() {
    // Load dot-products needed for PCG scalar formulas.
    let p_ap: f32  = scalar_results[params.p_ap_index];
    let rz_new: f32 = scalar_results[params.rz_new_index];
    let rz_old: f32 = scalar_results[params.rz_old_index];

    // Defensive defaults: if denominators are zero, keep outputs finite (0.0).
    // CPU-side code still performs breakdown checks after readback.
    var alpha: f32 = 0.0;
    var beta: f32  = 0.0;

    // alpha = rz_old / p_ap
    if (p_ap != 0.0) {
        alpha = rz_old / p_ap;
    }

    // beta = rz_new / rz_old
    if (rz_old != 0.0) {
        beta = rz_new / rz_old;
    }

    // Store results back into the scalar_results "mailbox".
    scalar_results[params.alpha_index] = alpha;
    scalar_results[params.minus_alpha_index] = -alpha;
    scalar_results[params.beta_index] = beta;
}
