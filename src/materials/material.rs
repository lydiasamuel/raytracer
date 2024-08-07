use crate::geometry::shape::Shape;
use crate::tuples::{color::Color, point_light::PointLight, tuple::Tuple};
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn lighting(
        &self,
        object: Arc<dyn Shape>,
        light: PointLight,
        world_point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color;

    fn reflective(&self) -> f64;

    fn transparency(&self) -> f64;

    fn refractive_index(&self) -> f64;
}
