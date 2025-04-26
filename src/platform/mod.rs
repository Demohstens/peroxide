pub mod platform;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

pub use platform::Platform;

#[cfg(target_os = "windows")]
pub use windows::{class_name, window_class};
