use crate::geometry::shape::Shape;
use crate::patterns::pattern::Pattern;
use crate::patterns::solid::Solid;
use crate::Color;
use crate::Matrix;
use crate::Tuple;

pub struct Striped {
    former: Box<dyn Pattern>,
    latter: Box<dyn Pattern>,
    transform: Matrix,
}

impl Striped {
    pub fn new(former: Box<dyn Pattern>, latter: Box<dyn Pattern>, transform: Matrix) -> Striped {
        Striped {
            former,
            latter,
            transform,
        }
    }

    pub fn default() -> Striped {
        Striped::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
            Matrix::identity(4),
        )
    }
}

impl Pattern for Striped {
    fn local_pattern_at(&self, pattern_point: Tuple) -> Color {
        assert!(pattern_point.is_point());

        if (pattern_point.x.floor() as i64) % 2 == 0 {
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
    use crate::patterns::striped::Striped;
    use crate::tuples::color::Color;
    use crate::tuples::tuple::Tuple;
    use std::sync::Arc;

    #[test]
    fn given_default_striped_pattern_when_getting_color_should_be_constant_in_y() {
        // Arrange
        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());

        let pattern = Striped::default();

        // Act
        let result_a = pattern.pattern_at(shape.clone(), Tuple::point(0.0, 0.0, 0.0));
        let result_b = pattern.pattern_at(shape.clone(), Tuple::point(0.0, 1.0, 0.0));
        let result_c = pattern.pattern_at(shape.clone(), Tuple::point(0.0, 2.0, 0.0));

        // Assert
        assert_eq!(Color::white(), result_a);
        assert_eq!(Color::white(), result_b);
        assert_eq!(Color::white(), result_c);
    }
}
