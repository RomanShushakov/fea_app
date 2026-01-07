use wasm_bindgen::{JsValue, JsCast};
use web_sys::{WebGlContextAttributes, HtmlCanvasElement, WebGlRenderingContext as GL, CanvasRenderingContext2d as CTX};

use crate::props::Props;


pub fn get_webgl_rendering_context(canvas: &HtmlCanvasElement) -> Result<GL, JsValue>
{
    let mut webgl_context_attributes = WebGlContextAttributes::new();
    webgl_context_attributes.premultiplied_alpha(false);

    let webgl_rendering_context = canvas
        .get_context_with_context_options("webgl", &webgl_context_attributes)?
        .unwrap()
        .dyn_into::<GL>()?;

    Ok(webgl_rendering_context)
}


pub fn get_canvas_rendering_context_2d(canvas: &HtmlCanvasElement) -> Result<CTX, JsValue>
{
    let canvas_rendering_context_2d = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CTX>()?;
    Ok(canvas_rendering_context_2d)
}


pub fn add_hints(ctx: &CTX, canvas_width: f32, canvas_height: f32, props: &Props)
{
    let hint_x = canvas_width * props.hint_shift_x;
    let rotate_hint_y = canvas_height * props.rotation_hint_shift_y;
    let rotate_hint = "Rotate - (Ctrl + Alt + MB1)";
    ctx.fill_text(rotate_hint, hint_x as f64, rotate_hint_y as f64).unwrap();
    let zoom_hint_y = canvas_height * props.zoom_hint_shift_y;
    let zoom_hint = "Zoom - (Ctrl + Alt + MB3) or Mouse Wheel";
    ctx.fill_text(zoom_hint, hint_x as f64, zoom_hint_y as f64).unwrap();
    let pan_hint_y = canvas_height * props.pan_hint_shift_y;
    let pan_hint = "Pan - (Ctrl + Alt + MB2)";
    ctx.fill_text(pan_hint, hint_x as f64, pan_hint_y as f64).unwrap();
}


pub fn add_denotation(ctx: &CTX, position: &[f32; 4], matrix: &[f32; 16],
    canvas_width: f32, canvas_height: f32, denotation: &str)
{
    let mut v = vec4::new_zero();
    let clip_space = vec4::transform_mat4(&mut v, position, matrix);
    let pixel_x = (clip_space[0] / clip_space[3] * 0.5 + 0.5) * canvas_width;
    let pixel_y = (clip_space[1] / clip_space[3] * -0.5 + 0.5) * canvas_height;
    ctx.fill_text(denotation, pixel_x as f64, pixel_y as f64).unwrap();
}


pub fn add_color_bar(ctx: &CTX, canvas_width: f32, canvas_height: f32, props: &Props)
{
    ctx.begin_path();
    let gradient: web_sys::CanvasGradient = ctx
        .create_linear_gradient(
            (canvas_width * props.color_bar_shift_x) as f64,
            (canvas_height * props.color_bar_y_bottom) as f64,
            (canvas_width * props.color_bar_shift_x) as f64,
            (canvas_height * props.color_bar_y_top) as f64,
        );
    let min_color = &format!(
        "rgb({}, {}, {})", 
        &props.color_bar_min_color[0] * 255.0, 
        &props.color_bar_min_color[1] * 255.0, 
        &props.color_bar_min_color[2] * 255.0,
    );
    let max_color = &format!(
        "rgb({}, {}, {})", 
        &props.color_bar_max_color[0] * 255.0, 
        &props.color_bar_max_color[1] * 255.0, 
        &props.color_bar_max_color[2] * 255.0,
    );
    gradient.add_color_stop(0f32, min_color).unwrap();
    gradient.add_color_stop(1f32, max_color).unwrap();
    ctx.set_fill_style(&gradient.into());
    ctx.fill_rect(
        (canvas_width * props.color_bar_shift_x) as f64,
        (canvas_height * props.color_bar_y_top) as f64,
        (canvas_width * props.color_bar_width) as f64,
        (canvas_height * (props.color_bar_y_bottom - props.color_bar_y_top)) as f64,
    );
    ctx.stroke();
}


pub fn add_color_bar_caption(
    ctx: &CTX, 
    canvas_width: f32, 
    canvas_height: f32, 
    caption_header: &str,
    components: &[&str],
    min_value: f32,
    max_value: f32,
    props: &Props,
)
{
    ctx.set_fill_style(&props.hints_color.clone().into());
    let caption_shift_x = canvas_width * props.color_bar_caption_shift_x;
    let caption_header_shift_y = canvas_height * props.color_bar_caption_header_shift_y;
    ctx.fill_text(caption_header, caption_shift_x as f64, caption_header_shift_y as f64).unwrap();

    let caption_component_shift_y = canvas_height * props.color_bar_caption_component_shift_y;
    let mut caption_components = String::new();
    for (i, component) in components.iter().enumerate()
    {
        if i != components.len() - 1
        {
            caption_components += &format!("{component}, ");
        }
        else
        {
            caption_components += component.as_ref();
        }
    }
    ctx.fill_text(
        &caption_components.to_uppercase(), caption_shift_x as f64, caption_component_shift_y as f64,
    ).unwrap();

    let result_extreme_value_shift_x = canvas_width * props.color_bar_caption_extreme_value_shift_x;
    let result_max_value_shift_y = canvas_height * props.color_bar_y_top;
    let max_value = &format!("Max {:+.4e}", max_value);
    ctx.fill_text(
        max_value, result_extreme_value_shift_x as f64, result_max_value_shift_y as f64,
    ).unwrap();
    let result_min_value_shift_y = canvas_height * props.color_bar_y_bottom;
    let min_value = &format!("Min {:+.4e}", min_value);
    ctx.fill_text(
        min_value, result_extreme_value_shift_x as f64, result_min_value_shift_y as f64,
    ).unwrap();
}
