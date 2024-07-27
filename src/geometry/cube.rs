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

pub struct Cube {
    id: Uuid,
    transform: Arc<Matrix>,
    material: Arc<dyn Material>,
    parent: RwLock<Weak<Group>>,
    casts_shadow: bool,
}

impl Cube {
    pub fn default() -> Cube {
        Cube {
            id: Uuid::new_v4(),
            transform: Arc::new(Matrix::identity(4)),
            material: Arc::new(Phong::default()),
            parent: RwLock::new(Weak::new()),
            casts_shadow: true,
        }
    }

    pub fn new(transform: Arc<Matrix>, material: Arc<dyn Material>, casts_shadow: bool) -> Cube {
        Cube {
            id: Uuid::new_v4(),
            transform,
            material,
            parent: RwLock::new(Weak::new()),
            casts_shadow,
        }
    }

    // Takes the ray-plane intersection formula and generalizes it to support planes that are offset
    // from the origin
    fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let tmin: f64;
        let tmax: f64;

        // If the denominator is effectively 0 we don't want to divide by it. So we multiply by INF
        // to make sure that tmin and tmax - while both being INF - have the correct sign
        if direction.abs() >= EPSILON {
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator / direction;
        } else {
            tmin = tmin_numerator * f64::INFINITY;
            tmax = tmax_numerator * f64::INFINITY;
        }

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }
}

impl Shape for Cube {
    fn id(&self) -> Uuid {
        self.id.clone()
    }

    fn local_intersect(self: Arc<Self>, local_ray: &Ray) -> Vec<Intersection> {
        let origin = local_ray.origin();
        let direction = local_ray.direction();

        // For each axes of the cube, check where ray intersects the corresponding plane
        let (xtmin, xtmax) = Cube::check_axis(origin.x, direction.x);
        let (ytmin, ytmax) = Cube::check_axis(origin.y, direction.y);
        let (ztmin, ztmax) = Cube::check_axis(origin.z, direction.z);

        // Return the largest minimum t value and the smallest maximum t value
        let tmin = f64::max(f64::max(xtmin, ytmin), ztmin);
        let tmax = f64::min(f64::min(xtmax, ytmax), ztmax);

        if tmin > tmax {
            vec![]
        } else {
            vec![
                Intersection::new(tmin, self.clone()),
                Intersection::new(tmax, self),
            ]
        }
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

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        let maxc = f64::max(
            f64::max(local_point.x.abs(), local_point.y.abs()),
            local_point.z.abs(),
        );

        if maxc == local_point.x.abs() {
            Tuple::vector(local_point.x, 0.0, 0.0)
        } else if maxc == local_point.y.abs() {
            Tuple::vector(0.0, local_point.y, 0.0)
        } else {
            Tuple::vector(0.0, 0.0, local_point.z)
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
    use crate::geometry::cube::Cube;
    use crate::geometry::shape::Shape;
    use crate::tuples::ray::Ray;
    use crate::tuples::tuple::Tuple;
    use std::sync::Arc;

    #[test]
    fn given_a_ray_when_intersecting_a_cube_should_identify_intersection_correctly_on_all_six_faces(
    ) {
        // Arrange
        let cube = Arc::new(Cube::default());

        let rays = vec![
            Ray::new(Tuple::point(5.0, 0.5, 0.0), Tuple::vector(-1.0, 0.0, 0.0)),
            Ray::new(Tuple::point(-5.0, 0.5, 0.0), Tuple::vector(1.0, 0.0, 0.0)),
            Ray::new(Tuple::point(0.5, 5.0, 0.0), Tuple::vector(0.0, -1.0, 0.0)),
            Ray::new(Tuple::point(0.5, -5.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)),
            Ray::new(Tuple::point(0.5, 0.0, 5.0), Tuple::vector(0.0, 0.0, -1.0)),
            Ray::new(Tuple::point(0.5, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)),
            Ray::new(Tuple::point(0.0, 0.5, 0.0), Tuple::vector(0.0, 0.0, 1.0)),
        ];

        let expected_intersects = vec![
            (4.0, 6.0),
            (4.0, 6.0),
            (4.0, 6.0),
            (4.0, 6.0),
            (4.0, 6.0),
            (4.0, 6.0),
            (-1.0, 1.0),
        ];

        // Act
        for i in 0..rays.len() {
            let intersects = cube.clone().local_intersect(&rays[i]);

            // Assert
            assert_eq!(2, intersects.len());
            assert_eq!(expected_intersects[i].0, intersects[0].time);
            assert_eq!(expected_intersects[i].1, intersects[1].time);
        }
    }

    #[test]
    fn given_a_ray_that_misses_when_intersecting_a_cube_should_result_in_zero_intersections() {
        // Arrange
        let cube = Arc::new(Cube::default());

        let rays = vec![
            Ray::new(
                Tuple::point(-2.0, 0.0, 0.0),
                Tuple::vector(0.2673, 0.5345, 0.8018),
            ),
            Ray::new(
                Tuple::point(0.0, -2.0, 0.0),
                Tuple::vector(0.8018, 0.2673, 0.5345),
            ),
            Ray::new(
                Tuple::point(0.0, 0.0, -2.0),
                Tuple::vector(0.5345, 0.8018, 0.2673),
            ),
            Ray::new(Tuple::point(2.0, 0.0, 2.0), Tuple::vector(0.0, 0.0, -1.0)),
            Ray::new(Tuple::point(0.0, 2.0, 2.0), Tuple::vector(0.0, -1.0, 0.0)),
            Ray::new(Tuple::point(2.0, 2.0, 0.0), Tuple::vector(-1.0, 0.0, 0.0)),
        ];

        // Act
        for i in 0..rays.len() {
            let intersects = cube.clone().local_intersect(&rays[i]);

            // Assert
            assert_eq!(0, intersects.len());
        }
    }

    #[test]
    fn given_a_default_cube_when_computing_normal_on_surface_should_return_correct_result() {
        // Arrange
        let cube = Arc::new(Cube::default());

        let expected_normals = vec![
            (Tuple::point(1.0, 0.5, -0.8), Tuple::vector(1.0, 0.0, 0.0)),
            (Tuple::point(-1.0, -0.2, 0.9), Tuple::vector(-1.0, 0.0, 0.0)),
            (Tuple::point(-0.4, 1.0, -0.1), Tuple::vector(0.0, 1.0, 0.0)),
            (Tuple::point(0.3, -1.0, -0.7), Tuple::vector(0.0, -1.0, 0.0)),
            (Tuple::point(-0.6, 0.3, 1.0), Tuple::vector(0.0, 0.0, 1.0)),
            (Tuple::point(0.4, 0.4, -1.0), Tuple::vector(0.0, 0.0, -1.0)),
            (Tuple::point(1.0, 1.0, 1.0), Tuple::vector(1.0, 0.0, 0.0)),
            (
                Tuple::point(-1.0, -1.0, -1.0),
                Tuple::vector(-1.0, 0.0, 0.0),
            ),
        ];

        // Act
        for i in 0..expected_normals.len() {
            let normal = cube.local_normal_at(expected_normals[i].0);

            // Assert
            assert_eq!(expected_normals[i].1, normal);
        }
    }
}
