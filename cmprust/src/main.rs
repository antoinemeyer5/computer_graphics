use georust::{Line, Point, Square, Triangle};
use ppmrust::Image;
use renrust::{colors::*, draw_line, draw_point, draw_square, draw_triangle};

/*************************************************************** MAIN PROGRAM */

fn main() {
    // Init. image
    let mut image = Image::new(200, 150, colors::BLACK);

    // Draw points
    let points = [
        Point::from_vec((0.0, 0.0)),
        Point::new(2.0, 2.0),
        Point::new(39.0, 49.0),
    ];
    for point in points {
        draw_point(&mut image, point, colors::RED);
    }

    // Draw lines
    let lines = [
        Line::from_vec((12.0, 15.0, 25.0, 45.0)),
        Line::from_points(Point::new(10.0, 0.0), Point::new(20.0, 2.0)),
    ];
    for line in lines {
        draw_line(&mut image, line, colors::WHITE);
    }

    // Draw triangle
    let triangle = Triangle::from_points(
        Point::new(8.0, 8.0),
        Point::new(27.0, 3.0),
        Point::new(26.0, 17.0),
    );
    draw_triangle(&mut image, triangle, colors::GREEN);

    // Draw squares
    let squares = [
        Square::from_points(Point::new(100.0, 10.0), Point::new(110.0, 20.0)),
        Square::from_points(Point::new(73.0, 70.0), Point::new(140.0, 85.0)),
    ];
    for square in squares {
        println!("square: {} x {}", square.get_height(), square.get_width());
        draw_square(&mut image, square, colors::YELLOW);
    }

    // Save image
    let saved = image.save("image.ppm");
    match saved {
        Ok(()) => println!("Well saved!"),
        Err(error) => println!("Not saved because {}", error),
    }
}
