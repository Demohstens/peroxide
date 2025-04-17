//! Library-level documentation
//! 
//! This is where you put the main documentation for your crate.


pub mod window;
pub mod widget;
pub mod platform;
pub mod rendering;

use std::sync::OnceLock;

#[cfg(target_os = "windows")]
use platform::windows;
pub use window::app::App;
pub use platform::Platform;
pub use widget::widget::*;
pub use rendering::render_object::RenderObject;
use winit::event_loop::EventLoopProxy;



pub fn run_app<A: Widget + 'static >(root_widget: A) {
    let platform: Platform;
    #[cfg(target_os = "windows")]
    windows::register_window_class();

    platform = Platform::Windows;

    // let win = run_app!(root_widget).unwrap();
    let mut app = App::new(root_widget, platform);
    // root_widget.draw();
    app.run().unwrap();

}

