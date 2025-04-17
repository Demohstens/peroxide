use std::time::{self, Instant};

/// If an event is not handled, it will be passed to the default handler.
/// This is the default handler for all events.
/// key_code, device_id, widget_id, and id can all be 0 meaning they are unused.
#[derive(Debug)]
pub struct PeroxideEvent {
    pub event_type: EventType,
    pub x: i32,
    pub y: i32,
    pub timestamp: Instant,
    pub widget_id: u32,
    pub device_id: u32,
    pub key_code: u32,
    pub id: u32,
}

impl PeroxideEvent {
    #[allow(non_snake_case)]
    pub fn MouseUp(x: i32, y: i32) -> Self {
        PeroxideEvent {
            event_type: EventType::MouseUp,
            x,
            y,
            timestamp: time::Instant::now(),
            widget_id: 0,
            device_id: 0,
            key_code: 0x01 | 0x02 | 0x04, // left, right, middle
            id: 0,
        }
    }

    #[allow(non_snake_case)]
    pub fn MouseMove(x: i32, y: i32) -> Self {
        PeroxideEvent {
            event_type: EventType::MouseMove(x as u32, y as u32),
            x,
            y,
            timestamp: time::Instant::now(),
            widget_id: 0,
            device_id: 0,
            key_code: 0x01 | 0x02 | 0x04, // left, right, middle
            id: 0,
        }
    }
}

/// Indicates the type of event that occurred and passes necessary data.
#[derive(Debug)]
pub enum EventType {
    MouseMove(u32, u32),
    MouseClick,
    MouseUp,
    KeyPress,
    KeyRelease,
    Resize(u32, u32),
    Close,
}
