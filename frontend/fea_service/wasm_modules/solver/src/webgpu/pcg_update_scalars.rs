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

/// Pipeline wrapper for `pcg_update_scalars.wgsl`.
///
/// This shader is a tiny "scalar math" pass used inside the PCG loop:
/// it reads dot-products (pAp, rz_old, rz_new) from `scalar_results_buffer`,
/// computes alpha, -alpha, beta, and writes them back into the same buffer.
///
/// It is typically dispatched with exactly 1 workgroup (workgroup_size(1)).
pub struct PcgUpdateScalarsPipeline {
    pub pipeline: GpuComputePipeline,
    pub pcg_update_scalars_bind_group_layout: GpuBindGroupLayout,
}

fn create_uniform_entry(binding: u32) -> GpuBindGroupLayoutEntry {
    let buffer_binding_layout = GpuBufferBindingLayout::new();
    buffer_binding_layout.set_type(GpuBufferBindingType::Uniform);
    buffer_binding_layout.set_has_dynamic_offset(false);

    let bind_group_layout_entry = GpuBindGroupLayoutEntry::new(binding, gpu_shader_stage::COMPUTE);
    bind_group_layout_entry.set_buffer(&buffer_binding_layout);
    bind_group_layout_entry
}

fn create_storage_entry(binding: u32, is_read_only: bool) -> GpuBindGroupLayoutEntry {
    let buffer_binding_layout = GpuBufferBindingLayout::new();
    buffer_binding_layout.set_type(if is_read_only {
        GpuBufferBindingType::ReadOnlyStorage
    } else {
        GpuBufferBindingType::Storage
    });

    let bind_group_layout_entry = GpuBindGroupLayoutEntry::new(binding, gpu_shader_stage::COMPUTE);
    bind_group_layout_entry.set_buffer(&buffer_binding_layout);
    bind_group_layout_entry
}

/// Create compute pipeline for `pcg_update_scalars.wgsl`.
///
/// WGSL bindings (group(0)):
///   binding(0) : uniform Params (u32 indices into scalar_results)
///   binding(1) : storage read_write scalar_results (array<f32>)
pub fn create_pcg_update_scalars_pipeline(
    ctx: &WebGpuCtx,
) -> Result<PcgUpdateScalarsPipeline, JsValue> {
    let device: &GpuDevice = &ctx.device;

    // Load WGSL.
    let code = include_str!("./wgsl/pcg_update_scalars.wgsl");
    let shader_module_desc = GpuShaderModuleDescriptor::new(code);
    shader_module_desc.set_label("pcg_update_scalars.wgsl");
    let shader_module = device.create_shader_module(&shader_module_desc);

    // Entry point.
    let programmable_stage = GpuProgrammableStage::new(&shader_module);
    programmable_stage.set_entry_point("compute_main");

    // Bind group layout matches WGSL bindings.
    let bind_group_layout_0_entry_0 = create_uniform_entry(0); // Params
    let bind_group_layout_0_entry_1 = create_storage_entry(1, false); // scalar_results RW

    let bind_group_layout_0_entries = [&bind_group_layout_0_entry_0, &bind_group_layout_0_entry_1]
        .iter()
        .collect::<Array>();

    let pcg_update_scalars_bind_group_layout_desc =
        GpuBindGroupLayoutDescriptor::new(&bind_group_layout_0_entries);
    pcg_update_scalars_bind_group_layout_desc.set_label("pcg_update_scalars bgl0");

    let pcg_update_scalars_bind_group_layout =
        device.create_bind_group_layout(&pcg_update_scalars_bind_group_layout_desc)?;

    // Pipeline layout.
    let bind_group_layouts = [&pcg_update_scalars_bind_group_layout]
        .iter()
        .collect::<Array>();
    let pipeline_layout_desc = GpuPipelineLayoutDescriptor::new(&bind_group_layouts);
    pipeline_layout_desc.set_label("pcg_update_scalars pipeline layout");
    let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_desc);

    // Compute pipeline.
    let compute_pipeline_desc =
        GpuComputePipelineDescriptor::new(&pipeline_layout, &programmable_stage);
    compute_pipeline_desc.set_label("pcg_update_scalars pipeline");

    let pipeline = device.create_compute_pipeline(&compute_pipeline_desc);

    Ok(PcgUpdateScalarsPipeline {
        pipeline,
        pcg_update_scalars_bind_group_layout,
    })
}

/// Create bind group for `pcg_update_scalars.wgsl`.
///
/// Inputs:
/// - params_buffer: uniform buffer containing indices (u32s) selecting which slots
///   in `scalar_results_buffer` are read/written.
/// - scalar_results_buffer: storage buffer containing dot-products and computed scalars.
pub fn create_pcg_update_scalars_bind_group(
    device: &GpuDevice,
    pcg_update_scalars_bind_group_layout: &GpuBindGroupLayout,
    params_buffer: &GpuBuffer,
    scalar_results_buffer: &GpuBuffer,
) -> GpuBindGroup {
    let bind_group_0_entry_0 = GpuBindGroupEntry::new(0, &GpuBufferBinding::new(params_buffer));
    let bind_group_0_entry_1 =
        GpuBindGroupEntry::new(1, &GpuBufferBinding::new(scalar_results_buffer));

    let bind_group_0_entries = [&bind_group_0_entry_0, &bind_group_0_entry_1]
        .iter()
        .collect::<Array>();

    let bind_group_desc =
        GpuBindGroupDescriptor::new(&bind_group_0_entries, pcg_update_scalars_bind_group_layout);
    bind_group_desc.set_label("pcg_update_scalars bind group 0");

    device.create_bind_group(&bind_group_desc)
}
