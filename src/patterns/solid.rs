use crate::matrices::matrix::Matrix;
use crate::patterns::pattern::Pattern;
use crate::Color;
use crate::Tuple;

pub struct Solid {
    color: Color,
}

impl Solid {
    pub fn new(color: Color) -> Solid {
        Solid { color }
    }

    pub fn default() -> Solid {
        Solid::new(Color::new(1.0, 1.0, 1.0))
    }
}

impl Pattern for Solid {
    fn local_pattern_at(&self, _: Tuple) -> Color {
        self.color
    }

    fn get_transform(&self) -> Matrix {
        Matrix::identity(4)
    }
}
