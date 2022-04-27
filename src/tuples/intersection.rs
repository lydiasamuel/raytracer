use std::rc::Rc;

use crate::geoentity::intersected::Intersected;

pub struct Intersection {
    pub time: f64,
    pub entity: Rc<dyn Intersected>
}

impl Intersection{
    pub fn new(time: f64, entity: Rc<dyn Intersected>) -> Intersection {
        return Intersection {
            time,
            entity
        }
    }

    pub fn hit(intersects: &Vec<Intersection>) -> Option<Intersection> {
        let mut hit = false;
        let mut king: usize = 0;
        let mut i: usize = 0;

        for x in intersects {
            let current = x.time;

            if current > 0.0 {
                let best = intersects.get(king).unwrap().time;

                if current < best {
                    king = i;
                }
                
                hit = true;
            }

            i = i + 1;
        }

        if hit {
            let tmp = intersects.get(king).unwrap();
            return Some(Intersection::new(tmp.time, tmp.entity.clone()));
        } 
        else {
            return None;
        }
    }
}