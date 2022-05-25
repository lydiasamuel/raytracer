use std::rc::Rc;

use crate::Point;
use crate::Intersectable;
use crate::Vector;

pub struct Computations {
    pub time: f64,
    pub object: Rc<dyn Intersectable>,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool
}

impl Computations {
    pub fn new(time: f64, object: Rc<dyn Intersectable>, point: Point, eyev: Vector, normalv: Vector, inside: bool) -> Computations {
        return Computations {
            time,
            object,
            point,
            eyev,
            normalv,
            inside
        }
    }
}