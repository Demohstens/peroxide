use std::sync::OnceLock;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalPosition;
use winit::error::EventLoopError;
use winit::event::{ElementState, MouseButton, TouchPhase, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::window::{Window, WindowId};

use crate::event::{PointerButton, PointerEvent};
use crate::widget::WidgetTree;
use crate::{PeroxideEvent, Platform, Widget};

use super::State;
// pub struct Window {
//     pub window: Option<winitWindow>,
// }

pub static PROXY: OnceLock<EventLoopProxy<PeroxideEvent>> = OnceLock::new();

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
        let event_loop = EventLoop::with_user_event().build()?; // let event_loop = EventLoopBuilder::default().with_any_thread(true).build().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        let proxy: EventLoopProxy<PeroxideEvent> = event_loop.create_proxy();
        PROXY.set(proxy).unwrap();

        event_loop.run_app(self)
    }

    pub fn handle_event(&self, event: PeroxideEvent) {
        self.widgets.handle_event(event);
    }
    pub fn handle_pointer_event(&mut self, event: PointerEvent) {
        self.widgets.handle_pointer_event(event)
    }
}

pub struct App {
    pub platform: Platform,
    pub state: Option<State>,
    pub widgets: WidgetTree,
}

impl<'a> ApplicationHandler<PeroxideEvent> for App {
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
                state.window().request_redraw();
            }
            None => {
                println!("No state found; cannot resume");
                event_loop.exit();
            }
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: PeroxideEvent) {
        self.handle_event(event);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                state.resize(new_size);
            }
            WindowEvent::CursorEntered { device_id } => {
                let pointer_event = PointerEvent::Entered(device_id);
                self.handle_pointer_event(pointer_event);
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                let pointer_event = PointerEvent::Move(device_id, position);
                self.handle_pointer_event(pointer_event);
            }
            WindowEvent::CursorLeft { device_id } => {
                let pointer_event = PointerEvent::Leave(device_id);
                self.handle_pointer_event(pointer_event);
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                let pointer_button: PointerButton = match button {
                    MouseButton::Left => PointerButton::Primary,
                    MouseButton::Right => PointerButton::Secondary,
                    MouseButton::Middle => PointerButton::Tertiary,
                    MouseButton::Back => PointerButton::Quaternary,
                    MouseButton::Forward => PointerButton::Quinary,
                    MouseButton::Other(id) => PointerButton::Other(id),
                };
                let pointer_event = match state {
                    ElementState::Pressed => PointerEvent::Down(device_id, pointer_button),
                    ElementState::Released => PointerEvent::Down(device_id, pointer_button),
                };
                self.handle_pointer_event(pointer_event);
            }
            WindowEvent::Touch(touch) => {
                unimplemented!()
            }
            WindowEvent::RedrawRequested => match &mut self.state {
                Some(state) => {
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
            _ => {}
        }
    }
}
