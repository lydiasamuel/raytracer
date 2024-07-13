use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::EPSILON;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(x: {}, y: {}, z: {}, w: {})",
            self.x, self.y, self.z, self.w
        )
    }
}

impl Tuple {
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn origin() -> Tuple {
        Tuple::point(0.0, 0.0, 0.0)
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 1.0)
    }

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn is_vector(&self) -> bool {
        (0.0 - self.w).abs() < EPSILON
    }

    pub fn is_point(&self) -> bool {
        (1.0 - self.w).abs() < EPSILON
    }

    pub fn magnitude(&self) -> f64 {
        assert!(self.is_vector());

        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)).sqrt()
    }

    /* Normalization is the process of taking an arbitrary vector and converting it into a unit vector while preserving its direction.
     * Normalization keeps the calculations anchored relative to a common scale (the unit vector). If it we skipped normalizing
     * ray vectors or surface normals, the calculations would be scaled differently for every ray cast, and the scenes would not
     * render correctly.
     */
    pub fn normalize(self) -> Tuple {
        assert!(self.is_vector());

        let m = self.magnitude();

        Tuple::new(self.x / m, self.y / m, self.z / m, self.w / m)
    }

    /* The dot product can feel pretty abstract, but here's one quick way to internalize it:
     *
     * The smaller the dot product, the larger the angle between the vectors.
     *
     * A dot product of 1 means that the vectors are identical, and a dot product of -1 means they point in opposite directions.
     * More specifically, and again if the two vectors are unit vectors, the dot product is actually the cosine of the angle between them.
     */
    pub fn dot(lhs: Tuple, rhs: Tuple) -> f64 {
        assert!(lhs.is_vector());
        assert!(rhs.is_vector());

        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z + lhs.w * rhs.w
    }

    // The cross product calculates a new vector that is perpendicular to both of the original vectors.
    pub fn cross(lhs: Tuple, rhs: Tuple) -> Tuple {
        assert!(lhs.is_vector());
        assert!(rhs.is_vector());

        Tuple::vector(
            lhs.y * rhs.z - lhs.z * rhs.y,
            lhs.z * rhs.x - lhs.x * rhs.z,
            lhs.x * rhs.y - lhs.y * rhs.x,
        )
    }

    pub fn reflect(incoming: Tuple, normal: Tuple) -> Tuple {
        assert!(incoming.is_vector());
        assert!(normal.is_vector());

        incoming - (normal * 2.0 * Tuple::dot(incoming, normal))
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        if (self.x - other.x).abs() > EPSILON {
            false
        } else if (self.y - other.y).abs() > EPSILON {
            false
        } else if (self.z - other.z).abs() > EPSILON {
            false
        } else if (self.w - other.w).abs() > EPSILON {
            false
        } else {
            true
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Tuple {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts;

    use super::*;

    #[test]
    fn given_normal_values_for_a_tuple_when_creating_a_vector_should_expect_w_to_be_zero() {
        let result = Tuple::vector(4.3, -4.2, 3.1);

        assert_eq!(0.0, result.w);
    }

    #[test]
    fn given_normal_values_for_a_tuple_when_creating_a_point_should_expect_w_to_be_one() {
        let result = Tuple::point(4.3, -4.2, 3.1);

        assert_eq!(1.0, result.w);
    }

    #[test]
    fn given_no_values_when_getting_origin_point_should_expect_all_values_to_be_zero_and_w_to_be_one(
    ) {
        let result = Tuple::origin();

        assert_eq!(0.0, result.x);
        assert_eq!(0.0, result.y);
        assert_eq!(0.0, result.z);
        assert_eq!(1.0, result.w);
    }

    #[test]
    fn given_one_vector_and_one_point_when_adding_them_should_result_in_a_point() {
        let point = Tuple::point(3.0, -2.0, 5.0);
        let vector = Tuple::vector(-2.0, 3.0, 1.0);

        let result = point + vector;

        assert_eq!(1.0, result.x);
        assert_eq!(1.0, result.y);
        assert_eq!(6.0, result.z);
        assert_eq!(true, result.is_point());
    }

    #[test]
    fn given_two_points_when_subtracting_them_should_result_in_a_vector() {
        let point_a = Tuple::point(3.0, 2.0, 1.0);
        let point_b = Tuple::point(5.0, 6.0, 7.0);

        let result = point_a - point_b;

        assert_eq!(-2.0, result.x);
        assert_eq!(-4.0, result.y);
        assert_eq!(-6.0, result.z);
        assert_eq!(true, result.is_vector());
    }

    #[test]
    fn given_one_point_and_one_vector_when_subtracting_them_should_result_in_a_point() {
        let point = Tuple::point(3.0, 2.0, 1.0);
        let vector = Tuple::vector(5.0, 6.0, 7.0);

        let result = point - vector;

        assert_eq!(-2.0, result.x);
        assert_eq!(-4.0, result.y);
        assert_eq!(-6.0, result.z);
        assert_eq!(true, result.is_point());
    }

    #[test]
    fn given_two_vectors_when_subtracting_them_should_result_in_a_vector() {
        let vector_a = Tuple::vector(3.0, 2.0, 1.0);
        let vector_b = Tuple::vector(5.0, 6.0, 7.0);

        let result = vector_a - vector_b;

        assert_eq!(-2.0, result.x);
        assert_eq!(-4.0, result.y);
        assert_eq!(-6.0, result.z);
        assert_eq!(true, result.is_vector());
    }

    #[test]
    fn given_a_zero_vector_and_another_when_subtracting_them_should_result_in_the_negated_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let vector = Tuple::vector(1.0, -2.0, 3.0);

        let result = zero - vector;

        assert_eq!(-1.0, result.x);
        assert_eq!(2.0, result.y);
        assert_eq!(-3.0, result.z);
        assert_eq!(true, result.is_vector());
    }

    #[test]
    fn given_a_tuple_when_applying_neg_operator_should_result_in_the_negated_tuple() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let result = -tuple;

        assert_eq!(-1.0, result.x);
        assert_eq!(2.0, result.y);
        assert_eq!(-3.0, result.z);
        assert_eq!(4.0, result.w);
    }

    #[test]
    fn given_a_tuple_when_multiplying_by_a_scalar_should_multiply_each_component_correctly() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let result = tuple * 3.5;

        assert_eq!(3.5, result.x);
        assert_eq!(-7.0, result.y);
        assert_eq!(10.5, result.z);
        assert_eq!(-14.0, result.w);
    }

    #[test]
    fn given_a_tuple_when_multiplying_by_a_fraction_should_multiply_each_component_correctly() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let result = tuple * 0.5;

        assert_eq!(0.5, result.x);
        assert_eq!(-1.0, result.y);
        assert_eq!(1.5, result.z);
        assert_eq!(-2.0, result.w);
    }

    #[test]
    fn given_a_tuple_when_dividing_by_a_scalar_should_divide_each_component_correctly() {
        let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);

        let result = tuple / 2.0;

        assert_eq!(0.5, result.x);
        assert_eq!(-1.0, result.y);
        assert_eq!(1.5, result.z);
        assert_eq!(-2.0, result.w);
    }

    #[test]
    fn given_basic_vectors_when_computing_magnitude_should_correctly_calculate_the_length() {
        let vector = Tuple::vector(1.0, 0.0, 0.0);

        assert_eq!(1.0, vector.magnitude());

        let vector = Tuple::vector(0.0, 1.0, 0.0);

        assert_eq!(1.0, vector.magnitude());

        let vector = Tuple::vector(0.0, 0.0, 1.0);

        assert_eq!(1.0, vector.magnitude());

        let vector = Tuple::vector(1.0, 2.0, 3.0);

        assert_eq!(14.0_f64.sqrt(), vector.magnitude());

        let vector = Tuple::vector(-1.0, -2.0, -3.0);

        assert_eq!(14.0_f64.sqrt(), vector.magnitude());
    }

    #[test]
    fn given_basic_vectors_when_normalizing_them_should_result_in_unit_vectors() {
        let vector = Tuple::vector(4.0, 0.0, 0.0);

        let result = vector.normalize();
        let expected = Tuple::vector(1.0, 0.0, 0.0);

        assert_eq!(expected, result);
        assert_eq!(1.0, result.magnitude());

        let vector = Tuple::vector(1.0, 2.0, 3.0);

        let result = vector.normalize();
        let expected = Tuple::vector(0.26726, 0.53452, 0.80178);

        assert_eq!(expected, result);
        assert_eq!(1.0, result.magnitude());
    }

    #[test]
    fn given_two_vectors_when_taking_dot_product_should_correctly_calculate_result() {
        let vector_a = Tuple::vector(1.0, 2.0, 3.0);
        let vector_b = Tuple::vector(2.0, 3.0, 4.0);

        let result = Tuple::dot(vector_a, vector_b);

        assert_eq!(20.0, result)
    }

    #[test]
    fn given_two_vectors_when_taking_cross_product_should_correctly_calculate_result() {
        let vector_a = Tuple::vector(1.0, 2.0, 3.0);
        let vector_b = Tuple::vector(2.0, 3.0, 4.0);

        let expected = Tuple::vector(-1.0, 2.0, -1.0);
        let result = Tuple::cross(vector_a, vector_b);

        assert_eq!(expected, result);

        let expected = Tuple::vector(1.0, -2.0, 1.0);
        let result = Tuple::cross(vector_b, vector_a);

        assert_eq!(expected, result);
    }

    #[test]
    fn given_a_vector_and_a_normal_when_reflecting_across_the_normal_at_45_degrees_should_correctly_calculate_result(
    ) {
        let incoming = Tuple::vector(1.0, -1.0, 0.0);
        let normal = Tuple::vector(0.0, 1.0, 0.0);

        let expected = Tuple::vector(1.0, 1.0, 0.0);
        let result = Tuple::reflect(incoming, normal);

        assert_eq!(expected, result);
    }

    #[test]
    fn given_a_vector_and_a_normal_when_reflecting_across_a_slanted_surface_should_correctly_calculate_result(
    ) {
        let incoming = Tuple::vector(0.0, -1.0, 0.0);
        let normal = Tuple::vector(consts::SQRT_2 / 2.0, consts::SQRT_2 / 2.0, 0.0);

        let expected = Tuple::vector(1.0, 0.0, 0.0);
        let result = Tuple::reflect(incoming, normal);

        assert_eq!(expected, result);
    }
}
