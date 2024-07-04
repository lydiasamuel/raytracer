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
            Box::new(Solid::new(Color::new(1.0, 0.0, 0.0))),
            Box::new(Solid::new(Color::new(1.0, 1.0, 1.0))),
            Matrix::identity(4),
        )
    }
}

impl Pattern for Blended {
    fn local_pattern_at(&self, local_point: &Tuple) -> Color {
        let former_color = self.former.as_ref().pattern_at(&local_point);
        let latter_color = self.latter.as_ref().pattern_at(&local_point);

        (former_color + latter_color) / 2.0
    }

    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }
}
