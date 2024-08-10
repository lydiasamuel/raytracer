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

pub struct Triangle {
    id: Uuid,
    transform: Arc<Matrix>,
    material: Arc<dyn Material>,
    parent: RwLock<Weak<dyn Shape>>,
    casts_shadow: bool,
    // Three corners that make up the triangle in object space, transforming unit triangles is hard
    p1: Tuple,
    p2: Tuple,
    p3: Tuple,
    // Used to determine if and where the ray intersects the triangle
    e1: Tuple,
    e2: Tuple,
    // Used as the normal at every point of intersection
    normal: Tuple,
}

impl Triangle {
    pub fn new(
        p1: Tuple,
        p2: Tuple,
        p3: Tuple,
        transform: Arc<Matrix>,
        material: Arc<dyn Material>,
        casts_shadow: bool,
    ) -> Triangle {
        assert!(p1.is_point());
        assert!(p2.is_point());
        assert!(p3.is_point());

        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = Tuple::cross(e2, e1).normalize();

        Triangle {
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
            normal,
        }
    }

    pub fn default(p1: Tuple, p2: Tuple, p3: Tuple) -> Triangle {
        Self::new(
            p1,
            p2,
            p3,
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::default()),
            true,
        )
    }
}

impl Shape for Triangle {
    fn id(&self) -> Uuid {
        self.id
    }

    // Implementation of the Möller–Trumbore intersection algorithm
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

        vec![Intersection::new(t, self)]
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

    fn local_normal_at(&self, _: Tuple, _: &Intersection) -> Tuple {
        self.normal
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
        panic!("Error: normals function is not implemented for this shape")
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
    use crate::geometry::triangle::Triangle;
    use crate::tuples::intersection::Intersection;
    use crate::tuples::ray::Ray;
    use crate::tuples::tuple::Tuple;
    use std::sync::Arc;

    #[test]
    fn given_basic_values_when_constructing_a_triangle_should_initialise_edge_vectors_and_normal_correctly(
    ) {
        // Arrange
        let p1 = Tuple::point(0.0, 1.0, 0.0);
        let p2 = Tuple::point(-1.0, 0.0, 0.0);
        let p3 = Tuple::point(1.0, 0.0, 0.0);

        // Act
        let triangle = Triangle::default(p1, p2, p3);

        // Assert
        assert_eq!(p1, triangle.p1);
        assert_eq!(p2, triangle.p2);
        assert_eq!(p3, triangle.p3);

        assert_eq!(Tuple::vector(-1.0, -1.0, 0.0), triangle.e1);
        assert_eq!(Tuple::vector(1.0, -1.0, 0.0), triangle.e2);

        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), triangle.normal);
    }

    #[test]
    fn given_any_point_when_finding_the_local_normal_should_return_the_precomputed_value() {
        // Arrange
        let triangle = Arc::new(Triangle::default(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
        ));

        let hit = Intersection::new(1.0, triangle.clone());

        // Act
        let n1 = triangle.local_normal_at(Tuple::point(0.0, 0.5, 0.0), &hit);
        let n2 = triangle.local_normal_at(Tuple::point(-0.5, 0.75, 0.0), &hit);
        let n3 = triangle.local_normal_at(Tuple::point(0.5, 0.25, 0.0), &hit);

        // Assert
        assert_eq!(n1, triangle.normal);
        assert_eq!(n2, triangle.normal);
        assert_eq!(n3, triangle.normal);
    }

    #[test]
    fn given_a_ray_parallel_to_the_triangle_when_calculating_intersections_should_be_none() {
        // Arrange
        let triangle = Arc::new(Triangle::default(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
        ));

        let ray = Ray::new(Tuple::point(0.0, -1.0, -2.0), Tuple::vector(0.0, 1.0, 0.0));

        // Act
        let intersects = triangle.local_intersect(&ray);

        // Assert
        assert_eq!(true, intersects.is_empty());
    }

    #[test]
    fn given_a_ray_that_misses_the_p1_p3_edge_when_calculating_intersections_should_be_none() {
        // Arrange
        let triangle = Arc::new(Triangle::default(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
        ));

        let ray = Ray::new(Tuple::point(1.0, 1.0, -2.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let intersects = triangle.local_intersect(&ray);

        // Assert
        assert_eq!(true, intersects.is_empty());
    }

    #[test]
    fn given_a_ray_that_misses_the_p1_p2_edge_when_calculating_intersections_should_be_none() {
        // Arrange
        let triangle = Arc::new(Triangle::default(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
        ));

        let ray = Ray::new(Tuple::point(-1.0, 1.0, -2.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let intersects = triangle.local_intersect(&ray);

        // Assert
        assert_eq!(true, intersects.is_empty());
    }

    #[test]
    fn given_a_ray_that_misses_the_p2_p3_edge_when_calculating_intersections_should_be_none() {
        // Arrange
        let triangle = Arc::new(Triangle::default(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
        ));

        let ray = Ray::new(Tuple::point(0.0, -1.0, -2.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let intersects = triangle.local_intersect(&ray);

        // Assert
        assert_eq!(true, intersects.is_empty());
    }

    #[test]
    fn given_a_ray_that_hits_when_calculating_intersections_should_return_correct_intersections() {
        // Arrange
        let triangle = Arc::new(Triangle::default(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
        ));

        let ray = Ray::new(Tuple::point(0.0, 0.5, -2.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let intersects = triangle.local_intersect(&ray);

        // Assert
        assert_eq!(1, intersects.len());
        assert_eq!(2.0, intersects[0].time());
    }
}
