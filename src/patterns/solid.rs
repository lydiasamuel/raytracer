use crate::matrices::matrix::Matrix;
use crate::patterns::pattern::Pattern;
use crate::Color;
use crate::Tuple;
use std::sync::Arc;

pub struct Solid {
    color: Color,
}

impl Solid {
    pub fn new(color: Color) -> Solid {
        Solid { color }
    }

    pub fn default() -> Solid {
        Solid::new(Color::white())
    }
}

impl Pattern for Solid {
    fn pattern_at(&self, _: Tuple) -> Color {
        self.color
    }

    fn get_transform(&self) -> Arc<Matrix> {
        Arc::new(Matrix::identity(4))
    }
}
