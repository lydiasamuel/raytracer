use crate::tuples::light::PointLight;
use crate::Vector;
use crate::Point;
use crate::Color;

pub trait Material {
    fn lighting(&self, point: &Point, light: &PointLight, eyev: &Vector, normalv: &Vector, in_shadow: bool) -> Color;
}