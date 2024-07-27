use crate::geometry::shape::Shape;
use crate::materials::material::Material;
use crate::matrices::matrix::Matrix;
use crate::tuples::color::Color;
use crate::tuples::intersection::Intersection;
use crate::tuples::pointlight::PointLight;
use crate::tuples::ray::Ray;
use crate::tuples::tuple::Tuple;
use std::sync::{Arc, RwLock, Weak};
use uuid::Uuid;

pub struct Group {
    id: Uuid,
    transform: Arc<Matrix>,
    material: Arc<dyn Material>,
    children: Vec<Arc<dyn Shape>>,
    parent: RwLock<Weak<Group>>,
    casts_shadow: bool,
}

impl Group {}

impl Shape for Group {
    fn id(&self) -> Uuid {
        todo!()
    }

    fn local_intersect(self: Arc<Self>, local_ray: &Ray) -> Vec<Intersection> {
        todo!()
    }

    fn get_transform(&self) -> Arc<Matrix> {
        todo!()
    }

    fn get_material(&self) -> Arc<dyn Material> {
        todo!()
    }

    fn get_parent(&self) -> Option<Arc<Group>> {
        todo!()
    }

    fn set_parent(&mut self, parent: Arc<Group>) {
        todo!()
    }

    fn casts_shadow(&self) -> bool {
        todo!()
    }

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        todo!()
    }

    fn light_material(
        self: Arc<Self>,
        world_point: Tuple,
        light: PointLight,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        todo!()
    }
}
