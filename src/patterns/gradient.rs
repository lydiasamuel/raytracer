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
            Box::new(Solid::new(Color::new(1.0, 0.0, 0.0))),
            Box::new(Solid::new(Color::new(1.0, 1.0, 1.0))),
            Matrix::identity(4),
        )
    }
}

impl Pattern for Gradient {
    fn local_pattern_at(&self, pattern_point: Tuple) -> Color {
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
