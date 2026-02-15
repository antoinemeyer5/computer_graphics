use georust::{Line, Oval, Point, Rectangle, Triangle};
use ppmrust::Image;
use renrust::{colors::*, draw_line, draw_point, draw_rectangle, draw_triangle, render::draw_oval};

/*************************************************************** MAIN PROGRAM */

fn main() {
    // Init. image
    let mut image = Image::new(200, 150, colors::BLACK);

    // Draw points
    let points = [
        Point::from_vec((75.0, 75.0)),
        Point::new(2.0, 2.0),
        Point::new(39.0, 49.0),
    ];
    for point in points {
        draw_point(&mut image, point, colors::RED);
    }

    // Draw lines
    let lines = [
        Line::from_vec((12.0, 15.0, 25.0, 45.0)),
        Line::from_points(Point::new(10.0, 9.0), Point::new(20.0, 2.0)),
    ];
    for line in lines {
        draw_line(&mut image, line, colors::WHITE);
    }

    // Draw triangle
    let triangle = Triangle::from_points(
        Point::new(18.0, 18.0),
        Point::new(47.0, 13.0),
        Point::new(36.0, 27.0),
    );
    draw_triangle(&mut image, triangle, colors::GREEN);

    // Draw square
    let square = Rectangle::square(Point::new(100.0, 10.0), 10.0, 0.0);
    draw_rectangle(&mut image, square, colors::BLUE);

    // Draw rectangles
    let rectangles = [
        Rectangle::new(Point::new(10.0, 80.0), 54.0, 25.0, 0.0),
        Rectangle::new(Point::new(98.0, 65.0), 14.0, 76.0, 33.0),
    ];
    for rectangle in rectangles {
        draw_rectangle(&mut image, rectangle, colors::BLUE);
    }

    // Draw ovals/ellipses
    let ovals = [
        Oval::new(Point::new(150.0, 90.0), 3.4, 18.9),
        Oval::new(Point::new(130.0, 30.0), 10.0, 5.0),
    ];
    for oval in ovals {
        draw_oval(&mut image, oval, colors::YELLOW);
    }

    // Draw circle
    let circle = Oval::circle(Point::new(170.0, 130.0), 19.0);
    draw_oval(&mut image, circle, colors::YELLOW);

    // Draw polygons (store a Vec<Point>)
    // TODO

    // Save image
    let saved = image.save("image.ppm");
    match saved {
        Ok(()) => println!("Well saved!"),
        Err(error) => println!("Not saved because {}", error),
    }
}
