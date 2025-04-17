use crate::{PeroxideEvent, Widget, widget::Interactable};

pub struct Button {
    pub parent: Option<Box<dyn Widget>>,
    pub child: Option<Box<dyn Widget>>,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub color: wgpu::Color,
    pub id: i32,
    pub on_click: Option<Box<dyn Fn(PeroxideEvent)>>,
    pub decoration: Option<Box<dyn Widget>>,    
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

impl Interactable for Button {
    fn on_click(&self, event: PeroxideEvent) {
        if let Some(callback) = &self.on_click {
            callback(event);
        } else {
            println!("Button clicked: {}", self.id);
        }
    }
}


