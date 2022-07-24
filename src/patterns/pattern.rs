use crate::Point;
use crate::Color;

pub trait Pattern {
    fn pattern_at(&self, object_point: &Point) -> Color;
}