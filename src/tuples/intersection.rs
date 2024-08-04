use std::cmp::Ordering;
use std::sync::Arc;

use crate::geometry::shape::Shape;
use crate::EPSILON;

#[derive(Clone)]
pub struct Intersection {
    time: f64,
    object: Arc<dyn Shape>,
    // Properties specific to a SmoothTriangle, used to help identify where on the triangle the
    // intersection occurred, relative to the triangle's corners.
    u: f64,
    v: f64,
}

impl Intersection {
    pub fn new(time: f64, object: Arc<dyn Shape>) -> Intersection {
        Intersection {
            time,
            object,
            u: 0.0,
            v: 0.0,
        }
    }

    pub fn new_with_uv(time: f64, object: Arc<dyn Shape>, u: f64, v: f64) -> Intersection {
        Intersection { time, object, u, v }
    }

    // Assumes list of intersection is in ascending order by time
    pub fn hit(intersections: &Vec<Intersection>) -> Option<(usize, bool)> {
        for i in 0..intersections.len() {
            let intersect = &intersections[i];
            if intersect.time > 0.0 {
                return Some((i, intersect.object.casts_shadow()));
            }
        }

        None
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn object(&self) -> Arc<dyn Shape> {
        self.object.clone()
    }

    pub fn u(&self) -> f64 {
        self.u
    }

    pub fn v(&self) -> f64 {
        self.v
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.total_cmp(&other.time)
    }
}

impl PartialOrd<Self> for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Intersection {}

impl PartialEq<Self> for Intersection {
    fn eq(&self, other: &Self) -> bool {
        if (self.time - other.time).abs() > EPSILON {
            false
        } else {
            Arc::ptr_eq(&self.object, &other.object)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::sphere::Sphere;

    use super::*;

    #[test]
    fn given_a_list_of_all_positive_time_intersections_when_identifying_the_hit_should_expect_the_lowest_positive_time_intersection(
    ) {
        let sphere = Sphere::unit();

        let shape: Arc<dyn Shape> = Arc::new(sphere);

        let intersections = vec![
            Intersection::new(1.0, shape.clone()),
            Intersection::new(2.0, shape.clone()),
        ];

        let hit = Intersection::hit(&intersections).unwrap();

        assert_eq!(true, intersections[0] == intersections[hit.0]);
    }

    #[test]
    fn given_a_list_intersections_with_some_negative_times_when_identifying_the_hit_should_expect_the_lowest_positive_time_intersection(
    ) {
        let sphere = Sphere::unit();

        let shape: Arc<dyn Shape> = Arc::new(sphere);

        let intersections = vec![
            Intersection::new(-1.0, shape.clone()),
            Intersection::new(1.0, shape.clone()),
        ];

        let hit = Intersection::hit(&intersections).unwrap();

        assert_eq!(true, intersections[1] == intersections[hit.0]);
    }

    #[test]
    fn given_a_list_intersections_with_all_negative_times_when_identifying_the_hit_should_return_nothing(
    ) {
        let sphere = Sphere::unit();

        let shape: Arc<dyn Shape> = Arc::new(sphere);

        let intersections = vec![
            Intersection::new(-2.0, shape.clone()),
            Intersection::new(-1.0, shape.clone()),
        ];

        let hit = Intersection::hit(&intersections);

        assert!(hit.is_none());
    }
}
