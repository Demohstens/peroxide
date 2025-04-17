use std::rc::Rc;

use crate::{Widget, widget::style::decoration::BoxDecoration};
use wgpu::Color;

pub struct Container {
    pub parent: Option<Box<dyn Widget>>,
    pub child: Option<Rc<dyn Widget>>,
    pub border: bool,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub color: Color, // Color of the container in RGB format
    pub id: i32,
    pub decoration: Option<BoxDecoration>,
}
impl Container {
    pub fn new() -> Self {
        Self {
            parent: None,
            child: None,
            border: false,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            color: Color::WHITE, // Default to white
            id: 0,
            decoration: None,
        }
    }
}

impl Widget for Container {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }
    fn color(&self) -> Color {
        self.color
    }
    fn id(&self) -> i32 {
        self.id
    }
    fn children(&self) -> Vec<Rc<dyn Widget>> {
        match &self.child {
            Some(child) => vec![Rc::clone(child)],
            None => vec![],
        }
    }
}

/// Generates a container struct with the given name and fields.
/// Requires:
/// width: i32
/// height: i32
/// color: Color
/// child: Option<Rc<dyn Widget>>
#[macro_export]
macro_rules! container {
    {
        width: $width:expr,
        height: $height:expr,
        color: $color:expr,
        child: $child:expr,
    } => {
        peroxide::widget::Container {
            height: $height,
            width: $width,
            color: $color,
            parent: None,
            child: $child,
            border: false,
            x: 0,
            y: 0,
            id: 0,
            decoration: None,
        };
    };
    {
        x: $x:expr,
        y: $y:expr,
        width: $width:expr,
        height: $height:expr,
        color: $color:expr,
        child: $child:expr,
        decoration: $decoration:expr,
    } => {
        peroxide::widget::Container {
            height: $height,
            width: $width,
            color: $color,
            parent: None,
            child: $child,
            border: false,
            x: $x,
            y: $y,
            id: 0,
            decoration: $decoration,

        };
    }
}
