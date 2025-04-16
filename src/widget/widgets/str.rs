use wgpu::Color;


use crate::{window::State, Widget};

impl Widget for &str {
    fn width(&self) -> i32 {
        100 
    }
    fn height(&self) -> i32 {
        100 
    }
    fn color(&self) -> Color {
        Color::WHITE
    }
    fn x(&self) -> i32 {
        0 
    }
    fn y(&self) -> i32 {
        0 
    }
   
}