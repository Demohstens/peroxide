use windows::{core::PCWSTR, Win32::{Foundation::HWND, UI::WindowsAndMessaging::{CreateWindowExW, HMENU, WINDOW_EX_STYLE, WS_CHILD, WS_TABSTOP, WS_VISIBLE}}};
use crate::{window::State, Widget};


pub struct Button {
    pub parent: Option<Box<dyn Widget>>,
    pub child: Option<Box<dyn Widget>>,
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub id: i32,
}

impl Widget for Button {
    fn width(&self) -> i32 {
        self.width
    }
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y 
    }

    fn height(&self) -> i32 {
        self.height
    }
    fn draw(&self, hwnd_parent: HWND)-> HWND {
        unsafe {
            let mut text = self.text.encode_utf16().collect::<Vec<u16>>();
            text.push(0); // Null-terminate the string
            let lpclassname = "BUTTON\0".encode_utf16().collect::<Vec<u16>>();
            let _button_hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                PCWSTR(lpclassname.as_ptr()),
                PCWSTR(text.as_ptr()),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP,
                self.x,
                self.y,
                self.width,
                self.height,
                Some(hwnd_parent),
                Some(HMENU::default()),
                None,
                None);
            println!("Created button with hwnd: {:?}", _button_hwnd);
            match &self.child {
                Some(child) => {
                    child.draw(_button_hwnd.unwrap())
                },
                None => {
                    _button_hwnd.unwrap()
                }
            }
        }
    }
}
#[macro_export]
macro_rules! button {
    {
        parent: $parent:expr,
        child: $child:expr,
        text: $text:expr,
        x: $x:expr,
        y: $y:expr,
        width: $width:expr,
        height: $height:expr,
        id: $id:expr $(,)?
    } => {
        Button {
            parent: $parent,
            child: Some(Box::new($child)),
            text: $text.to_string(),
            x: $x,
            y: $y,
            width: $width,
            height: $height,
            id: $id,
        }
    }
}

