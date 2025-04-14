use std::f32::consts::E;
use std::fmt::{Debug, Error};
use wgpu::SurfaceTarget;
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopBuilder};
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::{Window as winitWindow, WindowId};
pub struct Window {
    pub window: Option<winitWindow>,
}


impl Window {
    fn new() -> Self {
        Window { window: None }
    }

    pub fn run( ) -> Result<Self, EventLoopError>  {
        // let event_loop = EventLoop::new().unwrap();
        let event_loop = EventLoopBuilder::default().with_any_thread(true).build().unwrap();


        event_loop.set_control_flow(ControlFlow::Wait);

        let mut window = Window::new();
            
        let result = event_loop.run_app(&mut window);
        if let Err(e) = result {
            eprintln!("Error running the application: {:?}", e);
            return Err(e.into());    
        }

        Ok(window)
    }
}

impl<'a> From<Window> for SurfaceTarget<'a> {
    fn from(window: Window) -> Self {
        window.window.unwrap().into()
    }
}

impl Window {
    pub fn inner_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window.as_ref().unwrap().inner_size()
    }
}

impl ApplicationHandler for Window {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(winitWindow::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
