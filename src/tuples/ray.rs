use crate::Matrix;
use crate::matrices::matrix::MatrixTransform;
use crate::tuples::point::Point;
use crate::tuples::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        return Ray {
            origin,
            direction
        };
    }

    pub fn position(&self, time: f64) -> Point {
        return self.origin + (self.direction * time);
    }

    pub fn transform(&self, matrix: Matrix) -> Ray {
        let result: Ray;

        match matrix.transform_type {
            Some(MatrixTransform::Translation) => {
                let new_origin = matrix * self.origin;
                result = Ray::new(new_origin.unwrap(), self.direction);
            },
            Some(MatrixTransform::Scaling) => {
                let tmp = matrix.clone();

                let new_origin = matrix * self.origin;
                let new_direction = tmp * self.direction;

                result = Ray::new(new_origin.unwrap(), new_direction.unwrap());
            },
            Some(MatrixTransform::XRotation) => {
                result = self.clone();
            },
            Some(MatrixTransform::YRotation) => {
                result = self.clone();
            },
            Some(MatrixTransform::ZRotation) => {
                result = self.clone();
            },
            Some(MatrixTransform::Shearing) => {
                result = self.clone();
            },
            Some(MatrixTransform::Identity) => {
                result = self.clone();
            },
            None => {
                result = self.clone();
            }
        }

        return result;
    }
}