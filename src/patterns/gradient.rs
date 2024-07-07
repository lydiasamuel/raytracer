use crate::patterns::pattern::Pattern;
use crate::patterns::solid::Solid;
use crate::tuples::tuple::Tuple;
use crate::Color;
use crate::Matrix;

pub struct Gradient {
    former: Box<dyn Pattern>,
    latter: Box<dyn Pattern>,
    transform: Matrix,
}

impl Gradient {
    pub fn new(former: Box<dyn Pattern>, latter: Box<dyn Pattern>, transform: Matrix) -> Gradient {
        Gradient {
            former,
            latter,
            transform,
        }
    }

    pub fn default() -> Gradient {
        Gradient::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
            Matrix::identity(4),
        )
    }
}

impl Pattern for Gradient {
    fn pattern_at(&self, pattern_point: Tuple) -> Color {
        assert!(pattern_point.is_point());

        let former_color = self.former.as_ref().local_pattern_at(pattern_point);
        let latter_color = self.latter.as_ref().local_pattern_at(pattern_point);

        let distance = latter_color - former_color;
        let fraction = pattern_point.x - pattern_point.x.floor();

        former_color + (distance * fraction)
    }

    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::shape::Shape;
    use crate::geometry::sphere::Sphere;
    use crate::patterns::gradient::Gradient;
    use crate::patterns::pattern::Pattern;
    use crate::tuples::color::Color;
    use crate::tuples::tuple::Tuple;
    use std::sync::Arc;

    #[test]
    fn given_default_gradient_pattern_when_getting_color_should_linearly_interpolate_between_colors(
    ) {
        // Arrange
        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());

        let pattern = Gradient::default();

        // Act
        let results = vec![
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 0.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.25, 0.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.5, 0.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.75, 0.0, 0.0)),
        ];

        // Assert
        assert_eq!(Color::white(), results[0]);
        assert_eq!(Color::new(0.75, 0.75, 0.75), results[1]);
        assert_eq!(Color::new(0.5, 0.5, 0.5), results[2]);
        assert_eq!(Color::new(0.25, 0.25, 0.25), results[3]);
    }
}
