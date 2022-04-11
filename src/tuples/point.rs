use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt;
use crate::tuples::vector::Vector;

const EPSILON: f64 = 0.00001;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {}, y: {}, z: {}, w: {})", self.x, self.y, self.z, self.w)
    }
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        return Point::construct(x, y, z, 1.0);
    }

    pub fn construct(x: f64, y: f64, z: f64, w: f64) -> Point {
        return Point {
            x,
            y,
            z,
            w
        };
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        if (self.x - other.x).abs() > EPSILON {
            return false;
        }
        else if (self.y - other.y).abs() > EPSILON {
            return false;
        }
        else if (self.z - other.z).abs() > EPSILON {
            return false;
        }
        else if (self.w - other.w).abs() > EPSILON {
            return false;
        }
        else {
            return true;
        }
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        return Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        };
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        return Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        }
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        return Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs
        }
    }
}

// Might not make sense
impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        return Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        };
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self {
        return Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        };
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Vector {
        return Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        };
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Point {
        return Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        };
    }
}
