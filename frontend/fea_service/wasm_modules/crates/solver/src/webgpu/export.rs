use wasm_bindgen::{JsCast, JsValue};
use web_sys::DedicatedWorkerGlobalScope;

pub fn make_pcg_block_jacobi_csr_input_bundle_text(
    n: usize,
    nnz: usize,
    row_ptr_u32: &[u32],
    col_idx_u32: &[u32],
    values_f32: &[f32],
    b_f32: &[f32],
    x0_f32: &[f32],
    block_starts_usize: &[usize],
) -> String {
    fn write_u32_array(out: &mut String, name: &str, data: &[u32]) {
        out.push_str(name);
        out.push('\n');
        out.push_str(&format!("len {}\n", data.len()));
        for (i, v) in data.iter().enumerate() {
            out.push_str(&v.to_string());
            if (i + 1) % 16 == 0 {
                out.push('\n');
            } else {
                out.push(' ');
            }
        }
        if !out.ends_with('\n') {
            out.push('\n');
        }
        out.push('\n');
    }

    fn write_f32_array(out: &mut String, name: &str, data: &[f32]) {
        out.push_str(name);
        out.push('\n');
        out.push_str(&format!("len {}\n", data.len()));
        for (i, v) in data.iter().enumerate() {
            out.push_str(&format!("{:.9e}", v));
            if (i + 1) % 8 == 0 {
                out.push('\n');
            } else {
                out.push(' ');
            }
        }
        if !out.ends_with('\n') {
            out.push('\n');
        }
        out.push('\n');
    }

    let mut out = String::new();
    out.push_str("# wgpu-solver-backend dataset (text)\n");
    out.push_str("# generated inside worker; main thread should download\n\n");
    out.push_str(&format!("n {}\n", n));
    out.push_str(&format!("nnz {}\n\n", nnz));

    write_u32_array(&mut out, "row_ptr_u32", row_ptr_u32);
    write_u32_array(&mut out, "col_idx_u32", col_idx_u32);
    write_f32_array(&mut out, "values_f32", values_f32);
    write_f32_array(&mut out, "b_f32", b_f32);
    write_f32_array(&mut out, "x0_f32", x0_f32);

    let block_starts_u32: Vec<u32> = block_starts_usize.iter().map(|&v| v as u32).collect();
    write_u32_array(&mut out, "block_starts_u32", &block_starts_u32);

    out
}

pub fn send_export_to_main_thread(filename: &str, text: &str) -> Result<(), JsValue> {
    let scope: DedicatedWorkerGlobalScope = js_sys::global().dyn_into()?;
    let msg = js_sys::Object::new();
    js_sys::Reflect::set(&msg, &"header".into(), &"export_dataset".into())?;
    js_sys::Reflect::set(&msg, &"filename".into(), &filename.into())?;
    js_sys::Reflect::set(&msg, &"text".into(), &text.into())?;
    scope.post_message(&msg)?;
    Ok(())
}
