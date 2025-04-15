use peroxide::{button, widget::widgets::button::Button, Widget};

fn main() {
    let btn: Button = Button {
        parent: None,
        child: Some(Box::new("Hello World!")),
        text: "Hello".to_string(),
        x: 200,
        y: 50,
        width: 100,
        height: 30,
        id: 1,
    };
    peroxide::run_app(btn);
}