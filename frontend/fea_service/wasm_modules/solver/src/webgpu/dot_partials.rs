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

/// Pipeline wrapper for `dot_partials.wgsl`.
///
/// The shader computes one partial dot-product sum per workgroup:
///   partial[wg_id.x] = sum_{i in workgroup chunk} a[i] * b[i]
///
/// This is the first stage of `dot(a,b)` computation.
/// A separate reduction stage (`dot_reduce.wgsl`) is used to turn partials into a single scalar.
pub struct DotPartialsPipeline {
    pub pipeline: GpuComputePipeline,
    pub dot_partials_bind_group_layout: GpuBindGroupLayout,
}

/// Create the compute pipeline + bind group layout for `dot_partials.wgsl`.
///
/// WGSL bindings (group=0):
///   binding(0): uniform Params { n: u32, ... }
///   binding(1): storage, read      a: array<f32>
///   binding(2): storage, read      b: array<f32>
///   binding(3): storage, read_write partial: array<f32>
pub fn create_dot_partials_pipeline(ctx: &WebGpuCtx) -> Result<DotPartialsPipeline, JsValue> {
    let device: &GpuDevice = &ctx.device;

    // Load WGSL source and create shader module.
    let code = include_str!("./wgsl/dot_partials.wgsl");
    let shader_module_desc = GpuShaderModuleDescriptor::new(code);
    shader_module_desc.set_label("dot_partials.wgsl");
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

    // binding(1): a (storage, read)
    let read_only_layout = GpuBufferBindingLayout::new();
    read_only_layout.set_type(GpuBufferBindingType::ReadOnlyStorage);
    let bind_group_layout_0_entry_1 = GpuBindGroupLayoutEntry::new(1, gpu_shader_stage::COMPUTE);
    bind_group_layout_0_entry_1.set_buffer(&read_only_layout);

    // binding(2): b (storage, read)
    let bind_group_layout_0_entry_2 = GpuBindGroupLayoutEntry::new(2, gpu_shader_stage::COMPUTE);
    bind_group_layout_0_entry_2.set_buffer(&read_only_layout);

    // binding(3): partial output (storage, read_write)
    let read_write_layout = GpuBufferBindingLayout::new();
    read_write_layout.set_type(GpuBufferBindingType::Storage);
    let bind_group_layout_0_entry_3 = GpuBindGroupLayoutEntry::new(3, gpu_shader_stage::COMPUTE);
    bind_group_layout_0_entry_3.set_buffer(&read_write_layout);

    let bind_group_layout_0_entries = [
        &bind_group_layout_0_entry_0,
        &bind_group_layout_0_entry_1,
        &bind_group_layout_0_entry_2,
        &bind_group_layout_0_entry_3,
    ]
    .iter()
    .collect::<Array>();

    let dot_partials_bind_group_layout_desc =
        GpuBindGroupLayoutDescriptor::new(&bind_group_layout_0_entries);
    dot_partials_bind_group_layout_desc.set_label("dot_partials bgl0");

    let dot_partials_bind_group_layout =
        device.create_bind_group_layout(&dot_partials_bind_group_layout_desc)?;

    // -----------------------------
    // Pipeline layout + pipeline
    // -----------------------------
    let bind_group_layouts = [&dot_partials_bind_group_layout].iter().collect::<Array>();
    let pipeline_layout_desc = GpuPipelineLayoutDescriptor::new(&bind_group_layouts);
    pipeline_layout_desc.set_label("dot_partials pipeline layout");

    let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_desc);

    let compute_pipeline_desc =
        GpuComputePipelineDescriptor::new(&pipeline_layout, &programmable_stage);
    compute_pipeline_desc.set_label("dot_partials pipeline");

    let pipeline = device.create_compute_pipeline(&compute_pipeline_desc);

    Ok(DotPartialsPipeline {
        pipeline,
        dot_partials_bind_group_layout,
    })
}

/// Create bind group for one dot-partials invocation.
///
/// Buffers must match WGSL bindings (group=0):
///   binding(0): params_buffer  (uniform)
///   binding(1): a_buffer       (storage, read)
///   binding(2): b_buffer       (storage, read)
///   binding(3): partial_buffer (storage, read_write)
pub fn create_dot_partials_bind_group(
    device: &GpuDevice,
    dot_partials_bind_group_layout: &GpuBindGroupLayout,
    params_buffer: &GpuBuffer,
    a_buffer: &GpuBuffer,
    b_buffer: &GpuBuffer,
    partial_buffer: &GpuBuffer,
) -> GpuBindGroup {
    let bind_group_0_entry_0 = GpuBindGroupEntry::new(0, &GpuBufferBinding::new(params_buffer));
    let bind_group_0_entry_1 = GpuBindGroupEntry::new(1, &GpuBufferBinding::new(a_buffer));
    let bind_group_0_entry_2 = GpuBindGroupEntry::new(2, &GpuBufferBinding::new(b_buffer));
    let bind_group_0_entry_3 = GpuBindGroupEntry::new(3, &GpuBufferBinding::new(partial_buffer));

    let bind_group_0_entries = [
        &bind_group_0_entry_0,
        &bind_group_0_entry_1,
        &bind_group_0_entry_2,
        &bind_group_0_entry_3,
    ]
    .iter()
    .collect::<Array>();

    let dot_partials_bind_group_desc =
        GpuBindGroupDescriptor::new(&bind_group_0_entries, dot_partials_bind_group_layout);
    dot_partials_bind_group_desc.set_label("dot_partials bind group 0");

    device.create_bind_group(&dot_partials_bind_group_desc)
}
