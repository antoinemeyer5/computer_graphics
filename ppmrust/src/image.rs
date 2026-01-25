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
        let mut x0 = x0 as i32;
        let mut y0 = y0 as i32;
        let x1 = x1 as i32;
        let y1 = y1 as i32;

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && y0 >= 0 {
                self.set_pixel(x0 as usize, y0 as usize, color);
            }

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;

            if e2 >= dy {
                err += dy;
                x0 += sx;
            }

            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw_triangle(
        &mut self,
        ax: usize,
        ay: usize,
        bx: usize,
        by: usize,
        cx: usize,
        cy: usize,
        color: Rgb,
    ) {
        self.draw_line(ax, ay, bx, by, color);
        self.draw_line(bx, by, cx, cy, color);
        self.draw_line(cx, cy, ax, ay, color);
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
