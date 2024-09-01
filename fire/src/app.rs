extern crate nalgebra_glm as glm;

use std::f32::consts;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{WebGl2RenderingContext as GL, *};

use super::fire::Fire;
use super::shader::*;

pub struct App {
    pub gl: GL,
    width: u32,
    height: u32,
    pub shader_program: WebGlProgram,
    pub model: Fire,
}

impl App {
    pub fn new(height: u32, width: u32, model: Fire) -> Result<Self, JsValue> {
        let window = Self::init_window()?;
        let document = Self::init_document(&window)?;
        let canvas = Self::init_canvas(width, height, &document)?;
        let gl = Self::init_gl(&canvas)?;
        let shader_program = Self::init_shader_program(&gl)?;

        Ok(Self {
            gl,
            width,
            height,
            shader_program,
            model: model,
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

    pub fn render(&mut self) -> Result<(), JsValue> {
        self.gl.use_program(Some(&self.shader_program));
        let (vertices, colors) = self.model.update();

        let vbo_data: &[&[f32]] = &[&vertices, &colors];
        let locations = &[0, 1];
        let vertex_count = vertices.len() as i32 / 3;
        let indices: &Vec<u16> = &(0..vertex_count).into_iter().map(|i| i as u16).collect();

        let vao = self.create_vao(vbo_data, locations, &indices, vertex_count)?;
        self.gl.bind_vertex_array(Some(&vao));

        let mvp_location = self
            .gl
            .get_uniform_location(&self.shader_program, "mvpMatrix")
            .ok_or("Failed to get uniform location")?;

        self.gl.enable(GL::DEPTH_TEST);
        self.gl.depth_func(GL::LEQUAL);
        self.gl.enable(GL::CULL_FACE);

        // 視点を定義
        self.send_mvp_matrix(&mvp_location);

        // 描画
        let index_count = indices.len() as i32;
        self.draw(index_count);

        Ok(())
    }

    fn send_mvp_matrix(&self, location: &WebGlUniformLocation) {
        let eye = glm::Vec3::new(0.0, 0.0, 3.0);
        let center = glm::Vec3::new(0.0, 0.0, 0.0);
        let up = glm::Vec3::new(0.0, 1.0, 0.0);
        let view_matrix = glm::look_at(&eye, &center, &up);

        let aspect = self.width as f32 / self.height as f32;
        let fovy = 45.0 * consts::PI / 180.0;
        let near = 0.1;
        let far = 10.0;
        let projection_matrix = glm::perspective(aspect, fovy, near, far);

        let mvp_matrix = projection_matrix * view_matrix;
        let mvp_arrays: [[f32; 4]; 4] = mvp_matrix.into();
        let mvp_matrices = mvp_arrays.iter().flat_map(|a| *a).collect::<Vec<_>>();

        self.gl
            .uniform_matrix4fv_with_f32_array_and_src_offset_and_src_length(
                Some(location),
                false,
                &mvp_matrices,
                0,
                0,
            );
    }

    fn create_vao(
        &self,
        vbo_data: &[&[f32]],
        locations: &[u32],
        ibo_data: &[u16],
        vertex_count: i32,
    ) -> Result<WebGlVertexArrayObject, String> {
        let vao = self
            .gl
            .create_vertex_array()
            .ok_or("Failed to create vertex array object")?;
        self.gl.bind_vertex_array(Some(&vao));

        for i in 0..vbo_data.len() {
            let vbo = self.gl.create_buffer().ok_or("Failed to create buffer")?;
            self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));
            unsafe {
                let view = js_sys::Float32Array::view(&vbo_data[i]);
                self.gl.buffer_data_with_array_buffer_view(
                    GL::ARRAY_BUFFER,
                    &view,
                    GL::STATIC_DRAW,
                );
            }
            self.gl.enable_vertex_attrib_array(locations[i]);
            let size = vbo_data[i].len() as i32 / vertex_count;
            self.gl
                .vertex_attrib_pointer_with_i32(locations[i], size, GL::FLOAT, false, 0, 0);
        }

        let ibo = self.gl.create_buffer().ok_or("Failed to create buffer")?;
        self.gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&ibo));
        unsafe {
            let view = js_sys::Uint16Array::view(ibo_data);
            self.gl.buffer_data_with_array_buffer_view(
                GL::ELEMENT_ARRAY_BUFFER,
                &view,
                GL::STATIC_DRAW,
            );
        }

        self.gl.bind_vertex_array(None);

        Ok(vao)
    }

    fn draw(&self, index_count: i32) {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear_depth(1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        self.gl
            .draw_elements_with_i32(GL::POINTS, index_count, GL::UNSIGNED_SHORT, 0);
        self.gl.flush();
    }
}
