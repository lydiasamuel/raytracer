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
use uuid::{uuid, Uuid};

pub struct TestShape {
    parent: RwLock<Weak<Group>>,
    saved_ray: RwLock<Option<Ray>>,
}

impl TestShape {
    pub fn new() -> TestShape {
        TestShape {
            parent: RwLock::new(Weak::new()),
            saved_ray: RwLock::new(None),
        }
    }

    pub fn saved_ray(&self) -> Option<Ray> {
        *self.saved_ray.read().unwrap()
    }
}

impl Shape for TestShape {
    fn id(&self) -> Uuid {
        uuid!("00000000-0000-0000-0000-000000000000")
    }

    fn local_intersect(self: Arc<Self>, local_ray: &Ray) -> Vec<Intersection> {
        *self.saved_ray.write().unwrap() = Some(*local_ray);
        vec![]
    }

    fn get_transform(&self) -> Arc<Matrix> {
        Arc::new(Matrix::identity(4))
    }

    fn get_material(&self) -> Arc<dyn Material> {
        Arc::new(Phong::default())
    }

    fn get_parent(&self) -> Option<Arc<Group>> {
        self.parent.read().unwrap().upgrade()
    }

    fn set_parent(&self, parent: &Arc<Group>) {
        *self.parent.write().unwrap() = Arc::downgrade(parent);
    }

    fn casts_shadow(&self) -> bool {
        true
    }

    fn local_normal_at(&self, _: Tuple) -> Tuple {
        panic!("Error: can't take normal of test shape ")
    }

    fn bounds(&self) -> BoundingBox {
        BoundingBox::new(Tuple::point(-1.0, -1.0, -1.0), Tuple::point(1.0, 1.0, 1.0))
    }

    fn divide(self: Arc<Self>, _: usize) {}

    fn light_material(
        self: Arc<Self>,
        _world_point: Tuple,
        _light: PointLight,
        _eyev: Tuple,
        _normalv: Tuple,
        _in_shadow: bool,
    ) -> Color {
        panic!("Error: can't light test shape")
    }
}
