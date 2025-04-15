use peroxide::{run_app, widget::widgets::{button::Button, container::Container}};

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
    });
}