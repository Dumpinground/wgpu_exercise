use winit::event_loop::EventLoop;

mod challenge;
mod standard;

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        if cfg!(feature = "challenge") {
            pollster::block_on(challenge::run(event_loop, window));
        } else {
            pollster::block_on(standard::run(event_loop, window));
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        use winit::platform::web::WindowExtWebSys;
        web_sys::window().and_then(|win| win.document())
        .and_then(|doc| doc.body())
        .and_then(|body| {
            body.append_child(&web_sys::Element::from(window.canvas())).ok()
        }).expect("couldn't append canvas to document body");
        if cfg!(feature = "challenge") {
            wasm_bindgen_futures::spawn_local(challenge::run(event_loop, window));
        } else {
            wasm_bindgen_futures::spawn_local(standard::run(event_loop, window));
        }
    }
}

async fn app() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    if cfg!(target_arch = "challenge") {
        challenge::run(event_loop, window)
    } else {
        standard::run(event_loop, window)
    };
}
