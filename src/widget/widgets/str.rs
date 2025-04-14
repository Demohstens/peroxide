use windows::{core::PCWSTR, Win32::{Foundation::HWND, UI::WindowsAndMessaging::{CreateWindowExW, HMENU, WINDOW_EX_STYLE, WS_CHILD, WS_TABSTOP, WS_VISIBLE}}};

use crate::{window::State, Widget};

impl Widget for &str {
    fn draw(&self, hwnd_parent: HWND) -> HWND {
        unsafe {
        let mut text = self.encode_utf16().collect::<Vec<u16>>();
        text.push(0); // Null-terminate the string
        let lpclassname = "STATIC\0".encode_utf16().collect::<Vec<u16>>();
        let _str_hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            PCWSTR(lpclassname.as_ptr()),
            PCWSTR(text.as_ptr()),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP, 
            100, 
            0, 
            200, 
            100, 
            Some(hwnd_parent), 
            Some(HMENU::default()), 
            None, 
            None);
        println!("Created static Text with hwnd: {:?}", _str_hwnd);
        _str_hwnd.unwrap()
        }
    }
}