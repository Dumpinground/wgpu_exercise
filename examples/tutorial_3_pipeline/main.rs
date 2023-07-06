use std::panic::set_hook;

use pollster::block_on;
use winit::{event_loop::EventLoop, window::Window};

mod challenge;
mod standard;

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    if cfg!(feature = "challenge") {
        pollster::block_on(challenge::run());
    } else {
        pollster::block_on(standard::run());
    }

    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        use winit::platform::web::WindowExtWebSys;

        
    }
}
