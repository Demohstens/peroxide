//! Library-level documentation
//! 
//! This is where you put the main documentation for your crate.


pub mod window;
pub mod widget;
use std::fmt::Error;

pub use window::window::Window;
pub use widget::widget::*;

macro_rules! runApp  {
    ($widget:ty) => {
        $window:expr = Window::run().unwrap();
        return Ok(())
    };
}
// pub fn runApp(w: dyn Widget) -> Result<(), Error> {
//     let window = Window::run().unwrap();
//     Ok(())
// }

pub fn runApp() -> Result<(), Error> {
    runApp!(Window)
}