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
pub trait Widget {
    /// Draws the widget and returns a handle to the created widget.
    /// The parent window handle is passed as an argument.
    /// The widget should draw its children, if it has any.
    fn draw(&self, hwnd_parent: HWND) -> HWND;
    fn children(&self) -> Vec<*mut dyn Widget> {
        vec![] // Default implementation returns an empty vector
    }

    fn width(&self) -> i32 {
        0 // Default implementation returns 0
    }
    fn height(&self) -> i32 {
        0 // Default implementation returns 0
    }
    fn color(&self) -> String {
        "0x000000".to_string() // Default implementation returns black color
    }
    fn id(&self) -> i32 {
        0 // Default implementation returns 0
    }
    fn parent(&self) -> Option<Box<dyn Widget>> {
        None // Default implementation returns None
    }
}

/// Generates a widget struct with the given name and fields.
/// Requires the `from` keyword to specify the parent type. This has to be a built-in Widget type.
/// example:
/// ```rust
/// peroxide::widget! {
///     MyWidget from peroxide::Button {
///         text: String,  
///         x: i32,
///         y: i32,
///     }
/// }
#[macro_export]
macro_rules! widget {
    (
        $name: literal {
            $(parent: $parent:ident,)?
            $(child: $child:ident,)?
            $($field:ident : $type:ty),* $(,)?
        }
    ) => {
        match $name {
            Button =>
                peroxide::widget::Button {
                    parent: None,
                    child: None,
                    text: Default::default(),
                    x: Default::default(),
                    y: Default::default(),
                    width: Default::default(),
                    height: Default::default(),
                    id: 0,

            }
        }
    };
    {
        $name: ident from $parent:ty {
            $($field:ident : $type:ty),* $(,)?
        }
    } => { {
        use peroxide::Widget;
        use windows::Win32::Foundation::HWND;
        pub struct $name {
            parent: Option<Box<dyn Widget>>,
            child: Option<Box<dyn Widget>>,
            id: i32,
            $($field: $type),*
        }
        impl Widget for $name {
            fn draw(&self, hwnd_parent: HWND) -> HWND {
                todo!()
            }
        }
        $name {
            parent: None,
            child: None,
            $($field: Default::default()),*
            id: 0,
        }
        }
    };

}
