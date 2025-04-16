pub mod platform;

#[cfg(target_os = "windows")]
pub mod windows;

pub use platform::Platform;

#[cfg(target_os = "windows")]
pub use windows::class_name;