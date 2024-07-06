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
            Box::new(Solid::new(Color::new(1.0, 0.0, 0.0))),
            Box::new(Solid::new(Color::new(1.0, 1.0, 1.0))),
            Matrix::identity(4),
        )
    }
}

impl Pattern for Ring {
    fn local_pattern_at(&self, pattern_point: Tuple) -> Color {
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
