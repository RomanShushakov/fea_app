// CSR SpMV (Sparse Matrix-Vector multiply):
//   y = A * x
//
// Matrix storage: Compressed Sparse Row (CSR)
//   row_ptr: length = n_rows + 1
//   col_idx: length = nnz
//   values : length = nnz
//
// Each invocation computes one output row i:
//
//   start = row_ptr[i]
//   end   = row_ptr[i + 1]
//   sum   = Σ_{k=start..end-1} values[k] * x[col_idx[k]]
//   y[i]  = sum
//
// Bindings (group 0):
//   binding(0): uniform Params { n_rows }
//   binding(1): row_ptr  (u32) read-only storage
//   binding(2): col_idx  (u32) read-only storage
//   binding(3): values   (f32) read-only storage
//   binding(4): x        (f32) read-only storage
//   binding(5): y        (f32) read-write storage
//
// Notes:
// - This is the straightforward "one thread per row" CSR SpMV.
// - Performance depends heavily on row length distribution.
// - Workgroup size is 256; global_invocation_id.x selects the row.

struct Params {
    n_rows: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
};

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> row_ptr: array<u32>;
@group(0) @binding(2) var<storage, read> col_idx: array<u32>;
@group(0) @binding(3) var<storage, read> values: array<f32>;
@group(0) @binding(4) var<storage, read> x: array<f32>;
@group(0) @binding(5) var<storage, read_write> y: array<f32>;

@compute @workgroup_size(256)
fn compute_main(@builtin(global_invocation_id) gi_id: vec3<u32>) {
    let i = gi_id.x;

    // Guard against extra threads in the last workgroup.
    if (i >= params.n_rows) {
        return;
    }

    let start = row_ptr[i];
    let end = row_ptr[i + 1u];

    var sum: f32 = 0.0;

    // Iterate over the non-zeros of row i.
    for (var k = start; k < end; k = k + 1u) {
        let j = col_idx[k];
        sum = sum + values[k] * x[j];
    }

    y[i] = sum;
}
