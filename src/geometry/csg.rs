use crate::geometry::group::Group;
use crate::geometry::shape::Shape;
use crate::materials::material::Material;
use crate::materials::phong::Phong;
use crate::matrices::matrix::Matrix;
use crate::tuples::bounding_box::BoundingBox;
use crate::tuples::color::Color;
use crate::tuples::intersection::Intersection;
use crate::tuples::point_light::PointLight;
use crate::tuples::ray::Ray;
use crate::tuples::tuple::Tuple;
use std::sync::{Arc, RwLock, Weak};
use uuid::Uuid;

pub struct CSG {
    id: Uuid,
    transform: Arc<Matrix>,
    material: Arc<dyn Material>,
    operation: Operation,
    left: Arc<dyn Shape>,
    right: Arc<dyn Shape>,
    parent: RwLock<Weak<dyn Shape>>,
    casts_shadow: bool,
    bounds: RwLock<Option<BoundingBox>>,
}

pub enum Operation {
    Difference,
    Intersection,
    Union,
}

impl CSG {
    pub fn new(
        transform: Arc<Matrix>,
        material: Arc<dyn Material>,
        operation: Operation,
        left: Arc<dyn Shape>,
        right: Arc<dyn Shape>,
    ) -> Arc<CSG> {
        let result = Arc::new(CSG {
            id: Uuid::new_v4(),
            transform,
            material,
            operation,
            left: left.clone(),
            right: right.clone(),
            parent: RwLock::new(Weak::<Group>::new()),
            casts_shadow: false,
            bounds: Default::default(),
        });

        let tmp: Arc<dyn Shape> = result.clone();

        left.set_parent(&tmp);
        right.set_parent(&tmp);

        result
    }

    pub fn default(
        operation: Operation,
        left: Arc<dyn Shape>,
        right: Arc<dyn Shape>,
    ) -> Arc<CSG> {
        CSG::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::default()),
            operation,
            left,
            right
        )
    }

    pub fn left(&self) -> Arc<dyn Shape> {
        self.left.clone()
    }

    pub fn right(&self) -> Arc<dyn Shape> {
        self.right.clone()
    }

    fn find_bounds(&self) -> BoundingBox {
        BoundingBox::empty() + self.left().parent_space_bounds_of() + self.right().parent_space_bounds_of()
    }

    pub fn filter_intersections(&self, intersections: Vec<Intersection>) -> Vec<Intersection> {
        // Begin outside of both children
        let mut inl = false;
        let mut inr = false;

        // Prepare a list to receive the filtered intersections
        let mut result: Vec<Intersection> = Vec::new();

        for intersection in intersections {
            // if i.object is part of the "left" child, then lhit is true
            let lhit = self.left().includes(&intersection.object());

            if CSG::intersection_allowed(&self.operation, lhit, inl, inr) {
                result.push(intersection);
            }

            // Depending on which object was hit, toggle either inl or inr
            if lhit {
                inl = !inl;
            } else {
                inr = !inr;
            }
        }

        result
    }

    // op is the operation being evaluated
    // lhit is true if the left shape was hit, and false if the right shape was hit
    // inl is true if the hit occurs inside the left shape
    // inr is true if the hit occurs inside the right shape
    pub fn intersection_allowed(op: &Operation, lhit: bool, inl: bool, inr: bool) -> bool {
        match op {
            Operation::Difference => {
                // A difference preserves all intersections not exclusively inside the object on the
                // right, i.e. we only want those on the left that aren't inside the right, and
                // every one on the right that's inside the left.
                (lhit && !inr) || (!lhit && inl)
            }
            Operation::Intersection => {
                // An intersection preserves all intersections where both shapes overlap
                // i.e. we only want those intersections that strike one object while in another
                (lhit && inr) || (!lhit && inl)
            }
            Operation::Union => {
                // A union preserves all intersections on the exterior of both shapes
                // i.e. we only want the intersections that are not inside another object
                (lhit && !inr) || (!lhit && !inl)
            }
        }
    }
}

impl Shape for CSG {
    fn id(&self) -> Uuid {
        self.id
    }

    fn local_intersect(self: Arc<Self>, local_ray: &Ray) -> Vec<Intersection> {
        if self.bounds().intersects(local_ray) {
            let mut result = Vec::new();

            result.append(&mut self.left().intersect(local_ray));
            result.append(&mut self.right().intersect(local_ray));

            result.sort();

            self.filter_intersections(result)
        } else {
            vec![]
        }
    }

    fn get_transform(&self) -> Arc<Matrix> {
        self.transform.clone()
    }

    fn get_material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }

    fn get_parent(&self) -> Option<Arc<dyn Shape>> {
        self.parent.read().unwrap().upgrade()
    }

    fn set_parent(&self, parent: &Arc<dyn Shape>) {
        *self.parent.write().unwrap() = Arc::downgrade(parent);
    }

    fn includes(self: Arc<Self>, other: &Arc<dyn Shape>) -> bool {
        self.left().includes(other) || self.right().includes(other)
    }

    fn num_of_children(&self) -> usize {
        2
    }

    fn casts_shadow(&self) -> bool {
        self.casts_shadow
    }

    fn local_normal_at(&self, _: Tuple, _: &Intersection) -> Tuple {
        panic!("Error: Can't call local_normal_at on a csg shape")
    }

    fn bounds(&self) -> BoundingBox {
        {
            // Wrap this in its own scope so the read lock gets dropped before we potentially acquire
            // the write lock
            if let Some(bounds) = *self.bounds.read().unwrap() {
                return bounds;
            }
        }

        let bounds = self.find_bounds();
        *self.bounds.write().unwrap() = Some(bounds);
        bounds
    }

    fn points(&self) -> (Tuple, Tuple, Tuple) {
        panic!("Error: points function is not implemented for this shape")
    }

    fn normals(&self) -> (Tuple, Tuple, Tuple) {
        panic!("Error: normals function is not implemented for this shape")
    }

    fn edge_vectors(&self) -> (Tuple, Tuple) {
        panic!("Error: edge_vectors function is not implemented for this shape")
    }

    fn divide(self: Arc<Self>, threshold: usize) {
        self.left().divide(threshold);
        self.right().divide(threshold);
    }

    fn light_material(
        self: Arc<Self>,
        world_point: Tuple,
        light: PointLight,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        self.get_material()
            .lighting(self, light, world_point, eyev, normalv, in_shadow)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::geometry::csg::CSG;
    use crate::geometry::csg::Operation::{Difference, Intersection, Union};
    use crate::geometry::cube::Cube;
    use crate::geometry::shape::Shape;
    use crate::geometry::sphere::Sphere;
    use crate::geometry::test_shape::TestShape;
    use crate::materials::phong::Phong;
    use crate::matrices::matrix::Matrix;
    use crate::tuples::ray::Ray;
    use crate::tuples::tuple::Tuple;

    #[test]
    fn given_two_basic_shapes_and_an_operation_when_a_csg_is_created_should_set_parents_of_children_correctly() {
        // Arrange
        let s1: Arc<dyn Shape> = Arc::new(Sphere::unit());
        let s2: Arc<dyn Shape> = Arc::new(Cube::default());

        // Act
        let c = CSG::default(Union, s1.clone(), s2.clone());

        // Assert
        assert!(Arc::ptr_eq(&c.left(), &s1));
        assert!(Arc::ptr_eq(&c.right(), &s2));

        let tmp: Arc<dyn Shape> = c;

        assert!(Arc::ptr_eq(&s1.get_parent().unwrap(), &tmp));
        assert!(Arc::ptr_eq(&s2.get_parent().unwrap(), &tmp));
    }

    #[test]
    fn given_a_csg_shape_when_calculating_the_bounds_should_return_box_that_contains_its_children() {
        // Arrange
        let left: Arc<dyn Shape> = Arc::new(Sphere::unit());
        let right: Arc<dyn Shape> = Arc::new(Sphere::new(
            Arc::new(Matrix::translation(2.0, 3.0, 4.0)),
            Arc::new(Phong::default()),
            true
        ));

        let shape = CSG::default(Difference, left, right);

        // Act
        let bounds = shape.bounds();

        // Assert
        assert_eq!(Tuple::point(-1.0, -1.0, -1.0), bounds.min());
        assert_eq!(Tuple::point(3.0, 4.0, 5.0), bounds.max());
    }

    #[test]
    fn given_the_union_operation_when_evaluating_hits_should_preserve_all_on_exterior_of_both_shapes() {
        // Arrange
        let truth_table = vec![
            (true, true, true, false),
            (true, true, false, true),
            (true, false, true, false),
            (true, false, false, true),
            (false, true, true, false),
            (false, true, false, false),
            (false, false, true, true),
            (false, false, false, true),
        ];

        // Act
        for (lhit, inl, inr, result) in truth_table {
            // Assert
            assert_eq!(result, CSG::intersection_allowed(&Union, lhit, inl, inr));
        }
    }

    #[test]
    fn given_the_intersection_operation_when_evaluating_hits_should_preserve_all_where_shapes_overlap() {
        // Arrange
        let truth_table = vec![
            (true, true, true, true),
            (true, true, false, false),
            (true, false, true, true),
            (true, false, false, false),
            (false, true, true, true),
            (false, true, false, true),
            (false, false, true, false),
            (false, false, false, false),
        ];

        // Act
        for (lhit, inl, inr, result) in truth_table {
            // Assert
            assert_eq!(result, CSG::intersection_allowed(&Intersection, lhit, inl, inr));
        }
    }

    #[test]
    fn given_the_difference_operation_when_evaluating_hits_should_preserve_all_not_exclusively_inside_the_right_shape() {
        // Arrange
        let truth_table = vec![
            (true, true, true, false),
            (true, true, false, true),
            (true, false, true, false),
            (true, false, false, true),
            (false, true, true, true),
            (false, true, false, true),
            (false, false, true, false),
            (false, false, false, false),
        ];

        // Act
        for (lhit, inl, inr, result) in truth_table {
            // Assert
            assert_eq!(result, CSG::intersection_allowed(&Difference, lhit, inl, inr));
        }
    }

    #[test]
    fn given_a_csg_shape_when_filtering_a_list_of_intersections_should_filter_according_to_operation() {
        // Arrange
        let s1: Arc<dyn Shape> = Arc::new(Sphere::unit());
        let s2: Arc<dyn Shape> = Arc::new(Cube::default());

        let examples = vec![
            (Union, 0, 3),
            (Intersection, 1, 2),
            (Difference, 0, 1),
        ];

        // Act
        for (op, x0, x1) in examples {
            let intersections = vec![
                crate::tuples::intersection::Intersection::new(1.0, s1.clone()),
                crate::tuples::intersection::Intersection::new(2.0, s2.clone()),
                crate::tuples::intersection::Intersection::new(3.0, s1.clone()),
                crate::tuples::intersection::Intersection::new(4.0, s2.clone()),
            ];

            let x0_time = intersections[x0].time();
            let x0_object = intersections[x0].object();
            let x1_time = intersections[x1].time();
            let x1_object = intersections[x1].object();

            let c = CSG::default(op, s1.clone(), s2.clone());
            let result = c.filter_intersections(intersections);

            // Assert
            assert_eq!(2, result.len());

            assert_eq!(x0_time, result[0].time());
            assert!(Arc::ptr_eq(&x0_object, &result[0].object()));

            assert_eq!(x1_time, result[1].time());
            assert!(Arc::ptr_eq(&x1_object, &result[1].object()));
        }
    }

    #[test]
    fn given_a_csg_shape_when_intersecting_with_a_ray_that_misses_should_return_empty_list() {
        // Arrange
        let s1: Arc<dyn Shape> = Arc::new(Sphere::unit());
        let s2: Arc<dyn Shape> = Arc::new(Cube::default());

        let c = CSG::default(Union, s1.clone(), s2.clone());

        let ray = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let intersections = c.local_intersect(&ray);

        // Assert
        assert!(intersections.is_empty());
    }

    #[test]
    fn given_a_csg_shape_when_intersecting_with_a_ray_should_return_correct_hits() {
        // Arrange
        let s1: Arc<dyn Shape> = Arc::new(Sphere::unit());
        let s2: Arc<dyn Shape> = Arc::new(Sphere::new(
            Arc::new(Matrix::translation(0.0, 0.0, 0.5)),
            Arc::new(Phong::default()),
            true
        ));

        let c = CSG::default(Union, s1.clone(), s2.clone());

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let intersections = c.local_intersect(&ray);

        // Assert
        assert_eq!(2, intersections.len());

        assert_eq!(4.0, intersections[0].time());
        assert!(Arc::ptr_eq(&s1, &intersections[0].object()));

        assert_eq!(6.5, intersections[1].time());
        assert!(Arc::ptr_eq(&s2, &intersections[1].object()));
    }

    #[test]
    fn given_a_csg_shape_when_intersecting_with_a_ray_that_misses_should_not_test_children_if_bbox_is_missed() {
        // Arrange
        let left = Arc::new(TestShape::new());
        let right = Arc::new(TestShape::new());

        let shape = CSG::default(Difference, left.clone(), right.clone());

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));

        // Act
        let intersections = shape.intersect(&ray);

        // Assert

        assert!(left.saved_ray().is_none());
        assert!(right.saved_ray().is_none());
    }

    #[test]
    fn given_a_csg_shape_when_intersecting_with_a_ray_should_test_children_if_bbox_is_hit() {
        // Arrange
        let left = Arc::new(TestShape::new());
        let right = Arc::new(TestShape::new());

        let shape = CSG::default(Difference, left.clone(), right.clone());

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let intersections = shape.intersect(&ray);

        // Assert

        assert!(left.saved_ray().is_some());
        assert!(right.saved_ray().is_some());
    }
}