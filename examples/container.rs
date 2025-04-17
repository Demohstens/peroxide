use std::rc::Rc;

use peroxide::run_app;
use wgpu::Color;

fn main() {
    let con = macros::container!(
        width: 50,
        height: 50,
        color: Color::GREEN,
        child: Some(Rc::new("Hello World")),
    );
    
    run_app(macros::container!(
        x: 10,
        y: 10,
        width: 500,
        height: 500,
        color: Color::BLUE,
        child: Some(Rc::new(con)),
        decoration: None,
    ));
}