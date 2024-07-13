use crate::matrices::matrix::Matrix;
use crate::patterns::pattern::Pattern;
use crate::tuples::color::Color;
use crate::tuples::tuple::Tuple;
use std::sync::Arc;

pub struct TestPattern {
    transform: Arc<Matrix>,
}

impl TestPattern {
    pub fn new(transform: Arc<Matrix>) -> TestPattern {
        TestPattern { transform }
    }

    pub fn default() -> TestPattern {
        TestPattern {
            transform: Arc::new(Matrix::identity(4)),
        }
    }
}

impl Pattern for TestPattern {
    fn pattern_at(&self, point: Tuple) -> Color {
        Color::new(point.x, point.y, point.z)
    }

    fn get_transform(&self) -> Arc<Matrix> {
        self.transform.clone()
    }
}
