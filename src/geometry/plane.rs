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
use crate::EPSILON;
use std::sync::{Arc, RwLock, Weak};
use uuid::Uuid;

pub struct Plane {
    id: Uuid,
    transform: Arc<Matrix>, // Used to translate a point from object space to world space
    material: Arc<dyn Material>,
    parent: RwLock<Weak<Group>>,
    casts_shadow: bool,
}

impl Plane {
    pub fn new(transform: Arc<Matrix>, material: Arc<dyn Material>, casts_shadow: bool) -> Plane {
        Plane {
            id: Uuid::new_v4(),
            transform,
            material,
            parent: RwLock::new(Weak::new()),
            casts_shadow,
        }
    }

    pub fn default() -> Plane {
        Plane {
            id: Uuid::new_v4(),
            transform: Arc::new(Matrix::identity(4)),
            material: Arc::new(Phong::default()),
            parent: RwLock::new(Weak::new()),
            casts_shadow: true,
        }
    }
}

impl Shape for Plane {
    fn id(&self) -> Uuid {
        self.id.clone()
    }

    fn local_intersect(self: Arc<Self>, local_ray: &Ray) -> Vec<Intersection> {
        let origin = local_ray.origin();
        let direction = local_ray.direction();

        /*
            Four cases to consider
            1. Ray is parallel to the plane, and thus will never intersect it
            2. Ray is coplanar with the plane, which is to say the ray's origin is on the plane,
               and the ray's direction is parallel to the plane. Therefore, every point on the ray
               intersects the plane. We'll assume this misses.
            3. Ray origin is above the plane
            4. Ray origin is below the plane
        */

        if direction.y.abs() < EPSILON {
            // Plane is in xz therefore if there's no y slope it's parallel
            return vec![]; // No intersections in this case
        }

        // Ray is either above or below the plane so calculate the intersection time
        let time = -origin.y / direction.y;

        vec![Intersection::new(time, self)]
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

    fn set_parent(&self, parent: &Arc<Group>) {
        *self.parent.write().unwrap() = Arc::downgrade(parent);
    }

    fn casts_shadow(&self) -> bool {
        self.casts_shadow
    }

    fn local_normal_at(&self, _: Tuple) -> Tuple {
        (&self.transform.inverse().unwrap() * &Tuple::vector(0.0, 1.0, 0.0)).unwrap()
    }

    fn bounds(&self) -> BoundingBox {
        BoundingBox::new(
            Tuple::point(f64::NEG_INFINITY, 0.0, f64::NEG_INFINITY),
            Tuple::point(f64::INFINITY, 0.0, f64::INFINITY),
        )
    }

    fn points(&self) -> (Tuple, Tuple, Tuple) {
        panic!("Error: points function is not implemented for this shape")
    }

    fn edge_vectors(&self) -> (Tuple, Tuple) {
        panic!("Error: edge_vectors function is not implemented for this shape")
    }
    fn divide(self: Arc<Self>, _: usize) {}

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

#[cfg(test)]
mod tests {
    use crate::geometry::plane::Plane;
    use crate::geometry::shape::Shape;
    use crate::tuples::ray::Ray;
    use crate::tuples::tuple::Tuple;
    use std::sync::Arc;

    #[test]
    fn given_a_plane_when_calculating_normal_should_be_constant_everywhere() {
        // Arrange
        let p = Plane::default();

        // Act
        let n1 = p.local_normal_at(Tuple::point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(Tuple::point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(Tuple::point(-5.0, 0.0, 150.0));

        // Assert
        assert_eq!(Tuple::vector(0.0, 1.0, 0.0), n1);
        assert_eq!(Tuple::vector(0.0, 1.0, 0.0), n2);
        assert_eq!(Tuple::vector(0.0, 1.0, 0.0), n3);
    }

    #[test]
    fn given_a_plane_when_intersecting_a_ray_parallel_to_the_plane_should_not_hit() {
        // Arrange
        let p: Arc<dyn Shape> = Arc::new(Plane::default());
        let ray = Ray::new(Tuple::point(0.0, 10.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let result = p.local_intersect(&ray);

        // Assert
        assert_eq!(true, result.is_empty());
    }

    #[test]
    fn given_a_plane_when_intersecting_a_ray_coplanar_to_the_plane_should_not_hit() {
        // Arrange
        let p: Arc<dyn Shape> = Arc::new(Plane::default());
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let result = p.local_intersect(&ray);

        // Assert
        assert_eq!(true, result.is_empty());
    }

    #[test]
    fn given_a_plane_when_intersecting_a_ray_from_above_the_plane_should_hit() {
        // Arrange
        let p: Arc<dyn Shape> = Arc::new(Plane::default());
        let ray = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));

        // Act
        let result = p.clone().local_intersect(&ray);

        // Assert
        assert_eq!(1, result.len());
        assert_eq!(1.0, result[0].time);
        assert_eq!(true, Arc::ptr_eq(&p, &result[0].object));
    }

    #[test]
    fn given_a_plane_when_intersecting_a_ray_from_below_the_plane_should_hit() {
        // Arrange
        let p: Arc<dyn Shape> = Arc::new(Plane::default());
        let ray = Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));

        // Act
        let result = p.clone().local_intersect(&ray);

        // Assert
        assert_eq!(1, result.len());
        assert_eq!(1.0, result[0].time);
        assert_eq!(true, Arc::ptr_eq(&p, &result[0].object));
    }
}
