use crate::matrices::matrix::Matrix;
use crate::tuples::ray::Ray;
use crate::tuples::tuple::Tuple;
use crate::EPSILON;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    min: Tuple,
    max: Tuple,
}

impl BoundingBox {
    pub fn new(min: Tuple, max: Tuple) -> BoundingBox {
        BoundingBox { min, max }
    }

    pub fn empty() -> BoundingBox {
        BoundingBox {
            min: Tuple::point(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            max: Tuple::point(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }

    pub fn add_point(mut self, point: Tuple) -> Self {
        self.min = Tuple::point(
            f64::min(self.min.x, point.x),
            f64::min(self.min.y, point.y),
            f64::min(self.min.z, point.z),
        );

        self.max = Tuple::point(
            f64::max(self.max.x, point.x),
            f64::max(self.max.y, point.y),
            f64::max(self.max.z, point.z),
        );

        self
    }

    pub fn contains_point(&self, point: Tuple) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }

    pub fn contains_box(&self, other: BoundingBox) -> bool {
        other.min.x >= self.min.x
            && other.max.x <= self.max.x
            && other.min.y >= self.min.y
            && other.max.y <= self.max.y
            && other.min.z >= self.min.z
            && other.max.z <= self.max.z
    }

    pub fn transform(self, transform: &Matrix) -> Self {
        let p1 = self.min;
        let p2 = Tuple::point(self.min.x, self.min.y, self.max.z);
        let p3 = Tuple::point(self.min.x, self.max.y, self.min.z);
        let p4 = Tuple::point(self.min.x, self.max.y, self.max.z);
        let p5 = Tuple::point(self.max.x, self.min.y, self.min.z);
        let p6 = Tuple::point(self.max.x, self.min.y, self.max.z);
        let p7 = Tuple::point(self.max.x, self.max.y, self.min.z);
        let p8 = self.max;

        Self::empty()
            .add_point((transform * &p1).unwrap())
            .add_point((transform * &p2).unwrap())
            .add_point((transform * &p3).unwrap())
            .add_point((transform * &p4).unwrap())
            .add_point((transform * &p5).unwrap())
            .add_point((transform * &p6).unwrap())
            .add_point((transform * &p7).unwrap())
            .add_point((transform * &p8).unwrap())
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        let origin = ray.origin();
        let direction = ray.direction();

        // For each axes of the cube, check where ray intersects the corresponding plane
        let (xtmin, xtmax) = BoundingBox::check_axis(origin.x, direction.x, self.min.x, self.max.x);
        let (ytmin, ytmax) = BoundingBox::check_axis(origin.y, direction.y, self.min.y, self.max.y);
        let (ztmin, ztmax) = BoundingBox::check_axis(origin.z, direction.z, self.min.z, self.max.z);

        // Return the largest minimum t value and the smallest maximum t value
        let tmin = f64::max(f64::max(xtmin, ytmin), ztmin);
        let tmax = f64::min(f64::min(xtmax, ytmax), ztmax);

        if tmin > tmax {
            false
        } else {
            true
        }
    }

    // Takes the ray-plane intersection formula and generalizes it to support planes that are offset
    // from the origin
    fn check_axis(origin: f64, direction: f64, min: f64, max: f64) -> (f64, f64) {
        let tmin_numerator = min - origin;
        let tmax_numerator = max - origin;

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

    pub fn min(&self) -> Tuple {
        self.min
    }

    pub fn max(&self) -> Tuple {
        self.max
    }
}

impl Add for BoundingBox {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.add_point(rhs.min).add_point(rhs.max)
    }
}

#[cfg(test)]
mod tests {
    use crate::matrices::matrix::Matrix;
    use crate::tuples::bounding_box::BoundingBox;
    use crate::tuples::ray::Ray;
    use crate::tuples::tuple::Tuple;
    use std::f64::consts::PI;

    #[test]
    fn given_an_empty_box_when_adding_points_should_correctly_resize_box() {
        // Arrange
        let mut bounding_box = BoundingBox::empty();
        let p1 = Tuple::point(-5.0, 2.0, 0.0);
        let p2 = Tuple::point(7.0, 0.0, -3.0);

        // Act
        bounding_box = bounding_box.add_point(p1);
        bounding_box = bounding_box.add_point(p2);

        // Assert
        assert_eq!(Tuple::point(-5.0, 0.0, -3.0), bounding_box.min);
        assert_eq!(Tuple::point(7.0, 2.0, 0.0), bounding_box.max);
    }

    #[test]
    fn given_two_boxes_when_adding_them_together_should_correctly_size_the_new_box() {
        // Arrange
        let bounding_box_1 =
            BoundingBox::new(Tuple::point(-5.0, -2.0, 0.0), Tuple::point(7.0, 4.0, 4.0));
        let bounding_box_2 =
            BoundingBox::new(Tuple::point(8.0, -7.0, -2.0), Tuple::point(14.0, 2.0, 8.0));

        // Act
        let result = bounding_box_1 + bounding_box_2;

        // Assert
        assert_eq!(Tuple::point(-5.0, -7.0, -2.0), result.min);
        assert_eq!(Tuple::point(14.0, 4.0, 8.0), result.max);
    }

    #[test]
    fn given_a_box_when_checking_to_see_if_it_contains_a_point_should_correctly_identify_result() {
        // Arrange
        let bounding_box =
            BoundingBox::new(Tuple::point(5.0, -2.0, 0.0), Tuple::point(11.0, 4.0, 7.0));

        let points = vec![
            (Tuple::point(5.0, -2.0, 0.0), true),
            (Tuple::point(11.0, 4.0, 7.0), true),
            (Tuple::point(8.0, 1.0, 3.0), true),
            (Tuple::point(3.0, 0.0, 3.0), false),
            (Tuple::point(8.0, -4.0, 3.0), false),
            (Tuple::point(8.0, 1.0, -1.0), false),
            (Tuple::point(13.0, 1.0, 3.0), false),
            (Tuple::point(8.0, 5.0, 3.0), false),
            (Tuple::point(8.0, 1.0, 8.0), false),
        ];

        // Act
        for (point, expected) in points {
            let result = bounding_box.contains_point(point);
            // Assert
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn given_two_boxes_when_checking_to_see_if_first_contains_the_second_should_correctly_identify_result(
    ) {
        // Arrange
        let bounding_box =
            BoundingBox::new(Tuple::point(5.0, -2.0, 0.0), Tuple::point(11.0, 4.0, 7.0));

        let boxes = vec![
            (
                BoundingBox::new(Tuple::point(5.0, -2.0, 0.0), Tuple::point(11.0, 4.0, 7.0)),
                true,
            ),
            (
                BoundingBox::new(Tuple::point(6.0, -1.0, 1.0), Tuple::point(10.0, 3.0, 6.0)),
                true,
            ),
            (
                BoundingBox::new(Tuple::point(4.0, -3.0, -1.0), Tuple::point(10.0, 3.0, 6.0)),
                false,
            ),
            (
                BoundingBox::new(Tuple::point(6.0, -1.0, 1.0), Tuple::point(12.0, 5.0, 8.0)),
                false,
            ),
        ];

        // Act
        for (b, expected) in boxes {
            let result = bounding_box.contains_box(b);
            // Assert
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn given_a_box_when_transforming_it_should_apply_correctly_to_the_box() {
        // Arrange
        let bounding_box =
            BoundingBox::new(Tuple::point(-1.0, -1.0, -1.0), Tuple::point(1.0, 1.0, 1.0));

        let transform = &Matrix::rotation_x(PI / 4.0) * &Matrix::rotation_y(PI / 4.0);

        // Act
        let result = bounding_box.transform(&transform.unwrap());

        // Assert
        assert_eq!(Tuple::point(-1.41421, -1.7071, -1.7071), result.min);
        assert_eq!(Tuple::point(1.41421, 1.7071, 1.7071), result.max);
    }

    #[test]
    fn given_an_aabb_box_at_the_origin_when_intersecting_with_a_ray_should_identify_presence_of_a_hit_correctly(
    ) {
        // Arrange
        let bounding_box =
            BoundingBox::new(Tuple::point(-1.0, -1.0, -1.0), Tuple::point(1.0, 1.0, 1.0));

        let examples = vec![
            (
                Tuple::point(5.0, 0.5, 0.0),
                Tuple::vector(-1.0, 0.0, 0.0),
                true,
            ),
            (
                Tuple::point(-5.0, 0.5, 0.0),
                Tuple::vector(1.0, 0.0, 0.0),
                true,
            ),
            (
                Tuple::point(0.5, 5.0, 0.0),
                Tuple::vector(0.0, -1.0, 0.0),
                true,
            ),
            (
                Tuple::point(0.5, -5.0, 0.0),
                Tuple::vector(0.0, 1.0, 0.0),
                true,
            ),
            (
                Tuple::point(0.5, 0.0, 5.0),
                Tuple::vector(0.0, 0.0, -1.0),
                true,
            ),
            (
                Tuple::point(0.5, 0.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0),
                true,
            ),
            (
                Tuple::point(0.0, 0.5, 0.0),
                Tuple::vector(0.0, 0.0, 1.0),
                true,
            ),
            (
                Tuple::point(-2.0, 0.0, 0.0),
                Tuple::vector(2.0, 4.0, 6.0),
                false,
            ),
            (
                Tuple::point(0.0, -2.0, 0.0),
                Tuple::vector(6.0, 2.0, 4.0),
                false,
            ),
            (
                Tuple::point(0.0, 0.0, -2.0),
                Tuple::vector(4.0, 6.0, 2.0),
                false,
            ),
            (
                Tuple::point(2.0, 0.0, 2.0),
                Tuple::vector(0.0, 0.0, -1.0),
                false,
            ),
            (
                Tuple::point(0.0, 2.0, 2.0),
                Tuple::vector(0.0, -1.0, 0.0),
                false,
            ),
            (
                Tuple::point(2.0, 2.0, 0.0),
                Tuple::vector(-1.0, 0.0, 0.0),
                false,
            ),
        ];

        // Act
        for (origin, direction, expected) in examples {
            let ray = Ray::new(origin, direction);

            let result = bounding_box.intersects(&ray);

            // Assert
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn given_a_non_cubic_box_when_intersecting_with_a_ray_should_identify_presence_of_a_hit_correctly(
    ) {
        // Arrange
        let bounding_box =
            BoundingBox::new(Tuple::point(5.0, -2.0, 0.0), Tuple::point(11.0, 4.0, 7.0));

        let examples = vec![
            (
                Tuple::point(15.0, 1.0, 2.0),
                Tuple::vector(-1.0, 0.0, 0.0),
                true,
            ),
            (
                Tuple::point(-5.0, -1.0, 4.0),
                Tuple::vector(1.0, 0.0, 0.0),
                true,
            ),
            (
                Tuple::point(7.0, 6.0, 5.0),
                Tuple::vector(0.0, -1.0, 0.0),
                true,
            ),
            (
                Tuple::point(9.0, -5.0, 6.0),
                Tuple::vector(0.0, 1.0, 0.0),
                true,
            ),
            (
                Tuple::point(8.0, 2.0, 12.0),
                Tuple::vector(0.0, 0.0, -1.0),
                true,
            ),
            (
                Tuple::point(6.0, 0.0, -5.0),
                Tuple::vector(0.0, 0.0, 1.0),
                true,
            ),
            (
                Tuple::point(8.0, 1.0, 3.5),
                Tuple::vector(0.0, 0.0, 1.0),
                true,
            ),
            (
                Tuple::point(9.0, -1.0, -8.0),
                Tuple::vector(2.0, 4.0, 6.0),
                false,
            ),
            (
                Tuple::point(8.0, 3.0, -4.0),
                Tuple::vector(6.0, 2.0, 4.0),
                false,
            ),
            (
                Tuple::point(9.0, -1.0, -2.0),
                Tuple::vector(4.0, 6.0, 2.0),
                false,
            ),
            (
                Tuple::point(4.0, 0.0, 9.0),
                Tuple::vector(0.0, 0.0, -1.0),
                false,
            ),
            (
                Tuple::point(8.0, 6.0, -1.0),
                Tuple::vector(0.0, -1.0, 0.0),
                false,
            ),
            (
                Tuple::point(12.0, 5.0, 4.0),
                Tuple::vector(-1.0, 0.0, 0.0),
                false,
            ),
        ];

        // Act
        for (origin, direction, expected) in examples {
            let ray = Ray::new(origin, direction.normalize());

            let result = bounding_box.intersects(&ray);

            // Assert
            assert_eq!(expected, result);
        }
    }
}
