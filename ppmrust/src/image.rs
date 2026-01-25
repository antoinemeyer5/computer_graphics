use crate::Rgb;

/****************************************************************** STRUCTURE */

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Rgb>,
}

/************************************************************* IMPLEMENTATION */

impl Image {
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: Rgb) {
        let dx = if x1 > x0 { x1 - x0 } else { x0 - x1 };
        let dy = if y1 > y0 { y1 - y0 } else { y0 - y1 };

        let sx = if x0 < x1 { 1 } else { usize::MAX }; // -1 when usize safe
        let sy = if y0 < y1 { 1 } else { usize::MAX }; // -1 when usize safe

        let mut err = if dx > dy { dx } else { dy } / 2;
        let mut x = x0;
        let mut y = y0;

        loop {
            self.set_pixel(x, y, color.clone());

            if x == x1 && y == y1 {
                break;
            }

            let e2 = err;

            if e2 > dy {
                err -= dy;
                if sx == 1 {
                    x += 1;
                } else {
                    x -= 1;
                }
            }

            if e2 <= dx {
                err += dx;
                if sy == 1 {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
        }
    }

    pub fn new(width: usize, height: usize, clear: Rgb) -> Self {
        Self {
            width,
            height,
            pixels: vec![clear; width * height],
        }
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(filename).unwrap();

        writeln!(file, "P3").unwrap();
        writeln!(file, "{} {}", self.width, self.height).unwrap();
        writeln!(file, "255").unwrap();

        for pixel in &self.pixels {
            writeln!(file, "{} {} {}", pixel.r, pixel.g, pixel.b).unwrap();
        }

        Ok(())
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Rgb) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }
}
