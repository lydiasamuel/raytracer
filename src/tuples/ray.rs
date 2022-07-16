use crate::Matrix;
use crate::tuples::point::Point;
use crate::tuples::vector::Vector;

#[derive(Debug, Copy, Clone)]
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

    pub fn transform(&self, matrix: Matrix) -> Ray {
        let transform = matrix.clone();

        let new_origin = matrix * self.origin;
        let new_direction = transform * self.direction;

        return Ray::new(new_origin.unwrap(), new_direction.unwrap());
    }
}