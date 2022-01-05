use crate::tuples::point::Point;
use crate::tuples::vector::Vector;

#[derive(Debug)]
pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector
}

impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Environment {
        return Environment {
            gravity,
            wind
        }
    }
}
