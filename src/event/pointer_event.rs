use winit::{dpi::PhysicalPosition, event::DeviceId};

/// Indicates the type of event that occurred and passes necessary data.
pub enum PointerEvent {
    /// Called when a pointer enters a window peroxide is responsible for.
    /// Carries: DeviceId
    Entered(DeviceId),
    /// Called when a (assumadly tracked) pointer moves.
    /// Carries: DeviceId, physical position
    Move(DeviceId, PhysicalPosition<f64>),
    /// Called when a pointer comes in Contact with the element.
    /// Carries: DeviceId, Button Pressed
    Down(DeviceId, PointerButton),
    /// Called when a pointer is lifted/released.
    /// Carries: DeviceId
    Up(DeviceId, PointerButton),
    /// Called when a pointer leaves the element
    /// Carries: DeviceId
    Leave(DeviceId),
    /// Called when a pointer pressed with intent.
    /// Carries: ButtonID
    KeyPress(u32),
    /// Called when a pointer released with intent.
    /// Carries: ButtonID
    KeyRelease(u32),
}

pub enum PointerButton {
    Touch,
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
    Quinary,
    // Provide the index or ID and handle this yourself.
    Other(u16),
}
