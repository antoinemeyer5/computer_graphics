use georust::{Line, Point, Rectangle, Triangle};
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
    draw_line(image, triangle.get_edge(0), color); // from A to B
    draw_line(image, triangle.get_edge(1), color); // from B to C
    draw_line(image, triangle.get_edge(2), color); // from C to A
}

pub fn draw_rectangle(image: &mut Image, rectangle: Rectangle, color: Rgb) {
    draw_line(image, rectangle.get_edge(0), color); // from TL to TR
    draw_line(image, rectangle.get_edge(1), color); // from TR to BR
    draw_line(image, rectangle.get_edge(2), color); // from BR to BL
    draw_line(image, rectangle.get_edge(3), color); // from BL to TL
}
