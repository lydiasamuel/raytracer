use std::sync::Arc;

use crate::geometry::shape::Shape;

use crate::EPSILON;

#[derive(Clone)]
pub struct Intersection {
    pub time: f64,
    pub object: Arc<dyn Shape>,
}

impl Intersection {
    pub fn new(time: f64, object: Arc<dyn Shape>) -> Intersection {
        Intersection { time, object }
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
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        if (self.time - other.time).abs() > EPSILON {
            return false;
        }

        Arc::ptr_eq(&self.object, &other.object)
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

        let hit = Intersection::hit(&intersections);

        assert_eq!(true, intersections[0] == intersections[hit.unwrap()]);
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

        let hit = Intersection::hit(&intersections);

        assert_eq!(true, intersections[1] == intersections[hit.unwrap()]);
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
