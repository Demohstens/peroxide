use wgpu::Color;

pub struct Canvas {
    dimensions: (usize, usize),
    bitmap: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut bitmap = Vec::with_capacity(height);
        for _ in 0..height {
            bitmap.push(Vec::with_capacity(width));
        }
        Canvas {
            dimensions: (width, height),
            bitmap,
        }
    }

    pub fn fill(&mut self, color: Color) {
        for l in 0..self.bitmap.len() {
            for j in 0..self.bitmap.len() {
                self.bitmap[l][j] = color;
            }
        }
    }
}
