
use std::os::raw::c_void;

use windows::Win32::Foundation::{HINSTANCE, HWND};

pub struct WindowClass {
    pub class_name: String,
    pub instance: HINSTANCE,
    pub hwnd: Option<*mut c_void>,
}

impl WindowClass {
    pub fn new(class_name: &str, instance: HINSTANCE) -> Self {
        Self {
            class_name: class_name.to_string(),
            instance,
            hwnd: None,
        }
    }
    #[allow(non_snake_case)]
    pub fn MoveWindow(&self, x: i32, y: i32, width: i32, height: i32) {
        if let Some(hwnd) = self.hwnd {
            unsafe {
                let _ = windows::Win32::UI::WindowsAndMessaging::MoveWindow(
                    windows::Win32::Foundation::HWND(hwnd),
                    x,
                    y,
                    width,
                    height,
                    true,
                );
            }
        }
    }
    #[allow(non_snake_case)]
    pub fn GetWindowRect(&self) -> Option<(i32, i32, i32, i32)> {
        if let Some(hwnd) = self.hwnd {
            let mut rect = windows::Win32::Foundation::RECT::default();
            unsafe {
                let result = windows::Win32::UI::WindowsAndMessaging::GetWindowRect(
                    windows::Win32::Foundation::HWND(hwnd),
                    &mut rect,
                );
                match result {
                    Ok(_) => return Some((rect.left, rect.top, rect.right, rect.bottom)),
                    Err(_) => return None,
                }
            }
        }
        None
    }
    #[allow(non_snake_case)]
    pub fn GetParent(&self) -> Option<*mut c_void> {
        if let Some(hwnd) = self.hwnd {
            let parent_hwnd = unsafe { windows::Win32::UI::WindowsAndMessaging::GetParent(HWND(hwnd)) };
            match parent_hwnd {
                Ok(hwnd) => return Some(hwnd.0 as *mut c_void),
                Err(_) => return None,
            }
        }
        None
    }

    #[allow(non_snake_case)]
    pub fn IsChild(&self) -> bool {
        if let Some(hwnd) = self.hwnd {
            let parent_hwnd = unsafe { windows::Win32::UI::WindowsAndMessaging::GetParent(HWND(hwnd)) };
            match parent_hwnd {
                Ok(_) => return true,
                Err(_) => return false,
            }
        }
        false
    }
}