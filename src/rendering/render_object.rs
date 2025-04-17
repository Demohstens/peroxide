use std::os::raw::c_void;

use wgpu::Color;

#[cfg(target_os = "windows")]
use windows::{
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{
            CreateWindowExW, WINDOW_EX_STYLE, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
    core::PCWSTR,
};
use crate::PeroxideEvent;

pub struct RenderObject {
    pub id: u32,
    pub constraints: Constraints,
    pub x: i32,
    pub y: i32,
    pub color: Color,
    pub handle: Option<*mut c_void>,
    pub parent: Option<*mut c_void>,
    pub is_visible: bool,
    pub is_enabled: bool,
    pub input_handler: Option<Box<dyn FnMut(PeroxideEvent)>>,
}

impl RenderObject {
    pub fn layout(&mut self) {}

    #[cfg(target_os = "windows")]
    fn draw_windows(&mut self) {
        use crate::platform::windows::class_name;
        let class_name = class_name::ClassName::RENDEROBJECT.as_pcwstr();
        let window_title = format!("{:?}\0", self.id)
            .encode_utf16()
            .collect::<Vec<u16>>(); //
        let style = if self.parent.is_some() {
            WS_CHILD | WS_VISIBLE
        } else {
            WS_OVERLAPPEDWINDOW | WS_VISIBLE // No WS_CHILD for root
        };
        let parent_hwnd = self.parent.unwrap_or(std::ptr::null_mut());
        println!("{:?}", self.id);
        let hwnd_result = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE(0),
                class_name,
                PCWSTR(window_title.as_ptr()),
                style,
                self.x,
                self.y,
                self.constraints.width.unwrap_or(self.constraints.min_width) as i32,
                self.constraints
                    .height
                    .unwrap_or(self.constraints.min_height) as i32, // <-- Fix here
                Some(HWND(parent_hwnd)),
                None,
                None,
                Some(self as *mut RenderObject as *const std::ffi::c_void),
            )
        };
        match hwnd_result {
            Ok(handle) => {
                self.handle = Some(handle.0);
                // for child in &self. {
                //     let mut child = child.borrow_mut();
                //     child.parent = Some(handle);
                //     child.draw();
                //     println!("Children");

                // }
                println!("Window created");
            }
            Err(err) => {
                println!("Failed to create window: {:?}", err);
            }
        }
    }
    fn draw_linux(&mut self) {
        // Linux drawing logic goes here
        // This is where you would use X11 or Wayland to create and draw the window
        println!("Drawing on Linux");
    }
    pub fn draw(&mut self) {
        #[cfg(target_os = "windows")]
        self.draw_windows();
        #[cfg(target_os = "linux")]
        self.draw_linux();
    }

    /// Creates a debugging render object with default values.
    /// This is useful for testing and debugging purposes.
    /// It is a simple square
    pub fn debug() -> Self {
        RenderObject {
            id: 0,
            constraints: Constraints {
                min_width: 100,
                min_height: 100,
                max_width: Some(1000),
                max_height: Some(1000),
                width: Some(100),
                height: Some(100),
            },
            x: 10,
            y: 10,
            color: Color::BLUE,
            handle: None,
            parent: None,
            is_visible: true,
            is_enabled: true,
            input_handler: None,
        }
    }
}

pub struct Constraints {
    pub min_width: i32,
    pub min_height: i32,
    pub max_width: Option<i32>,
    pub max_height: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}
