// SCALE kernel (scalar read from scalar_results buffer):
//   x[i] = x[i] * scalar_results[scalar_index]
//
// Bindings (group 0):
//   binding(0): uniform Params
//   binding(1): x              read-write storage buffer
//   binding(2): scalar_results read-only storage buffer
//
// Uniform layout (16 bytes):
//   u32 n            @ offset 0
//   u32 scalar_index @ offset 4
//   u32 _pad0        @ offset 8
//   u32 _pad1        @ offset 12

struct Params {
    n: u32,
    scalar_index: u32,
    _pad0: u32,
    _pad1: u32,
};

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read_write> x: array<f32>;
@group(0) @binding(2) var<storage, read> scalar_results: array<f32>;

@compute @workgroup_size(256)
fn compute_main(@builtin(global_invocation_id) gi_id: vec3<u32>) {
    let i = gi_id.x;

    // Guard against extra threads in the last workgroup.
    if (i >= params.n) {
        return;
    }

    let beta: f32 = scalar_results[params.scalar_index];

    x[i] = x[i] * beta;
}
