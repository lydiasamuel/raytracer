use crate::Point;
use crate::Color;

#[derive(Debug, Copy, Clone)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Point
}

impl PointLight {
    pub fn new(intensity: Color, position: Point) -> PointLight{
        return PointLight {
            intensity,
            position
        };
    }
}