use linrust::Linvec2;

use crate::{Line, Point};

/****************************************************************** STRUCTURE */

pub struct Square {
    top_left: Point,
    bottom_right: Point,
}

/************************************************************* IMPLEMENTATION */

impl Square {
    pub fn from_points(tl: Point, bt: Point) -> Self {
        Self::new(tl.x(), tl.y(), bt.x(), bt.y())
    }

    pub fn get_corner(&self, n: usize) -> Point {
        let tlx = self.top_left.x();
        let tly = self.top_left.y();
        let brx = self.bottom_right.x();
        let bry = self.bottom_right.y();

        let center = Point::new((tlx + brx) * 0.5, (tly + bry) * 0.5);

        let diagonal_vector = Linvec2::new(tlx - center.x(), tly - center.y());
        let perpendicular_vector = Linvec2::new(-diagonal_vector.y, diagonal_vector.x);

        match n % 4 {
            0 => Point::new(tlx, tly),             // top left
            1 => center.add(perpendicular_vector), // top right
            2 => Point::new(brx, bry),             // bottom right
            _ => center.sub(perpendicular_vector), // bottom left
        }
    }

    pub fn get_edge(&self, n: usize) -> Line {
        let top_left = self.get_corner(0);
        let top_right = self.get_corner(1);
        let bottom_right = self.get_corner(2);
        let bottom_left = self.get_corner(3);
        match n % 4 {
            0 => Line::from_points(top_left, top_right),
            1 => Line::from_points(top_right, bottom_right),
            2 => Line::from_points(bottom_right, bottom_left),
            _ => Line::from_points(bottom_left, top_left),
        }
    }

    pub fn get_width(&self) -> f32 {
        let tl: Point = self.get_corner(0);
        let tr: Point = self.get_corner(1);

        let dx = tr.x() - tl.x();
        let dy = tr.y() - tl.y();

        (dx * dx + dy * dy).sqrt()
    }

    pub fn get_height(&self) -> f32 {
        let tl = self.get_corner(0);
        let bl = self.get_corner(3);

        let dx = bl.x() - tl.x();
        let dy = bl.y() - tl.y();

        (dx * dx + dy * dy).sqrt()
    }

    fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            top_left: Point::new(x1, y1),
            bottom_right: Point::new(x2, y2),
        }
    }
}
