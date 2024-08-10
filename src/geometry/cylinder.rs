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

pub struct Cylinder {
    id: Uuid,
    transform: Arc<Matrix>,
    material: Arc<dyn Material>,
    parent: RwLock<Weak<dyn Shape>>,
    casts_shadow: bool,
    minimum: f64,
    maximum: f64,
    closed: bool,
}

impl Cylinder {
    pub fn default() -> Cylinder {
        Cylinder {
            id: Uuid::new_v4(),
            transform: Arc::new(Matrix::identity(4)),
            material: Arc::new(Phong::default()),
            parent: RwLock::new(Weak::<Group>::new()),
            casts_shadow: true,
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
            closed: false,
        }
    }

    pub fn new(
        transform: Arc<Matrix>,
        material: Arc<dyn Material>,
        casts_shadow: bool,
        minimum: f64,
        maximum: f64,
        closed: bool,
    ) -> Cylinder {
        Cylinder {
            id: Uuid::new_v4(),
            transform,
            material,
            parent: RwLock::new(Weak::<Group>::new()),
            casts_shadow,
            minimum,
            maximum,
            closed,
        }
    }

    // Checks to see if the intersection at 't', is within a radius of 1 from the y-axis.
    fn check_cap(ray: &Ray, t: f64) -> bool {
        let origin = ray.origin();
        let direction = ray.direction();

        let x = origin.x + t * direction.x;
        let z = origin.z + t * direction.z;

        ((x * x) + (z * z)) <= 1.0
    }

    fn intersect_caps(self: Arc<Self>, ray: &Ray, intersections: &mut Vec<Intersection>) {
        let direction = ray.direction();

        // Caps only matter if the cylinder is closed, and might possibly be intersected by the ray
        if !self.closed || direction.y.abs() < EPSILON {
            return;
        }

        let origin = ray.origin();

        // Check for an intersection with the lower end cap by intersecting the ray with the plane
        // at y = cyl.minimum
        let t0 = (self.minimum - origin.y) / direction.y;
        if Cylinder::check_cap(ray, t0) {
            intersections.push(Intersection::new(t0, self.clone()));
        }

        // Check for an intersection with the upper end cap by intersecting the ray with the plane
        // at y = cyl.maximum
        let t1 = (self.maximum - origin.y) / direction.y;
        if Cylinder::check_cap(ray, t1) {
            intersections.push(Intersection::new(t1, self.clone()));
        }
    }
}

impl Shape for Cylinder {
    fn id(&self) -> Uuid {
        self.id
    }

    fn local_intersect(self: Arc<Self>, local_ray: &Ray) -> Vec<Intersection> {
        let origin = local_ray.origin();
        let direction = local_ray.direction();

        let mut intersections: Vec<Intersection> = vec![];

        let a = direction.x.powi(2) + direction.z.powi(2);

        // Ray is parallel to the y-axis, so it will not hit the cylinder walls. Though it might
        // possibly intersect the end caps.
        if a < EPSILON {
            Cylinder::intersect_caps(self.clone(), local_ray, &mut intersections);

            return intersections;
        }

        let b = (2.0 * origin.x * direction.x) + (2.0 * origin.z * direction.z);
        let c = (origin.x * origin.x) + (origin.z * origin.z) - 1.0;

        let discriminant = (b * b) - (4.0 * a * c);

        // Ray intersects the cylinder walls if the discriminant is >= 0
        if discriminant >= 0.0 {
            let sqrt_discriminant = discriminant.sqrt();

            let mut t0 = (-b - sqrt_discriminant) / (2.0 * a);
            let mut t1 = (-b + sqrt_discriminant) / (2.0 * a);

            if t0 > t1 {
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }

            // If the y coordinates are between the min and max values of the cylinder, then the
            // intersection is valid
            let y0 = origin.y + t0 * direction.y;
            if self.minimum < y0 && y0 < self.maximum {
                intersections.push(Intersection::new(t0, self.clone()));
            }

            let y1 = origin.y + t1 * direction.y;
            if self.minimum < y1 && y1 < self.maximum {
                intersections.push(Intersection::new(t1, self.clone()));
            }
        }

        Cylinder::intersect_caps(self, local_ray, &mut intersections);

        intersections
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

    fn local_normal_at(&self, local_point: Tuple, _: &Intersection) -> Tuple {
        // Compute the square distance from the y-axis
        let dist = local_point.x * local_point.x + local_point.z * local_point.z;

        if dist < 1.0 && local_point.y >= self.maximum - EPSILON {
            return Tuple::vector(0.0, 1.0, 0.0);
        }

        if dist < 1.0 && local_point.y <= self.minimum + EPSILON {
            return Tuple::vector(0.0, -1.0, 0.0);
        }

        Tuple::vector(local_point.x, 0.0, local_point.z)
    }

    fn bounds(&self) -> BoundingBox {
        BoundingBox::new(
            Tuple::point(-1.0, self.minimum, -1.0),
            Tuple::point(1.0, self.maximum, 1.0),
        )
    }

    fn points(&self) -> (Tuple, Tuple, Tuple) {
        panic!("Error: points function is not implemented for this shape")
    }

    fn normals(&self) -> (Tuple, Tuple, Tuple) {
        panic!("Error: normals function is not implemented for this shape")
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
    use crate::geometry::cylinder::Cylinder;
    use crate::geometry::shape::Shape;
    use crate::materials::phong::Phong;
    use crate::matrices::matrix::Matrix;
    use crate::tuples::intersection::Intersection;
    use crate::tuples::ray::Ray;
    use crate::tuples::tuple::Tuple;
    use crate::EPSILON;
    use std::sync::Arc;

    #[test]
    fn given_a_ray_when_intersecting_a_cylinder_should_identify_intersections_correctly() {
        // Arrange
        let cylinder = Arc::new(Cylinder::default());

        let rays = vec![
            Ray::new(
                Tuple::point(1.0, 0.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0).normalize(),
            ),
            Ray::new(
                Tuple::point(0.0, 0.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0).normalize(),
            ),
            Ray::new(
                Tuple::point(0.5, 0.0, -5.0),
                Tuple::vector(0.1, 1.0, 1.0).normalize(),
            ),
        ];

        let expected_intersects = vec![(5.0, 5.0), (4.0, 6.0), (6.80798, 7.08872)];

        // Act
        for i in 0..rays.len() {
            let intersects = cylinder.clone().local_intersect(&rays[i]);

            // Assert
            assert_eq!(2, intersects.len());
            assert!((expected_intersects[i].0 - intersects[0].time()).abs() < EPSILON);
            assert!((expected_intersects[i].1 - intersects[1].time()).abs() < EPSILON);
        }
    }

    #[test]
    fn given_a_ray_when_intersecting_a_constrained_cylinder_should_identify_intersections_correctly(
    ) {
        // Arrange
        let cylinder = Arc::new(Cylinder::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::default()),
            true,
            1.0,
            2.0,
            false,
        ));

        let rays = vec![
            Ray::new(
                Tuple::point(0.0, 1.5, 0.0),
                Tuple::vector(0.1, 1.0, 0.0).normalize(),
            ),
            Ray::new(
                Tuple::point(0.0, 3.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0).normalize(),
            ),
            Ray::new(
                Tuple::point(0.0, 0.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0).normalize(),
            ),
            Ray::new(
                Tuple::point(0.0, 2.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0).normalize(),
            ),
            Ray::new(
                Tuple::point(0.0, 1.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0).normalize(),
            ),
        ];

        let expected_intersect_counts = vec![0, 0, 0, 0, 0, 2];

        // Act
        for i in 0..rays.len() {
            let intersects = cylinder.clone().local_intersect(&rays[i]);

            // Assert
            assert_eq!(expected_intersect_counts[i], intersects.len());
        }
    }

    #[test]
    fn given_a_ray_that_misses_when_intersecting_cylinder_should_not_return_any_hits() {
        // Arrange
        let cylinder = Arc::new(Cylinder::default());

        let rays = vec![
            Ray::new(Tuple::point(1.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)),
            Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)),
            Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(1.0, 1.0, 1.0)),
        ];

        // Act
        for i in 0..rays.len() {
            let intersects = cylinder.clone().local_intersect(&rays[i]);

            // Assert
            assert_eq!(0, intersects.len());
        }
    }

    #[test]
    fn given_a_default_cylinder_when_computing_normal_on_surface_should_return_correct_result() {
        // Arrange
        let cylinder = Arc::new(Cylinder::default());

        let hit = Intersection::new(1.0, cylinder.clone());

        let expected_normals = vec![
            (Tuple::point(1.0, 0.0, 0.0), Tuple::vector(1.0, 0.0, 0.0)),
            (Tuple::point(0.0, 5.0, -1.0), Tuple::vector(0.0, 0.0, -1.0)),
            (Tuple::point(0.0, -2.0, 1.0), Tuple::vector(0.0, 0.0, 1.0)),
            (Tuple::point(-1.0, 1.0, 0.0), Tuple::vector(-1.0, 0.0, 0.0)),
        ];

        // Act
        for i in 0..expected_normals.len() {
            let normal = cylinder.local_normal_at(expected_normals[i].0, &hit);

            // Assert
            assert_eq!(expected_normals[i].1, normal);
        }
    }

    #[test]
    fn given_a_ray_when_intersecting_a_constrained_capped_cylinder_should_identify_intersections_correctly(
    ) {
        // Arrange
        let cylinder = Arc::new(Cylinder::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::default()),
            true,
            1.0,
            2.0,
            true,
        ));

        let rays = vec![
            Ray::new(
                Tuple::point(0.0, 3.0, 0.0),
                Tuple::vector(0.0, -1.0, 0.0).normalize(),
            ),
            Ray::new(
                Tuple::point(0.0, 3.0, -2.0),
                Tuple::vector(0.0, -1.0, 2.0).normalize(),
            ),
            Ray::new(
                Tuple::point(0.0, 4.0, -2.0),
                Tuple::vector(0.0, -1.0, 1.0).normalize(),
            ),
            Ray::new(
                Tuple::point(0.0, 0.0, -2.0),
                Tuple::vector(0.0, 1.0, 2.0).normalize(),
            ),
            Ray::new(
                Tuple::point(0.0, -1.0, -2.0),
                Tuple::vector(0.0, 1.0, 1.0).normalize(),
            ),
        ];

        let expected_intersect_counts = vec![2, 2, 2, 2, 2];

        // Act
        for i in 0..rays.len() {
            let intersects = cylinder.clone().local_intersect(&rays[i]);

            // Assert
            assert_eq!(expected_intersect_counts[i], intersects.len());
        }
    }

    #[test]
    fn given_a_cylinder_when_computing_normal_on_the_end_caps_should_return_correct_result() {
        // Arrange
        let cylinder = Arc::new(Cylinder::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::default()),
            true,
            1.0,
            2.0,
            true,
        ));

        let hit = Intersection::new(1.0, cylinder.clone());

        let expected_normals = vec![
            (Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0)),
            (Tuple::point(0.5, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0)),
            (Tuple::point(0.0, 1.0, 0.5), Tuple::vector(0.0, -1.0, 0.0)),
            (Tuple::point(0.0, 2.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)),
            (Tuple::point(0.5, 2.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)),
            (Tuple::point(0.0, 2.0, 0.5), Tuple::vector(0.0, 1.0, 0.0)),
        ];

        // Act
        for i in 0..expected_normals.len() {
            let normal = cylinder.local_normal_at(expected_normals[i].0, &hit);

            // Assert
            assert_eq!(expected_normals[i].1, normal);
        }
    }
}
