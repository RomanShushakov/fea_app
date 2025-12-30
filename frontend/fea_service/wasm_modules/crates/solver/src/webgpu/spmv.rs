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

// SpMV pipeline wiring for CSR SpMV shader (spmv.wgsl).
//
// This module only builds:
//   - bind group layout (group 0, bindings 0..5)
//   - compute pipeline
//   - bind group creator (given buffers from an executor)
//
// The actual persistent buffers + encoding logic live in SpmvExecutor.

pub struct SpmvPipeline {
    pub pipeline: GpuComputePipeline,
    pub spmv_bind_group_layout: GpuBindGroupLayout,
}

fn create_uniform_entry(binding: u32) -> GpuBindGroupLayoutEntry {
    let uniform_layout = GpuBufferBindingLayout::new();
    uniform_layout.set_type(GpuBufferBindingType::Uniform);
    uniform_layout.set_has_dynamic_offset(false);

    let bind_group_layout_entry = GpuBindGroupLayoutEntry::new(binding, gpu_shader_stage::COMPUTE);
    bind_group_layout_entry.set_buffer(&uniform_layout);
    bind_group_layout_entry
}

fn create_storage_entry(binding: u32, is_read_only: bool) -> GpuBindGroupLayoutEntry {
    let storage_layout = GpuBufferBindingLayout::new();
    storage_layout.set_type(if is_read_only {
        GpuBufferBindingType::ReadOnlyStorage
    } else {
        GpuBufferBindingType::Storage
    });

    let bind_group_layout_entry = GpuBindGroupLayoutEntry::new(binding, gpu_shader_stage::COMPUTE);
    bind_group_layout_entry.set_buffer(&storage_layout);
    bind_group_layout_entry
}

pub fn create_spmv_pipeline(ctx: &WebGpuCtx) -> Result<SpmvPipeline, JsValue> {
    let device: &GpuDevice = &ctx.device;

    // ------------------------------------------------------------------------
    // Shader module
    // ------------------------------------------------------------------------
    let code = include_str!("./wgsl/spmv.wgsl");
    let shader_module_desc = GpuShaderModuleDescriptor::new(code);
    shader_module_desc.set_label("spmv.wgsl");
    let shader_module = device.create_shader_module(&shader_module_desc);

    let programmable_stage = GpuProgrammableStage::new(&shader_module);
    programmable_stage.set_entry_point("compute_main");

    // ------------------------------------------------------------------------
    // Bind group layout (group(0), bindings 0..5), matches spmv.wgsl:
    //   0: Params uniform
    //   1: row_ptr (RO storage)
    //   2: col_idx (RO storage)
    //   3: values  (RO storage)
    //   4: x       (RO storage)
    //   5: y       (RW storage)
    // ------------------------------------------------------------------------
    let bind_group_layout_0_entry_0 = create_uniform_entry(0);
    let bind_group_layout_0_entry_1 = create_storage_entry(1, true);
    let bind_group_layout_0_entry_2 = create_storage_entry(2, true);
    let bind_group_layout_0_entry_3 = create_storage_entry(3, true);
    let bind_group_layout_0_entry_4 = create_storage_entry(4, true);
    let bind_group_layout_0_entry_5 = create_storage_entry(5, false);

    let bind_group_layout_0_entries = [
        &bind_group_layout_0_entry_0,
        &bind_group_layout_0_entry_1,
        &bind_group_layout_0_entry_2,
        &bind_group_layout_0_entry_3,
        &bind_group_layout_0_entry_4,
        &bind_group_layout_0_entry_5,
    ]
    .iter()
    .collect::<Array>();

    let spmv_bind_group_layout_desc =
        GpuBindGroupLayoutDescriptor::new(&bind_group_layout_0_entries);
    spmv_bind_group_layout_desc.set_label("spmv bgl0");

    let spmv_bind_group_layout = device.create_bind_group_layout(&spmv_bind_group_layout_desc)?;

    // ------------------------------------------------------------------------
    // Pipeline layout
    // ------------------------------------------------------------------------
    let bind_group_layouts = [&spmv_bind_group_layout].iter().collect::<Array>();
    let pipeline_layout_desc = GpuPipelineLayoutDescriptor::new(&bind_group_layouts);
    pipeline_layout_desc.set_label("spmv pipeline layout");
    let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_desc);

    // ------------------------------------------------------------------------
    // Compute pipeline
    // ------------------------------------------------------------------------
    let compute_pipeline_desc =
        GpuComputePipelineDescriptor::new(&pipeline_layout, &programmable_stage);
    compute_pipeline_desc.set_label("spmv pipeline");

    let pipeline = device.create_compute_pipeline(&compute_pipeline_desc);

    Ok(SpmvPipeline {
        pipeline,
        spmv_bind_group_layout,
    })
}

pub fn create_spmv_bind_group(
    device: &GpuDevice,
    spmv_bind_group_layout: &GpuBindGroupLayout,
    params_buffer: &GpuBuffer,
    row_ptr_buffer: &GpuBuffer,
    col_idx_buffer: &GpuBuffer,
    values_buffer: &GpuBuffer,
    x_buffer: &GpuBuffer,
    y_buffer: &GpuBuffer,
) -> GpuBindGroup {
    // group(0) bindings must match the layout and WGSL exactly.
    let bind_group_0_entry_0 = GpuBindGroupEntry::new(0, &GpuBufferBinding::new(params_buffer));
    let bind_group_0_entry_1 = GpuBindGroupEntry::new(1, &GpuBufferBinding::new(row_ptr_buffer));
    let bind_group_0_entry_2 = GpuBindGroupEntry::new(2, &GpuBufferBinding::new(col_idx_buffer));
    let bind_group_0_entry_3 = GpuBindGroupEntry::new(3, &GpuBufferBinding::new(values_buffer));
    let bind_group_0_entry_4 = GpuBindGroupEntry::new(4, &GpuBufferBinding::new(x_buffer));
    let bind_group_0_entry_5 = GpuBindGroupEntry::new(5, &GpuBufferBinding::new(y_buffer));

    let bind_group_layout_0_entries = [
        &bind_group_0_entry_0,
        &bind_group_0_entry_1,
        &bind_group_0_entry_2,
        &bind_group_0_entry_3,
        &bind_group_0_entry_4,
        &bind_group_0_entry_5,
    ]
    .iter()
    .collect::<Array>();

    let spmv_bind_group_desc =
        GpuBindGroupDescriptor::new(&bind_group_layout_0_entries, spmv_bind_group_layout);
    spmv_bind_group_desc.set_label("spmv bind group 0");

    device.create_bind_group(&spmv_bind_group_desc)
}
