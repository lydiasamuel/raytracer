use crate::Shape;
use std::rc::Rc;

pub struct Intersection {
    pub time: f64,
    pub entity: Rc<dyn Shape>
}

impl Intersection{
    pub fn new(time: f64, entity: Rc<dyn Shape>) -> Intersection {
        return Intersection {
            time,
            entity
        }
    }

    // Expects the intersect list to be in ascending sorted order
    pub fn hit(intersects: &Vec<Intersection>) -> Option<Intersection> {
        for i in 0..intersects.len() {
            if intersects[i].time > 0.0 {
                let intersect = Intersection::new(intersects[i].time, intersects[i].entity.clone());
                return Some(intersect);
            }
        }

        return None;
    }
}