use std::sync::Arc;

use crate::geometry::shape::Shape;
use crate::tuples::tuple::Tuple;

pub struct Computations {
    pub time: f64,
    pub object: Arc<dyn Shape>,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

impl Computations {
    pub fn new(
        time: f64,
        object: Arc<dyn Shape>,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        inside: bool,
    ) -> Computations {
        if !point.is_point() {
            panic!("Error: Tuple given for parameter 'point' is not a Point.")
        }
        if !eyev.is_vector() {
            panic!("Error: Tuple given for parameter 'eyev' is not a Vector.")
        }
        if !normalv.is_vector() {
            panic!("Error: Tuple given for parameter 'normalv' is not a Vector.")
        }

        return Computations {
            time,
            object,
            point,
            eyev,
            normalv,
            inside,
        };
    }
}
