use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new(red: f32, blue: f32, green: f32) -> Color {
        return Color {
            red,
            green,
            blue
        };
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