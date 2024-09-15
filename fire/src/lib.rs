mod app;
mod fire;
mod shader;

use app::App;
use fire::Fire;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;

const WIDTH: u32 = 2560;
const HEIGHT: u32 = 1600;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let fire = Fire::new();
    let mut app = App::new(WIDTH, HEIGHT, fire)?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i > 1000 {
            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }

        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        i += 1;

        // Schedule ourself for another requestAnimationFrame callback.
        let _ = app.render();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn request_animation_frame(closure: &Closure<dyn FnMut()>) {
    let window = web_sys::window().expect("should have a window in this context");
    window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
