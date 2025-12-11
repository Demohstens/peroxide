use crate::{PeroxideEvent, Widget, widget::Interactable};

pub struct Button {
    pub parent: Option<Box<dyn Widget>>,
    pub child: Option<Box<dyn Widget>>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub color: wgpu::Color,
    pub id: i32,
    pub on_click: Option<Box<dyn Fn(PeroxideEvent)>>,
    pub decoration: Option<Box<dyn Widget>>,
}

impl Widget for Button {
    fn width(&self) -> f64 {
        self.width
    }
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }

    fn height(&self) -> f64 {
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
