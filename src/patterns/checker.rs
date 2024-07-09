use crate::patterns::pattern::Pattern;
use crate::patterns::solid::Solid;
use crate::tuples::tuple::Tuple;
use crate::Color;
use crate::Matrix;
use std::sync::Arc;

pub struct Checker {
    former: Box<dyn Pattern>,
    latter: Box<dyn Pattern>,
    transform: Arc<Matrix>,
}

impl Checker {
    pub fn new(
        former: Box<dyn Pattern>,
        latter: Box<dyn Pattern>,
        transform: Arc<Matrix>,
    ) -> Checker {
        Checker {
            former,
            latter,
            transform,
        }
    }

    pub fn default() -> Checker {
        Checker::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
            Arc::new(Matrix::identity(4)),
        )
    }
}

impl Pattern for Checker {
    fn pattern_at(&self, pattern_point: Tuple) -> Color {
        assert!(pattern_point.is_point());

        let x_floor = pattern_point.x.floor();
        let y_floor = pattern_point.y.floor();
        let z_floor = pattern_point.z.floor();

        if ((x_floor + y_floor + z_floor) as i64) % 2 == 0 {
            self.former.as_ref().local_pattern_at(pattern_point)
        } else {
            self.latter.as_ref().local_pattern_at(pattern_point)
        }
    }

    fn get_transform(&self) -> Arc<Matrix> {
        self.transform.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::shape::Shape;
    use crate::geometry::sphere::Sphere;
    use crate::patterns::checker::Checker;
    use crate::patterns::pattern::Pattern;
    use crate::tuples::color::Color;
    use crate::tuples::tuple::Tuple;
    use std::sync::Arc;

    #[test]
    fn given_default_checker_pattern_when_getting_color_should_repeat_in_x() {
        // Arrange
        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());

        let pattern = Checker::default();

        // Act
        let results = vec![
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 0.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.99, 0.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(1.01, 0.0, 0.0)),
        ];

        // Assert
        assert_eq!(Color::white(), results[0]);
        assert_eq!(Color::white(), results[1]);
        assert_eq!(Color::black(), results[2]);
    }

    #[test]
    fn given_default_checker_pattern_when_getting_color_should_repeat_in_y() {
        // Arrange
        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());

        let pattern = Checker::default();

        // Act
        let results = vec![
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 0.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 0.99, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 1.01, 0.0)),
        ];

        // Assert
        assert_eq!(Color::white(), results[0]);
        assert_eq!(Color::white(), results[1]);
        assert_eq!(Color::black(), results[2]);
    }

    #[test]
    fn given_default_checker_pattern_when_getting_color_should_repeat_in_z() {
        // Arrange
        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());

        let pattern = Checker::default();

        // Act
        let results = vec![
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 0.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 0.0, 0.99)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 0.0, 1.01)),
        ];

        // Assert
        assert_eq!(Color::white(), results[0]);
        assert_eq!(Color::white(), results[1]);
        assert_eq!(Color::black(), results[2]);
    }
}
