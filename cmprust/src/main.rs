use linrust::{Linvec2, Linvec4};
use ppmrust::{Image, Rgb};

/****************************************************************** CONSTANTS */

const BLACK: Rgb = Rgb { r: 0, g: 0, b: 0 };
const WHITE: Rgb = Rgb {
    r: 255,
    g: 255,
    b: 255,
};

/*************************************************************** MAIN PROGRAM */

fn main() {
    let mut image = Image::new(100, 100, BLACK);

    // Draw points
    let points = [
        Linvec2 { x: 0.0, y: 0.0 },
        Linvec2 { x: 2.0, y: 2.0 },
    ];
    for point in points {
        image.set_pixel(point.x as usize, point.y as usize, WHITE);
    }

    // Draw line
    let line = Linvec4 {
        x: 80.0,
        y: 80.0,
        z: 70.0,
        w: 70.0,
    };
    image.draw_line(
        line.x as usize,
        line.y as usize,
        line.z as usize,
        line.w as usize,
        WHITE,
    );

    let saved = image.save("image.ppm");
    match saved {
        Ok(()) => println!("Well saved!"),
        Err(error) => println!("Not saved because {}", error),
    }
}
