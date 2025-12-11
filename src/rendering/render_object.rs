use std::{os::raw::c_void, rc::Rc};

use wgpu::{Color, Surface, Texture};
use winit::dpi::PhysicalPosition;

use crate::PeroxideEvent;

pub struct RenderObject {
    pub id: u32,
    pub constraints: Constraints,
    pub x: f64,
    pub y: f64,
    pub color: Color,
    pub handle: Option<*mut c_void>,
    pub parent: Option<*mut c_void>,
    pub is_visible: bool,
    pub is_enabled: bool,
    pub input_handler: Option<Box<dyn FnMut(PeroxideEvent)>>,
    // #[cfg(target_os = "windows")]
    // pub native_element: Option<WindowClass>,
}

impl RenderObject {
    pub fn hit_test(&self, position: Rc<PhysicalPosition<f64>>) -> bool {
        if let Some(width) = self.constraints.width {
            if let Some(height) = self.constraints.height {
                if (self.x <= position.x && position.x <= self.x + width)
                    && (self.y <= position.y && position.y <= self.y + height)
                {
                    return true;
                } else {
                    return false;
                }
            }
        }
        // TODO: Attempt to relayout the widget.
        false
    }
    pub fn layout(&mut self) {
        // TODO
        self.constraints.width = Some(self.constraints.min_width);
        self.constraints.height = Some(self.constraints.min_height);
    }

    // #[cfg(target_os = "windows")]
    // fn draw_windows(&mut self) {
    //     use crate::platform::windows::class_name;
    //     use windows::{
    //         Win32::{
    //             Foundation::HWND,
    //             UI::WindowsAndMessaging::{
    //                 CreateWindowExW, WINDOW_EX_STYLE, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
    //             },
    //         },
    //         core::PCWSTR,
    //     };
    //     let class_name = class_name::ClassName::RENDEROBJECT.as_pcwstr();
    //     let window_title = format!("{:?}\0", self.id)
    //         .encode_utf16()
    //         .collect::<Vec<u16>>(); //
    //     let style = if self.parent.is_some() {
    //         WS_CHILD | WS_VISIBLE
    //     } else {
    //         WS_OVERLAPPEDWINDOW | WS_VISIBLE // No WS_CHILD for root
    //     };
    //     let parent_hwnd = self.parent.unwrap_or(std::ptr::null_mut());
    //     let hwnd_result: Result<HWND, windows::core::Error> = unsafe {
    //         CreateWindowExW(
    //             WINDOW_EX_STYLE(0),
    //             class_name,
    //             PCWSTR(window_title.as_ptr()),
    //             style,
    //             self.x,
    //             self.y,
    //             self.constraints.width.unwrap_or(self.constraints.min_width) as i32,
    //             self.constraints
    //                 .height
    //                 .unwrap_or(self.constraints.min_height) as i32, // <-- Fix here
    //             Some(HWND(parent_hwnd)),
    //             None,
    //             None,
    //             Some(self as *mut RenderObject as *const std::ffi::c_void),
    //         )
    //     };
    //     match hwnd_result {
    //         Ok(handle) => {
    //             self.handle = Some(handle.0);
    //             println!("Window created");
    //         }
    //         Err(err) => {
    //             println!("Failed to create window: {:?}", err);
    //         }
    //     }
    // }

    #[cfg(target_os = "linux")]
    fn draw_linux(&mut self) {
        // Placeholder for Linux drawing logic
        // TODO
    }

    #[cfg(target_os = "macos")]
    fn draw_macos(&mut self) {
        use cacao::appkit::window::{Window, WindowConfig};
        use cacao::geometry::Rect;

        let window = Window::new(WindowConfig {
            style: 0,
            initial_dimensions: Rect::new(
                self.x as f64,
                self.y as f64,
                self.constraints.width.unwrap_or(self.constraints.min_width) as f64,
                self.constraints
                    .height
                    .unwrap_or(self.constraints.min_height) as f64,
            ),
            defer: true,
            ..Default::default()
        });
        window.set_title(&format!("{:?}", self.id));
        let button = crate::platform::macos::widgets::create_macos_button("Click me");
        window.set_content_view(&button);
        window.show();
        self.handle = Some(&window as *const Window as *mut c_void);

        println!("macOS window created");
    }
    pub fn draw(&mut self, target: &mut Texture) {
        target
    }
}

pub struct Constraints {
    pub min_width: f64,
    pub min_height: f64,
    pub max_width: Option<f64>,
    pub max_height: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}
