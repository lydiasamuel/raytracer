use std::ops::Add;
use crate::matrices::matrix::Matrix;
use crate::tuples::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    min: Tuple,
    max: Tuple,
}

impl BoundingBox {
    pub fn new(min: Tuple, max: Tuple) -> BoundingBox {
        BoundingBox {
            min,
            max
        }
    }

    pub fn empty() -> BoundingBox {
        BoundingBox {
            min: Tuple::point(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            max: Tuple::point(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }

    pub fn add_point(mut self, point: Tuple) -> Self {
        self.min = Tuple::point(
            f64::min(self.min.x, point.x),
            f64::min(self.min.y, point.y),
            f64::min(self.min.z, point.z),
        );

        self.max = Tuple::point(
            f64::max(self.max.x, point.x),
            f64::max(self.max.y, point.y),
            f64::max(self.max.z, point.z),
        );

        self
    }

    pub fn box_contains_box(&self, point: Tuple) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }

    pub fn transform(self, transform: &Matrix) -> Self {
        let p1 = self.min;
        let p2 = Tuple::point(self.min.x, self.min.y, self.max.z);
        let p3 = Tuple::point(self.min.x, self.max.y, self.min.z);
        let p4 = Tuple::point(self.min.x, self.max.y, self.max.z);
        let p5 = Tuple::point(self.max.x, self.min.y, self.min.z);
        let p6 = Tuple::point(self.max.x, self.min.y, self.max.z);
        let p7 = Tuple::point(self.max.x, self.max.y, self.min.z);
        let p8 = self.max;

        Self::empty()
            .add_point((transform * &p1).unwrap())
            .add_point((transform * &p2).unwrap())
            .add_point((transform * &p3).unwrap())
            .add_point((transform * &p4).unwrap())
            .add_point((transform * &p5).unwrap())
            .add_point((transform * &p6).unwrap())
            .add_point((transform * &p7).unwrap())
            .add_point((transform * &p8).unwrap())
    }

    pub fn min(&self) -> Tuple {
        self.min
    }

    pub fn max(&self) -> Tuple {
        self.max
    }
}

impl Add for BoundingBox {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.add_point(rhs.min).add_point(rhs.max)
    }
}