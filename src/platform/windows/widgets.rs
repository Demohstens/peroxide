use windows::{
    core::PCWSTR, Win32::{Foundation::HWND, UI::WindowsAndMessaging::{
        CreateWindowExW, HMENU, WINDOW_EX_STYLE, WS_CHILD, WS_TABSTOP, WS_VISIBLE
    }}
};

use super::class_name::ClassName;

pub fn create_button_hwnd(text: String, hwnd_parent: HWND) -> Result<HWND, windows::core::Error> {
    unsafe {
        let mut text = text.encode_utf16().collect::<Vec<u16>>();
        text.push(0); // Null-terminate the string
        let _button_hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            ClassName::Button.as_pcwstr(),
            PCWSTR(text.as_ptr()),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP,
            0,
            0,
            0,
            0,
            Some(hwnd_parent),
            Some(HMENU::default()),
            None,
            None,
        );
        return _button_hwnd
    }
}
