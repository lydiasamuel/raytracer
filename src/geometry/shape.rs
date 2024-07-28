use std::sync::Arc;
use uuid::Uuid;

use crate::geometry::group::Group;
use crate::tuples::bounding_box::BoundingBox;
use crate::tuples::color::Color;
use crate::tuples::point_light::PointLight;
use crate::{
    materials::material::Material,
    matrices::matrix::Matrix,
    tuples::{intersection::Intersection, ray::Ray, tuple::Tuple},
};

pub trait Shape: Sync + Send {
    fn id(&self) -> Uuid;

    // Transforms the ray and then calculates the resulting intersections with that ray
    //
    // Using an arbitrary self type here, basically allows for polymorphic Shape types
    // and requires that they must be wrapped in an Arc so that the intersection result can grab a reference.
    //
    // Note that: Intersections are returned in increasing order.
    //
    fn intersect(self: Arc<Self>, world_ray: &Ray) -> Vec<Intersection> {
        let inverse_transform = self.get_transform().inverse().unwrap();

        let local_ray = world_ray.transform(inverse_transform);

        self.local_intersect(&local_ray)
    }

    fn local_intersect(self: Arc<Self>, local_ray: &Ray) -> Vec<Intersection>;

    // Transformation matrix transforms points from object space to world space, and the inverse goes the other way.
    fn get_transform(&self) -> Arc<Matrix>;

    fn get_material(&self) -> Arc<dyn Material>;

    fn get_parent(&self) -> Option<Arc<Group>>;

    // Have to use a reference here, since if we take ownership the value immediately drops and we
    // lose the weak reference and thus our parent
    fn set_parent(&self, parent: &Arc<Group>);

    fn casts_shadow(&self) -> bool;

    // Assumes that the point will always be on the shape
    fn normal_at(&self, world_point: Tuple) -> Tuple {
        assert!(world_point.is_point());

        let local_point = self.world_to_object(world_point);
        let local_normal = self.local_normal_at(local_point);

        self.normal_to_world(local_normal)
    }

    fn local_normal_at(&self, local_point: Tuple) -> Tuple;

    // Converts a point from world space to object space, recursively taking into consideration any
    // parent objects between the two spaces
    fn world_to_object(&self, point: Tuple) -> Tuple {
        assert!(point.is_point());

        let parent = self.get_parent();
        let object_point: Tuple;

        if let Some(shape) = parent {
            object_point = shape.world_to_object(point);
        } else {
            object_point = point;
        }

        let inverse_transform = self.get_transform().inverse().unwrap();
        (&inverse_transform * &object_point).unwrap()
    }

    // Converts a normal vector from object space to world space, recursively taking into
    // consideration any parent objects between the two spaces
    fn normal_to_world(&self, normal: Tuple) -> Tuple {
        assert!(normal.is_vector());

        let inverse_transform = self.get_transform().inverse().unwrap();

        let mut result = (&inverse_transform.transpose() * &normal).unwrap();
        result.w = 0.0;
        result = result.normalize();

        let parent = self.get_parent();

        match parent {
            None => result,
            Some(shape) => shape.normal_to_world(result),
        }
    }

    // Gets the bounding extents for the shape (transformed if for a group)
    fn bounds(&self) -> BoundingBox;

    fn parent_space_bounds_of(&self) -> BoundingBox {
        self.bounds().transform(self.get_transform().as_ref())
    }

    fn divide(self: Arc<Self>, threshold: usize);

    fn light_material(
        self: Arc<Self>,
        world_point: Tuple,
        light: PointLight,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color;
}
