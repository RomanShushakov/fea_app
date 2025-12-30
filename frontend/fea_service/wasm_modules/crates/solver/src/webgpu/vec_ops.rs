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

/// Pipeline for the classic AXPY kernel:
///   y[i] = y[i] + alpha * x[i]
/// where `alpha` is provided in a uniform buffer.
pub struct AxpyPipeline {
    pub pipeline: GpuComputePipeline,
    pub axpy_bind_group_layout: GpuBindGroupLayout,
}

/// Pipeline for AXPY where alpha is read from `scalar_results_buffer[scalar_index]`.
pub struct AxpyFromScalarResultsPipeline {
    pub pipeline: GpuComputePipeline,
    pub axpy_from_scalar_results_bind_group_layout: GpuBindGroupLayout,
}

/// Pipeline for SCALE where beta is read from `scalar_results_buffer[scalar_index]`:
///   x[i] = x[i] * beta
pub struct ScaleFromScalarResultsPipeline {
    pub pipeline: GpuComputePipeline,
    pub scale_from_scalar_results_bind_group_layout: GpuBindGroupLayout,
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

pub fn create_axpy_pipeline(ctx: &WebGpuCtx) -> Result<AxpyPipeline, JsValue> {
    let device: &GpuDevice = &ctx.device;

    let code = include_str!("./wgsl/axpy.wgsl");
    let shader_module_desc = GpuShaderModuleDescriptor::new(code);
    shader_module_desc.set_label("axpy.wgsl");
    let shader_module = device.create_shader_module(&shader_module_desc);

    let programmable_stage = GpuProgrammableStage::new(&shader_module);
    programmable_stage.set_entry_point("compute_main");

    // WGSL group(0) bindings:
    //   binding(0): uniform Params
    //   binding(1): x RO storage
    //   binding(2): y RW storage
    let bind_group_layout_0_entry_0 = create_uniform_entry(0);
    let bind_group_layout_0_entry_1 = create_storage_entry(1, true);
    let bind_group_layout_0_entry_2 = create_storage_entry(2, false);

    let bind_group_layout_0_entries = [
        &bind_group_layout_0_entry_0,
        &bind_group_layout_0_entry_1,
        &bind_group_layout_0_entry_2,
    ]
    .iter()
    .collect::<Array>();

    let axpy_bind_group_layout_desc =
        GpuBindGroupLayoutDescriptor::new(&bind_group_layout_0_entries);
    axpy_bind_group_layout_desc.set_label("axpy bgl0");

    let axpy_bind_group_layout = device.create_bind_group_layout(&axpy_bind_group_layout_desc)?;

    let bind_group_layouts = [&axpy_bind_group_layout].iter().collect::<Array>();
    let pipeline_layout_desc = GpuPipelineLayoutDescriptor::new(&bind_group_layouts);
    pipeline_layout_desc.set_label("axpy pipeline layout");
    let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_desc);

    let compute_pipeline_desc =
        GpuComputePipelineDescriptor::new(&pipeline_layout, &programmable_stage);
    compute_pipeline_desc.set_label("axpy pipeline");

    let pipeline = device.create_compute_pipeline(&compute_pipeline_desc);

    Ok(AxpyPipeline {
        pipeline,
        axpy_bind_group_layout,
    })
}

pub fn create_axpy_from_scalar_results_pipeline(
    ctx: &WebGpuCtx,
) -> Result<AxpyFromScalarResultsPipeline, JsValue> {
    let device: &GpuDevice = &ctx.device;

    let code = include_str!("./wgsl/axpy_from_scalar_results.wgsl");
    let shader_module_desc = GpuShaderModuleDescriptor::new(code);
    shader_module_desc.set_label("axpy_from_scalar_results.wgsl");
    let shader_module = device.create_shader_module(&shader_module_desc);

    let programmable_stage = GpuProgrammableStage::new(&shader_module);
    programmable_stage.set_entry_point("compute_main");

    // WGSL group(0) bindings:
    //   binding(0): uniform Params (n, scalar_index)
    //   binding(1): x RO storage
    //   binding(2): y RW storage
    //   binding(3): scalar_results RO storage
    let bind_group_layout_0_entry_0 = create_uniform_entry(0);
    let bind_group_layout_0_entry_1 = create_storage_entry(1, true);
    let bind_group_layout_0_entry_2 = create_storage_entry(2, false);
    let bind_group_layout_0_entry_3 = create_storage_entry(3, true);

    let bind_group_layout_0_entries = [
        &bind_group_layout_0_entry_0,
        &bind_group_layout_0_entry_1,
        &bind_group_layout_0_entry_2,
        &bind_group_layout_0_entry_3,
    ]
    .iter()
    .collect::<Array>();

    let axpy_from_scalar_results_bind_group_layout_desc =
        GpuBindGroupLayoutDescriptor::new(&bind_group_layout_0_entries);
    axpy_from_scalar_results_bind_group_layout_desc.set_label("axpy_from_scalar_results bgl0");

    let axpy_from_scalar_results_bind_group_layout =
        device.create_bind_group_layout(&axpy_from_scalar_results_bind_group_layout_desc)?;

    let bind_group_layouts = [&axpy_from_scalar_results_bind_group_layout]
        .iter()
        .collect::<Array>();
    let pipeline_layout_desc = GpuPipelineLayoutDescriptor::new(&bind_group_layouts);
    pipeline_layout_desc.set_label("axpy_from_scalar_results pipeline layout");
    let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_desc);

    let compute_pipeline_desc =
        GpuComputePipelineDescriptor::new(&pipeline_layout, &programmable_stage);
    compute_pipeline_desc.set_label("axpy_from_scalar_results pipeline");

    let pipeline = device.create_compute_pipeline(&compute_pipeline_desc);

    Ok(AxpyFromScalarResultsPipeline {
        pipeline,
        axpy_from_scalar_results_bind_group_layout,
    })
}

pub fn create_scale_from_scalar_results_pipeline(
    ctx: &WebGpuCtx,
) -> Result<ScaleFromScalarResultsPipeline, JsValue> {
    let device: &GpuDevice = &ctx.device;

    let code = include_str!("./wgsl/scale_from_scalar_results.wgsl");
    let shader_module_desc = GpuShaderModuleDescriptor::new(code);
    shader_module_desc.set_label("scale_from_scalar_results.wgsl");
    let shader_module = device.create_shader_module(&shader_module_desc);

    let programmable_stage = GpuProgrammableStage::new(&shader_module);
    programmable_stage.set_entry_point("compute_main");

    // WGSL group(0) bindings:
    //   binding(0): uniform Params (n, scalar_index)
    //   binding(1): x RW storage
    //   binding(2): scalar_results RO storage
    let bind_group_layout_0_entry_0 = create_uniform_entry(0);
    let bind_group_layout_0_entry_1 = create_storage_entry(1, false);
    let bind_group_layout_0_entry_2 = create_storage_entry(2, true);

    let bind_group_layout_0_entries = [
        &bind_group_layout_0_entry_0,
        &bind_group_layout_0_entry_1,
        &bind_group_layout_0_entry_2,
    ]
    .iter()
    .collect::<Array>();

    let scale_from_scalar_results_bind_group_layout_desc =
        GpuBindGroupLayoutDescriptor::new(&bind_group_layout_0_entries);
    scale_from_scalar_results_bind_group_layout_desc.set_label("scale_from_scalar_results bgl0");

    let scale_from_scalar_results_bind_group_layout =
        device.create_bind_group_layout(&scale_from_scalar_results_bind_group_layout_desc)?;

    let bind_group_layouts = [&scale_from_scalar_results_bind_group_layout]
        .iter()
        .collect::<Array>();
    let pipeline_layout_desc = GpuPipelineLayoutDescriptor::new(&bind_group_layouts);
    pipeline_layout_desc.set_label("scale_from_scalar_results pipeline layout");
    let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_desc);

    let compute_pipeline_desc =
        GpuComputePipelineDescriptor::new(&pipeline_layout, &programmable_stage);
    compute_pipeline_desc.set_label("scale_from_scalar_results pipeline");

    let pipeline = device.create_compute_pipeline(&compute_pipeline_desc);

    Ok(ScaleFromScalarResultsPipeline {
        pipeline,
        scale_from_scalar_results_bind_group_layout,
    })
}

pub fn create_axpy_bind_group(
    device: &GpuDevice,
    axpy_bind_group_layout: &GpuBindGroupLayout,
    params_buffer: &GpuBuffer, // binding(0)
    x_buffer: &GpuBuffer,      // binding(1)
    y_buffer: &GpuBuffer,      // binding(2)
) -> GpuBindGroup {
    let bind_group_0_entry_0 = GpuBindGroupEntry::new(0, &GpuBufferBinding::new(params_buffer));
    let bind_group_0_entry_1 = GpuBindGroupEntry::new(1, &GpuBufferBinding::new(x_buffer));
    let bind_group_0_entry_2 = GpuBindGroupEntry::new(2, &GpuBufferBinding::new(y_buffer));

    let bind_group_0_entries = [
        &bind_group_0_entry_0,
        &bind_group_0_entry_1,
        &bind_group_0_entry_2,
    ]
    .iter()
    .collect::<Array>();

    let axpy_bind_group_desc =
        GpuBindGroupDescriptor::new(&bind_group_0_entries, axpy_bind_group_layout);
    axpy_bind_group_desc.set_label("axpy bind group 0");

    device.create_bind_group(&axpy_bind_group_desc)
}

pub fn create_axpy_from_scalar_results_bind_group(
    device: &GpuDevice,
    axpy_from_scalar_results_bind_group_layout: &GpuBindGroupLayout,
    params_buffer: &GpuBuffer,         // binding(0)
    x_buffer: &GpuBuffer,              // binding(1)
    y_buffer: &GpuBuffer,              // binding(2)
    scalar_results_buffer: &GpuBuffer, // binding(3)
) -> GpuBindGroup {
    let bind_group_0_entry_0 = GpuBindGroupEntry::new(0, &GpuBufferBinding::new(params_buffer));
    let bind_group_0_entry_1 = GpuBindGroupEntry::new(1, &GpuBufferBinding::new(x_buffer));
    let bind_group_0_entry_2 = GpuBindGroupEntry::new(2, &GpuBufferBinding::new(y_buffer));
    let bind_group_0_entry_3 =
        GpuBindGroupEntry::new(3, &GpuBufferBinding::new(scalar_results_buffer));

    let bind_group_0_entries = [
        &bind_group_0_entry_0,
        &bind_group_0_entry_1,
        &bind_group_0_entry_2,
        &bind_group_0_entry_3,
    ]
    .iter()
    .collect::<Array>();

    let bind_group_desc = GpuBindGroupDescriptor::new(
        &bind_group_0_entries,
        axpy_from_scalar_results_bind_group_layout,
    );
    bind_group_desc.set_label("axpy_from_scalar_results bind group 0");

    device.create_bind_group(&bind_group_desc)
}

pub fn create_scale_from_scalar_results_bind_group(
    device: &GpuDevice,
    scale_from_scalar_results_bind_group_layout: &GpuBindGroupLayout,
    params_buffer: &GpuBuffer,         // binding(0)
    x_buffer: &GpuBuffer,              // binding(1)
    scalar_results_buffer: &GpuBuffer, // binding(2)
) -> GpuBindGroup {
    let bind_group_0_entry_0 = GpuBindGroupEntry::new(0, &GpuBufferBinding::new(params_buffer));
    let bind_group_0_entry_1 = GpuBindGroupEntry::new(1, &GpuBufferBinding::new(x_buffer));
    let bind_group_0_entry_2 =
        GpuBindGroupEntry::new(2, &GpuBufferBinding::new(scalar_results_buffer));

    let bind_group_0_entries = [
        &bind_group_0_entry_0,
        &bind_group_0_entry_1,
        &bind_group_0_entry_2,
    ]
    .iter()
    .collect::<Array>();

    let bind_group_desc = GpuBindGroupDescriptor::new(
        &bind_group_0_entries,
        scale_from_scalar_results_bind_group_layout,
    );
    bind_group_desc.set_label("scale_from_scalar_results bind group 0");

    device.create_bind_group(&bind_group_desc)
}
