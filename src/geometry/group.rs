use std::sync::{Arc, RwLock, Weak};
use uuid::Uuid;

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

pub struct Group {
    id: Uuid,
    transform: Arc<Matrix>,
    material: Arc<dyn Material>,
    children: RwLock<Vec<Arc<dyn Shape>>>,
    parent: RwLock<Weak<Group>>,
    casts_shadow: bool,
    bounds: RwLock<Option<BoundingBox>>, // Lazy initialisation of bounding box for the group
}

impl Group {
    pub fn default() -> Group {
        Group {
            id: Uuid::new_v4(),
            transform: Arc::new(Matrix::identity(4)),
            material: Arc::new(Phong::default()),
            children: RwLock::new(Vec::new()),
            parent: RwLock::new(Weak::new()),
            casts_shadow: true,
            bounds: RwLock::new(None),
        }
    }

    pub fn new(transform: Arc<Matrix>, children: Vec<Arc<dyn Shape>>) -> Group {
        Group {
            id: Uuid::new_v4(),
            transform,
            material: Arc::new(Phong::default()),
            children: RwLock::new(children),
            parent: RwLock::new(Weak::new()),
            casts_shadow: true,
            bounds: RwLock::new(None),
        }
    }

    pub fn add_child(self: &Arc<Self>, child: Arc<dyn Shape>) {
        // Set the child's parent using the interior mutability method
        child.clone().set_parent(self);
        // Add it to the child list
        self.children.write().unwrap().push(child);
        // Invalidate the stored bounds
        *self.bounds.write().unwrap() = None;
    }

    fn find_bounds(&self) -> BoundingBox {
        let mut result = BoundingBox::empty();

        let children = self.children.read().unwrap();

        for child in (*children).iter() {
            let shape_bounds = child.clone().parent_space_bounds_of();
            result = result + shape_bounds;
        }

        result
    }
}

impl Shape for Group {
    fn id(&self) -> Uuid {
        self.id
    }

    fn local_intersect(self: Arc<Self>, local_ray: &Ray) -> Vec<Intersection> {
        if self.bounds().intersects(local_ray) {
            let mut result = Vec::new();

            let children = self.children.read().unwrap();

            for child in (*children).iter() {
                let mut intersections = child.clone().intersect(local_ray);
                result.append(&mut intersections);
            }

            // Possible improvement by using a sorted list here instead to save on a bit of work,
            // but may actually be slower in practise
            if children.len() > 1 {
                result.sort();
            }

            result
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

    fn get_parent(&self) -> Option<Arc<Group>> {
        self.parent.read().unwrap().upgrade()
    }

    fn set_parent(&self, parent: &Arc<Group>) {
        *self.parent.write().unwrap() = Arc::downgrade(parent);
    }

    fn casts_shadow(&self) -> bool {
        self.casts_shadow
    }

    fn local_normal_at(&self, _: Tuple) -> Tuple {
        panic!("Error: Can't call local_normal_at on a group")
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
    use crate::geometry::cylinder::Cylinder;
    use crate::geometry::group::Group;
    use crate::geometry::shape::Shape;
    use crate::geometry::sphere::Sphere;
    use crate::geometry::test_shape::TestShape;
    use crate::materials::phong::Phong;
    use crate::matrices::matrix::Matrix;
    use crate::tuples::ray::Ray;
    use crate::tuples::tuple::Tuple;
    use std::f64::consts::PI;
    use std::sync::Arc;

    #[test]
    fn given_a_ray_when_intersecting_with_an_empty_group_should_return_no_hits() {
        // Arrange
        let group = Arc::new(Group::default());
        let ray = Ray::new(Tuple::origin(), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let intersects = group.local_intersect(&ray);

        // Assert
        assert_eq!(0, intersects.len())
    }

    #[test]
    fn given_a_ray_when_intersecting_with_a_non_empty_group_should_return_all_the_correct_hits() {
        // Arrange
        let group = Arc::new(Group::default());
        let s1: Arc<dyn Shape> = Arc::new(Sphere::unit());
        let s2: Arc<dyn Shape> = Arc::new(Sphere::new(
            Arc::new(Matrix::translation(0.0, 0.0, -3.0)),
            Arc::new(Phong::default()),
            true,
        ));
        let s3: Arc<dyn Shape> = Arc::new(Sphere::new(
            Arc::new(Matrix::translation(5.0, 0.0, 0.0)),
            Arc::new(Phong::default()),
            true,
        ));

        group.clone().add_child(s1.clone());
        group.clone().add_child(s2.clone());
        group.clone().add_child(s3.clone());

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let intersects = group.local_intersect(&ray);

        // Assert
        assert_eq!(4, intersects.len());
        assert_eq!(true, Arc::ptr_eq(&intersects[0].object, &s2));
        assert_eq!(true, Arc::ptr_eq(&intersects[1].object, &s2));
        assert_eq!(true, Arc::ptr_eq(&intersects[2].object, &s1));
        assert_eq!(true, Arc::ptr_eq(&intersects[3].object, &s1));
    }

    #[test]
    fn given_a_ray_when_intersecting_with_a_non_empty_transformed_group_should_reflect_group_transform_in_all_child_intersects(
    ) {
        // Arrange
        let group = Arc::new(Group::new(
            Arc::new(Matrix::scaling(2.0, 2.0, 2.0)),
            Vec::new(),
        ));
        let sphere: Arc<dyn Shape> = Arc::new(Sphere::new(
            Arc::new(Matrix::translation(5.0, 0.0, 0.0)),
            Arc::new(Phong::default()),
            true,
        ));

        group.clone().add_child(sphere.clone());

        let ray = Ray::new(Tuple::point(10.0, 0.0, -10.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let intersects = group.intersect(&ray);

        // Assert
        assert_eq!(2, intersects.len());
    }

    #[test]
    fn given_a_point_when_converting_it_from_world_to_object_space_should_apply_each_transform_in_sequence(
    ) {
        // Arrange
        let g1 = Arc::new(Group::new(
            Arc::new(Matrix::rotation_y(PI / 2.0)),
            Vec::new(),
        ));

        let g2 = Arc::new(Group::new(
            Arc::new(Matrix::scaling(2.0, 2.0, 2.0)),
            Vec::new(),
        ));

        let sphere = Arc::new(Sphere::new(
            Arc::new(Matrix::translation(5.0, 0.0, 0.0)),
            Arc::new(Phong::default()),
            true,
        ));

        g1.add_child(g2.clone());
        g2.add_child(sphere.clone());

        // Act
        let p = sphere.world_to_object(Tuple::point(-2.0, 0.0, -10.0));

        // Assert
        assert_eq!(Tuple::point(0.0, 0.0, -1.0), p);
    }

    #[test]
    fn given_a_normal_when_converting_it_from_object_to_world_space_should_apply_each_transform_in_sequence(
    ) {
        // Arrange
        let g1 = Arc::new(Group::new(
            Arc::new(Matrix::rotation_y(PI / 2.0)),
            Vec::new(),
        ));

        let g2 = Arc::new(Group::new(
            Arc::new(Matrix::scaling(1.0, 2.0, 3.0)),
            Vec::new(),
        ));

        let sphere = Arc::new(Sphere::new(
            Arc::new(Matrix::translation(5.0, 0.0, 0.0)),
            Arc::new(Phong::default()),
            true,
        ));

        g1.add_child(g2.clone());
        g2.add_child(sphere.clone());

        // Act
        let n = sphere.normal_to_world(Tuple::vector(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));

        // Assert
        assert_eq!(Tuple::vector(0.28571, 0.42857, -0.85714), n);
    }

    #[test]
    fn given_a_child_object_when_finding_normal_should_apply_each_transform_in_sequence() {
        // Arrange
        let g1 = Arc::new(Group::new(
            Arc::new(Matrix::rotation_y(PI / 2.0)),
            Vec::new(),
        ));

        let g2 = Arc::new(Group::new(
            Arc::new(Matrix::scaling(1.0, 2.0, 3.0)),
            Vec::new(),
        ));

        let sphere = Arc::new(Sphere::new(
            Arc::new(Matrix::translation(5.0, 0.0, 0.0)),
            Arc::new(Phong::default()),
            true,
        ));

        g1.add_child(g2.clone());
        g2.add_child(sphere.clone());

        // Act
        let n = sphere.normal_at(Tuple::point(1.7321, 1.1547, -5.5774));

        // Assert
        assert_eq!(Tuple::vector(0.28570, 0.42854, -0.85716), n);
    }

    #[test]
    fn given_a_group_when_finding_the_bounding_box_should_return_box_that_encloses_each_one() {
        // Arrange
        let g = Arc::new(Group::default());

        let sphere = Arc::new(Sphere::new(
            Arc::new(
                (&Matrix::translation(2.0, 5.0, -3.0) * &Matrix::scaling(2.0, 2.0, 2.0)).unwrap(),
            ),
            Arc::new(Phong::default()),
            true,
        ));

        let cylinder = Arc::new(Cylinder::new(
            Arc::new(
                (&Matrix::translation(-4.0, -1.0, 4.0) * &Matrix::scaling(0.5, 1.0, 0.5)).unwrap(),
            ),
            Arc::new(Phong::default()),
            true,
            -2.0,
            2.0,
            true,
        ));

        g.add_child(sphere);
        g.add_child(cylinder);

        // Act
        let bounds = g.bounds();

        // Assert
        assert_eq!(bounds.min(), Tuple::point(-4.5, -3.0, -5.0));
        assert_eq!(bounds.max(), Tuple::point(4.0, 7.0, 4.5));
    }

    #[test]
    fn given_a_ray_that_misses_when_intersecting_with_a_non_empty_group_should_not_test_children() {
        // Arrange
        let group = Arc::new(Group::default());

        let s: Arc<TestShape> = Arc::new(TestShape::new());

        group.clone().add_child(s.clone());

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));

        // Act
        let intersects = group.local_intersect(&ray);

        // Assert
        assert_eq!(true, s.saved_ray().is_none());
    }

    #[test]
    fn given_a_ray_that_hits_when_intersecting_with_a_non_empty_group_should_test_children() {
        // Arrange
        let group = Arc::new(Group::default());

        let s: Arc<TestShape> = Arc::new(TestShape::new());

        group.clone().add_child(s.clone());

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let intersects = group.local_intersect(&ray);

        // Assert
        assert_eq!(true, s.saved_ray().is_some());
    }
}
