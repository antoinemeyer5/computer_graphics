/****************************************************************** STRUCTURE */

#[derive(Debug)]
pub struct Linvec2 {
    pub x: f32,
    pub y: f32,
}

/************************************************************* IMPLEMENTATION */

impl Linvec2 {
    pub fn add(self, other: Linvec2) -> Linvec2 {
        Linvec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn dot(self, other: Linvec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn scale(self, scalar: f32) -> Linvec2 {
        Linvec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn sub(self, other: Linvec2) -> Linvec2 {
        Linvec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/****************************************************************** UNIT TEST */

#[test]
fn test_add() {
    let a = Linvec2 { x: 1.0, y: 2.0 };
    let b = Linvec2 { x: 4.0, y: 5.0 };

    let result = a.add(b);

    assert!(result.x == 5.0);
    assert!(result.y == 7.0);
}

#[test]
fn test_dot() {
    let a = Linvec2 { x: 1.0, y: 2.0 };
    let b = Linvec2 { x: 4.0, y: 5.0 };

    let result = a.dot(b);

    assert!(result == 14.0);
}

#[test]
fn test_scale() {
    let a = Linvec2 { x: 1.0, y: 2.0 };
    let b = 4.0;

    let result = a.scale(b);

    assert!(result.x == 4.0);
    assert!(result.y == 8.0);
}

#[test]
fn test_sub() {
    let a = Linvec2 { x: 1.0, y: 2.0 };
    let b = Linvec2 { x: 4.0, y: 5.0 };

    let result = a.sub(b);

    assert!(result.x == -3.0);
    assert!(result.y == -3.0);
}
