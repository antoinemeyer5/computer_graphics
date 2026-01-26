use crate::{Line, Point};

/****************************************************************** STRUCTURE */

pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

/************************************************************* IMPLEMENTATION */

impl Triangle {
    pub fn from_points(a: Point, b: Point, c: Point) -> Self {
        Self::new(a.x(), a.y(), b.x(), b.y(), c.x(), c.y())
    }

    pub fn get_corner(&self, n: usize) -> Point {
        match n % 3 {
            0 => Point::new(self.a.x(), self.a.y()),
            1 => Point::new(self.b.x(), self.b.y()),
            _ => Point::new(self.c.x(), self.c.y()),
        }
    }

    pub fn get_edge(&self, n: usize) -> Line {
        let a = self.get_corner(0);
        let b = self.get_corner(1);
        let c = self.get_corner(2);
        match n % 3 {
            0 => Line::from_points(a, b),
            1 => Line::from_points(b, c),
            _ => Line::from_points(c, a),
        }
    }

    fn new(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> Self {
        Self {
            a: Point::new(x1, y1),
            b: Point::new(x2, y2),
            c: Point::new(x3, y3),
        }
    }
}
