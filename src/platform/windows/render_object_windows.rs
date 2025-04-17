use std::sync::Mutex;

use wgpu::Color;
use windows::{
    core::PCWSTR, Win32::{
        Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::{
            BeginPaint, CreateSolidBrush, DeleteObject, EndPaint, FillRect, PAINTSTRUCT,
        },
        UI::WindowsAndMessaging::{
            DefWindowProcW, GetWindowLongPtrW, RegisterClassW, SetWindowLongPtrW, CS_HREDRAW, CS_VREDRAW, GWLP_USERDATA, WM_CREATE, WM_LBUTTONUP, WM_PAINT, WNDCLASSW
        },
    }
};

use crate::{RenderObject, window::app::UserEvent};

// Register the window class once at startup
pub fn register_window_class() -> PCWSTR {
    use std::sync::Once;
    static mut CLASS_NAME: Option<Vec<u16>> = None;
    static mut CLASS_NAME_PTR: usize = 0;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let class_name = "RENDEROBJECT\0".encode_utf16().collect::<Vec<u16>>();
        let ptr = class_name.as_ptr() as usize;
        let wnd_class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(render_object_proc),
            hInstance: HINSTANCE(std::ptr::null_mut()), // Use the current instance
            lpszClassName: PCWSTR(ptr as *const u16),
            ..Default::default()
        };
        unsafe { RegisterClassW(&wnd_class) };
        unsafe { CLASS_NAME = Some(class_name) };
        unsafe {
            CLASS_NAME_PTR = ptr;
        }
    });
    unsafe { PCWSTR(CLASS_NAME_PTR as *const u16) }
}

extern "system" fn render_object_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match msg {
            WM_CREATE => {
                // Store pointer to RenderObject in window user data
                let createstruct =
                    lparam.0 as *const windows::Win32::UI::WindowsAndMessaging::CREATESTRUCTW;
                let render_object_ptr = (*createstruct).lpCreateParams as *mut RenderObject;
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, render_object_ptr as isize);
                LRESULT(0)
            }
            WM_PAINT => {
                let render_object_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut RenderObject;
                let render_object = &*render_object_ptr;
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);
                let color: Color = render_object.color;
                // From wgpu::color to COLORREF
                let r = (color.r * 255.0) as u32;
                let g = (color.g * 255.0) as u32;
                let b = (color.b * 255.0) as u32;
                let colorref = COLORREF(r << 16 | g << 8 | b);
                let brush = CreateSolidBrush(colorref);
                FillRect(hdc, &ps.rcPaint, brush);
                let _ = DeleteObject(windows::Win32::Graphics::Gdi::HGDIOBJ(brush.0));
                let _ = EndPaint(hwnd, &ps);
                LRESULT(0)
            }
            WM_LBUTTONUP => {
                if let Some(proxy) = crate::window::PROXY.get() {
                    let x = (lparam.0 & 0xffff) as i16 as i32;
                    let y = ((lparam.0 >> 16) & 0xffff) as i16 as i32;
                    // 3) send the event
                    // let res: Result<(), windows::core::Error> = MoveWindow(hwnd, x, x, x * 10, y*5, true);

                    let _ = proxy.send_event(UserEvent::MouseUp { x, y });
                }
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}
