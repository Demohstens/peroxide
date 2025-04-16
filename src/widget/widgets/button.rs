use std::os::raw::c_void;

use crate::{window::State, Widget};


pub struct Button {
    pub parent: Option<Box<dyn Widget>>,
    pub child: Option<Box<dyn Widget>>,
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub id: i32,
}

impl Widget for Button {
    fn width(&self) -> i32 {
        self.width
    }
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y 
    }

    fn height(&self) -> i32 {
        self.height
    }
}
#[macro_export]
macro_rules! button {
    {
        parent: $parent:expr,
        child: $child:expr,
        text: $text:expr,
        x: $x:expr,
        y: $y:expr,
        width: $width:expr,
        height: $height:expr,
        id: $id:expr $(,)?
    } => {
        Button {
            parent: $parent,
            child: Some(Box::new($child)),
            text: $text.to_string(),
            x: $x,
            y: $y,
            width: $width,
            height: $height,
            id: $id,
        }
    }
}

