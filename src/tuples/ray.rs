use crate::matrices::matrix::Matrix;

use super::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Tuple,
    direction: Tuple
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        return Ray {
            origin,
            direction
        };
    }

    pub fn origin(&self) -> Tuple {
        return self.origin.clone();
    }

    pub fn direction(&self) -> Tuple {
        return self.direction.clone();
    }

    pub fn position(&self, time: f64) -> Tuple {
        return self.origin + (self.direction * time);
    }

    pub fn transform(&self, transform: Matrix) -> Ray {
        let transformed_origin = transform.clone() * self.origin;
        let transformed_direction = transform * self.direction;

        return Ray::new(transformed_origin.unwrap(), transformed_direction.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_a_point_and_a_vector_when_creating_a_ray_should_initialise_origin_and_direction_correctly() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction);

        let expected_origin = Tuple::point(1.0, 2.0, 3.0);
        let expected_direction = Tuple::vector(4.0, 5.0, 6.0);

        assert_eq!(expected_origin, ray.origin);
        assert_eq!(expected_direction, ray.direction);
    }

    #[test]
    fn given_a_ray_and_times_when_computing_a_point_from_a_distance_should_return_correct_position() {
        let origin = Tuple::point(2.0, 3.0, 4.0);
        let direction = Tuple::vector(1.0, 0.0, 0.0);

        let ray = Ray::new(origin, direction);

        let position = ray.position(0.0);
        let expected_position = Tuple::point(2.0, 3.0, 4.0);
        
        assert_eq!(expected_position, position);

        let position = ray.position(1.0);
        let expected_position = Tuple::point(3.0, 3.0, 4.0);
        
        assert_eq!(expected_position, position);

        let position = ray.position(-1.0);
        let expected_position = Tuple::point(1.0, 3.0, 4.0);
        
        assert_eq!(expected_position, position);

        let position = ray.position(2.5);
        let expected_position = Tuple::point(4.5, 3.0, 4.0);
        
        assert_eq!(expected_position, position);
    }
}