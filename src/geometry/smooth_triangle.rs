use std::sync::{Arc, RwLock, Weak};
use uuid::Uuid;

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

pub struct SmoothTriangle {
    id: Uuid,
    transform: Arc<Matrix>,
    material: Arc<dyn Material>,
    parent: RwLock<Weak<dyn Shape>>,
    casts_shadow: bool,
    p1: Tuple,
    p2: Tuple,
    p3: Tuple,
    e1: Tuple,
    e2: Tuple,
    n1: Tuple,
    n2: Tuple,
    n3: Tuple,
}

impl SmoothTriangle {
    pub fn new(
        p1: Tuple,
        p2: Tuple,
        p3: Tuple,
        n1: Tuple,
        n2: Tuple,
        n3: Tuple,
        transform: Arc<Matrix>,
        material: Arc<dyn Material>,
        casts_shadow: bool,
    ) -> SmoothTriangle {
        assert!(p1.is_point());
        assert!(p2.is_point());
        assert!(p3.is_point());
        assert!(n1.is_vector());
        assert!(n2.is_vector());
        assert!(n3.is_vector());

        let e1 = p2 - p1;
        let e2 = p3 - p1;

        SmoothTriangle {
            id: Uuid::new_v4(),
            transform,
            material,
            parent: RwLock::new(Weak::<Group>::new()),
            casts_shadow,
            p1,
            p2,
            p3,
            e1,
            e2,
            n1,
            n2,
            n3,
        }
    }

    pub fn default(
        p1: Tuple,
        p2: Tuple,
        p3: Tuple,
        n1: Tuple,
        n2: Tuple,
        n3: Tuple,
    ) -> SmoothTriangle {
        Self::new(
            p1,
            p2,
            p3,
            n1,
            n2,
            n3,
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::default()),
            true,
        )
    }
}

impl Shape for SmoothTriangle {
    fn id(&self) -> Uuid {
        self.id
    }

    // Implementation of the Möller–Trumbore intersection algorithm which preserves u & v in the
    // intersection
    fn local_intersect(self: Arc<Self>, local_ray: &Ray) -> Vec<Intersection> {
        let dir_cross_e2 = Tuple::cross(local_ray.direction(), self.e2);
        let det = Tuple::dot(self.e1, dir_cross_e2);

        // If the determinant is close to zero, then the ray is parallel to the triangle and misses
        if det.abs() < EPSILON {
            return vec![];
        }

        let f = 1.0 / det;

        let p1_to_origin = local_ray.origin() - self.p1;
        let u = f * Tuple::dot(p1_to_origin, dir_cross_e2);

        if u < 0.0 || u > 1.0 {
            return vec![];
        }

        let origin_cross_e1 = Tuple::cross(p1_to_origin, self.e1);
        let v = f * Tuple::dot(local_ray.direction(), origin_cross_e1);

        if v < 0.0 || (u + v) > 1.0 {
            return vec![];
        }

        let t = f * Tuple::dot(self.e2, origin_cross_e1);

        vec![Intersection::new_with_uv(t, self, u, v)]
    }

    fn get_transform(&self) -> Arc<Matrix> {
        self.transform.clone()
    }

    fn get_material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }

    fn get_parent(&self) -> Option<Arc<dyn Shape>> {
        self.parent.read().unwrap().upgrade()
    }

    fn set_parent(&self, parent: &Arc<dyn Shape>) {
        *self.parent.write().unwrap() = Arc::downgrade(parent);
    }

    fn casts_shadow(&self) -> bool {
        self.casts_shadow
    }

    fn local_normal_at(&self, _: Tuple, hit: &Intersection) -> Tuple {
        (self.n2 * hit.u()) + (self.n3 * hit.v()) + (self.n1 * (1.0 - hit.u() - hit.v()))
    }

    fn bounds(&self) -> BoundingBox {
        let result = BoundingBox::empty();

        result
            .add_point(self.p1)
            .add_point(self.p2)
            .add_point(self.p3)
    }

    fn points(&self) -> (Tuple, Tuple, Tuple) {
        (self.p1, self.p2, self.p3)
    }

    fn normals(&self) -> (Tuple, Tuple, Tuple) {
        (self.n1, self.n2, self.n3)
    }

    fn edge_vectors(&self) -> (Tuple, Tuple) {
        (self.e1, self.e2)
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
    use crate::geometry::shape::Shape;
    use crate::geometry::smooth_triangle::SmoothTriangle;
    use crate::tuples::intersection::Intersection;
    use crate::tuples::ray::Ray;
    use crate::tuples::tuple::Tuple;
    use crate::EPSILON;
    use std::sync::Arc;

    #[test]
    fn given_a_smooth_triangle_when_calculating_the_intersection_should_store_the_u_and_v() {
        // Arrange
        let triangle = Arc::new(SmoothTriangle::default(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
            Tuple::vector(-1.0, 0.0, 0.0),
            Tuple::vector(1.0, 0.0, 0.0),
        ));

        let ray = Ray::new(Tuple::point(-0.2, 0.3, -2.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let intersections = triangle.local_intersect(&ray);

        // Assert
        assert_eq!(true, (0.45 - intersections[0].u()).abs() < EPSILON);
        assert_eq!(true, (0.25 - intersections[0].v()).abs() < EPSILON);
    }

    #[test]
    fn given_a_smooth_triangle_when_finding_normal_should_interpolate_the_result() {
        // Arrange
        let triangle = Arc::new(SmoothTriangle::default(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
            Tuple::vector(-1.0, 0.0, 0.0),
            Tuple::vector(1.0, 0.0, 0.0),
        ));

        let intersection = Intersection::new_with_uv(1.0, triangle.clone(), 0.45, 0.25);

        // Act
        let normal = triangle.normal_at(Tuple::origin(), &intersection);

        // Assert
        assert_eq!(Tuple::vector(-0.5547, 0.83205, 0.0), normal);
    }
}
