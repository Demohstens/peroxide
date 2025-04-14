use peroxide::{button, widget::widgets::button::Button, Widget};

fn main() {
    let btn: Button = button!(
        parent: None,
        child: "Hello World!",
        text: "Hello",
        x: 500,
        y: 50,
        width: 100,
        height: 30,
        id: 1,
    );
    peroxide::run_app(btn);
}