/****************************************************************** STRUCTURE */

#[derive(Debug)]
pub struct Linvec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/************************************************************* IMPLEMENTATION */

impl Linvec4 {
    pub fn add(self, other: Linvec4) -> Linvec4 {
        Linvec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

/****************************************************************** UNIT TEST */

#[test]
fn test_add() {
    let a = Linvec4 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        w: 50.0,
    };
    let b = Linvec4 {
        x: 4.0,
        y: 5.0,
        z: 6.0,
        w: 51.0,
    };

    let result = a.add(b);

    assert!(result.x == 5.0);
    assert!(result.y == 7.0);
    assert!(result.z == 9.0);
    assert!(result.w == 101.0);
}
