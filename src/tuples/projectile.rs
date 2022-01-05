use crate::tuples::point::Point;
use crate::tuples::vector::Vector;

#[derive(Debug)]
pub struct Projectile {
    pub position: Point,
    pub velocity: Vector
}

impl Projectile {
    pub fn new(position: Point, velocity: Vector) -> Projectile {
        return Projectile {
            position,
            velocity
        };
    }
}