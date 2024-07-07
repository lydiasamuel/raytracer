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
    fn pattern_at(&self, pattern_point: Tuple) -> Color {
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
    use crate::materials::phong::Phong;
    use crate::matrices::matrix::Matrix;
    use crate::patterns::pattern::Pattern;
    use crate::patterns::solid::Solid;
    use crate::patterns::striped::Striped;
    use crate::tuples::color::Color;
    use crate::tuples::pointlight::PointLight;
    use crate::tuples::tuple::Tuple;
    use std::sync::Arc;

    #[test]
    fn given_default_striped_pattern_when_getting_color_should_be_constant_in_y() {
        // Arrange
        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());

        let pattern = Striped::default();

        // Act
        let results = vec![
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 0.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 1.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 2.0, 0.0)),
        ];

        // Assert
        assert_eq!(Color::white(), results[0]);
        assert_eq!(Color::white(), results[1]);
        assert_eq!(Color::white(), results[2]);
    }

    #[test]
    fn given_default_striped_pattern_when_getting_color_should_be_constant_in_z() {
        // Arrange
        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());

        let pattern = Striped::default();

        // Act
        let results = vec![
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 0.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 0.0, 1.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 0.0, 2.0)),
        ];

        // Assert
        assert_eq!(Color::white(), results[0]);
        assert_eq!(Color::white(), results[1]);
        assert_eq!(Color::white(), results[2]);
    }

    #[test]
    fn given_default_striped_pattern_when_getting_color_should_be_alternate_in_x() {
        // Arrange
        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());

        let pattern = Striped::default();

        // Act
        let results = vec![
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.0, 0.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(0.9, 0.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(1.0, 0.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(-0.1, 0.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(-1.0, 0.0, 0.0)),
            pattern.pattern_at_shape(shape.clone(), Tuple::point(-1.1, 0.0, 0.0)),
        ];

        // Assert
        assert_eq!(Color::white(), results[0]);
        assert_eq!(Color::white(), results[1]);
        assert_eq!(Color::black(), results[2]);
        assert_eq!(Color::black(), results[3]);
        assert_eq!(Color::black(), results[4]);
        assert_eq!(Color::white(), results[5]);
    }

    #[test]
    fn given_shape_with_default_striped_pattern_when_lighting_should_return_correct_colors() {
        // Arrange
        let pattern = Box::new(Striped::default());
        let material = Arc::new(Phong::new(pattern, 1.0, 0.0, 0.0, 200.0));

        let shape: Arc<dyn Shape> = Arc::new(Sphere::new(Matrix::identity(4), material));

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::white());

        // Act
        let c1 =
            shape
                .clone()
                .light_material(Tuple::point(0.9, 0.0, 0.0), light, eyev, normalv, false);
        let c2 = shape.light_material(Tuple::point(1.1, 0.0, 0.0), light, eyev, normalv, false);

        // Assert
        assert_eq!(Color::white(), c1);
        assert_eq!(Color::black(), c2);
    }

    #[test]
    fn given_transformed_shape_with_default_striped_pattern_when_lighting_should_return_correct_colors(
    ) {
        // Arrange
        let pattern = Box::new(Striped::default());
        let expected_pattern = Striped::default();

        let material = Arc::new(Phong::new(pattern, 0.1, 0.9, 0.9, 200.0));

        let shape: Arc<dyn Shape> = Arc::new(Sphere::new(Matrix::scaling(2.0, 2.0, 2.0), material));

        // Act
        let result = expected_pattern.pattern_at_shape(shape.clone(), Tuple::point(1.5, 0.0, 0.0));

        // Assert
        assert_eq!(Color::white(), result);
    }

    #[test]
    fn given_shape_with_transformed_striped_pattern_when_lighting_should_return_correct_colors() {
        // Arrange
        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());

        let pattern = Striped::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
            Matrix::scaling(2.0, 2.0, 2.0),
        );

        // Act
        let result = pattern.pattern_at_shape(shape.clone(), Tuple::point(1.5, 0.0, 0.0));

        // Assert
        assert_eq!(Color::white(), result);
    }

    #[test]
    fn given_transformed_shape_and_transformed_striped_pattern_when_lighting_should_return_correct_colors(
    ) {
        // Arrange
        let pattern = Box::new(Striped::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
            Matrix::translation(0.5, 0.0, 0.0),
        ));
        let expected_pattern = Striped::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
            Matrix::translation(0.5, 0.0, 0.0),
        );

        let material = Arc::new(Phong::new(pattern, 0.1, 0.9, 0.9, 200.0));

        let shape: Arc<dyn Shape> = Arc::new(Sphere::new(Matrix::scaling(2.0, 2.0, 2.0), material));

        // Act
        let result = expected_pattern.pattern_at_shape(shape.clone(), Tuple::point(2.5, 0.0, 0.0));

        // Assert
        assert_eq!(Color::white(), result);
    }
}
