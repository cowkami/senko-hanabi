mod app;
mod shader;

extern crate nalgebra_glm as glm;

use app::App;
use std::{cell::RefCell, f32::consts, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{WebGl2RenderingContext as GL, *};

const WIDTH: u32 = 768;
const HEIGHT: u32 = 768;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let app = App::new(WIDTH, HEIGHT)?;

    let vertices = get_vertices();
    let colors = get_colors();
    app.render(&vertices, &colors)?;

    // let mut frame_count = 0;

    // let closure = Rc::new(RefCell::new(None));
    // let clone = closure.clone();
    // *clone.borrow_mut() = Some(Closure::<dyn FnMut() -> Result<i32, JsValue>>::new(
    //     move || {
    //         frame_count += 1;
    //         draw(&gl, index_count);
    //         request_animation_frame(closure.borrow().as_ref().unwrap())
    //     },
    // ));
    // request_animation_frame(clone.borrow().as_ref().unwrap())?;

    Ok(())
}

fn get_vertices() -> Vec<f32> {
    vec![
        // 前面
        -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, // 背面
        -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5, -0.5, // 上面
        -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, // 下面
        -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5, // 右面
        0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, // 左面
        -0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5,
    ]
}

fn get_colors() -> Vec<f32> {
    [
        1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0,
    ]
    .repeat(6)
}

fn request_animation_frame(
    closure: &Closure<dyn FnMut() -> Result<i32, JsValue>>,
) -> Result<i32, JsValue> {
    let window = web_sys::window().unwrap();
    window.request_animation_frame(closure.as_ref().unchecked_ref())
}
