use crate::matrices::matrix::Matrix;
use crate::Color;
use crate::Tuple;

pub trait Pattern: Sync + Send {
    fn pattern_at(&self, point: &Tuple) -> Color {
        let local_point = self.get_transform().inverse().unwrap() * (*point);

        self.local_pattern_at(&local_point.unwrap())
    }

    fn local_pattern_at(&self, local_point: &Tuple) -> Color;

    fn get_transform(&self) -> Matrix;
}
