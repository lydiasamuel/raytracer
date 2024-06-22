use crate::tuples::{color::Color, light::Light, tuple::Tuple};

pub trait Material {
    fn lighting(&self, light: &Light, point: &Tuple, eyev: &Tuple, normalv: &Tuple) -> Color;
}
