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
            Box::new(Solid::new(Color::new(1.0, 0.0, 0.0))),
            Box::new(Solid::new(Color::new(1.0, 1.0, 1.0))),
            Matrix::identity(4),
        )
    }
}

impl Pattern for Striped {
    fn local_pattern_at(&self, local_point: &Tuple) -> Color {
        if (local_point.x.floor() as i64) % 2 == 0 {
            self.former.as_ref().pattern_at(&local_point)
        } else {
            self.latter.as_ref().pattern_at(&local_point)
        }
    }

    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }
}
