use js_sys::Array;
use wasm_bindgen::JsValue;
use web_sys::gpu_shader_stage;
use web_sys::{
    GpuBindGroup, GpuBindGroupDescriptor, GpuBindGroupEntry, GpuBindGroupLayout,
    GpuBindGroupLayoutDescriptor, GpuBindGroupLayoutEntry, GpuBuffer, GpuBufferBinding,
    GpuBufferBindingLayout, GpuBufferBindingType, GpuComputePipeline, GpuComputePipelineDescriptor,
    GpuDevice, GpuPipelineLayoutDescriptor, GpuProgrammableStage, GpuShaderModuleDescriptor,
};

use crate::webgpu::ctx::WebGpuCtx;

/// Pipeline wrapper for `dot_reduce.wgsl`.
///
/// This shader reduces an input array into a smaller output array by summing chunks:
///   output[wg_id.x] = sum_{j in chunk} input[j]
///
/// It is used iteratively to reduce partial sums (from `dot_partials.wgsl`) down to one scalar.
pub struct DotReducePipeline {
    pub pipeline: GpuComputePipeline,
    pub dot_reduce_bind_group_layout: GpuBindGroupLayout,
}

/// Create the compute pipeline + bind group layout for `dot_reduce.wgsl`.
///
/// WGSL bindings (group=0):
///   binding(0): uniform Params { n: u32, ... }
///   binding(1): storage, read      input: array<f32>
///   binding(2): storage, read_write output: array<f32>
pub fn create_dot_reduce_pipeline(ctx: &WebGpuCtx) -> Result<DotReducePipeline, JsValue> {
    let device: &GpuDevice = &ctx.device;

    // Load WGSL source and create shader module.
    let code = include_str!("./wgsl/dot_reduce.wgsl");
    let shader_module_desc = GpuShaderModuleDescriptor::new(code);
    shader_module_desc.set_label("dot_reduce.wgsl");
    let shader_module = device.create_shader_module(&shader_module_desc);

    // Entry point must match WGSL: `@compute fn compute_main(...)`.
    let programmable_stage = GpuProgrammableStage::new(&shader_module);
    programmable_stage.set_entry_point("compute_main");

    // -----------------------------
    // Bind group layout (group 0)
    // -----------------------------

    // binding(0): uniform Params
    let uniform_layout = GpuBufferBindingLayout::new();
    uniform_layout.set_type(GpuBufferBindingType::Uniform);
    uniform_layout.set_has_dynamic_offset(false);
    let bind_group_layout_0_entry_0 = GpuBindGroupLayoutEntry::new(0, gpu_shader_stage::COMPUTE);
    bind_group_layout_0_entry_0.set_buffer(&uniform_layout);

    // binding(1): input (storage, read)
    let read_only_layout = GpuBufferBindingLayout::new();
    read_only_layout.set_type(GpuBufferBindingType::ReadOnlyStorage);
    let bind_group_layout_0_entry_1 = GpuBindGroupLayoutEntry::new(1, gpu_shader_stage::COMPUTE);
    bind_group_layout_0_entry_1.set_buffer(&read_only_layout);

    // binding(2): output (storage, read_write)
    let read_write_layout = GpuBufferBindingLayout::new();
    read_write_layout.set_type(GpuBufferBindingType::Storage);
    let bind_group_layout_0_entry_2 = GpuBindGroupLayoutEntry::new(2, gpu_shader_stage::COMPUTE);
    bind_group_layout_0_entry_2.set_buffer(&read_write_layout);

    let bind_group_layout_0_entries = [
        &bind_group_layout_0_entry_0,
        &bind_group_layout_0_entry_1,
        &bind_group_layout_0_entry_2,
    ]
    .iter()
    .collect::<Array>();

    let dot_reduce_bind_group_layout_desc =
        GpuBindGroupLayoutDescriptor::new(&bind_group_layout_0_entries);
    dot_reduce_bind_group_layout_desc.set_label("dot_reduce bgl0");

    let dot_reduce_bind_group_layout =
        device.create_bind_group_layout(&dot_reduce_bind_group_layout_desc)?;

    // -----------------------------
    // Pipeline layout + pipeline
    // -----------------------------
    let bind_group_layouts = [&dot_reduce_bind_group_layout].iter().collect::<Array>();
    let pipeline_layout_desc = GpuPipelineLayoutDescriptor::new(&bind_group_layouts);
    pipeline_layout_desc.set_label("dot_reduce pipeline layout");

    let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_desc);

    let compute_pipeline_desc =
        GpuComputePipelineDescriptor::new(&pipeline_layout, &programmable_stage);
    compute_pipeline_desc.set_label("dot_reduce pipeline");

    let pipeline = device.create_compute_pipeline(&compute_pipeline_desc);

    Ok(DotReducePipeline {
        pipeline,
        dot_reduce_bind_group_layout,
    })
}

/// Create bind group for one dot-reduce invocation.
///
/// Buffers must match WGSL bindings (group=0):
///   binding(0): params_buffer  (uniform)
///   binding(1): input_buffer   (storage, read)
///   binding(2): output_buffer  (storage, read_write)
pub fn create_dot_reduce_bind_group(
    device: &GpuDevice,
    dot_reduce_bind_group_layout: &GpuBindGroupLayout,
    params_buffer: &GpuBuffer,
    input_buffer: &GpuBuffer,
    output_buffer: &GpuBuffer,
) -> GpuBindGroup {
    let bind_group_0_entry_0 = GpuBindGroupEntry::new(0, &GpuBufferBinding::new(params_buffer));
    let bind_group_0_entry_1 = GpuBindGroupEntry::new(1, &GpuBufferBinding::new(input_buffer));
    let bind_group_0_entry_2 = GpuBindGroupEntry::new(2, &GpuBufferBinding::new(output_buffer));

    let bind_group_0_entries = [
        &bind_group_0_entry_0,
        &bind_group_0_entry_1,
        &bind_group_0_entry_2,
    ]
    .iter()
    .collect::<Array>();

    let dot_reduce_bind_group_desc =
        GpuBindGroupDescriptor::new(&bind_group_0_entries, dot_reduce_bind_group_layout);
    dot_reduce_bind_group_desc.set_label("dot_reduce bind group 0");

    device.create_bind_group(&dot_reduce_bind_group_desc)
}
