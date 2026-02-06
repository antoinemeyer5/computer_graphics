use linrust::Linvec2;

use crate::{Line, Point};

/****************************************************************** STRUCTURE */

pub struct Rectangle {
    top_left: Point,
    width: f32,
    height: f32,
    angle_deg: f32,
}

/************************************************************* IMPLEMENTATION */

impl Rectangle {
    pub fn new(tl: Point, w: f32, h: f32, a: f32) -> Self {
        Self {
            top_left: tl,
            width: w,
            height: h,
            angle_deg: a,
        }
    }

    pub fn square(top_left: Point, size: f32, a: f32) -> Self {
        Self {
            top_left,
            width: size,
            height: size,
            angle_deg: a,
        }
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_corner(&self, n: usize) -> Point {
        let angle_rad = self.angle_deg.to_radians();
        let cos = angle_rad.cos();
        let sin = angle_rad.sin();

        let width_vector = Linvec2::new(self.width * cos, self.width * sin);
        let height_vector = Linvec2::new(-self.height * sin, self.height * cos);

        let tl: Point = self.top_left;
        let tr: Point = tl.add(width_vector);
        let br: Point = tr.add(height_vector);
        let bl: Point = tl.add(height_vector);

        match n % 4 {
            0 => tl, // top left
            1 => tr, // top right
            2 => br, // bottom right
            _ => bl, // bottom left
        }
    }

    pub fn get_edge(&self, n: usize) -> Line {
        let corners = [
            self.get_corner(0),
            self.get_corner(1),
            self.get_corner(2),
            self.get_corner(3),
        ];
        let start = corners[n % 4];
        let end = corners[(n + 1) % 4];
        Line::from_points(start, end)
    }
}
