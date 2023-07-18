use winit::{dpi::PhysicalSize, event::WindowEvent, window::{WindowId, WindowBuilder}, event_loop::EventLoop};

pub trait Action {
    fn new(app: app_surface::AppSurface) -> Self;
    fn get_adapter_info(&self) -> wgpu::AdapterInfo;
    fn current_window_id(&self) -> WindowId;
    fn resize(&mut self, size: &PhysicalSize<u32>);
    fn request_redraw(&mut self);
    fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }
    fn update(&mut self) {}
    fn render(&mut self) -> Result<(), wgpu::SurfaceError>;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run<A: Action + 'static>(
    wh_ratio: Option<f32>,
    _html_canvas_container_id: Option<&'static str>,
) {
    use winit::event_loop;

    env_logger::init();

}

pub fn run<A: Action + 'static>(
    wh_ratio: Option<f32>,
    html_canvas_container_id: Option<&'static str>,
) {
    use wasm_bindgen::prelude::*;

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn).expect("无法初始化日志库");

    wasm_bindgen_futures::spawn_local(async move {
        let (event_loop, instance) = create_action_instance::<A>(wh_ratio, html_canvas_container_id).await;
        let run_closure = Closure::once_into_js(move || start_event_loop::<A>(event_loop, instance));
    });
}

pub fn create_action_instance<A: Action + 'static>(
    wh_ratio: Option<f32>,
    html_canvas_container_id: Option<&'static str>,
) -> (EventLoop<()>, A) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let scale_factor = window.scale_factor() as f32;

    let height = (if cfg!(target_arch = "wasm32") { 550. } else { 600. } * scale_factor) as u32;

    let width = if let Some(ratio) = wh_ratio {
        (height as f32 * ratio) as u32
    } else {
        height
    };

    if cfg!(not(target_arch = "wasm32")) {
        
    }
}