use std::rc::Rc;

use crate::Color;
use crate::PointLight;
use crate::Matrix;
use crate::geoentity::shape::Shape;
use crate::tuples::intersection::Intersection;
use crate::materials::material::Material;
use crate::Vector;
use crate::Point;
use crate::Ray;

pub struct Sphere {
    id: u64,
    center: Point,
    radius: f64,
    transform: Matrix, // Used to translate a point from object space to world space
    material: Box<dyn Material>
}

impl Sphere {
    pub fn new(id: u64, center: Point, radius: f64, transform: Matrix, material: Box<dyn Material>) -> Sphere {
        return Sphere {
            id,
            center,
            radius,
            transform,
            material
        }
    }

    pub fn unit(id: u64, transform: Matrix, material: Box<dyn Material>) -> Sphere {
        return Sphere::new(id, Point::new(0.0, 0.0, 0.0), 1.0, transform, material);
    }
}

impl Shape for Sphere {
    fn get_id(&self) -> u64 {
        return self.id;
    }

    fn intersect(self: Rc<Self>, world_ray: &Ray) -> Vec<Intersection> {
        let object_ray = world_ray.transform(self.transform.inverse());

        // (p - c) . (p - c) = r^2  Eq.1 of a Circle i.e. all points p equal distance from the center c
        // p = o + (t * d)  Eq.2 for a ray i.e. all points starting from the origin in that direction

        // Subbing Eq.2 into Eq.1 gives the following:
        // (o + td - c) . (o + td - c) = r^2

        // Solving to find the zeros will give you the points of intersection with the plane of the sphere.
        // For this we can use the quadratic equation:
        // t = (-B +/- sqrt(B^2 - 4*A*Y)) / (2*A)
        // A = d . d
        // B = 2(o - c) . d
        // Y = (o - c) . (o - c) - r^2
        let dist = object_ray.origin - self.center;

        let alpha = Vector::dot(object_ray.direction, object_ray.direction);
        let beta = Vector::dot(dist * 2.0, object_ray.direction);
        let gamma = Vector::dot(dist, dist) - (self.radius * self.radius);

        let discriminant = (beta * beta) - ((alpha *  gamma) * 4.0);

        // There are no solutions to this quadratic equation
        if discriminant < 0.0 {
            return Vec::new();
        }

        let t1 = (-beta + discriminant.sqrt()) / (alpha * 2.0);
        let t2 = (-beta - discriminant.sqrt()) / (alpha * 2.0);

        let i1 = Intersection::new(t1, self.clone());
        let i2 = Intersection::new(t2, self.clone());

        return vec![i1, i2];
    }

    fn normal_at(&self, world_point: &Point) -> Vector {
        let origin = Point::new(0.0, 0.0, 0.0);
        let object_point = (self.transform.inverse() * (*world_point)).unwrap();

        let object_normal_vector = (object_point - origin).normalize();
        let world_vector_transform = self.transform/*.submatrix(3, 3)*/.inverse().transpose();
        let mut world_normal_vector = (world_vector_transform * object_normal_vector).unwrap();

        world_normal_vector.w = 0.0; // Hack to reset the vector w marker back to 0, rather than multiplying by 3x3

        return world_normal_vector.normalize();
    }

    fn light_material(&self, world_point: &Point, light: &PointLight, eyev: &Vector, normalv: &Vector, in_shadow: bool) -> Color {
        let object_point = (self.transform.inverse() * (*world_point)).unwrap();

        return self.material.lighting(
            world_point, 
            object_point, 
            light,
            eyev,
            normalv,
            in_shadow);
    }
}