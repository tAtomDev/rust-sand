pub mod math;
pub struct Buffer {
    width: usize,
    height: usize,
    pub data: Vec<u8>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            data: std::vec::from_elem(50, width * height * 4),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_pixel_color(&mut self, x: usize, y: usize, color: (u8, u8, u8, u8)) {
        let idx = self.height * y + x;
        self.data[idx * 4] = color.0;
        self.data[idx * 4 + 1] = color.1;
        self.data[idx * 4 + 2] = color.2;
        self.data[idx * 4 + 3] = 255;
    }
}
