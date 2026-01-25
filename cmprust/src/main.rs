use linrust::Linvec2;
use ppmrust::Image;
use ppmrust::Rgb;

/****************************************************************** CONSTANTS */

const BLACK: Rgb = Rgb { r: 0, g: 0, b: 0 };
const WHITE: Rgb = Rgb { r: 255, g: 255, b: 255 };

/*************************************************************** MAIN PROGRAM */

fn main() {
    let mut image = Image::new(10, 15, BLACK);

    let a = Linvec2 { x: 0.0, y: 0.0 };
    image.set_pixel(a.x as usize, a.y as usize, WHITE);

    let b = Linvec2 { x: 1.0, y: 1.0 };
    image.set_pixel(b.x as usize, b.y as usize, WHITE);

    image.save("image.ppm");
}
