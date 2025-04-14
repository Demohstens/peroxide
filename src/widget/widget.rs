use std::fmt::Error;

use windows::Win32::Foundation::HWND;


/// A Widget defines a UI element and can be implemented on basically any struct. 
/// It requires the `draw` method to be implemented, which takes a parent window handle and returns a handle to the created widget. It should itself draw it's children, if any.
/// Be aware that a proper Widget struct SHOULD but does not NEED to have a parent and children as well as other paramenters requires by native components.
/// Example Widget struct:
/// ```rust
/// struct MyWidget {
///    parent: Option<Box<dyn Widget>>, // use Option<Box<dyn Widget>> to allow for dynamic dispatch
///    child: Option<Box<dyn Widget>>, 
///    text: String, // Text to be rendered onto the widget
///    x: i32, // X position of the widget relative to the parent widget
///    y: i32, 
///    width: i32, // Width of the widget
///    height: i32, // Height of the widget
///    id: i32, // ID of the widget. Not used yet.
/// }
pub trait Widget  {
    /// Draws the widget and returns a handle to the created widget.
    /// The parent window handle is passed as an argument.
    /// The widget should draw its children, if it has any.
    fn draw(&self, hwnd_parent: HWND)-> HWND; 
}