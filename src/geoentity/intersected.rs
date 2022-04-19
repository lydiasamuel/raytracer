use crate::Ray;

pub trait Intersected {
    fn get_id(&self) -> u64;
}