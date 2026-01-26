use georust::Point;
use ppmrust::{Image, Rgb};

/************************************************************* IMPLEMENTATION */

pub fn draw_point(image: &mut Image, point: Point, color: Rgb) {
    image.set_pixel(point.x() as usize, point.y() as usize, color);
}

// TODO: draw line, draw triangle
