use wgpu::{
    Color, ExperimentalFeatures, SurfaceTexture, Texture,
    rwh::{HasWindowHandle, RawWindowHandle},
};

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HWND;
use winit::{event::WindowEvent, window::Window};

use crate::{Platform, rendering::canvas::Canvas, widget::WidgetTree};

pub struct State {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: &'static Window, // We use box to avoid lifetime issues with wgpu::Surface
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Window, widget_tree: &mut WidgetTree, platform: &Platform) -> State {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance;
        #[cfg(not(target_os = "macos"))]
        {
            instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                #[cfg(not(target_arch = "wasm32"))]
                backends: wgpu::Backends::PRIMARY,
                ..Default::default()
            });
        }
        #[cfg(target_os = "macos")]
        {
            instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                backends: wgpu::Backends::METAL,
                ..Default::default()
            });
        }
        let window_ref: &'static Window = Box::leak(Box::new(window));
        let surface = instance.create_surface(window_ref).unwrap();
        // The adapter is a handle to a specific GPU on the system

        #[cfg(target_os = "windows")]
        {
            let hwnd = match window_ref.window_handle().unwrap().as_raw() {
                RawWindowHandle::Win32(handle) => HWND(handle.hwnd.get() as *mut _),
                _ => panic!("Unsupported window handle type"),
            };

            widget_tree.draw(hwnd.0);
        }
        #[cfg(target_os = "macos")]
        {
            let ns_view = match window_ref.window_handle().unwrap().as_raw() {
                RawWindowHandle::AppKit(handle) => handle.ns_view,
                _ => panic!("Unsupported window handle type on macOS"),
            };
            widget_tree.draw(ns_view.as_ptr());
        }

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                experimental_features: ExperimentalFeatures::disabled(),
                required_features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web, we'll have to disable some.
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        println!("Surface format:{:?} {:?}", size.width, size.height);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        State {
            config,
            device,
            queue,
            surface,
            size,
            window: window_ref,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 || new_size.width > 16384 {
            return; // Ignore invalid M1 resize events[1][4]
        }
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        let texture = self.surface.get_current_texture().unwrap();
        let mut canvas = Canvas::new(self.config.width as usize, self.config.height as usize);
        canvas.fill(Color::GREEN);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    depth_slice: None, // Needs to be set for 3D
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
