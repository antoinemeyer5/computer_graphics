use linrust::Linvec3;

/****************************************************************** STRUCTURE */

pub struct Point3 {
    pos: Linvec3,
}

/************************************************************* IMPLEMENTATION */

impl Point3 {
    pub fn from_vec(p: (f32, f32, f32)) -> Self {
        Self::new(p.0, p.1, p.2)
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            pos: Linvec3::new(x, y, z),
        }
    }
}
