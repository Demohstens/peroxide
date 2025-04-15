use peroxide::widget::widgets::container::Container;

fn main () {
    // Replace with the correct way to create a Button widget
    let btn = peroxide::widget::Button {
        parent: None,
        text: String::from("Click me"),
        x: 0,
        y: 0,
        width: 100,
        height: 40,
        id: 2,
        child: None,
    };
    let container = Container {
        parent: None,
        child: Some(Box::new(btn)),
        x: 200,
        y: 50,
        width: 1000,
        height: 500,
        id: 1,
        border: true,
        color: windows::Win32::Foundation::COLORREF(0x0000FFF), // Blue color
        decoration: None,
    };
    
    peroxide::run_app(container);

}