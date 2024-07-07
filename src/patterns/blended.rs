use crate::patterns::pattern::Pattern;
use crate::patterns::solid::Solid;
use crate::Color;
use crate::Matrix;
use crate::Tuple;

pub struct Blended {
    former: Box<dyn Pattern>,
    latter: Box<dyn Pattern>,
    transform: Matrix,
}

impl Blended {
    pub fn new(former: Box<dyn Pattern>, latter: Box<dyn Pattern>, transform: Matrix) -> Blended {
        Blended {
            former,
            latter,
            transform,
        }
    }

    pub fn default() -> Blended {
        Blended::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
            Matrix::identity(4),
        )
    }
}

impl Pattern for Blended {
    fn pattern_at(&self, pattern_point: Tuple) -> Color {
        assert!(pattern_point.is_point());

        let former_color = self.former.as_ref().local_pattern_at(pattern_point);
        let latter_color = self.latter.as_ref().local_pattern_at(pattern_point);

        (former_color + latter_color) / 2.0
    }

    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }
}
