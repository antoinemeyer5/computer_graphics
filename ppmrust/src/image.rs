use crate::Rgb;

/****************************************************************** STRUCTURE */

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Rgb>,
}

/************************************************************* IMPLEMENTATION */

impl Image {
    pub fn new(width: usize, height: usize, clear: Rgb) -> Self {
        Self {
            width,
            height,
            data: vec![clear; width * height],
        }
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(filename).unwrap();

        writeln!(file, "P3").unwrap();
        writeln!(file, "{} {}", self.width, self.height).unwrap();
        writeln!(file, "255").unwrap();

        for pixel in &self.data {
            writeln!(file, "{} {} {}", pixel.r, pixel.g, pixel.b).unwrap();
        }

        Ok(())
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Rgb) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = color;
        }
    }
}
