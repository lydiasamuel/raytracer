use crate::geometry::shape::Shape;
use crate::matrices::matrix::Matrix;
use crate::Color;
use crate::Tuple;
use std::sync::Arc;

pub trait Pattern: Sync + Send {
    fn pattern_at_shape(&self, object: Arc<dyn Shape>, world_point: Tuple) -> Color {
        assert!(world_point.is_point());

        let object_inverse_transform = object.get_transform().inverse().unwrap();

        let object_point = (object_inverse_transform.clone() * world_point).unwrap();

        self.local_pattern_at(object_point)
    }

    fn local_pattern_at(&self, object_point: Tuple) -> Color {
        let pattern_inverse_transform = self.get_transform().inverse().unwrap();

        let pattern_point = (pattern_inverse_transform.clone() * object_point).unwrap();

        self.pattern_at(pattern_point)
    }

    fn pattern_at(&self, pattern_point: Tuple) -> Color;

    fn get_transform(&self) -> Matrix;
}
