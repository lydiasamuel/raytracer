use crate::geometry::group::Group;
use crate::geometry::shape::Shape;
use crate::materials::material::Material;
use crate::materials::phong::Phong;
use crate::matrices::matrix::Matrix;
use crate::tuples::bounding_box::BoundingBox;
use crate::tuples::color::Color;
use crate::tuples::intersection::Intersection;
use crate::tuples::point_light::PointLight;
use crate::tuples::ray::Ray;
use crate::tuples::tuple::Tuple;
use std::sync::{Arc, RwLock, Weak};
use uuid::Uuid;

pub struct CSG {
    id: Uuid,
    transform: Arc<Matrix>,
    material: Arc<dyn Material>,
    operation: Operation,
    left: RwLock<Arc<dyn Shape>>,
    right: RwLock<Arc<dyn Shape>>,
    parent: RwLock<Weak<dyn Shape>>,
    casts_shadow: bool,
    bounds: RwLock<Option<BoundingBox>>,
}

pub enum Operation {
    Difference,
    Intersection,
    Union,
}

impl CSG {

}

impl Shape for CSG {
    fn id(&self) -> Uuid {
        self.id
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

    fn get_parent(&self) -> Option<Arc<dyn Shape>> {
        self.parent.read().unwrap().upgrade()
    }

    fn set_parent(&self, parent: &Arc<dyn Shape>) {
        *self.parent.write().unwrap() = Arc::downgrade(parent);
    }

    fn casts_shadow(&self) -> bool {
        todo!()
    }

    fn local_normal_at(&self, local_point: Tuple, hit: &Intersection) -> Tuple {
        todo!()
    }

    fn bounds(&self) -> BoundingBox {
        todo!()
    }

    fn points(&self) -> (Tuple, Tuple, Tuple) {
        todo!()
    }

    fn normals(&self) -> (Tuple, Tuple, Tuple) {
        todo!()
    }

    fn edge_vectors(&self) -> (Tuple, Tuple) {
        todo!()
    }

    fn divide(self: Arc<Self>, threshold: usize) {
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
