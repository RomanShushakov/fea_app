// Block-Jacobi preconditioner apply (GPU)
//
// This kernel applies a block-diagonal preconditioner M^{-1} to an input vector r:
//
//     z = M^{-1} r
//
// Each block is solved independently using precomputed LU factors stored for that block:
//
//     (L * U) x = r_block
//
// implemented as:
//     forward substitution:  L y = r_block
//     backward substitution: U x = y
//
// Work mapping:
//   - 1 workgroup per block (workgroup_size = 1)
//   - workgroup_id.x == block_id
//
// Block partitioning:
//   - block_starts has length (num_blocks + 1)
//   - block_id covers indices in [offset, next):
//       offset = block_starts[block_id]
//       next   = block_starts[block_id + 1]
//
// Data layout contract (CPU ↔ GPU):
//   - BLOCK_SIZE is compile-time fixed (here 6)
//   - lu_blocks packs one dense 6x6 matrix per block, row-major
//   - For a short final block (m < 6), only the leading m×m portion is used
//
// LU storage contract (must match CPU builder):
//   - Strict lower triangle stores L(i,j) for i > j
//   - Diagonal and upper triangle store U(i,j) for i <= j
//   - L has an implicit unit diagonal (L(i,i) == 1), i.e. it is NOT stored
//
// IMPORTANT:
//   - No pivoting is performed here.
//   - U(i,i) must be non-zero; otherwise results become Inf/NaN.

struct Params {
    n: u32,          // full vector length
    num_blocks: u32, // == block_starts.len - 1
    _pad1: u32,
    _pad2: u32,
};

@group(0) @binding(0) var<uniform> params: Params;

// Packed LU factors: num_blocks * (6*6) floats (row-major per block).
@group(0) @binding(1) var<storage, read> lu_blocks: array<f32>;

// Block boundaries: length == num_blocks + 1.
@group(0) @binding(2) var<storage, read> block_starts: array<u32>;

// Input vector r (length n).
@group(0) @binding(3) var<storage, read> r: array<f32>;

// Output vector z (length n).
@group(0) @binding(4) var<storage, read_write> z: array<f32>;

const BLOCK_SIZE: u32 = 6u;
const LU_STRIDE: u32 = 36u; // 6*6

@compute @workgroup_size(1)
fn compute_main(@builtin(workgroup_id) wg_id: vec3<u32>) {
    let block_id: u32 = wg_id.x;
    if (block_id >= params.num_blocks) {
        return;
    }

    let offset: u32 = block_starts[block_id];
    let next: u32 = block_starts[block_id + 1u];

    // Defensive checks against malformed block_starts.
    if (offset >= params.n || next <= offset) {
        return;
    }

    // Effective block size (<= 6).
    let m: u32 = min(BLOCK_SIZE, next - offset);

    // Base index into lu_blocks for this block.
    let base: u32 = block_id * LU_STRIDE;

    // Fixed-size temporaries (only [0..m) are used).
    var y: array<f32, 6>;
    var x: array<f32, 6>;

    for (var i: u32 = 0u; i < 6u; i = i + 1u) {
        y[i] = 0.0;
        x[i] = 0.0;
    }

    // Forward solve: L y = r_block (unit diagonal).
    for (var i: u32 = 0u; i < m; i = i + 1u) {
        var sum: f32 = r[offset + i];
        for (var j: u32 = 0u; j < i; j = j + 1u) {
            let l_ij: f32 = lu_blocks[base + i * 6u + j];
            sum = sum - l_ij * y[j];
        }
        y[i] = sum;
    }

    // Backward solve: U x = y.
    var ii: i32 = i32(m) - 1;
    loop {
        if (ii < 0) {
            break;
        }

        let i: u32 = u32(ii);
        var sum: f32 = y[i];

        for (var j: u32 = i + 1u; j < m; j = j + 1u) {
            let u_ij: f32 = lu_blocks[base + i * 6u + j];
            sum = sum - u_ij * x[j];
        }

        let u_ii: f32 = lu_blocks[base + i * 6u + i];
        x[i] = sum / u_ii;

        ii = ii - 1;
    }

    for (var i: u32 = 0u; i < m; i = i + 1u) {
        z[offset + i] = x[i];
    }
}
