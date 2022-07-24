use crate::Matrix;
use crate::Point;
use crate::patterns::pattern::Pattern;
use crate::Color;

pub struct Solid {
    color: Color,
    transform: Matrix
}

impl Solid {
    pub fn new(color: Color, transform: Matrix) -> Solid {
        return Solid {
            color,
            transform
        }
    }

    pub fn default() -> Solid {
        return Solid::new(
            Color::new(1.0, 1.0, 1.0),
            Matrix::identity(4)
        );
    }
}

impl Pattern for Solid {
    fn pattern_at(&self, object_point: &Point) -> Color {
        return self.color.clone();
    }
}