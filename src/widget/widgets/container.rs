use windows::{core::PCWSTR, Win32::{Foundation::{COLORREF, HWND, RECTL}, Graphics::Gdi::{CreateSolidBrush, FillRect, GetDC, ReleaseDC}, UI::WindowsAndMessaging::{CreateWindowExW, HMENU, WINDOW_EX_STYLE, WINDOW_STYLE, WS_BORDER, WS_CHILD, WS_TABSTOP, WS_VISIBLE}}};
use crate::{widget::style::decoration::BoxDecoration, window::State, Widget};


pub struct Container {
    pub parent: Option<Box<dyn Widget>>,
    pub child: Option<Box<dyn Widget>>,
    pub border: bool,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub color: COLORREF, // Color of the container in RGB format
    pub id: i32,
    pub decoration: Option<BoxDecoration>,
}
impl Container {
    pub fn new() -> Self {
        Self {
            parent: None,
            child: None,
            border: false,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            color: COLORREF(0xFFFFFF), // Default to white
            id: 0,
            decoration: None,
        }
    }
}

impl Widget for Container {
    fn draw(&self, hwnd_parent: HWND)-> HWND {
        unsafe {
            let lpclassname = "STATIC\0".encode_utf16().collect::<Vec<u16>>();
            let _container_hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                PCWSTR(lpclassname.as_ptr()),
                PCWSTR(std::ptr::null()),
                WS_CHILD | WS_VISIBLE |if self.border {  WS_BORDER } else { WINDOW_STYLE(0) },
                self.x,
                self.y,
                self.width,
                self.height,
                Some(hwnd_parent),
                Some(HMENU::default()),
                None,
                None);
            println!("Created button with hwnd: {:?}", _container_hwnd);
            if let Ok(_container_hwnd) = _container_hwnd {
                let hdc = GetDC(Some(_container_hwnd));
                if hdc.0 != std::ptr::null_mut() {
                    let brush = CreateSolidBrush(self.color); // Create a brush with the specified color
                    if brush.0 != std::ptr::null_mut() {
                        let rect = windows::Win32::Foundation::RECT {
                            left: 0,
                            top: 0,
                            right: self.width,
                            bottom: self.height,
                        };
                        FillRect(hdc, &rect as *const _, brush); // Fill the rectangle with the brush
                        // windows::Win32::Foundation::DeleteObject(brush); // Clean up the brush
                    }
                    ReleaseDC(Some(_container_hwnd), hdc); // Release the device context
                }

            } else {
                panic!("Failed to create container window")
            }

            match &self.child {
                Some(child) => {
                    child.draw(_container_hwnd.unwrap())
                },
                None => {
                    _container_hwnd.unwrap()
                }
            }
        }
    }
}