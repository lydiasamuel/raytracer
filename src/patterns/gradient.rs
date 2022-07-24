use crate::Matrix;
use crate::Point;
use crate::patterns::pattern::Pattern;
use crate::Color;

pub struct Gradient {
    former: Color,
    latter: Color,
    transform: Matrix
}

impl Gradient {
    pub fn new(former: Color, latter: Color, transform: Matrix) -> Gradient {
        return Gradient {
            former,
            latter,
            transform
        }
    }

    pub fn default() -> Gradient {
        return Gradient::new(
            Color::new(1.0, 0.0, 0.0), 
            Color::new(1.0, 1.0, 1.0),
            Matrix::identity(4)
        );
    }
}

impl Pattern for Gradient {
    fn pattern_at(&self, object_point: &Point) -> Color {
        let pattern_point = (self.transform.inverse() * (*object_point)).unwrap();
        
        let distance = self.latter - self.former;
        let fraction = pattern_point.x - pattern_point.x.floor();

        return self.former + (distance * fraction);
    }
}