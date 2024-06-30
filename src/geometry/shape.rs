use std::rc::Rc;
use uuid::Uuid;

use crate::tuples::color::Color;
use crate::tuples::pointlight::PointLight;
use crate::{
    materials::material::Material,
    matrices::matrix::Matrix,
    tuples::{intersection::Intersection, ray::Ray, tuple::Tuple},
};

pub trait Shape {
    fn id(&self) -> Uuid;

    /* Using an arbitraty self type here, basically allows for polymorphic Shape types
     * and requires that they must be wrapped in an Rc so that the intersection result can grab a reference.
     *
     * Note that: Intersections are returned in increasing order.
     */
    fn intersect(self: Rc<Self>, world_ray: &Ray) -> Vec<Intersection>;

    // Transformation matrix transforms points from object space to world space, and the inverse goes the other way.
    fn get_transform(&self) -> Matrix;

    fn set_transform(&mut self, transform: Matrix);

    fn get_material(&self) -> Rc<dyn Material>;

    fn set_material(&mut self, material: &Rc<dyn Material>);

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
