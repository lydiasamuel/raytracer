use std::sync::Arc;

use crate::geometry::shape::Shape;
use crate::tuples::tuple::Tuple;

pub struct Computations {
    pub time: f64,
    pub object: Arc<dyn Shape>,
    pub point: Tuple,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub n1: f64,
    pub n2: f64,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub reflectv: Tuple,
    pub inside: bool,
}

impl Computations {
    pub fn new(
        time: f64,
        object: Arc<dyn Shape>,
        point: Tuple,
        over_point: Tuple,
        under_point: Tuple,
        n1: f64,
        n2: f64,
        eyev: Tuple,
        normalv: Tuple,
        reflectv: Tuple,
        inside: bool,
    ) -> Computations {
        assert!(point.is_point());
        assert!(over_point.is_point());
        assert!(eyev.is_vector());
        assert!(normalv.is_vector());
        assert!(reflectv.is_vector());

        return Computations {
            time,
            object,
            point,
            over_point,
            under_point,
            n1,
            n2,
            eyev,
            normalv,
            reflectv,
            inside,
        };
    }
}
