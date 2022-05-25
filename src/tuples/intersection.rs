use std::cmp::max;
use std::rc::Rc;

use crate::geoentity::intersectable::Intersectable;

pub struct Intersection {
    pub time: f64,
    pub entity: Rc<dyn Intersectable>
}

impl Intersection{
    pub fn new(time: f64, entity: Rc<dyn Intersectable>) -> Intersection {
        return Intersection {
            time,
            entity
        }
    }

    // Expects the intersect list to be in ascending sorted order
    pub fn hit(intersects: &Vec<Intersection>) -> Option<Intersection> {
        let mut result = None;

        for i in 0..intersects.len() {
            if intersects[i].time > 0.0 {
                let intersect = Intersection::new(intersects[i].time, intersects[i].entity.clone());
                return Some(intersect);
            }
        }

        return result;
    }
}