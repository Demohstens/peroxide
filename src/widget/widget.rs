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

/// Generates a widget struct with the given name and fields. 
/// Requires the `from` keyword to specify the parent type. This has to be a built-in Widget type. 
/// example:
/// ```rust
/// widget! {
///     MyWidget from Button {
///         text: String,  
///         x: i32,
///         y: i32,
///     }
#[macro_export]
macro_rules! widget {
    (
        $name:ident from $super: ty {
            $($field:ident : $type:ty),* $(,)?
        }
    ) => {  
        /// A widget struct with the given name and fields.
        /// The widget struct is a child of the given parent type.
        ;
        struct $name {
            $($field: $type),*
        }
        impl $name {
            fn new() -> Self {
                Self {
                    $($field: Default::default()),*
                }
            }
        }
    };
    {
        $name: ident $($rest:tt)*
    } => {
        compile_error!(concat!("Invalid widget definition: ", stringify!($name), " you're likely missing the 'from' keyword"));
    }; 
}