use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::tuples::point::Point;

const EPSILON: f64 = 0.00001;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        return Vector {
            x,
            y,
            z,
            w: 0.0
        };
    }

    pub fn magnitude(&self) -> f64 {
        return (self.x*self.x 
            + self.y*self.y 
            + self.z*self.z 
            + self.w*self.w).sqrt();
    }

    pub fn normalize(&self) -> Vector {
        let mag = self.magnitude();

        return Vector {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w
        };
    }

    pub fn dot(u: Vector, v: Vector) -> f64 {
        return u.x * v.x 
            + u.y * v.y 
            + u.z * v.z 
            + u.w * v.w
    }

    pub fn cross(u: Vector, v: Vector) -> Vector {
        return Vector {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
            w: 0.0
        };
    }
}

impl PartialEq for Vector {
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

impl Neg for Vector {
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

impl Mul<f64> for Vector {
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

impl Div<f64> for Vector {
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

impl Add for Vector {
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

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        return Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        };
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        return Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        };
    }
}

// Might not make sense
impl Sub<Point> for Vector {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        return Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        };
    }
}
