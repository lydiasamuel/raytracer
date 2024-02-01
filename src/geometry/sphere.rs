use std::rc::Rc;

use uuid::Uuid;

use crate::{matrices::matrix::Matrix, tuples::{intersection::Intersection, ray::Ray, tuple::Tuple}};

use super::shape::Shape;

#[derive(Debug, Clone)]
pub struct Sphere {
    id: Uuid,
    transform: Rc<Matrix>
}

impl Sphere {
    pub fn unit() -> Sphere {
        return Sphere {
            id: Uuid::new_v4(),
            transform: Rc::new(Matrix::identity(4))
        }
    }
}

impl Shape for Sphere {
    fn id(&self) -> Uuid {
        return self.id.clone();
    }

    fn intersect(self: Rc<Self>, world_ray: &Ray) -> Vec<Intersection> {
        let inverse_transform = self.get_transform().inverse().unwrap();
        let object_ray = world_ray.transform(inverse_transform);

        let geometric_origin = Tuple::point(0.0, 0.0, 0.0);
        let ray_direction = object_ray.direction();
        let sphere_to_ray = object_ray.origin() - geometric_origin;

        let a = Tuple::dot(&ray_direction, &ray_direction);
        let b = 2.0 * Tuple::dot(&ray_direction, &sphere_to_ray);
        let c = Tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b + discriminant.sqrt()) / (a * 2.0);
        let t2 = (-b - discriminant.sqrt()) / (a * 2.0);

        let i1 = Intersection::new(t1, self.clone());
        let i2 = Intersection::new(t2, self.clone());

        if t1 < t2 {
            return vec![i1, i2]; 
        }
        else {
            return vec![i2, i1];
        }
    }

    fn get_transform(&self) -> Rc<Matrix> {
        return self.transform.clone();
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = Rc::new(transform);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::borrow::Borrow;

    #[test]
    fn given_a_ray_and_a_sphere_when_calculating_the_intersections_should_expect_two_points() {
        let ray = Ray::new(
            Tuple::point(0.0, 0.0, -5.0), 
            Tuple::vector(0.0, 0.0, 1.0));

        let sphere = Sphere::unit();
        
        let shape: Rc<dyn Shape> = Rc::new(sphere);

        let intersections = shape.intersect(&ray);

        assert_eq!(2, intersections.len());
        assert_eq!(4.0, intersections[0].time);
        assert_eq!(6.0, intersections[1].time);
    }

    #[test]
    fn given_a_ray_and_a_sphere_when_calculating_the_intersections_at_a_tangent_should_expect_two_equal_points() {
        let ray = Ray::new(
            Tuple::point(0.0, 1.0, -5.0), 
            Tuple::vector(0.0, 0.0, 1.0));

        let sphere = Sphere::unit();
        
        let shape: Rc<dyn Shape> = Rc::new(sphere);

        let intersections = shape.intersect(&ray);

        assert_eq!(2, intersections.len());
        assert_eq!(5.0, intersections[0].time);
        assert_eq!(5.0, intersections[1].time);
    }

    #[test]
    fn given_a_sphere_and_a_ray_that_will_miss_when_calculating_the_intersections_should_expect_no_points() {
        let ray = Ray::new(
            Tuple::point(0.0, 2.0, -5.0), 
            Tuple::vector(0.0, 0.0, 1.0));

        let sphere = Sphere::unit();
        
        let shape: Rc<dyn Shape> = Rc::new(sphere);

        let intersections = shape.intersect(&ray);

        assert_eq!(0, intersections.len());
    }

    #[test]
    fn given_a_sphere_and_a_ray_that_is_inside_it_when_calculating_the_intersections_should_expect_two_points() {
        let ray = Ray::new(
            Tuple::point(0.0, 0.0, 0.0), 
            Tuple::vector(0.0, 0.0, 1.0));

        let sphere = Sphere::unit();
        
        let shape: Rc<dyn Shape> = Rc::new(sphere);

        let intersections = shape.intersect(&ray);

        assert_eq!(2, intersections.len());
        assert_eq!(-1.0, intersections[0].time);
        assert_eq!(1.0, intersections[1].time);
    }

    #[test]
    fn given_a_ray_and_a_sphere_that_is_behind_it_when_calculating_the_intersections_should_expect_two_negative_points() {
        let ray = Ray::new(
            Tuple::point(0.0, 0.0, 5.0), 
            Tuple::vector(0.0, 0.0, 1.0));

        let sphere = Sphere::unit();
        
        let shape: Rc<dyn Shape> = Rc::new(sphere);

        let intersections = shape.intersect(&ray);

        assert_eq!(2, intersections.len());
        assert_eq!(-6.0, intersections[0].time);
        assert_eq!(-4.0, intersections[1].time);
    }

    #[test]
    fn given_a_ray_and_a_sphere_when_calculating_the_intersections_should_expect_intersections_to_reference_the_sphere() {
        let ray = Ray::new(
            Tuple::point(0.0, 0.0, -5.0), 
            Tuple::vector(0.0, 0.0, 1.0));

        let sphere = Sphere::unit();
        
        let shape: Rc<dyn Shape> = Rc::new(sphere);

        let intersections = shape.clone().intersect(&ray);

        assert_eq!(2, intersections.len());
        assert_eq!(4.0, intersections[0].time);
        assert_eq!(6.0, intersections[1].time);

        assert!(Rc::ptr_eq(&shape, &intersections[0].shape));
        assert!(Rc::ptr_eq(&shape, &intersections[1].shape));
    }

    #[test]
    fn given_a_new_unit_sphere_when_constructing_it_should_expect_default_transformation_to_be_identity_matrix() {
        let sphere = Sphere::unit();
        
        assert_eq!(Matrix::identity(4), *sphere.get_transform().borrow());
    }

    #[test]
    fn given_a_new_unit_sphere_when_updating_the_transform_should_expect_transform_to_be_set() {
        let transform = Matrix::translation(2.0, 3.0, 4.0);
        
        let mut sphere = Sphere::unit();

        sphere.set_transform(transform.clone());
        
        assert_eq!(transform, *sphere.get_transform().borrow());
    }

    #[test]
    fn given_a_ray_and_a_scaled_sphere_when_calculating_the_intersections_should_expect_correctly_scaled_points() {
        let transform = Matrix::scaling(2.0, 2.0, 2.0);
        
        let ray = Ray::new(
            Tuple::point(0.0, 0.0, -5.0), 
            Tuple::vector(0.0, 0.0, 1.0));

        let mut sphere = Sphere::unit();
        
        sphere.set_transform(transform);

        let shape: Rc<dyn Shape> = Rc::new(sphere);

        let intersections = shape.clone().intersect(&ray);

        assert_eq!(2, intersections.len());
        assert_eq!(3.0, intersections[0].time);
        assert_eq!(7.0, intersections[1].time);
    }

    #[test]
    fn given_a_ray_and_a_translated_sphere_when_calculating_the_intersections_should_expect_correctly_translated_points() {
        let transform = Matrix::translation(5.0, 0.0, 0.0);
        
        let ray = Ray::new(
            Tuple::point(0.0, 0.0, -5.0), 
            Tuple::vector(0.0, 0.0, 1.0));

        let mut sphere = Sphere::unit();
        
        sphere.set_transform(transform);

        let shape: Rc<dyn Shape> = Rc::new(sphere);

        let intersections = shape.clone().intersect(&ray);

        assert_eq!(0, intersections.len());
    }
}