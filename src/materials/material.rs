use crate::tuples::{color::Color, pointlight::PointLight, tuple::Tuple};

pub trait Material {
    fn lighting(&self, light: &PointLight, point: &Tuple, eyev: &Tuple, normalv: &Tuple) -> Color;
}
