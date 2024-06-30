use std::rc::Rc;

use crate::geometry::shape::Shape;
use crate::geometry::sphere::Sphere;
use crate::materials::phong::Phong;
use crate::matrices::matrix::Matrix;
use crate::tuples::color::Color;
use crate::tuples::intersection::Intersection;
use crate::tuples::pointlight::PointLight;
use crate::tuples::ray::Ray;
use crate::tuples::tuple::Tuple;
use crate::universe::computations::Computations;

pub struct World {
    objects: Vec<Rc<dyn Shape>>,
    lights: Vec<Rc<PointLight>>,
}

impl World {
    pub fn new(objects: Vec<Rc<dyn Shape>>, lights: Vec<Rc<PointLight>>) -> World {
        return World { objects, lights };
    }

    pub fn default() -> World {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let outer = Sphere::new(
            Matrix::identity(4),
            Rc::new(Phong::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0)),
        );

        let inner = Sphere::new(Matrix::scaling(0.5, 0.5, 0.5), Rc::new(Phong::default()));

        let objects: Vec<Rc<dyn Shape>> = vec![Rc::new(outer), Rc::new(inner)];
        let lights = vec![Rc::new(light)];

        return World { objects, lights };
    }

    pub fn intersect_world(&self, ray: &Ray) -> Vec<Intersection> {
        let mut result: Vec<Intersection> = vec![];

        for i in 0..self.objects.len() {
            let obj = self.objects[i].clone();

            let mut intersects = obj.intersect(ray);
            result.append(&mut intersects);
        }

        result.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        return result;
    }

    pub fn prepare_computations(intersection: &Intersection, ray: &Ray) -> Computations {
        let time = intersection.time;
        let object = intersection.object.clone();
        let point = ray.position(time);

        let eyev = -ray.direction();
        let mut normalv = object.as_ref().normal_at(point);

        let mut inside = false;

        if Tuple::dot(&normalv, &eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        return Computations::new(time, object, point, eyev, normalv, inside);
    }

    pub fn shade_hit(&self, comps: &Computations) -> Color {
        let mut result = Color::new(0.0, 0.0, 0.0);

        for i in 0..self.lights.len() {
            let light = self.lights[i].as_ref();

            let shape = comps.object.as_ref();

            result =
                result + shape.light_material(&comps.point, light, &comps.eyev, &comps.normalv);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::shape::Shape;
    use crate::geometry::sphere::Sphere;
    use crate::tuples::color::Color;
    use crate::tuples::intersection::Intersection;
    use crate::tuples::pointlight::PointLight;
    use crate::tuples::ray::Ray;
    use crate::tuples::tuple::Tuple;
    use crate::universe::world::World;
    use std::rc::Rc;

    #[test]
    fn given_default_world_when_calculating_intersects_with_ray_should_return_correct_intersections_sorted_by_time(
    ) {
        // Arrange
        let world = World::default();

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let result = world.intersect_world(&ray);

        // Assert
        assert_eq!(4, result.len());

        assert_eq!(4.0, result[0].time);
        assert_eq!(4.5, result[1].time);
        assert_eq!(5.5, result[2].time);
        assert_eq!(6.0, result[3].time);
    }

    #[test]
    fn given_standard_values_when_calling_prepare_computations_should_return_correct_values_for_lighting_function(
    ) {
        // Arrange
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let shape: Rc<dyn Shape> = Rc::new(Sphere::unit());

        let intersection = Intersection::new(4.0, shape.clone());

        // Act
        let result = World::prepare_computations(&intersection, &ray);

        // Assert
        assert_eq!(intersection.time, result.time);
        assert_eq!(true, Rc::ptr_eq(&intersection.object, &result.object));
        assert_eq!(Tuple::point(0.0, 0.0, -1.0), result.point);
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), result.eyev);
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), result.normalv);
    }

    #[test]
    fn given_standard_values_when_the_intersect_occurs_on_the_outside_of_an_object_should_set_inside_to_false(
    ) {
        // Arrange
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let shape: Rc<dyn Shape> = Rc::new(Sphere::unit());

        let intersection = Intersection::new(4.0, shape.clone());

        // Act
        let result = World::prepare_computations(&intersection, &ray);

        // Assert
        assert_eq!(false, result.inside);
    }

    #[test]
    fn given_standard_values_when_the_intersect_occurs_on_the_inside_of_an_object_should_set_inside_to_true(
    ) {
        // Arrange
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

        let shape: Rc<dyn Shape> = Rc::new(Sphere::unit());

        let intersection = Intersection::new(1.0, shape.clone());

        // Act
        let result = World::prepare_computations(&intersection, &ray);

        // Assert
        assert_eq!(Tuple::point(0.0, 0.0, 1.0), result.point);
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), result.eyev);
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), result.normalv);
        assert_eq!(true, result.inside);
    }

    #[test]
    fn given_standard_values_when_shading_the_hits_should_correctly_color_the_hit() {
        // Arrange
        let world = World::default();

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let shape: Rc<dyn Shape> = world.objects[0].clone();

        let intersection = Intersection::new(4.0, shape.clone());

        // Act
        let comps = World::prepare_computations(&intersection, &ray);
        let result = World::shade_hit(&world, &comps);

        // Assert
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), result);
    }

    #[test]
    fn given_standard_values_when_shading_the_hits_from_the_inside_should_correctly_color_the_hit()
    {
        // Arrange
        let mut world = World::default();

        world.lights.pop();
        world.lights.push(Rc::new(PointLight::new(
            Tuple::point(0.0, 0.25, 0.0),
            Color::white(),
        )));

        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

        let shape: Rc<dyn Shape> = world.objects[1].clone();

        let intersection = Intersection::new(0.5, shape.clone());

        // Act
        let comps = World::prepare_computations(&intersection, &ray);
        let result = World::shade_hit(&world, &comps);

        // Assert
        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), result);
    }
}
