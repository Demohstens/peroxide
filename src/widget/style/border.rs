pub struct Border {
    pub width: u32,
    pub color: String,
}
impl Border {
    pub fn new(width: u32, color: String) -> Self {
        Border { width, color }
    }
}
