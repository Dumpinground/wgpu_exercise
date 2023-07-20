use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, WindowId},
};

use crate::app_surface;

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

#[cfg(target_arch = "wasm32")]
pub fn run<A: Action + 'static>(
    wh_ratio: Option<f32>,
    html_canvas_container_id: Option<&'static str>,
) {
    use wasm_bindgen::prelude::*;

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn).expect("无法初始化日志库");

    wasm_bindgen_futures::spawn_local(async move {
        let (event_loop, instance) =
            create_action_instance::<A>(wh_ratio, html_canvas_container_id).await;
        let run_closure =
            Closure::once_into_js(move || start_event_loop::<A>(event_loop, instance));
    });
}

async fn create_action_instance<A: Action + 'static>(
    wh_ratio: Option<f32>,
    #[cfg(target_arch = "wasm32")] html_canvas_container_id: Option<&'static str>,
) -> (EventLoop<()>, A) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let scale_factor = window.scale_factor() as f32;

    let height = (if cfg!(target_arch = "wasm32") {
        550.
    } else {
        600.
    } * scale_factor) as u32;

    let width = if let Some(ratio) = wh_ratio {
        (height as f32 * ratio) as u32
    } else {
        height
    };

    if cfg!(not(target_arch = "wasm32")) {
        window.set_inner_size(PhysicalSize::new(width, height));
    }

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .map(|doc| {
                let element_id = if let Some(container_id) = html_canvas_container_id {
                    container_id
                } else {
                    "wasm-example"
                };
                match doc.get_element_by_id(&element_id) {
                    Some(dst) => {
                        let rect = dst.get_bounding_client_rect();
                        let width_limit = rect.width() as u32;
                        let width = width_limit as f32 * scale_factor;
                        let height = (width) as u32;
                        window.set_inner_size(PhysicalSize::new(width as u32, height));
                        let _ = dst.append_child(&web_sys::Element::from(window.canvas()));
                    }
                    None => {
                        window.set_inner_size(PhysicalSize::new(width, height));
                        let canvas = window.canvas();
                        canvas.style().set_css_text(
                            &(canvas.style().css_text()
                                + "background-color: black; display: block; margin: 20px auto;"),
                        );
                        doc.body()
                            .map(|body| body.append_child(&web_sys::Element::from(canvas)));
                    }
                };
            })
            .expect("Couldn't append canvas to document body.");
    };

    let app = app_surface::AppSurface::new(window).await;
    let instance = A::new(app);

    let adapter_info = instance.get_adapter_info();
    let gpu_info = format!(
        "正在使用 {}, 后端图形接口为 {:?}。",
        adapter_info.name, adapter_info.backend
    );

    #[cfg(not(target_arch = "wasm32"))]
    println!("{gpu_info}");
    #[cfg(target_arch = "wasm32")]
    log::warn!("{gpu_info:?}\n这不是一条警告");

    (event_loop, instance)
}

fn start_event_loop<A: Action + 'static>(event_loop: EventLoop<()>, instance: A) {
    let mut state = instance;
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.current_window_id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        if physical_size.width == 0 || physical_size.height == 0 {
                            println!("Window minimized!")
                        } else {
                            state.resize(physical_size);
                        }
                    }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor: _,
                        new_inner_size,
                    } => {
                        state.resize(new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == state.current_window_id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => eprintln!("Surface is lost"),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{e:?}"),
            }
        }
        Event::MainEventsCleared => {
            state.request_redraw();
        }
        _ => {}
    });
}
