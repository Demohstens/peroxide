//! Library-level documentation
//! 
//! This is where you put the main documentation for your crate.


pub mod window;
pub mod widget;
use std::fmt::Error;

pub use window::window::Window;
pub use widget::widget::*;


#[macro_export]
macro_rules! run_app  {
    ($widget:ty) => {{
        let window = Window::run().unwrap();
        Ok(window)
    }};
}
// pub fn runApp(w: dyn Widget) -> Result<(), Error> {
//     let window = Window::run().unwrap();
//     Ok(())
// }

pub fn run_app<A: Widget>(root_widget: A) -> Result<Window, Error> {
    run_app!(root_widget)
}

