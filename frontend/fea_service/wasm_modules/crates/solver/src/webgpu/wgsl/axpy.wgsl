// AXPY kernel (immediate scalar):
//   y[i] = y[i] + alpha * x[i]
//
// Bindings (group 0):
//   binding(0): uniform Params  (n, alpha)
//   binding(1): x  read-only storage buffer
//   binding(2): y  read-write storage buffer
//
// Notes:
// - We keep padding fields so the Params struct is safely aligned for uniform
//   buffer layout (and matches the Rust-side write_u32 packing you use).
// - Workgroup size is 256; each invocation handles one element.

struct Params {
    // Number of elements in x/y.
    n: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,

    // Scalar multiplier.
    alpha: f32,
    _pad3: u32,
    _pad4: u32,
    _pad5: u32,
};

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> x: array<f32>;
@group(0) @binding(2) var<storage, read_write> y: array<f32>;

@compute @workgroup_size(256)
fn compute_main(@builtin(global_invocation_id) gi_id: vec3<u32>) {
    let i = gi_id.x;

    // Guard against extra threads in the last workgroup.
    if (i >= params.n) {
        return;
    }

    y[i] = y[i] + params.alpha * x[i];
}
