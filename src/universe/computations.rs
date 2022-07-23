use std::rc::Rc;

use crate::Point;
use crate::geoentity::shape::Shape;
use crate::Vector;

pub struct Computations {
    pub time: f64,
    pub object: Rc<dyn Shape>,
    pub point: Point,
    pub over_point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool
}

impl Computations {
    pub fn new(time: f64, object: Rc<dyn Shape>, point: Point, over_point: Point, eyev: Vector, normalv: Vector, inside: bool) -> Computations {
        return Computations {
            time,
            object,
            point,
            over_point,
            eyev,
            normalv,
            inside
        }
    }
}