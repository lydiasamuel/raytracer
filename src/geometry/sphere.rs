use std::sync::Arc;
use uuid::Uuid;

use crate::tuples::color::Color;
use crate::tuples::pointlight::PointLight;
use crate::{
    materials::{material::Material, phong::Phong},
    matrices::matrix::Matrix,
    tuples::{intersection::Intersection, ray::Ray, tuple::Tuple},
};

use super::shape::Shape;

pub struct Sphere {
    id: Uuid,
    transform: Arc<Matrix>,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn unit() -> Sphere {
        Sphere {
            id: Uuid::new_v4(),
            transform: Arc::new(Matrix::identity(4)),
            material: Arc::new(Phong::default()),
        }
    }

    pub fn new(transform: Arc<Matrix>, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            id: Uuid::new_v4(),
            transform,
            material,
        }
    }
}

impl Shape for Sphere {
    fn id(&self) -> Uuid {
        self.id.clone()
    }

    fn local_intersect(self: Arc<Self>, local_ray: &Ray) -> Vec<Intersection> {
        let geometric_origin = Tuple::origin();
        let ray_direction = local_ray.direction();
        let sphere_to_ray = local_ray.origin() - geometric_origin;

        let a = Tuple::dot(ray_direction, ray_direction);
        let b = 2.0 * Tuple::dot(ray_direction, sphere_to_ray);
        let c = Tuple::dot(sphere_to_ray, sphere_to_ray) - 1.0;

        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            return Vec::new();
        }

        let t1 = (-b + discriminant.sqrt()) / (a * 2.0);
        let t2 = (-b - discriminant.sqrt()) / (a * 2.0);

        let i1 = Intersection::new(t1, self.clone());
        let i2 = Intersection::new(t2, self.clone());

        if t1 < t2 {
            vec![i1, i2]
        } else {
            vec![i2, i1]
        }
    }

    fn get_transform(&self) -> Arc<Matrix> {
        self.transform.clone()
    }

    fn get_material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        local_point - Tuple::origin()
    }

    fn light_material(
        self: Arc<Self>,
        world_point: Tuple,
        light: PointLight,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        self.get_material()
            .lighting(self, light, world_point, eyev, normalv, in_shadow)
    }
}

#[cfg(test)]
mod tests {
    use crate::patterns::solid::Solid;
    use crate::tuples::color::Color;
    use std::f64::consts;

    use super::*;

    #[test]
    fn given_a_ray_and_a_sphere_when_calculating_the_intersections_should_expect_two_points() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let sphere = Sphere::unit();

        let shape: Arc<dyn Shape> = Arc::new(sphere);

        let intersections = shape.intersect(&ray);

        assert_eq!(2, intersections.len());
        assert_eq!(4.0, intersections[0].time);
        assert_eq!(6.0, intersections[1].time);
    }

    #[test]
    fn given_a_ray_and_a_sphere_when_calculating_the_intersections_at_a_tangent_should_expect_two_equal_points(
    ) {
        let ray = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let sphere = Sphere::unit();

        let shape: Arc<dyn Shape> = Arc::new(sphere);

        let intersections = shape.intersect(&ray);

        assert_eq!(2, intersections.len());
        assert_eq!(5.0, intersections[0].time);
        assert_eq!(5.0, intersections[1].time);
    }

    #[test]
    fn given_a_sphere_and_a_ray_that_will_miss_when_calculating_the_intersections_should_expect_no_points(
    ) {
        let ray = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let sphere = Sphere::unit();

        let shape: Arc<dyn Shape> = Arc::new(sphere);

        let intersections = shape.intersect(&ray);

        assert_eq!(0, intersections.len());
    }

    #[test]
    fn given_a_sphere_and_a_ray_that_is_inside_it_when_calculating_the_intersections_should_expect_two_points(
    ) {
        let ray = Ray::new(Tuple::origin(), Tuple::vector(0.0, 0.0, 1.0));

        let sphere = Sphere::unit();

        let shape: Arc<dyn Shape> = Arc::new(sphere);

        let intersections = shape.intersect(&ray);

        assert_eq!(2, intersections.len());
        assert_eq!(-1.0, intersections[0].time);
        assert_eq!(1.0, intersections[1].time);
    }

    #[test]
    fn given_a_ray_and_a_sphere_that_is_behind_it_when_calculating_the_intersections_should_expect_two_negative_points(
    ) {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));

        let sphere = Sphere::unit();

        let shape: Arc<dyn Shape> = Arc::new(sphere);

        let intersections = shape.intersect(&ray);

        assert_eq!(2, intersections.len());
        assert_eq!(-6.0, intersections[0].time);
        assert_eq!(-4.0, intersections[1].time);
    }

    #[test]
    fn given_a_ray_and_a_sphere_when_calculating_the_intersections_should_expect_intersections_to_reference_the_sphere(
    ) {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let sphere = Sphere::unit();

        let shape: Arc<dyn Shape> = Arc::new(sphere);

        let intersections = shape.clone().intersect(&ray);

        assert_eq!(2, intersections.len());
        assert_eq!(4.0, intersections[0].time);
        assert_eq!(6.0, intersections[1].time);

        assert!(Arc::ptr_eq(&shape, &intersections[0].object));
        assert!(Arc::ptr_eq(&shape, &intersections[1].object));
    }

    #[test]
    fn given_a_new_unit_sphere_when_constructing_it_should_expect_default_transformation_to_be_identity_matrix(
    ) {
        let sphere = Sphere::unit();

        assert_eq!(Matrix::identity(4), *sphere.get_transform());
    }

    #[test]
    fn given_a_new_unit_sphere_when_updating_the_transform_should_expect_transform_to_be_set() {
        let transform = Arc::new(Matrix::translation(2.0, 3.0, 4.0));

        let sphere = Sphere::new(transform.clone(), Arc::new(Phong::default()));

        assert!(Arc::ptr_eq(&transform, &sphere.get_transform()));
    }

    #[test]
    fn given_a_ray_and_a_scaled_sphere_when_calculating_the_intersections_should_expect_correctly_scaled_points(
    ) {
        let transform = Arc::new(Matrix::scaling(2.0, 2.0, 2.0));

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let shape: Arc<dyn Shape> = Arc::new(Sphere::new(transform, Arc::new(Phong::default())));

        let intersections = shape.clone().intersect(&ray);

        assert_eq!(2, intersections.len());
        assert_eq!(3.0, intersections[0].time);
        assert_eq!(7.0, intersections[1].time);
    }

    #[test]
    fn given_a_ray_and_a_translated_sphere_when_calculating_the_intersections_should_expect_correctly_translated_points(
    ) {
        let transform = Arc::new(Matrix::translation(5.0, 0.0, 0.0));

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let sphere = Sphere::new(transform, Arc::new(Phong::default()));

        let shape: Arc<dyn Shape> = Arc::new(sphere);

        let intersections = shape.clone().intersect(&ray);

        assert_eq!(0, intersections.len());
    }

    #[test]
    fn given_a_point_on_the_x_axis_and_a_unit_sphere_when_calculating_the_normal_at_the_point_should_expect_normal_pointing_along_the_axis(
    ) {
        let point = Tuple::point(1.0, 0.0, 0.0);

        let sphere = Sphere::unit();

        let normal = sphere.normal_at(point);

        let expected = Tuple::vector(1.0, 0.0, 0.0);

        assert_eq!(expected, normal);
    }

    #[test]
    fn given_a_point_on_the_y_axis_and_a_unit_sphere_when_calculating_the_normal_at_the_point_should_expect_normal_pointing_along_the_axis(
    ) {
        let point = Tuple::point(0.0, 1.0, 0.0);

        let sphere = Sphere::unit();

        let normal = sphere.normal_at(point);

        let expected = Tuple::vector(0.0, 1.0, 0.0);

        assert_eq!(expected, normal);
    }

    #[test]
    fn given_a_point_on_the_z_axis_and_a_unit_sphere_when_calculating_the_normal_at_the_point_should_expect_normal_pointing_along_the_axis(
    ) {
        let point = Tuple::point(0.0, 0.0, 1.0);

        let sphere = Sphere::unit();

        let normal = sphere.normal_at(point);

        let expected = Tuple::vector(0.0, 0.0, 1.0);

        assert_eq!(expected, normal);
    }

    #[test]
    fn given_a_nonaxial_point_and_a_unit_sphere_when_calculating_the_normal_at_the_point_should_expect_correct_normal(
    ) {
        let value = 3.0_f64.sqrt() / 3.0;

        let point = Tuple::point(value, value, value);

        let sphere = Sphere::unit();

        let normal = sphere.normal_at(point);

        let expected = Tuple::vector(value, value, value);

        assert_eq!(expected, normal);
    }

    #[test]
    fn given_an_axial_point_and_a_translated_sphere_when_calculating_the_normal_at_the_point_should_expect_correct_normal(
    ) {
        let point = Tuple::point(0.0, 1.0, 0.0);

        let transform = Arc::new(Matrix::translation(0.0, 1.0, 0.0));

        let sphere = Sphere::new(transform, Arc::new(Phong::default()));

        let normal = sphere.normal_at(point);

        let expected = Tuple::vector(0.0, 1.70711, -0.70711);

        assert_eq!(expected, normal);
    }

    #[test]
    fn given_a_nonaxial_point_and_a_transformed_sphere_when_calculating_the_normal_at_the_point_should_expect_correct_normal(
    ) {
        let point = Tuple::point(0.0, consts::SQRT_2 / 2.0, -consts::SQRT_2 / 2.0);

        let transform = &Matrix::scaling(1.0, 0.5, 1.0) * &Matrix::rotation_z(consts::PI / 5.0);

        let sphere = Sphere::new(Arc::new(transform.unwrap()), Arc::new(Phong::default()));

        let normal = sphere.normal_at(point);

        let expected = Tuple::vector(0.0, 0.9701425, -0.2425356);

        assert_eq!(expected, normal);
    }

    #[test]
    fn given_a_unit_sphere_when_assigning_material_to_it_should_expect_material_to_be_set() {
        let material: Arc<dyn Material> = Arc::new(Phong::new(
            Box::new(Solid::new(Color::red())),
            0.2,
            1.0,
            0.9,
            220.0,
            0.0,
        ));

        let sphere = Sphere::new(Arc::new(Matrix::identity(4)), material.clone());

        let result = sphere.get_material();

        assert!(Arc::ptr_eq(&material, &result));
    }
}
