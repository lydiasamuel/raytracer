use std::sync::Arc;

use crate::geometry::shape::Shape;
use crate::tuples::tuple::Tuple;

pub struct Computations {
    pub time: f64,
    pub object: Arc<dyn Shape>,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

impl Computations {
    pub fn new(
        time: f64,
        object: Arc<dyn Shape>,
        point: Tuple,
        over_point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        inside: bool,
    ) -> Computations {
        assert!(point.is_point());
        assert!(over_point.is_point());
        assert!(eyev.is_vector());
        assert!(normalv.is_vector());

        return Computations {
            time,
            object,
            point,
            over_point,
            eyev,
            normalv,
            inside,
        };
    }
}
