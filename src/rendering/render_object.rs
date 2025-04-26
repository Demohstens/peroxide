use std::os::raw::c_void;

use wgpu::Color;

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
    // #[cfg(target_os = "windows")]
    // pub native_element: Option<WindowClass>,
}

impl RenderObject {
    pub fn layout(&mut self) {}

    #[cfg(target_os = "windows")]
    fn draw_windows(&mut self) {
        use crate::platform::windows::class_name;
        use windows::{
            core::PCWSTR,
            Win32::{
                Foundation::HWND,
                UI::WindowsAndMessaging::{
                    CreateWindowExW, WINDOW_EX_STYLE, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
                },
            },
        };
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
        let hwnd_result: Result<HWND, windows::core::Error> = unsafe {
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
                println!("Window created");
            }
            Err(err) => {
                println!("Failed to create window: {:?}", err);
            }
        }
    }

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
    pub fn draw(&mut self) {
        println!("Drawing render object with id: {}", self.id);
        #[cfg(target_os = "windows")]
        self.draw_windows();
        #[cfg(target_os = "linux")]
        self.draw_linux();
        #[cfg(target_os = "macos")]
        self.draw_macos();
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
