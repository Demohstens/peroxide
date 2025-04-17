use std::rc::Rc;

use peroxide::run_app;
use wgpu::Color;

fn main() {
    let con = macros::container!(
        id: 0,
        width: 50,
        height: 50,
        color: Color::GREEN,
        children: Rc::new(vec![(Rc::new("Hello"))]),
    );
    let con2 = macros::container!(
        id: 1,
        x: 0,
        y: 0,
        width: 400,
        height: 100,
        color: Color::RED,
        children: Rc::new(vec![(Rc::new("Hello"))]),
    );
    
    run_app(macros::container!(
        id: 2,
        x: 10,
        y: 10,
        width: 500,
        height: 500,
        color: Color::BLUE,
        children: Rc::new(vec![
            Rc::new(con),
            Rc::new(con2),
            ]),
        decoration: None,
    ));
}