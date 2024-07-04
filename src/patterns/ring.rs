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
    fn local_pattern_at(&self, local_point: &Tuple) -> Color {
        let x_sq = local_point.x * local_point.x;
        let z_sq = local_point.z * local_point.z;

        if ((x_sq + z_sq).sqrt() as i64) % 2 == 0 {
            self.former.as_ref().pattern_at(&local_point)
        } else {
            self.latter.as_ref().pattern_at(&local_point)
        }
    }

    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }
}
