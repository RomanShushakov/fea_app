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

// BlockJacobiPipeline: pipeline + bind group layout for block_jacobi.wgsl
//
// WGSL bindings (group(0)):
//   binding(0) params          : uniform Params { n, num_blocks, ... }
//   binding(1) lu_blocks       : storage, read      (num_blocks * 36 f32)
//   binding(2) block_starts    : storage, read      (num_blocks + 1 u32)
//   binding(3) r               : storage, read      (n f32)
//   binding(4) z               : storage, read_write(n f32)
pub struct BlockJacobiPipeline {
    pub pipeline: GpuComputePipeline,
    pub block_jacobi_bind_group_layout: GpuBindGroupLayout,
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

pub fn create_block_jacobi_pipeline(ctx: &WebGpuCtx) -> Result<BlockJacobiPipeline, JsValue> {
    let device: &GpuDevice = &ctx.device;

    // Shader module
    let code = include_str!("./wgsl/block_jacobi.wgsl");
    let shader_module_desc = GpuShaderModuleDescriptor::new(code);
    shader_module_desc.set_label("block_jacobi_apply.wgsl");
    let shader_module = device.create_shader_module(&shader_module_desc);

    let programmable_stage = GpuProgrammableStage::new(&shader_module);
    programmable_stage.set_entry_point("compute_main");

    // Bind group layout (group 0)
    let bing_group_layout_0_entry_0 = create_uniform_entry(0); // params
    let bing_group_layout_0_entry_1 = create_storage_entry(1, true); // lu_blocks (RO)
    let bing_group_layout_0_entry_2 = create_storage_entry(2, true); // block_starts (RO)
    let bing_group_layout_0_entry_3 = create_storage_entry(3, true); // r (RO)
    let bing_group_layout_0_entry_4 = create_storage_entry(4, false); // z (RW)

    let bing_group_layout_0_entries = [
        &bing_group_layout_0_entry_0,
        &bing_group_layout_0_entry_1,
        &bing_group_layout_0_entry_2,
        &bing_group_layout_0_entry_3,
        &bing_group_layout_0_entry_4,
    ]
    .iter()
    .collect::<Array>();

    let block_jacobi_bind_group_layout_desc =
        GpuBindGroupLayoutDescriptor::new(&bing_group_layout_0_entries);
    block_jacobi_bind_group_layout_desc.set_label("block_jacobi bgl0");

    let block_jacobi_bind_group_layout =
        device.create_bind_group_layout(&block_jacobi_bind_group_layout_desc)?;

    // Pipeline layout
    let bind_group_layouts = [&block_jacobi_bind_group_layout].iter().collect::<Array>();
    let pipeline_layout_desc = GpuPipelineLayoutDescriptor::new(&bind_group_layouts);
    pipeline_layout_desc.set_label("block_jacobi pipeline layout");
    let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_desc);

    // Compute pipeline
    let compute_pipeline_desc =
        GpuComputePipelineDescriptor::new(&pipeline_layout, &programmable_stage);
    compute_pipeline_desc.set_label("block_jacobi pipeline");
    let pipeline = device.create_compute_pipeline(&compute_pipeline_desc);

    Ok(BlockJacobiPipeline {
        pipeline,
        block_jacobi_bind_group_layout,
    })
}

pub fn create_block_jacobi_bind_group(
    device: &GpuDevice,
    block_jacobi_bind_group_layout: &GpuBindGroupLayout,
    params_buffer: &GpuBuffer,
    lu_blocks_buffer: &GpuBuffer,
    block_starts_buffer: &GpuBuffer,
    r_buffer: &GpuBuffer,
    z_buffer: &GpuBuffer,
) -> GpuBindGroup {
    let bing_group_layout_0_entry_0 =
        GpuBindGroupEntry::new(0, &GpuBufferBinding::new(params_buffer));
    let bing_group_layout_0_entry_1 =
        GpuBindGroupEntry::new(1, &GpuBufferBinding::new(lu_blocks_buffer));
    let bing_group_layout_0_entry_2 =
        GpuBindGroupEntry::new(2, &GpuBufferBinding::new(block_starts_buffer));
    let bing_group_layout_0_entry_3 = GpuBindGroupEntry::new(3, &GpuBufferBinding::new(r_buffer));
    let bing_group_layout_0_entry_4 = GpuBindGroupEntry::new(4, &GpuBufferBinding::new(z_buffer));

    let bing_group_layout_0_entries = [
        &bing_group_layout_0_entry_0,
        &bing_group_layout_0_entry_1,
        &bing_group_layout_0_entry_2,
        &bing_group_layout_0_entry_3,
        &bing_group_layout_0_entry_4,
    ]
    .iter()
    .collect::<Array>();

    let block_jacobi_bind_group_desc =
        GpuBindGroupDescriptor::new(&bing_group_layout_0_entries, block_jacobi_bind_group_layout);
    block_jacobi_bind_group_desc.set_label("block_jacobi bind group 0");

    device.create_bind_group(&block_jacobi_bind_group_desc)
}
