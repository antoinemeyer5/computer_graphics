use georust::{Line, Point, Triangle};
use ppmrust::{Image, Rgb};

/************************************************************* IMPLEMENTATION */

pub fn draw_line(image: &mut Image, line: Line, color: Rgb) {
    let mut x0 = line.start().x() as i32;
    let mut y0 = line.start().y() as i32;
    let x1 = line.end().x() as i32;
    let y1 = line.end().y() as i32;

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && y0 >= 0 {
            draw_point(image, Point::new(x0 as f32, y0 as f32), color);
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

pub fn draw_point(image: &mut Image, point: Point, color: Rgb) {
    image.set_pixel(point.x() as usize, point.y() as usize, color);
}

pub fn draw_triangle(image: &mut Image, triangle: Triangle, color: Rgb) {
    let line_ab: Line = triangle.get_edge(0);
    let line_bc: Line = triangle.get_edge(1);
    let line_ca: Line = triangle.get_edge(2);

    draw_line(image, line_ab, color);
    draw_line(image, line_bc, color);
    draw_line(image, line_ca, color);
}
