use std::fmt::Error;

use windows::Win32::Foundation::HWND;



pub trait Widget  {
    fn draw(&self, hwnd_parent: HWND)-> HWND;
}



// unsafe {
//     let button_text: Vec<u16> = "Hello\0".encode_utf16().collect();
//     let class_name: Vec<u16> = "BUTTON\0".encode_utf16().collect();

//     let _button_hwnd = CreateWindowExW(
//         WINDOW_EX_STYLE(0),
//         PCWSTR(class_name.as_ptr()),
//         PCWSTR(button_text.as_ptr()),
//         WS_CHILD | WS_VISIBLE | WS_TABSTOP,
//         50,
//         50,
//         100,
//         30,
//         Some(hwnd),
//         Some(HMENU::default()),
//         None,
//         None,
//     );
// }