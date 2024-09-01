extern crate nalgebra_glm as glm;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{WebGl2RenderingContext as GL, *};

use super::shader::*;

pub struct App {
    pub gl: GL,
    window: Window,
    width: u32,
    height: u32,
    pub shader_program: WebGlProgram,
}

impl App {
    pub fn new(height: u32, width: u32) -> Result<Self, JsValue> {
        let window = Self::init_window()?;
        let document = Self::init_document(&window)?;
        let canvas = Self::init_canvas(width, height, &document)?;
        let gl = Self::init_gl(&canvas)?;
        let shader_program = Self::init_shader_program(&gl)?;

        gl.use_program(Some(&shader_program));

        Ok(Self {
            gl,
            window,
            width,
            height,
            shader_program,
        })
    }

    fn init_window() -> Result<Window, JsValue> {
        web_sys::window().ok_or_else(|| JsValue::from_str("Failed to get window"))
    }

    fn init_document(window: &Window) -> Result<Document, JsValue> {
        window
            .document()
            .ok_or_else(|| JsValue::from_str("Failed to get document"))
    }

    fn init_canvas(
        width: u32,
        height: u32,
        document: &Document,
    ) -> Result<HtmlCanvasElement, JsValue> {
        let canvas = document
            .get_element_by_id("canvas")
            .map(|e| e.dyn_into::<HtmlCanvasElement>())
            .ok_or_else(|| JsValue::from_str("Failed to get canvas"))??;

        canvas.set_width(width);
        canvas.set_height(height);
        Ok(canvas)
    }

    fn init_gl(canvas: &HtmlCanvasElement) -> Result<GL, JsValue> {
        canvas
            .get_context("webgl2")
            .and_then(|op| {
                op.ok_or_else(|| JsValue::from_str("Failed to get WebGl2RenderingContext"))
            })
            .and_then(|context| {
                context.dyn_into::<GL>().map_err(|_| {
                    JsValue::from_str("Failed to cast context to WebGl2RenderingContext")
                })
            })
    }

    fn init_shader_program(gl: &GL) -> Result<WebGlProgram, JsValue> {
        let vertex_shader =
            create_shader(&gl, GL::VERTEX_SHADER, include_str!("shader/vertex.glsl"))?;
        let fragment_shader = create_shader(
            &gl,
            GL::FRAGMENT_SHADER,
            include_str!("shader/fragment.glsl"),
        )?;
        let program = link_program(gl, &vertex_shader, &fragment_shader)?;
        Ok(program)
    }
}
