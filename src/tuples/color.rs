use std::ops::{Add, Sub, Mul, Div};

use crate::EPSILON;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        return Color {
            red,
            green,
            blue
        };
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        if (self.red - other.red).abs() > EPSILON {
            return false;
        }
        else if (self.green - other.green).abs() > EPSILON {
            return false;
        }
        else if (self.blue - other.blue).abs() > EPSILON {
            return false;
        }
        else {
            return true;
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        return Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue
        };
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        return Self {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue
        };
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        return Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue
        };
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        return Self {
            red: self.red * rhs,
            blue: self.blue * rhs,
            green: self.green * rhs
        }
    }
}

impl Div for Color {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        return Self {
            red: self.red / rhs.red,
            green: self.green / rhs.green,
            blue: self.blue / rhs.blue
        };
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        return Self {
            red: self.red / rhs,
            blue: self.blue / rhs,
            green: self.green / rhs
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_normal_values_for_a_color_when_creating_it_should_expect_values_to_be_set_correctly() {
        let value = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(-0.5, value.red);
        assert_eq!(0.4, value.green);
        assert_eq!(1.7, value.blue);
    }

    #[test]
    fn given_two_colors_when_adding_them_should_calculate_result_correctly() {
        let color_a = Color::new(0.9, 0.6, 0.75);
        let color_b = Color::new(0.7, 0.1, 0.25);

        let expected = Color::new(1.6, 0.7, 1.0);
        let result = color_a + color_b;

        assert_eq!(expected, result);
    }

    #[test]
    fn given_two_colors_when_subtracting_them_should_calculate_result_correctly() {
        let color_a = Color::new(0.9, 0.6, 0.75);
        let color_b = Color::new(0.7, 0.1, 0.25);

        let expected = Color::new(0.2, 0.5, 0.5);
        let result = color_a - color_b;

        assert_eq!(expected, result);
    }

    #[test]
    fn given_a_color_and_a_scalar_when_multiplying_them_should_calculate_result_correctly() {
        let color = Color::new(0.2, 0.3, 0.4);
        let scalar = 2.0;

        let expected = Color::new(0.4, 0.6, 0.8);
        let result = color * scalar;

        assert_eq!(expected, result);
    }

    #[test]
    fn given_two_colors_when_multiplying_them_should_calculate_result_correctly() {
        let color_a = Color::new(1.0, 0.2, 0.4);
        let color_b = Color::new(0.9, 1.0, 0.1);

        let expected = Color::new(0.9, 0.2, 0.04);
        let result = color_a * color_b;

        assert_eq!(expected, result);
    }
}