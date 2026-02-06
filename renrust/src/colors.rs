use ppmrust::Rgb;

/********************************************************************* MODULE */

pub mod colors {
    use super::Rgb;

    pub const BLACK: Rgb = Rgb { r: 0, g: 0, b: 0 };
    pub const BLUE: Rgb = Rgb { r: 0, g: 0, b: 255 };
    pub const WHITE: Rgb = Rgb {
        r: 255,
        g: 255,
        b: 255,
    };
    pub const RED: Rgb = Rgb { r: 255, g: 0, b: 0 };
    pub const GREEN: Rgb = Rgb { r: 0, g: 255, b: 0 };
    pub const YELLOW: Rgb = Rgb {
        r: 255,
        g: 255,
        b: 0,
    };
}
