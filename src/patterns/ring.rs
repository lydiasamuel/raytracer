use crate::patterns::pattern::Pattern;
use crate::patterns::solid::Solid;
use crate::tuples::tuple::Tuple;
use crate::Color;
use crate::Matrix;

pub struct Ring {
    former: Box<dyn Pattern>,
    latter: Box<dyn Pattern>,
    transform: Matrix,
}

impl Ring {
    pub fn new(former: Box<dyn Pattern>, latter: Box<dyn Pattern>, transform: Matrix) -> Ring {
        Ring {
            former,
            latter,
            transform,
        }
    }

    pub fn default() -> Ring {
        Ring::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
            Matrix::identity(4),
        )
    }
}

impl Pattern for Ring {
    fn pattern_at(&self, pattern_point: Tuple) -> Color {
        assert!(pattern_point.is_point());

        let x_sq = pattern_point.x * pattern_point.x;
        let z_sq = pattern_point.z * pattern_point.z;

        if ((x_sq + z_sq).sqrt() as i64) % 2 == 0 {
            self.former.as_ref().local_pattern_at(pattern_point)
        } else {
            self.latter.as_ref().local_pattern_at(pattern_point)
        }
    }

    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::shape::Shape;
    use crate::geometry::sphere::Sphere;
    use crate::patterns::pattern::Pattern;
    use crate::patterns::ring::Ring;
    use crate::tuples::color::Color;
    use crate::tuples::tuple::Tuple;
    use std::sync::Arc;

    #[test]
    fn given_default_ring_pattern_when_getting_color_should_extend_in_both_x_and_z() {
        // Arrange
        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());

        let pattern = Ring::default();

        // Act
        let results = vec![
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 0.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(1.0, 0.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 0.0, 1.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.708, 0.0, 0.708)),
        ];

        // Assert
        assert_eq!(Color::white(), results[0]);
        assert_eq!(Color::black(), results[1]);
        assert_eq!(Color::black(), results[2]);
        assert_eq!(Color::black(), results[3]);
    }
}
