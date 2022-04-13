use crate::Vector;
use crate::Point;
use crate::Ray;

const EPSILON: f64 = 0.00001;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    id: u64,
    center: Point,
    radius: f64
}

impl Sphere {
    pub fn new(id: u64, center: Point, radius: f64) -> Sphere {
        return Sphere {
            id,
            center,
            radius
        }
    }

    pub fn unit(id: u64) -> Sphere {
        return Sphere::new(id, Point::new(0.0, 0.0, 0.0), 1.0);
    }

    pub fn intersect(&self, ray: Ray) -> Vec<f64> {
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

        let dist = ray.origin - self.center;

        let alpha = Vector::dot(ray.direction, ray.direction);
        let beta = Vector::dot(dist * 2.0, ray.direction);
        let gamma = Vector::dot(dist, dist) - (self.radius * self.radius);

        let discriminant = (beta * beta) - ((alpha *  gamma) * 4.0);

        // There are no solutions to this quadratic equation
        if discriminant < EPSILON || (alpha - 0.0).abs() < EPSILON {
            return Vec::new();
        }

        let t1 = (-beta + discriminant.sqrt()) / (alpha * 2.0);
        let t2 = (-beta - discriminant.sqrt()) / (alpha * 2.0);

        return vec![t1, t2];
    }
} 
