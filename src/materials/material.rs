use crate::tuples::{color::Color, pointlight::PointLight, tuple::Tuple};

pub trait Material: Send + Sync {
    fn lighting(
        &self,
        light: &PointLight,
        world_point: &Tuple,
        object_point: Tuple,
        eyev: &Tuple,
        normalv: &Tuple,
        in_shadow: bool,
    ) -> Color;
}
