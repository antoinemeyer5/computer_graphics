use std::f32::consts::PI;

use crate::Point;

/****************************************************************** STRUCTURE */

pub struct Oval {
    pub center: Point,
    pub radius_x: f32,
    pub radius_y: f32,
}

/************************************************************* IMPLEMENTATION */

impl Oval {
    pub fn new(c: Point, rx: f32, ry: f32) -> Self {
        Self {
            center: c,
            radius_x: rx,
            radius_y: ry,
        }
    }

    pub fn circle(c: Point, r: f32) -> Self {
        Self {
            center: c,
            radius_x: r,
            radius_y: r,
        }
    }

    pub fn area(&self) -> f32 {
        PI * self.radius_x * self.radius_y
    }
}
