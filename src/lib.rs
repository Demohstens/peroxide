//! Library-level documentation
//! 
//! This is where you put the main documentation for your crate.


pub mod window;
pub mod widget;

pub use window::app::App;
pub use widget::widget::*;




#[macro_export]
macro_rules! run_app  {
    ($widget:ty) => {{
        let window = Window::run().unwrap();
        Ok(window)
    }};
}


pub fn run_app<A: Widget + 'static >(root_widget: A) {
    // let win = run_app!(root_widget).unwrap();
    let mut app = App::new(root_widget);
    // root_widget.draw();
    app.run().unwrap();

}

