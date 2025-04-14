//! Library-level documentation
//! 
//! This is where you put the main documentation for your crate.


pub mod window;
pub mod widget;
use std::fmt::Error;

pub use window::window::Window;
pub use widget::widget::*;
use window::State;




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

pub async fn run_app<A: Widget>(root_widget: A) {
    // let win = run_app!(root_widget).unwrap();
    let win = Window::run().unwrap();
    let winitWindow = win.window.as_ref().unwrap();
    let state = State::new(winitWindow);

}

