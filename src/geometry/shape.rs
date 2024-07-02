use std::sync::Arc;
use uuid::Uuid;

use crate::tuples::color::Color;
use crate::tuples::pointlight::PointLight;
use crate::{
    materials::material::Material,
    matrices::matrix::Matrix,
    tuples::{intersection::Intersection, ray::Ray, tuple::Tuple},
};

pub trait Shape: Sync + Send {
    fn id(&self) -> Uuid;

    /* Using an arbitraty self type here, basically allows for polymorphic Shape types
     * and requires that they must be wrapped in an Arc so that the intersection result can grab a reference.
     *
     * Note that: Intersections are returned in increasing order.
     */
    fn intersect(self: Arc<Self>, world_ray: &Ray) -> Vec<Intersection>;

    // Transformation matrix transforms points from object space to world space, and the inverse goes the other way.
    fn get_transform(&self) -> Matrix;

    fn get_material(&self) -> Arc<dyn Material>;

    // Assumes that the point will always be on the shape
    fn normal_at(&self, world_point: Tuple) -> Tuple;

    fn light_material(
        &self,
        world_point: &Tuple,
        light: &PointLight,
        eyev: &Tuple,
        normalv: &Tuple,
    ) -> Color;
}
