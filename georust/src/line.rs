use linrust::{Linvec2, Linvec4};

/****************************************************************** STRUCTURE */

pub struct Line {
    data: Linvec4,
}

/************************************************************* IMPLEMENTATION */

impl Line {
    pub fn end(&self) -> Linvec2 {
        Linvec2::new(self.data.z, self.data.w)
    }

    pub fn from_vec(v: (f32, f32, f32, f32)) -> Self {
        Self::new(v.0, v.1, v.2, v.3)
    }

    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            data: Linvec4::new(x1, y1, x2, y2),
        }
    }

    pub fn start(&self) -> Linvec2 {
        Linvec2::new(self.data.x, self.data.y)
    }
}
