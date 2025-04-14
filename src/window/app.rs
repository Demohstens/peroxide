use std::f32::consts::E;
use std::fmt::{Debug, Error};
use std::rc::Rc;
use wgpu::SurfaceTarget;
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopBuilder};
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::{Window, WindowId};

use crate::Widget;

use super::State;
// pub struct Window {
//     pub window: Option<winitWindow>,
// }

impl App {
    pub fn new<A: Widget + 'static>(root: A) -> Self {
        App { state: None, root_widget: Box::new(root) }
    }

    pub fn run(&mut self) -> Result<(), EventLoopError> {
        let event_loop = EventLoop::new().unwrap();
        // let event_loop = EventLoopBuilder::default().with_any_thread(true).build().unwrap();

        event_loop.set_control_flow(ControlFlow::Wait);

        event_loop.run_app(self)
    }
}

pub struct App {
    pub state: Option<State>,
    pub root_widget: Box<dyn Widget>,
}

impl<'a> ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        self.state = Some(pollster::block_on(State::new(window, self.root_widget.as_ref())));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => match &self.state {
                Some(state) => state.window().request_redraw(),
                None => {
                    println!("No state found; cannot redraw");
                    event_loop.exit();
                }
            },
            _ => (),
        }
    }
}
