use linrust::Linvec2;

/****************************************************************** STRUCTURE */

pub struct Triangle {
    pub a: Linvec2,
    pub b: Linvec2,
    pub c: Linvec2,
}

/************************************************************* IMPLEMENTATION */

impl Triangle {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> Self {
        Self {
            a: Linvec2::new(x1, y1),
            b: Linvec2::new(x2, y2),
            c: Linvec2::new(x3, y3),
        }
    }
}
