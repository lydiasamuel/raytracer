use crate::Matrix;
use crate::Point;
use crate::patterns::pattern::Pattern;
use crate::Color;

pub struct Checker {
    former: Color,
    latter: Color,
    transform: Matrix
}

impl Checker {
    pub fn new(former: Color, latter: Color, transform: Matrix) -> Checker {
        return Checker {
            former,
            latter,
            transform
        }
    }

    pub fn default() -> Checker{
        return Checker::new(
            Color::new(1.0, 0.0, 0.0), 
            Color::new(1.0, 1.0, 1.0),
            Matrix::identity(4)
        );
    }
}

impl Pattern for Checker {
    fn pattern_at(&self, object_point: &Point) -> Color {
        let pattern_point = object_point.transform(self.transform.inverse());
        
        let x_floor = pattern_point.x.floor();
        let y_floor = pattern_point.y.floor();
        let z_floor = pattern_point.z.floor();

        if ((x_floor + y_floor + z_floor) as i64) % 2 == 0 {
            return self.former;
        }
        else {
            return self.latter;
        }
    }
}