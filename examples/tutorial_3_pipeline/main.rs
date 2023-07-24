use wgpu_exercise::log_print;
use winit::{event_loop::EventLoop, window::Window};

mod challenge;
mod standard;

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(app(event_loop, window));
    }

    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
        wasm_bindgen_futures::spawn_local(app(event_loop, window));
    }
}

async fn app(event_loop: EventLoop<()>, window: Window) {
    if cfg!(feature = "challenge") {
        log_print("run challenge example");
        challenge::run(event_loop, window).await
    } else {
        log_print("run standard example");
        standard::run(event_loop, window).await
    };
}
