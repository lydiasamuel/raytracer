use crate::geometry::group::Group;
use crate::geometry::shape::Shape;
use crate::materials::material::Material;
use crate::materials::phong::Phong;
use crate::matrices::matrix::Matrix;
use crate::tuples::color::Color;
use crate::tuples::intersection::Intersection;
use crate::tuples::pointlight::PointLight;
use crate::tuples::ray::Ray;
use crate::tuples::tuple::Tuple;
use crate::EPSILON;
use std::sync::{Arc, RwLock, Weak};
use uuid::Uuid;

pub struct Cone {
    id: Uuid,
    transform: Arc<Matrix>,
    material: Arc<dyn Material>,
    parent: RwLock<Weak<Group>>,
    casts_shadow: bool,
    minimum: f64,
    maximum: f64,
    closed: bool,
}

impl Cone {
    pub fn default() -> Cone {
        Cone {
            id: Uuid::new_v4(),
            transform: Arc::new(Matrix::identity(4)),
            material: Arc::new(Phong::default()),
            parent: RwLock::new(Weak::new()),
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
    ) -> Cone {
        Cone {
            id: Uuid::new_v4(),
            transform,
            material,
            parent: RwLock::new(Weak::new()),
            casts_shadow,
            minimum,
            maximum,
            closed,
        }
    }

    // Checks to see if the intersection at 't', is within a radius of y from the y-axis. This is
    // because a cone's radius at any given y is the absolute value of that y.
    fn check_cap(ray: &Ray, y: f64, t: f64) -> bool {
        let origin = ray.origin();
        let direction = ray.direction();

        let x = origin.x + t * direction.x;
        let z = origin.z + t * direction.z;

        ((x * x) + (z * z)) <= y
    }

    fn intersect_caps(self: Arc<Self>, ray: &Ray, intersections: &mut Vec<Intersection>) {
        let direction = ray.direction();

        // Caps only matter if the cone is closed, and might possibly be intersected by the ray
        if !self.closed || direction.y.abs() < EPSILON {
            return;
        }

        let origin = ray.origin();

        // Check for an intersection with the lower end cap by intersecting the ray with the plane
        // at y = cone.minimum
        let t0 = (self.minimum - origin.y) / direction.y;
        if Cone::check_cap(ray, self.minimum.abs(), t0) {
            intersections.push(Intersection::new(t0, self.clone()));
        }

        // Check for an intersection with the upper end cap by intersecting the ray with the plane
        // at y = cone.maximum
        let t1 = (self.maximum - origin.y) / direction.y;
        if Cone::check_cap(ray, self.maximum.abs(), t1) {
            intersections.push(Intersection::new(t1, self.clone()));
        }
    }
}

impl Shape for Cone {
    fn id(&self) -> Uuid {
        self.id
    }

    fn local_intersect(self: Arc<Self>, local_ray: &Ray) -> Vec<Intersection> {
        let origin = local_ray.origin();
        let direction = local_ray.direction();

        let mut intersections: Vec<Intersection> = vec![];

        let a =
            (direction.x * direction.x) - (direction.y * direction.y) + (direction.z * direction.z);
        let b = (2.0 * origin.x * direction.x) - (2.0 * origin.y * direction.y)
            + (2.0 * origin.z * direction.z);
        let c = (origin.x * origin.x) - (origin.y * origin.y) + (origin.z * origin.z);

        // if a is zero, it means the ray is parallel to one of the cone's halves
        if a.abs() < EPSILON {
            // if b is also zero, then it will miss
            if b.abs() < EPSILON {
                return vec![];
            }

            let t = -c / (2.0 * b);

            intersections.push(Intersection::new(t, self.clone()));

            Cone::intersect_caps(self, local_ray, &mut intersections);

            return intersections;
        }

        let discriminant = (b * b) - (4.0 * a * c);

        // Ray intersects the cylinders walls if the discriminant is >= 0
        if discriminant >= 0.0 {
            let sqrt_discriminant = discriminant.sqrt();

            let mut t0 = (-b - sqrt_discriminant) / (2.0 * a);
            let mut t1 = (-b + sqrt_discriminant) / (2.0 * a);

            if t0 > t1 {
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }

            // If the y coordinates are between the min and max values of the cone, then the
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

        Cone::intersect_caps(self, local_ray, &mut intersections);

        intersections
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
        // Compute the square distance from the y-axis
        let dist = local_point.x * local_point.x + local_point.z * local_point.z;

        if dist < 1.0 && local_point.y >= self.maximum - EPSILON {
            return Tuple::vector(0.0, 1.0, 0.0);
        }

        if dist < 1.0 && local_point.y <= self.minimum + EPSILON {
            return Tuple::vector(0.0, -1.0, 0.0);
        }

        let y = dist.sqrt();

        if local_point.y > 0.0 {
            Tuple::vector(local_point.x, -y, local_point.z)
        } else {
            Tuple::vector(local_point.x, y, local_point.z)
        }
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

#[cfg(test)]
mod tests {
    use crate::geometry::cone::Cone;
    use crate::geometry::shape::Shape;
    use crate::materials::phong::Phong;
    use crate::matrices::matrix::Matrix;
    use crate::tuples::ray::Ray;
    use crate::tuples::tuple::Tuple;
    use crate::EPSILON;
    use std::f64::consts::SQRT_2;
    use std::sync::Arc;

    #[test]
    fn given_a_ray_when_intersecting_a_cone_should_identify_intersections_correctly() {
        // Arrange
        let cone = Arc::new(Cone::default());

        let rays = vec![
            Ray::new(
                Tuple::point(0.0, 0.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0).normalize(),
            ),
            Ray::new(
                Tuple::point(0.0, 0.0, -5.0),
                Tuple::vector(1.0, 1.0, 1.0).normalize(),
            ),
            Ray::new(
                Tuple::point(1.0, 1.0, -5.0),
                Tuple::vector(-0.5, -1.0, 1.0).normalize(),
            ),
        ];

        let expected_intersects = vec![(5.0, 5.0), (8.66025, 8.66025), (4.55006, 49.44994)];

        // Act
        for i in 0..rays.len() {
            let intersects = cone.clone().local_intersect(&rays[i]);

            // Assert
            assert_eq!(2, intersects.len());
            assert!((expected_intersects[i].0 - intersects[0].time).abs() < EPSILON);
            assert!((expected_intersects[i].1 - intersects[1].time).abs() < EPSILON);
        }
    }

    #[test]
    fn given_a_cone_when_intersecting_a_ray_parallel_to_one_of_the_halves_should_identify_intersection_correctly(
    ) {
        // Arrange
        let cone = Arc::new(Cone::default());

        let ray = Ray::new(
            Tuple::point(0.0, 0.0, -1.0),
            Tuple::vector(0.0, 1.0, 1.0).normalize(),
        );

        // Act
        let intersects = cone.clone().local_intersect(&ray);

        // Assert
        assert_eq!(1, intersects.len());
        assert!((0.35355 - intersects[0].time).abs() < EPSILON);
    }

    #[test]
    fn given_a_ray_when_intersecting_a_constrained_capped_cone_should_identify_intersections_correctly(
    ) {
        // Arrange
        let cone = Arc::new(Cone::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::default()),
            true,
            -0.5,
            0.5,
            true,
        ));

        let rays = vec![
            Ray::new(
                Tuple::point(0.0, 0.0, -5.0),
                Tuple::vector(0.0, 1.0, 0.0).normalize(),
            ),
            Ray::new(
                Tuple::point(0.0, 0.0, -0.25),
                Tuple::vector(0.0, 1.0, 1.0).normalize(),
            ),
            Ray::new(
                Tuple::point(0.0, 0.0, -0.25),
                Tuple::vector(0.0, 1.0, 0.0).normalize(),
            ),
        ];

        let expected_intersect_counts = vec![0, 2, 4];

        // Act
        for i in 0..rays.len() {
            let intersects = cone.clone().local_intersect(&rays[i]);

            // Assert
            assert_eq!(expected_intersect_counts[i], intersects.len());
        }
    }

    #[test]
    fn given_a_cylinder_when_computing_normals_should_return_correct_result() {
        // Arrange
        let cone = Arc::new(Cone::default());

        let expected_normals = vec![
            (Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 0.0)),
            (
                Tuple::point(1.0, 1.0, 1.0),
                Tuple::vector(1.0, -SQRT_2, 1.0),
            ),
            (Tuple::point(-1.0, -1.0, 0.0), Tuple::vector(-1.0, 1.0, 0.0)),
        ];

        // Act
        for i in 0..expected_normals.len() {
            let normal = cone.local_normal_at(expected_normals[i].0);

            // Assert
            assert_eq!(expected_normals[i].1, normal);
        }
    }
}
