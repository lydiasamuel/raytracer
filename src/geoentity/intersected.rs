use crate::Phong;
use crate::Point;
use crate::Vector;

pub trait Intersected {
    fn get_id(&self) -> u64;
    fn normal_at(&self, point: Point) -> Vector;
    fn material(&self) -> Phong;
}