use std::rc::Rc;

use crate::{matrices::matrix::Matrix, tuples::{intersection::Intersection, ray::Ray}};

use uuid::Uuid;

pub trait Shape {
    fn id(&self) -> Uuid;

    /* Using an arbitraty self type here, basically allows for polymorphic Shape types
     * and requires that they must be wrapped in an Rc so that the intersection result can grab a reference.
     *
     * Note that: Intersections are returned in increasing order.
     */
    fn intersect(self: Rc<Self>, world_ray: &Ray) -> Vec<Intersection>;

    fn get_transform(&self) -> Rc<Matrix>;

    fn set_transform(&mut self, transform: Matrix);
}