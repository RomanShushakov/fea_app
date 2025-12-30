// Purpose:
//   Compute partial sums for a dot product:
//
//     dot(a,b) = sum_i a[i] * b[i]
//
//   This kernel does NOT produce the final scalar. Instead it outputs one partial sum
//   per workgroup into `partial[workgroup_id.x]`.
//
// Dispatch convention:
//   - @workgroup_size(256)
//   - dispatch_workgroups(groups_x) where groups_x = ceil(n / 256)
//   - global_invocation_id.x maps 1:1 to input index i
//
// Output:
//   partial[k] holds the sum over indices i in [k*256, k*256+255], with bounds checking.

struct Params {
    n: u32,     // length of vectors a and b
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
};

@group(0) @binding(0) var<uniform> params: Params;

// Input vectors (length >= params.n)
@group(0) @binding(1) var<storage, read> a: array<f32>;
@group(0) @binding(2) var<storage, read> b: array<f32>;

// Output partial sums:
//   partial[wg_id.x] = sum over this workgroup's chunk
@group(0) @binding(3) var<storage, read_write> partial: array<f32>;

// Workgroup shared memory for reduction. One element per thread.
var<workgroup> shared_memory: array<f32, 256>;

@compute @workgroup_size(256)
fn compute_main(
    @builtin(local_invocation_id) li_id: vec3<u32>,
    @builtin(global_invocation_id) gi_id: vec3<u32>,
    @builtin(workgroup_id) wg_id: vec3<u32>,
) {
    let thread_id: u32 = li_id.x;
    let i: u32 = gi_id.x;

    // 1) Load product into shared memory with bounds check.
    //    Threads with i >= n contribute 0.0.
    var v: f32 = 0.0;
    if (i < params.n) {
        v = a[i] * b[i];
    }
    shared_memory[thread_id] = v;
    workgroupBarrier();

    // 2) Reduce shared_memory[0..255] to shared_memory[0] via tree reduction.
    //    After each step, barrier ensures writes are visible to other threads.
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

    // 3) Thread 0 writes this workgroup's partial sum.
    if (thread_id == 0u) {
        partial[wg_id.x] = shared_memory[0];
    }
}
