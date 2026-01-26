use crate::Point;

/****************************************************************** STRUCTURE */

pub struct Line {
    start: Point,
    end: Point,
}

/************************************************************* IMPLEMENTATION */

impl Line {
    pub fn end(&self) -> Point {
        Point::new(self.end.x(), self.end.y())
    }

    pub fn from_points(p1: Point, p2: Point) -> Self {
        Self::new(p1.x(), p1.y(), p2.x(), p2.y())
    }

    pub fn from_vec(v: (f32, f32, f32, f32)) -> Self {
        Self::new(v.0, v.1, v.2, v.3)
    }

    fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            start: Point::new(x1, y1),
            end: Point::new(x2, y2),
        }
    }

    pub fn start(&self) -> Point {
        Point::new(self.start.x(), self.start.y())
    }
}
