use std::time::{self, Instant};

use winit::event::DeviceId;

use crate::event::{PointerButton, PointerEvent};
/// If an event is not handled, it will be passed to the default handler.
/// This is the default handler for all events.
/// key_code, device_id, widget_id, and id can all be 0 meaning they are unused.
pub struct PeroxideEvent {
    pub event_type: PeroxideEventType,
    pub x: f64,
    pub y: f64,
    pub timestamp: Instant,
    pub widget_id: u32,
    pub device_id: DeviceId,
    pub id: u32,
}

impl PeroxideEvent {
    #[allow(non_snake_case)]
    pub fn PointerUp(
        button: PointerButton,
        x: f64,
        y: f64,
        widget_id: u32,
        device_id: DeviceId,
        id: u32,
    ) -> Self {
        PeroxideEvent {
            event_type: PeroxideEventType::PointerUp(button),
            x,
            y,
            timestamp: time::Instant::now(),
            widget_id,
            device_id,
            id,
        }
    }
    #[allow(non_snake_case)]
    pub fn PointerDown(
        button: PointerButton,
        x: f64,
        y: f64,
        widget_id: u32,
        device_id: DeviceId,
        id: u32,
    ) -> Self {
        PeroxideEvent {
            event_type: PeroxideEventType::PointerDown(button),
            x,
            y,
            timestamp: time::Instant::now(),
            widget_id,
            device_id,
            id,
        }
    }
    #[allow(non_snake_case)]
    pub fn PointerEnter(widget_id: u32, device_id: DeviceId, id: u32) -> Self {
        PeroxideEvent {
            event_type: PeroxideEventType::PointerEnter,
            x: 0.0, // TODO find a way to handle enter events without coordinates
            y: 0.0,
            timestamp: time::Instant::now(),
            widget_id,
            device_id,
            id,
        }
    }

    #[allow(non_snake_case)]
    pub fn PointerLeave(widget_id: u32, device_id: DeviceId, id: u32) -> Self {
        PeroxideEvent {
            event_type: PeroxideEventType::PointerLeave,
            x: 0.0, // TODO find a way to handle enter events without coordinates
            y: 0.0,
            timestamp: time::Instant::now(),
            widget_id,
            device_id,
            id,
        }
    }

    #[allow(non_snake_case)]
    pub fn PointerMove(x: f64, y: f64, widget_id: u32, device_id: DeviceId, id: u32) -> Self {
        PeroxideEvent {
            event_type: PeroxideEventType::PointerMove,
            x,
            y,
            timestamp: time::Instant::now(),
            widget_id,
            device_id,
            id,
        }
    }
}

pub enum PeroxideEventType {
    PointerEnter,
    PointerLeave,
    PointerDown(PointerButton),
    PointerMove,
    PointerUp(PointerButton),
}
