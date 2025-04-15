use std::f32::consts::E;
use std::fmt::{Debug, Error};
use std::rc::Rc;
use std::sync::Arc;
use wgpu::SurfaceTarget;
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopBuilder};
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::{Window, WindowId};

use crate::widget::WidgetTree;
use crate::{Platform, Widget};

use super::State;
// pub struct Window {
//     pub window: Option<winitWindow>,
// }

impl App {
    pub fn new<A: Widget + 'static>(root: A, platform: Platform) -> Self {
        let widgets = WidgetTree::new(Box::new(root));
        App {
            state: None,
            platform,
            widgets,
        }
    }

    pub fn run(&mut self) -> Result<(), EventLoopError> {
        let event_loop = EventLoop::new().unwrap();
        // let event_loop = EventLoopBuilder::default().with_any_thread(true).build().unwrap();

        event_loop.set_control_flow(ControlFlow::Wait);

        event_loop.run_app(self)
    }

    pub fn handle_event(&self, event: WindowEvent) {
        match event {
            WindowEvent::CursorEntered { device_id, .. } => {
                println!("Cursor entered the window; device ID: {:?}", device_id);
            }
            _ => (),
        }
    }
}

pub struct App {
    pub platform: Platform,
    pub state: Option<State>,
    pub widgets: WidgetTree,
}

impl<'a> ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        self.state = Some(pollster::block_on(State::new(
            window,
            &mut self.widgets,
            &self.platform,
        )));
        match &self.state {
            Some(state) => {
                state.window().set_visible(true);
                state.window().request_redraw();
            }
            None => {
                println!("No state found; cannot resume");
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                self.state.as_mut().unwrap().resize(new_size);
            }
            WindowEvent::RedrawRequested => match &mut self.state {
                Some(state) =>  {

                    state.window().request_redraw();
                    state.update();

                    match state.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            state.resize(state.size)
                        }
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
                            log::error!("OutOfMemory");
                            event_loop.exit();
                        }

                        // This happens when the a frame takes too long to present
                        Err(wgpu::SurfaceError::Timeout) => {
                            log::warn!("Surface timeout")
                        }
                    }
                }

                None => {
                    println!("No state found; cannot redraw");
                    event_loop.exit();
                }
            },
            event => self.handle_event(event),
        }
    }
}
