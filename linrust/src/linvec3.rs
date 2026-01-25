/****************************************************************** STRUCTURE */

#[derive(Debug)]
pub struct Linvec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/************************************************************* IMPLEMENTATION */

impl Linvec3 {
    pub fn add(self, other: Linvec3) -> Linvec3 {
        Linvec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn cross(self, other: Linvec3) -> Linvec3 {
        Linvec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

/****************************************************************** UNIT TEST */

#[test]
fn test_add() {
    let a = Linvec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let b = Linvec3 {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    };

    let result = a.add(b);

    assert!(result.x == 5.0);
    assert!(result.y == 7.0);
    assert!(result.z == 9.0);
}
