use std::sync::{Arc, RwLock, Weak};
use uuid::Uuid;

use crate::geometry::shape::Shape;
use crate::materials::material::Material;
use crate::matrices::matrix::Matrix;
use crate::tuples::color::Color;
use crate::tuples::intersection::Intersection;
use crate::tuples::pointlight::PointLight;
use crate::tuples::ray::Ray;
use crate::tuples::tuple::Tuple;
use crate::materials::phong::Phong;

pub struct Group {
    id: Uuid,
    transform: Arc<Matrix>,
    material: Arc<dyn Material>,
    children: RwLock<Vec<Arc<dyn Shape>>>,
    parent: RwLock<Weak<Group>>,
    casts_shadow: bool,
}

impl Group {
    pub fn default() -> Group {
        Group {
            id: Uuid::new_v4(),
            transform: Arc::new(Matrix::identity(4)),
            material: Arc::new(Phong::default()),
            children: RwLock::new(Vec::new()),
            parent: RwLock::new(Weak::new()),
            casts_shadow: true,
        }
    }

    pub fn new(transform: Arc<Matrix>, children: Vec<Arc<dyn Shape>>) -> Group {
        Group {
            id: Uuid::new_v4(),
            transform,
            material: Arc::new(Phong::default()),
            children: RwLock::new(children),
            parent: RwLock::new(Weak::new()),
            casts_shadow: true,
        }
    }

    pub fn add_child(self: Arc<Self>, child: Arc<dyn Shape>) {
        // Set the child's parent using the interior mutability method
        child.set_parent(self.clone());
        // Add it to the child list
        let mut tmp = self.children.write().unwrap();
        tmp.push(child.clone());
    }
}

impl Shape for Group {
    fn id(&self) -> Uuid {
        self.id
    }

    fn local_intersect(self: Arc<Self>, local_ray: &Ray) -> Vec<Intersection> {
        todo!()
    }

    fn get_transform(&self) -> Arc<Matrix> {
        self.transform.clone()
    }

    fn get_material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }

    fn get_parent(&self) -> Option<Arc<Group>> {
        self.parent.read().unwrap().upgrade()
    }

    fn set_parent(&self, parent: Arc<Group>) {
        let mut tmp = self.parent.write().unwrap();
        *tmp = Arc::downgrade(&parent);
    }

    fn casts_shadow(&self) -> bool {
        self.casts_shadow
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
        self.get_material()
            .lighting(self, light, world_point, eyev, normalv, in_shadow)
    }
}
