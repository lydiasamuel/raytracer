use crate::tuples::point::Point;
use crate::tuples::vector::Vector;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        return Ray {
            origin,
            direction
        };
    }

    pub fn position(&self, time: f64) -> Point {
        return self.origin + (self.direction * time);
    }
}