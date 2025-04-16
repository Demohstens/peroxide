//! Library-level documentation
//! 
//! This is where you put the main documentation for your crate.


pub mod window;
pub mod widget;
pub mod platform;
pub mod rendering;

use platform::windows;
pub use window::app::App;
pub use platform::Platform;
pub use widget::widget::*;
pub use rendering::render_object::RenderObject;


pub fn run_app<A: Widget + 'static >(root_widget: A) {
    let platform = if cfg!(target_os = "windows") {
        Platform::Windows
    } else if cfg!(target_os = "linux") {
        Platform::Linux
    } else {
        todo!("Platform not supported")
    };
    match platform {
        Platform::Windows => {
            windows::register_window_class();
        }
        _ => {
            // other specific code here
        }
    }
    // let win = run_app!(root_widget).unwrap();
    let mut app = App::new(root_widget, platform);
    // root_widget.draw();
    app.run().unwrap();

}

