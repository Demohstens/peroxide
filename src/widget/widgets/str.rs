use wgpu::Color;

use crate::{Widget, window::State};

impl Widget for &str {
    fn width(&self) -> f64 {
        100.0
    }
    fn height(&self) -> f64 {
        100.0
    }
    fn color(&self) -> Color {
        Color::WHITE
    }
    fn x(&self) -> f64 {
        0.0
    }
    fn y(&self) -> f64 {
        0.0
    }
}
