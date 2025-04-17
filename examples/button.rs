use peroxide::{widget::widgets::button::Button, Widget};
use wgpu::Color;

fn main() {
    let btn: Button = macros::button!(
        x: 50,
        y: 20,
        width: 100,
        color: Color::RED,
        child: Some(Box::new("Click me")),
        height: 40,
        id: 2,
        on_click: Some(Box::new(|_| {
            println!("Button clicked!");
        }))
    );
    peroxide::run_app(btn);
}