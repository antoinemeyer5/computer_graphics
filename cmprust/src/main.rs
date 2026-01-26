use georust::{Line, Point, Triangle};
use ppmrust::{Image, Rgb};

/****************************************************************** CONSTANTS */

const BLACK: Rgb = Rgb { r: 0, g: 0, b: 0 };
const GREEN: Rgb = Rgb { r: 0, g: 255, b: 0 };
const RED: Rgb = Rgb { r: 255, g: 0, b: 0 };
const WHITE: Rgb = Rgb {
    r: 255,
    g: 255,
    b: 255,
};

/*************************************************************** MAIN PROGRAM */

fn main() {
    let mut image = Image::new(40, 50, BLACK);

    // Draw points
    let points = [Point::from_vec((0.0, 0.0)), Point::new(2.0, 2.0)];
    for point in points {
        image.set_pixel(point.x() as usize, point.y() as usize, RED);
    }

    // Draw lines
    let lines = [
        Line::from_vec((12.0, 15.0, 25.0, 45.0)),
        Line::from_points(Point::new(10.0, 0.0), Point::new(20.0, 2.0)),
    ];
    for line in lines {
        image.draw_line(
            line.start().x() as usize,
            line.start().y() as usize,
            line.end().x() as usize,
            line.end().y() as usize,
            WHITE,
        );
    }

    // Draw triangle
    let triangle = Triangle::new(8.0, 8.0, 27.0, 3.0, 26.0, 17.0);
    image.draw_triangle(
        triangle.a.x as usize,
        triangle.a.y as usize,
        triangle.b.x as usize,
        triangle.b.y as usize,
        triangle.c.x as usize,
        triangle.c.y as usize,
        GREEN,
    );

    let saved = image.save("image.ppm");
    match saved {
        Ok(()) => println!("Well saved!"),
        Err(error) => println!("Not saved because {}", error),
    }
}
