use std::rc::Rc;

use crate::Matrix;
use crate::geoentity::intersected::Intersected;
use crate::tuples::intersection::Intersection;
use crate::Vector;
use crate::Point;
use crate::Ray;

#[derive(Debug, Clone)]
pub struct Sphere {
    id: u64,
    center: Point,
    radius: f64,
    transform: Matrix
}

impl Sphere {
    pub fn new(id: u64, center: Point, radius: f64, transform: Matrix) -> Sphere {
        return Sphere {
            id,
            center,
            radius,
            transform
        }
    }

    pub fn unit(id: u64) -> Sphere {
        return Sphere::new(id, Point::new(0.0, 0.0, 0.0), 1.0, Matrix::identity(4));
    }

    pub fn intersect(this: &Rc<Sphere>, ray: Ray) -> Vec<Intersection> {
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

        let transform = this.transform.inverse();
        let ray = ray.transform(transform);

        let dist = ray.origin - this.center;

        let alpha = Vector::dot(ray.direction, ray.direction);
        let beta = Vector::dot(dist * 2.0, ray.direction);
        let gamma = Vector::dot(dist, dist) - (this.radius * this.radius);

        let discriminant = (beta * beta) - ((alpha *  gamma) * 4.0);

        // There are no solutions to this quadratic equation
        if discriminant < 0.0 {
            return Vec::new();
        }

        let t1 = (-beta + discriminant.sqrt()) / (alpha * 2.0);
        let t2 = (-beta - discriminant.sqrt()) / (alpha * 2.0);

        let i1 = Intersection::new(t1, this.clone());
        let i2 = Intersection::new(t2, this.clone());

        return vec![i1, i2];
    }

    pub fn set_transform(self, transform: Matrix) -> Sphere {
        return Sphere {
            id: self.id,
            center: self.center,
            radius: self.radius,
            transform
        }
    }
}

impl Intersected for Sphere {
    fn get_id(&self) -> u64 {
        return self.id;
    }
}