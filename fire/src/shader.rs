extern crate nalgebra_glm as glm;

use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext as GL, *};

pub fn create_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, JsValue> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| JsValue::from_str("Failed to create shader object"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let log = gl
            .get_shader_info_log(&shader)
            .ok_or_else(|| JsValue::from_str("Failed to get log in create shader object"))?;
        gl.delete_shader(Some(&shader));
        Err(JsValue::from_str(&log))
    }
}

pub fn link_program(
    gl: &GL,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, JsValue> {
    let program = gl
        .create_program()
        .ok_or_else(|| JsValue::from_str("Failed to create program object"))?;
    gl.attach_shader(&program, vertex_shader);
    gl.attach_shader(&program, fragment_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        let log = gl
            .get_program_info_log(&program)
            .ok_or_else(|| JsValue::from_str("Failed to get log in link shader program"))?;
        gl.delete_program(Some(&program));
        Err(JsValue::from_str(&log))
    }
}
