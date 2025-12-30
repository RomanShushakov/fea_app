// Purpose:
//   Reduce an input array into a smaller output array by summing chunks of 256.
//
//   This is used as a generic "reduce-by-sum" step, typically after dot_partials,
//   and may be invoked repeatedly until only one element remains.
//
// Dispatch convention:
//   - @workgroup_size(256)
//   - dispatch_workgroups(out_len) where out_len = ceil(n / 256)
//
// Mapping:
//   - workgroup k handles input indices [k*256, k*256+255]
//   - output[k] = sum of those inputs (bounds checked)
//
// Output:
//   output length must be >= ceil(n / 256)

struct Params {
    n: u32,     // number of valid elements in `input` for THIS reduction pass
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
};

@group(0) @binding(0) var<uniform> params: Params;

// Input array (length >= params.n)
@group(0) @binding(1) var<storage, read> input: array<f32>;

// Output reduced array (length >= ceil(params.n / 256))
@group(0) @binding(2) var<storage, read_write> output: array<f32>;

// Shared memory reduction scratch.
var<workgroup> shared_memory: array<f32, 256>;

@compute @workgroup_size(256)
fn compute_main(
    @builtin(local_invocation_id) li_id: vec3<u32>,
    @builtin(workgroup_id) wg_id: vec3<u32>,
) {
    let thread_id: u32 = li_id.x;

    // This workgroup's base index into the input.
    let base: u32 = wg_id.x * 256u;
    let idx: u32 = base + thread_id;

    // 1) Load input[idx] into shared memory, bounds checked.
    var v: f32 = 0.0;
    if (idx < params.n) {
        v = input[idx];
    }
    shared_memory[thread_id] = v;
    workgroupBarrier();

    // 2) Reduce shared_memory[0..255] to shared_memory[0].
    var offset: u32 = 128u;
    loop {
        if (thread_id < offset) {
            shared_memory[thread_id] =
                shared_memory[thread_id] + shared_memory[thread_id + offset];
        }
        workgroupBarrier();

        if (offset == 1u) {
            break;
        }
        offset = offset / 2u;
    }

    // 3) Thread 0 writes the reduced sum for this workgroup.
    if (thread_id == 0u) {
        output[wg_id.x] = shared_memory[0];
    }
}
