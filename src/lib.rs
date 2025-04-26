//! Library-level documentation
//!
//! This is where you put the main documentation for your crate.

pub mod event;
pub mod platform;
pub mod rendering;
pub mod widget;
pub mod window;

use std::sync::OnceLock;

pub use event::{EventType, PeroxideEvent};
#[cfg(target_os = "windows")]
use platform::windows;
pub use platform::Platform;
pub use rendering::render_object::RenderObject;
pub use widget::widget::*;
pub use window::app::App;

pub fn run_app<A: Widget + 'static>(root_widget: A) {
    let platform: Platform;
    #[cfg(target_os = "windows")]
    {
        windows::register_window_class();
        platform = Platform::Windows;
    }

    #[cfg(target_os = "macos")]
    {
        platform = Platform::MacOS;
    }

    // let win = run_app!(root_widget).unwrap();
    let mut app = App::new(root_widget, platform);
    // root_widget.draw();
    app.run().unwrap();
}
