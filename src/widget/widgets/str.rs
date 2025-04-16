use wgpu::Color;
use windows::{core::PCWSTR, Win32::{Foundation::HWND, UI::WindowsAndMessaging::{CreateWindowExW, HMENU, WINDOW_EX_STYLE, WS_BORDER, WS_CHILD, WS_TABSTOP, WS_THICKFRAME, WS_TILED, WS_VISIBLE}}};

use crate::{window::State, Widget};

impl Widget for &str {
    fn width(&self) -> i32 {
        100 
    }
    fn height(&self) -> i32 {
        100 
    }
    fn color(&self) -> Color {
        Color::WHITE
    }
    fn x(&self) -> i32 {
        0 
    }
    fn y(&self) -> i32 {
        0 
    }
   
}