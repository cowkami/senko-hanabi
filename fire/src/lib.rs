use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::{WebGl2RenderingContext, WebGlShader};

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    // Initialize the canvas
    let canvas = init_canvas(500, 500)?;
    // Get the WebGL2 context
    let context = canvas
        .get_context("webgl2")?
        .ok_or_else(|| JsValue::from_str("fail to get context"))?
        .dyn_into::<web_sys::WebGl2RenderingContext>()?;
    // Clear the canvas
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(GL::COLOR_BUFFER_BIT);

    // Initilize shader program
    let program = init_shader_program(&context)?;
    context.use_program(Some(&program));

    Ok(())
}

fn init_canvas(width: u16, height: u16) -> Result<web_sys::HtmlCanvasElement, JsValue> {
    // Get the window object
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window exists"))?;
    // Get the document object
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("should have document"))?;
    // Get the canvas element
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    // Set the canvas width and height
    canvas.set_width(width.into());
    canvas.set_height(height.into());
    // Return the canvas
    Ok(canvas)
}

fn init_shader_program(context: &GL) -> Result<web_sys::WebGlProgram, JsValue> {
    // Load shader program files
    let vertex_shader = include_str!("shader/vertex.glsl");
    let fragment_shader = include_str!("shader/fragment.glsl");

    // Create the vertex shader
    let vertex_shader = compile_shader(context, GL::VERTEX_SHADER, vertex_shader)?;
    // Create the fragment shader
    let fragment_shader = compile_shader(context, GL::FRAGMENT_SHADER, fragment_shader)?;
    // Create the shader program
    let program = context
        .create_program()
        .ok_or_else(|| JsValue::from_str("fail to create program"))?;
    // Attach the shaders to the program
    context.attach_shader(&program, &vertex_shader);
    context.attach_shader(&program, &fragment_shader);
    // Link the program
    context.link_program(&program);
    // Check if the program is linked
    if !context
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        return Err(JsValue::from_str(&format!(
            "fail to link program: {}",
            context
                .get_program_info_log(&program)
                .unwrap_or_else(|| "unknown error".to_string())
        )));
    }
    // Return the program
    Ok(program)
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))?
    }
}
