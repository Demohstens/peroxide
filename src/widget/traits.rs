use crate::{PeroxideEvent, Widget};

#[allow(unused_variables)]

pub trait Interactable {
    fn on_click(&self, event: PeroxideEvent) {}
    fn on_hover(&self, callback: fn()) {}
    fn on_key_press(&self, callback: fn()) {}
    fn on_key_release(&self, callback: fn()) {}
    fn on_mouse_move(&self, callback: fn()) {}
    fn on_mouse_click(&self, callback: fn()) {}
    fn on_mouse_release(&self, callback: fn()) {}
}

/// A trait for widgets that have a state. This is used for widgets that need to maintain a state, such as buttons, checkboxes, etc.
/// The state is a generic type the developer can define themselves. 
#[allow(unused_variables)]
pub trait Stateful<T> {
    fn set_state(&mut self, state: T) {}
    fn get_state(&self) -> T  {
        todo!()
    }
}

pub trait Button: Interactable + Widget {
    /// Default implementation calls the callback immediately
    fn on_click(&self, event: PeroxideEvent) {
        todo!()
    }
    /// Default implementation calls the callback immediately
    fn on_hover(&self, callback: fn()) {
        callback();
    }
    fn width(&self) -> i32 {
        100 // Default width
    }
    fn height(&self) -> i32 {
        50 // Default height
    }
}