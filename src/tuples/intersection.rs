use std::rc::Rc;

use crate::geoentity::intersected::Intersected;

pub struct Intersection {
    time: f64,
    entity: Rc<dyn Intersected>
}

impl Intersection{
    pub fn new(time: f64, entity: Rc<dyn Intersected>) -> Intersection {
        return Intersection {
            time,
            entity
        }
    }

    pub fn hit(intersects: &Vec<Intersection>) -> Vec<Intersection> {
        let mut result: Vec<Intersection> = Vec::new();

        for i in intersects {
            if i.time > 0.0 {
                result.push(Intersection::new(i.time, i.entity.clone()));
            }
        }

        return result;
    }
}