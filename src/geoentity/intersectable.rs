use std::rc::Rc;
use crate::Intersection;

use crate::Ray;
use crate::Phong;
use crate::Point;
use crate::Vector;

pub trait Intersectable {
    fn get_id(&self) -> u64;

    // Using arbitrary self parameter here to allow for polymorphism in the world objects
    fn intersect(self: Rc<Self>, ray: &Ray) -> Vec<Intersection>;
    fn normal_at(&self, point: &Point) -> Vector;
    fn material(&self) -> Phong;
}