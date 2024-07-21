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
use std::sync::Arc;
use uuid::Uuid;

pub struct Cylinder {
    id: Uuid,
    transform: Arc<Matrix>,
    material: Arc<dyn Material>,
    casts_shadow: bool,
    minimum: f64,
    maximum: f64,
}

impl Cylinder {
    pub fn default() -> Cylinder {
        Cylinder {
            id: Uuid::new_v4(),
            transform: Arc::new(Matrix::identity(4)),
            material: Arc::new(Phong::default()),
            casts_shadow: true,
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
        }
    }

    pub fn new(
        transform: Arc<Matrix>,
        material: Arc<dyn Material>,
        casts_shadow: bool,
        minimum: f64,
        maximum: f64,
    ) -> Cylinder {
        Cylinder {
            id: Uuid::new_v4(),
            transform,
            material,
            casts_shadow,
            minimum,
            maximum,
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

        let a = direction.x.powi(2) + direction.z.powi(2);

        // Ray is parallel to the y-axis, so it will not hit the cylinder
        if a < EPSILON {
            return vec![];
        }

        let b = (2.0 * origin.x * direction.x) + (2.0 * origin.z * direction.z);
        let c = (origin.x * origin.x) + (origin.z * origin.z) - 1.0;

        let discriminant = (b * b) - (4.0 * a * c);

        // Ray does not intersect the cylinder
        if discriminant < 0.0 {
            vec![]
        } else {
            let sqrt_discriminant = discriminant.sqrt();

            let mut t0 = (-b - sqrt_discriminant) / (2.0 * a);
            let mut t1 = (-b + sqrt_discriminant) / (2.0 * a);

            if t0 > t1 {
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }

            let mut intersections: Vec<Intersection> = vec![];

            // If the y coordinates are between the min and max values of the cylinder, then the
            // intersection is valid
            let y0 = origin.y + t0 * direction.y;
            if self.minimum < y0 && y0 < self.maximum {
                intersections.push(Intersection::new(t0, self.clone()));
            }

            let y1 = origin.y + t1 * direction.y;
            if self.minimum < y1 && y1 < self.maximum {
                intersections.push(Intersection::new(t1, self));
            }

            intersections
        }
    }

    fn get_transform(&self) -> Arc<Matrix> {
        self.transform.clone()
    }

    fn get_material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }

    fn casts_shadow(&self) -> bool {
        self.casts_shadow
    }

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        Tuple::vector(local_point.x, 0.0, local_point.z)
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
    use crate::geometry::cylinder::Cylinder;
    use crate::geometry::shape::Shape;
    use crate::materials::phong::Phong;
    use crate::matrices::matrix::Matrix;
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
            assert!((expected_intersects[i].0 - intersects[0].time).abs() < EPSILON);
            assert!((expected_intersects[i].1 - intersects[1].time).abs() < EPSILON);
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

        let expected_normals = vec![
            (Tuple::point(1.0, 0.0, 0.0), Tuple::vector(1.0, 0.0, 0.0)),
            (Tuple::point(0.0, 5.0, -1.0), Tuple::vector(0.0, 0.0, -1.0)),
            (Tuple::point(0.0, -2.0, 1.0), Tuple::vector(0.0, 0.0, 1.0)),
            (Tuple::point(-1.0, 1.0, 0.0), Tuple::vector(-1.0, 0.0, 0.0)),
        ];

        // Act
        for i in 0..expected_normals.len() {
            let normal = cylinder.local_normal_at(expected_normals[i].0);

            // Assert
            assert_eq!(expected_normals[i].1, normal);
        }
    }
}
