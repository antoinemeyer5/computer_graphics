use linrust::{Linvec2, Linvec4};

/****************************************************************** STRUCTURE */

pub struct Line {
    pub data: Linvec4,
}

/************************************************************* IMPLEMENTATION */

impl Line {
    pub fn end(&self) -> Linvec2 {
        Linvec2::new(self.data.z, self.data.w)
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
