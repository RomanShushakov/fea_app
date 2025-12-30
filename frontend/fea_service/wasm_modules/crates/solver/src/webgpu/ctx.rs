use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{GpuAdapter, GpuDevice, GpuQueue, WorkerGlobalScope, WorkerNavigator};

/// WebGpuCtx
///
/// Minimal WebGPU context shared across executors.
/// In web-sys, `GpuQueue` is retrieved from the device and can be stored directly.
pub struct WebGpuCtx {
    pub device: GpuDevice,
    pub queue: GpuQueue,
}

/// Get `navigator` from a Web Worker global scope.
///
/// NOTE:
/// This code is intentionally worker-only (it expects `globalThis` to be a `WorkerGlobalScope`).
fn get_navigator() -> Result<WorkerNavigator, JsValue> {
    // `globalThis` works in a worker.
    let global = js_sys::global();

    // Dedicated worker / shared worker / service worker all implement WorkerGlobalScope in web-sys.
    let worker_global_scope: WorkerGlobalScope = global
        .dyn_into::<WorkerGlobalScope>()
        .map_err(|_| JsValue::from_str("get_navigator: globalThis is not a WorkerGlobalScope"))?;

    Ok(worker_global_scope.navigator())
}

/// Initialize WebGPU (adapter + device + queue).
///
/// Returns:
///   - Ok(WebGpuCtx) on success
///   - Err(String) with a descriptive message on failure
///
/// NOTE:
/// This function does not request any optional device features or limits.
/// If you need them later, this is the place to configure request_device().
pub async fn init_webgpu() -> Result<WebGpuCtx, String> {
    let navigator =
        get_navigator().map_err(|e| format!("init_webgpu: get_navigator failed: {:?}", e))?;

    let gpu = navigator.gpu();

    // -------------------------------------------------------------------------
    // Adapter
    // -------------------------------------------------------------------------
    let adapter_promise = gpu.request_adapter();
    let adapter_js_value = JsFuture::from(adapter_promise)
        .await
        .map_err(|e| format!("init_webgpu: request_adapter await failed: {:?}", e))?;

    let adapter: GpuAdapter = adapter_js_value
        .dyn_into()
        .map_err(|_| "init_webgpu: request_adapter returned non-GpuAdapter".to_string())?;

    // -------------------------------------------------------------------------
    // Device
    // -------------------------------------------------------------------------
    let device_promise = adapter.request_device();
    let device_js_value = JsFuture::from(device_promise)
        .await
        .map_err(|e| format!("init_webgpu: request_device await failed: {:?}", e))?;

    let device: GpuDevice = device_js_value
        .dyn_into()
        .map_err(|_| "init_webgpu: request_device returned non-GpuDevice".to_string())?;

    let queue: GpuQueue = device.queue();

    Ok(WebGpuCtx { device, queue })
}
