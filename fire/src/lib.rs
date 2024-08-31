use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext as GL;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window exists"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("should have document"))?;
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_attribute("width", "500")?;
    canvas.set_attribute("height", "500")?;

    let gl = canvas
        .get_context("webgl2")?
        .ok_or_else(|| JsValue::from_str("fail to get context"))?
        .dyn_into::<web_sys::WebGl2RenderingContext>()?;
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(GL::COLOR_BUFFER_BIT);

    Ok(())
}
