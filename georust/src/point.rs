use linrust::Linvec2;

/****************************************************************** STRUCTURE */

pub struct Point {
    pub pos: Linvec2,
}

/************************************************************* IMPLEMENTATION */

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Linvec2::new(x, y),
        }
    }
}
