use windows::Win32::Foundation::COLORREF;

use super::border::Border;

pub struct BoxDecoration {
    pub border: Option<Border>,
    pub background_color: Option<String>, // Use String for simplicity, can be replaced with a more complex type
    pub border_radius: Option<u32>, // Assuming border radius is a single value for simplicity
}

impl BoxDecoration {
    pub fn new(border: Option<Border>, background_color: Option<String>, border_radius: Option<u32>) -> Self {
        BoxDecoration {
            border: None,
            background_color: None,
            border_radius: None,
        }
    }
}