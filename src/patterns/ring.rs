use crate::Matrix;
use crate::Point;
use crate::patterns::pattern::Pattern;
use crate::Color;

pub struct Ring {
    former: Color,
    latter: Color,
    transform: Matrix
}

impl Ring {
    pub fn new(former: Color, latter: Color, transform: Matrix) -> Ring {
        return Ring {
            former,
            latter,
            transform
        }
    }

    pub fn default() -> Ring {
        return Ring::new(
            Color::new(1.0, 0.0, 0.0), 
            Color::new(1.0, 1.0, 1.0),
            Matrix::identity(4)
        );
    }
}

impl Pattern for Ring {
    fn pattern_at(&self, object_point: &Point) -> Color {
        let pattern_point = (self.transform.inverse() * (*object_point)).unwrap();
        
        let x_sq = pattern_point.x * pattern_point.x;
        let z_sq = pattern_point.z * pattern_point.z;

        if ((x_sq + z_sq).sqrt() as i64) % 2 == 0{
            return self.former;
        }
        else {
            return self.latter;
        }
    }
}