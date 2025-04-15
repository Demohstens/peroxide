use peroxide::{run_app, widget::{style::decoration::BoxDecoration, widgets::{button::Button, container::Container}}};

fn main() {
    run_app(Container{
        parent: None,
        child: Some(Box::new(Button {
            parent: None,
            child: Some(Box::new("Hello World!")),
            text: "Hello".to_string(),
            x: 200,
            y: 50,
            width: 100,
            height: 30,
            id: 1,
        })),
        x: 200,
        y: 50,
        width: 1000,
        height: 500,
        id: 1,
        border: true,
        color: windows::Win32::Foundation::COLORREF(0x0000FFF), // Blue color
        decoration: Some(BoxDecoration::new(None, Some("0x0000FFF".to_owned()), None)),
    });
}

macro_rules! container {
    {
        parent: $parent:expr,
        child: $child:expr,
        x: $x:expr,
        y: $y:expr,
        width: $width:expr,
        height: $height:expr,
        id: $id:expr,
        border: $border:expr,
        color: $color:expr,
        decoration: $decoration:expr
    } => {
        Container {
            parent: $parent,
            child: $child,
            x: $x,
            y: $y,
            width: $width,
            height: $height,
            id: $id,
            border: $border,
            color: $color,
            decoration: $decoration.unwrap_or(None),
        }        
    };
}