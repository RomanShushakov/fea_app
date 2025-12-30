use js_sys::Float32Array;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    GpuBuffer, GpuBufferDescriptor, GpuCommandEncoder, GpuDevice, GpuQueue, gpu_buffer_usage,
    gpu_map_mode,
};

/// Creates a STORAGE buffer that holds `len` f32 values.
///
/// Usage flags:
/// - STORAGE: can be bound as storage buffer in WGSL
/// - COPY_DST: allows queue.write_buffer (host->GPU upload)
/// - extra_usage: optionally add COPY_SRC, etc (e.g. for readback copies)
pub fn create_storage_buffer_f32(
    device: &GpuDevice,
    len: usize,
    label: &str,
    extra_usage: u32,
) -> Result<GpuBuffer, JsValue> {
    let usage = gpu_buffer_usage::STORAGE | gpu_buffer_usage::COPY_DST | extra_usage;
    let desc = GpuBufferDescriptor::new((len * 4) as f64, usage);
    desc.set_label(label);

    device.create_buffer(&desc).map_err(|e| {
        JsValue::from_str(&format!(
            "create_storage_buffer_f32: create_buffer failed: {:?}",
            e
        ))
    })
}

/// Creates a UNIFORM buffer of `byte_len` bytes.
///
/// Usage flags:
/// - UNIFORM: can be bound as uniform buffer in WGSL
/// - COPY_DST: allows queue.write_buffer for updates
pub fn create_uniform_buffer(
    device: &GpuDevice,
    byte_len: usize,
    label: &str,
) -> Result<GpuBuffer, JsValue> {
    let usage = gpu_buffer_usage::UNIFORM | gpu_buffer_usage::COPY_DST;
    let desc = GpuBufferDescriptor::new(byte_len as f64, usage);
    desc.set_label(label);

    device.create_buffer(&desc).map_err(|e| {
        JsValue::from_str(&format!(
            "create_uniform_buffer: create_buffer failed: {:?}",
            e
        ))
    })
}

/// Writes u32 values into a GPU buffer at offset 0 using queue.write_buffer.
///
/// IMPORTANT: `buffer` must have COPY_DST usage.
pub fn write_u32(queue: &GpuQueue, buffer: &GpuBuffer, data: &[u32]) -> Result<(), JsValue> {
    let mut bytes = Vec::with_capacity(data.len() * 4);
    for &v in data {
        bytes.extend_from_slice(&v.to_le_bytes());
    }

    queue
        .write_buffer_with_u32_and_u8_slice(buffer, 0, &bytes)
        .map_err(|e| {
            JsValue::from_str(&format!(
                "write_u32: write_buffer_with_u32_and_u8_slice failed: {:?}",
                e
            ))
        })
}

/// Writes f32 values into a GPU buffer at offset 0 using queue.write_buffer.
///
/// IMPORTANT: `buffer` must have COPY_DST usage.
pub fn write_f32(queue: &GpuQueue, buffer: &GpuBuffer, data: &[f32]) -> Result<(), JsValue> {
    let mut bytes = Vec::with_capacity(data.len() * 4);
    for &v in data {
        bytes.extend_from_slice(&v.to_le_bytes());
    }

    queue
        .write_buffer_with_u32_and_u8_slice(buffer, 0, &bytes)
        .map_err(|e| {
            JsValue::from_str(&format!(
                "write_f32: write_buffer_with_u32_and_u8_slice failed: {:?}",
                e
            ))
        })
}

/// Creates a MAP_READ + COPY_DST buffer for readback.
///
/// Typical flow:
/// - GPU copies some buffer into this readback buffer (COPY_DST)
/// - CPU maps it for reading (MAP_READ) via `read_back_f32`
pub fn create_readback_buffer(
    device: &GpuDevice,
    byte_len: usize,
    label: &str,
) -> Result<GpuBuffer, JsValue> {
    let usage = gpu_buffer_usage::MAP_READ | gpu_buffer_usage::COPY_DST;
    let desc = GpuBufferDescriptor::new(byte_len as f64, usage);
    desc.set_label(label);

    device.create_buffer(&desc).map_err(|e| {
        JsValue::from_str(&format!(
            "create_readback_buffer: create_buffer failed: {:?}",
            e
        ))
    })
}

/// Maps a readback buffer and reads `len` f32 values.
///
/// REQUIREMENTS:
/// - The buffer must have MAP_READ usage.
/// - You must have scheduled a GPU copy into this buffer before calling.
/// - This is an async CPU/GPU sync point.
pub async fn read_back_f32(readback: &GpuBuffer, len: usize) -> Result<Vec<f32>, JsValue> {
    JsFuture::from(readback.map_async(gpu_map_mode::READ))
        .await
        .map_err(|e| JsValue::from_str(&format!("read_back_f32: map_async failed: {:?}", e)))?;

    let mapped = readback.get_mapped_range().map_err(|e| {
        JsValue::from_str(&format!("read_back_f32: get_mapped_range failed: {:?}", e))
    })?;

    let arr = Float32Array::new(&mapped);
    let mut out = vec![0.0f32; len];
    arr.copy_to(&mut out);

    readback.unmap();
    Ok(out)
}

/// Creates a u32 storage buffer and uploads initial data via queue.write_buffer.
///
/// Usage flags:
/// - STORAGE: bindable as storage buffer
/// - COPY_DST: required for upload
/// - extra_usage: optionally add COPY_SRC etc.
pub fn create_storage_buffer_u32_with_data(
    device: &GpuDevice,
    queue: &GpuQueue,
    data: &[u32],
    label: &str,
    extra_usage: u32,
) -> Result<GpuBuffer, JsValue> {
    let usage = gpu_buffer_usage::STORAGE | gpu_buffer_usage::COPY_DST | extra_usage;
    let desc = GpuBufferDescriptor::new((data.len() * 4) as f64, usage);
    desc.set_label(label);

    let buf = device.create_buffer(&desc).map_err(|e| {
        JsValue::from_str(&format!(
            "create_storage_buffer_u32_with_data: create_buffer failed: {:?}",
            e
        ))
    })?;

    write_u32(queue, &buf, data)?;
    Ok(buf)
}

/// Creates an f32 storage buffer and uploads initial data via queue.write_buffer.
///
/// Usage flags:
/// - STORAGE: bindable as storage buffer
/// - COPY_DST: required for upload
/// - extra_usage: optionally add COPY_SRC etc.
pub fn create_storage_buffer_f32_with_data(
    device: &GpuDevice,
    queue: &GpuQueue,
    data: &[f32],
    label: &str,
    extra_usage: u32,
) -> Result<GpuBuffer, JsValue> {
    let usage = gpu_buffer_usage::STORAGE | gpu_buffer_usage::COPY_DST | extra_usage;
    let desc = GpuBufferDescriptor::new((data.len() * 4) as f64, usage);
    desc.set_label(label);

    let buf = device.create_buffer(&desc).map_err(|e| {
        JsValue::from_str(&format!(
            "create_storage_buffer_f32_with_data: create_buffer failed: {:?}",
            e
        ))
    })?;

    write_f32(queue, &buf, data)?;
    Ok(buf)
}

/// Encodes: dst_storage_buffer[dst_index] = value (as f32).
///
/// This works by:
/// 1) creating a tiny 4-byte staging buffer (COPY_DST|COPY_SRC)
/// 2) queue.write_buffer staging <- value
/// 3) command_encoder.copy_buffer_to_buffer staging -> dst_storage_buffer at offset dst_index*4
///
/// NOTE (important): this creates a GPU buffer every call, which is slow.
/// It's OK for "once per iteration" while prototyping, but for performance:
/// - move staging buffer into an executor (persistent GpuBuffer)
/// - or write rz_old via a uniform buffer / scalar storage buffer update strategy
pub fn encode_write_f32_into_storage_buffer_at_index(
    device: &GpuDevice,
    queue: &GpuQueue,
    command_encoder: &GpuCommandEncoder,
    dst_storage_buffer: &GpuBuffer,
    dst_index: u32,
    value: f32,
    label: &str,
) -> Result<(), JsValue> {
    // Must include COPY_DST for queue.write_buffer and COPY_SRC for copy_buffer_to_buffer.
    let usage = gpu_buffer_usage::COPY_DST | gpu_buffer_usage::COPY_SRC;

    let desc = GpuBufferDescriptor::new(4f64, usage);
    desc.set_label(label);

    let staging_u32_buffer = device.create_buffer(&desc).map_err(|e| {
        JsValue::from_str(&format!(
            "encode_write_f32_into_storage_buffer_at_index: create_buffer failed: {:?}",
            e
        ))
    })?;

    // Upload 4 bytes into staging buffer via queue.write_buffer.
    write_u32(queue, &staging_u32_buffer, &[value.to_bits()])?;

    // Copy staging -> destination storage buffer at float offset dst_index.
    command_encoder.copy_buffer_to_buffer_with_u32_and_u32_and_u32(
        &staging_u32_buffer,
        0,
        dst_storage_buffer,
        dst_index * 4,
        4,
    )?;

    Ok(())
}
