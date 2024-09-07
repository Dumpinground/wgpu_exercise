pub struct AppSurface {
    pub view: winit::window::Window,
    pub scale_factor: f32,
    pub maximum_frames: i32,
    pub sdq: crate::SurfaceDeviceQueue,
    pub callback_to_app: Option<extern "C" fn(arg: i32)>,
    pub temporary_directory: &'static str,
    pub library_directory: &'static str,
}

impl AppSurface {

    pub async fn new(view: winit::window::Window) -> Self {
        let scale_factor = view.scale_factor();
        let default_backends = if cfg!(feature = "webgl") {
            wgpu::Backends::all()
        } else {
            wgpu::Backends::PRIMARY
        };
        let backend = wgpu::util::backend_bits_from_env().unwrap_or(default_backends);
        let dx12_shader_compiler = wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: backend, dx12_shader_compiler: dx12_shader_compiler });
        let physical = view.inner_size();
        let surface = unsafe {
            let surface = instance.create_surface(&view);
            match surface {
                Ok(surface) => surface,
                Err(e) => {
                    panic!("Failed to create surface: {e:?}");
                }
            }
        };
        todo!()
    }
}

