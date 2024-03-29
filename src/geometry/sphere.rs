use std::rc::Rc;

use uuid::Uuid;

use crate::{materials::{material::Material, phong::Phong}, matrices::matrix::Matrix, tuples::{intersection::Intersection, ray::Ray, tuple::Tuple}};

use super::shape::Shape;

pub struct Sphere {
    id: Uuid,
    transform: Matrix,
    material: Rc<dyn Material>
}

impl Sphere {
    pub fn unit() -> Sphere {
        return Sphere {
            id: Uuid::new_v4(),
            transform: Matrix::identity(4),
            material: Rc::new(Phong::default())
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

        let geometric_origin = Tuple::origin();
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

    fn get_transform(&self) -> Matrix {
        return self.transform.clone();
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn get_material(&self) -> Rc<dyn Material> {
        return self.material.clone();
    }

    fn set_material(&mut self, material: &Rc<dyn Material>) {
        self.material = material.clone();
    }

    fn normal_at(&self, world_point: Tuple) -> Tuple {
        let inverse_transform = self.transform.inverse().unwrap();

        let object_point = inverse_transform.clone() * world_point;
        let object_normal = object_point.unwrap() - Tuple::origin();
        
        let mut world_normal = (inverse_transform.transpose() * object_normal).unwrap();
        world_normal.w = 0.0;

        return world_normal.normalize();
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts;

    use crate::tuples::color::Color;

    use super::*;

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
            Tuple::origin(), 
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
        
        assert_eq!(Matrix::identity(4), sphere.get_transform());
    }

    #[test]
    fn given_a_new_unit_sphere_when_updating_the_transform_should_expect_transform_to_be_set() {
        let transform = Matrix::translation(2.0, 3.0, 4.0);
        
        let mut sphere = Sphere::unit();

        sphere.set_transform(transform.clone());
        
        assert_eq!(transform, sphere.get_transform());
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

    #[test]
    fn given_a_point_on_the_x_axis_and_a_unit_sphere_when_calculating_the_normal_at_the_point_should_expect_normal_pointing_along_the_axis() {      
        let point = Tuple::point(1.0, 0.0, 0.0);

        let sphere = Sphere::unit();

        let normal = sphere.normal_at(point);

        let expected = Tuple::vector(1.0, 0.0, 0.0);

        assert_eq!(expected, normal);
    }

    #[test]
    fn given_a_point_on_the_y_axis_and_a_unit_sphere_when_calculating_the_normal_at_the_point_should_expect_normal_pointing_along_the_axis() {      
        let point = Tuple::point(0.0, 1.0, 0.0);

        let sphere = Sphere::unit();

        let normal = sphere.normal_at(point);

        let expected = Tuple::vector(0.0, 1.0, 0.0);

        assert_eq!(expected, normal);
    }

    #[test]
    fn given_a_point_on_the_z_axis_and_a_unit_sphere_when_calculating_the_normal_at_the_point_should_expect_normal_pointing_along_the_axis() {      
        let point = Tuple::point(0.0, 0.0, 1.0);

        let sphere = Sphere::unit();

        let normal = sphere.normal_at(point);

        let expected = Tuple::vector(0.0, 0.0, 1.0);

        assert_eq!(expected, normal);
    }

    #[test]
    fn given_a_nonaxial_point_and_a_unit_sphere_when_calculating_the_normal_at_the_point_should_expect_correct_normal() {   
        let value = 3.0_f64.sqrt() / 3.0;

        let point = Tuple::point(value, value, value);

        let sphere = Sphere::unit();

        let normal = sphere.normal_at(point);

        let expected = Tuple::vector(value, value, value);

        assert_eq!(expected, normal);
    }

    #[test]
    fn given_an_axial_point_and_a_translated_sphere_when_calculating_the_normal_at_the_point_should_expect_correct_normal() {   
        let point = Tuple::point(0.0, 1.0, 0.0);

        let mut sphere = Sphere::unit();
        sphere.set_transform(Matrix::translation(0.0, 1.0, 0.0));

        let normal = sphere.normal_at(point);

        let expected = Tuple::vector(0.0, 1.70711, -0.70711);

        assert_eq!(expected, normal);
    }

    #[test]
    fn given_a_nonaxial_point_and_a_transformed_sphere_when_calculating_the_normal_at_the_point_should_expect_correct_normal() {   
        let point = Tuple::point(0.0, consts::SQRT_2 / 2.0, -consts::SQRT_2 / 2.0);

        let transform = Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(consts::PI / 5.0);

        let mut sphere = Sphere::unit();
        sphere.set_transform(transform.unwrap());

        let normal = sphere.normal_at(point);

        let expected = Tuple::vector(0.0, 0.9701425, -0.2425356);

        assert_eq!(expected, normal);
    }

    #[test]
    fn given_a_unit_sphere_when_assigning_material_to_it_should_expect_material_to_be_set() {   
        let mut sphere = Sphere::unit();
        let expected: Rc<dyn Material> = Rc::new(Phong::new(Color::red(), 0.2, 1.0, 0.9, 220.0)); 
    
        sphere.set_material(&expected);

        let result = sphere.get_material();

        assert!(Rc::ptr_eq(&expected, &result));
    }
}