use std::rc::Rc;

use crate::PointLight;
use crate::Color;
use crate::Intersection;
use crate::Ray;
use crate::Point;
use crate::Vector;

pub trait Shape {
    fn get_id(&self) -> u64;

    fn transform_ray_to_obj_space(&self, world_ray: &Ray) -> Ray;
    fn transform_point_to_obj_space(&self, world_point: &Point) -> Point;
    // Using arbitrary self parameter here to allow for polymorphism in the world objects
    fn intersect(self: Rc<Self>, world_ray: &Ray) -> Vec<Intersection>;
    fn normal_at(&self, world_point: &Point) -> Vector;
    fn light_material(&self, world_point: &Point, light: &PointLight, eyev: &Vector, normalv: &Vector, in_shadow: bool) -> Color;
}
