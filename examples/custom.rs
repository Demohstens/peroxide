use std::rc::Rc;

use peroxide::widget::widgets::container::Container;
use wgpu::Color;

fn main() {
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
        child: Some(Rc::new(Container {
            parent: None,
            child: Some(Rc::new(btn)),
            x: 0,
            y: 0,
            width: 400,
            height: 100,
            id: 3,
            decoration: None,
            color: Color::RED, // Blue color
            border: true,
        })),
        x: 50,
        y: 50,
        width: 500,
        height: 500,
        id: 1,
        border: true,
        color: Color::BLUE, // Blue color
        decoration: None,
    };

    peroxide::run_app(container);
}
