use crate::Matrix;
use crate::Point;
use crate::patterns::pattern::Pattern;
use crate::Color;

pub struct Striped {
    former: Color,
    latter: Color,
    transform: Matrix
}

impl Striped {
    pub fn new(former: Color, latter: Color, transform: Matrix) -> Striped {
        return Striped {
            former,
            latter,
            transform
        }
    }

    pub fn default() -> Striped {
        return Striped::new(
            Color::new(1.0, 0.0, 0.0),
            Color::new(1.0, 1.0, 1.0),
            Matrix::identity(4)
        );
    }
}

impl Pattern for Striped {
    fn pattern_at(&self, object_point: &Point) -> Color {
        let pattern_point = (self.transform.inverse() * (*object_point)).unwrap();
        
        if (pattern_point.x.floor() as i64) % 2 == 0 {
            return self.former;
        }
        else {
            return self.latter;
        }
    }
}