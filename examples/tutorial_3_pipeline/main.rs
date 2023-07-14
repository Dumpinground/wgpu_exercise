use std::panic::set_hook;

use pollster::block_on;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::{event_loop::EventLoop, window::Window};

mod challenge;
mod standard;

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();

    // #[cfg(not(target_arch = "wasm32"))]
    if cfg!(feature = "challenge") {
        pollster::block_on(challenge::run());
    } else {
        pollster::block_on(standard::run());
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn wasm_main() {
    if cfg!(feature = "challenge") {
        challenge::run().await;
    } else {
        standard::run().await;
    }
}
