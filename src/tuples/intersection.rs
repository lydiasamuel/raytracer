use std::rc::Rc;

use crate::geometry::shape::Shape;

pub struct Intersection {
    pub time: f64,
    pub shape: Rc<dyn Shape>
}

impl Intersection {
    pub fn new(time: f64, shape: Rc<dyn Shape>) -> Intersection {
        return Intersection {
            time,
            shape
        }
    }
}