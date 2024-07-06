use crate::patterns::pattern::Pattern;
use crate::patterns::solid::Solid;
use crate::tuples::tuple::Tuple;
use crate::Color;
use crate::Matrix;

pub struct Checker {
    former: Box<dyn Pattern>,
    latter: Box<dyn Pattern>,
    transform: Matrix,
}

impl Checker {
    pub fn new(former: Box<dyn Pattern>, latter: Box<dyn Pattern>, transform: Matrix) -> Checker {
        Checker {
            former,
            latter,
            transform,
        }
    }

    pub fn default() -> Checker {
        Checker::new(
            Box::new(Solid::new(Color::new(1.0, 0.0, 0.0))),
            Box::new(Solid::new(Color::new(1.0, 1.0, 1.0))),
            Matrix::identity(4),
        )
    }
}

impl Pattern for Checker {
    fn local_pattern_at(&self, pattern_point: Tuple) -> Color {
        assert!(pattern_point.is_point());

        let x_floor = pattern_point.x.floor();
        let y_floor = pattern_point.y.floor();
        let z_floor = pattern_point.z.floor();

        if ((x_floor + y_floor + z_floor) as i64) % 2 == 0 {
            self.former.as_ref().local_pattern_at(pattern_point)
        } else {
            self.latter.as_ref().local_pattern_at(pattern_point)
        }
    }

    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }
}
