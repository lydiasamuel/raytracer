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

    // Transforms the ray and then calculates the resulting intersections with that ray
    //
    // Using an arbitrary self type here, basically allows for polymorphic Shape types
    // and requires that they must be wrapped in an Arc so that the intersection result can grab a reference.
    //
    // Note that: Intersections are returned in increasing order.
    //
    fn intersect(self: Arc<Self>, world_ray: &Ray) -> Vec<Intersection> {
        let inverse_transform = self.get_transform().inverse().unwrap();

        let local_ray = world_ray.transform(inverse_transform);

        self.local_intersect(&local_ray)
    }

    fn local_intersect(self: Arc<Self>, local_ray: &Ray) -> Vec<Intersection>;

    // Transformation matrix transforms points from object space to world space, and the inverse goes the other way.
    fn get_transform(&self) -> Arc<Matrix>;

    fn get_material(&self) -> Arc<dyn Material>;

    fn casts_shadow(&self) -> bool;

    // Assumes that the point will always be on the shape
    fn normal_at(&self, world_point: Tuple) -> Tuple {
        assert!(world_point.is_point());

        let inverse_transform = self.get_transform().inverse().unwrap();

        let local_point = (&inverse_transform * &world_point).unwrap();
        let local_normal = self.local_normal_at(local_point);

        let mut world_normal = (&inverse_transform.transpose() * &local_normal).unwrap();
        world_normal.w = 0.0;

        world_normal.normalize()
    }

    fn local_normal_at(&self, local_point: Tuple) -> Tuple;

    fn light_material(
        self: Arc<Self>,
        world_point: Tuple,
        light: PointLight,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color;
}
