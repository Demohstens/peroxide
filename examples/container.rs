use std::rc::Rc;

use peroxide::{container, run_app};
use wgpu::Color;

fn main() {
    let con = peroxide::container!(
        width: 50,
        height: 50,
        color: Color::GREEN,
        child: Some(Rc::new("Hello World")),
    );
    run_app(peroxide::container!(
        x: 10,
        y: 10,
        width: 500,
        height: 500,
        color: Color::BLUE,
        child: Some(Rc::new(con)),
        decoration: None,
    ));
}