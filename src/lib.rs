use std::{ops::Deref, sync::Arc};

use app_surface::AppSurface;
use winit::event::Touch;

mod app_surface;
mod framework;

pub fn log_print(text: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&text.into());

    #[cfg(not(target_arch = "wasm32"))]
    println!("{text}");
}

pub struct SurfaceDeviceQueue {
    pub surface: wgpu::Surface,
    pub config: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
}

impl SurfaceDeviceQueue {
    pub fn update_config_format(&mut self, format: wgpu::TextureFormat) {
        self.config.format = format;
        self.config.view_formats = vec![format.add_srgb_suffix(), format.remove_srgb_suffix()];
        self.surface.configure(&self.device, & self.config);
    }
}

impl Deref for AppSurface {
    type Target = SurfaceDeviceQueue;
    fn deref(&self) -> &Self::Target {
        &self.sdq
    }
}

pub trait SurfaceFrame {
    // fn view_size(&self) -> ViewSize;
    fn resize_surface(&mut self);
    fn pintch(&mut self, _touch: Touch) {}
    fn touch(&mut self, _touch: Touch) {}
    // TODO lots of methods
    // see github.com/jinleili/wgpu-in-app/blob/master/app-surface/src/lib.rs
}
