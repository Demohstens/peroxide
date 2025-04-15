use std::{cell::RefCell, rc::Rc};

use windows::{core::{PCSTR, PCWSTR}, Win32::{Foundation::HWND, Graphics::Gdi::GetWindowExtEx, UI::WindowsAndMessaging::{CreateWindowExW, WINDOW_EX_STYLE, WS_CHILD, WS_VISIBLE}}};
use winit::event::WindowEvent;

use crate::platform::windows::class_name;

pub struct RenderObject {
    pub id: u32,
    pub constraints: Constraints,
    pub x: i32,
    pub y: i32,
    pub color: String,
    pub handle: Option<HWND>,
    pub parent: Option<HWND>,
    pub children: Vec<Rc<RefCell<RenderObject>>>,
    pub is_visible: bool,
    pub is_enabled: bool,
    pub input_handler: Option<Box<dyn FnMut(WindowEvent)>>,
}

impl RenderObject {
    pub fn layout(&mut self) {
        
    }
    pub fn draw(&mut self) {
        let class_name = class_name::ClassName::Static.as_pcwstr(); 
        let hwnd = unsafe { CreateWindowExW(
                WINDOW_EX_STYLE(0),
                class_name,
                PCWSTR(self.color.encode_utf16().collect::<Vec<u16>>().as_ptr()),
                WS_CHILD | WS_VISIBLE,
                self.x,
                self.y,
                self.constraints.width.unwrap_or(self.constraints.min_width) as i32,
                self.constraints.height.unwrap_or(self.constraints.min_width) as i32,
                self.parent,           // Option<HWND>
                None,                  // Option<HMENU>
                None,                  // Option<HINSTANCE>
                None,)
        };
        match hwnd {
            Ok(handle) => {
                self.handle = Some(handle);
                self.is_visible = true;
                // Set the parent window handle
                self.parent = Some(handle);
            },
            Err(_err) => {
                println!("Failed to create window");
            }
        }
        
    }
    pub fn handle_event(&mut self, event: WindowEvent) {
        // Event handling logic goes here
        // This is where you would handle events like mouse clicks, key presses, etc.
        if let Some(handler) = &mut self.input_handler {
            handler(event);
        }
    }
}

pub struct Constraints {
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}
