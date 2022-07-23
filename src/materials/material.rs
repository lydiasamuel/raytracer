use crate::tuples::light::PointLight;
use crate::Vector;
use crate::Point;
use crate::Color;

pub trait Material {
    fn lighting(&self, point: &Point, light: &PointLight, eyev: &Vector, normalv: &Vector, in_shadow: bool) -> Color;
    fn box_clone(&self) -> Box<dyn Material>;
}

impl Clone for Box<dyn Material>
{
    fn clone(&self) -> Box<dyn Material> {
        self.box_clone()
    }
}
