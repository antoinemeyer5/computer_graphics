use linrust::Linvec2;

/****************************************************************** STRUCTURE */

#[derive(Copy, Clone)]
pub struct Point {
    pos: Linvec2,
}

/************************************************************* IMPLEMENTATION */

impl Point {
    pub fn add(&self, v: Linvec2) -> Point {
        Point {
            pos: self.pos.add(v),
        }
    }

    pub fn sub(&self, v: Linvec2) -> Point {
        Point {
            pos: self.pos.sub(v),
        }
    }

    pub fn from_vec(v: (f32, f32)) -> Self {
        Self::new(v.0, v.1)
    }

    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Linvec2::new(x, y),
        }
    }

    pub fn x(&self) -> f32 {
        self.pos.x
    }

    pub fn y(&self) -> f32 {
        self.pos.y
    }
}
