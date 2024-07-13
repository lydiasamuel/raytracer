use crate::geometry::shape::Shape;
use crate::geometry::sphere::Sphere;
use crate::materials::phong::Phong;
use crate::matrices::matrix::Matrix;
use crate::patterns::solid::Solid;
use crate::scene::computations::Computations;
use crate::tuples::color::Color;
use crate::tuples::intersection::Intersection;
use crate::tuples::pointlight::PointLight;
use crate::tuples::ray::Ray;
use crate::tuples::tuple::Tuple;
use crate::EPSILON;
use std::sync::Arc;

pub struct World {
    objects: Vec<Arc<dyn Shape>>,
    lights: Vec<Arc<PointLight>>,
}

impl World {
    pub fn new(objects: Vec<Arc<dyn Shape>>, lights: Vec<Arc<PointLight>>) -> World {
        return World { objects, lights };
    }

    pub fn default() -> World {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());

        let outer = Sphere::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::new(
                Box::new(Solid::new(Color::new(0.8, 1.0, 0.6))),
                0.1,
                0.7,
                0.2,
                200.0,
                0.0,
            )),
        );

        let inner = Sphere::new(
            Arc::new(Matrix::scaling(0.5, 0.5, 0.5)),
            Arc::new(Phong::default()),
        );

        let objects: Vec<Arc<dyn Shape>> = vec![Arc::new(outer), Arc::new(inner)];
        let lights = vec![Arc::new(light)];

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

    fn prepare_computations(intersection: &Intersection, ray: &Ray) -> Computations {
        let time = intersection.time;
        let object = intersection.object.clone();
        let point = ray.position(time);

        let eyev = -ray.direction();
        let mut normalv = object.as_ref().normal_at(point);

        let mut inside = false;

        if Tuple::dot(normalv, eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        let reflectv = Tuple::reflect(ray.direction(), normalv);

        // EPSILON is used to bump the intersection point slightly in the direction of the surface
        // normal to help prevent self shadowing
        let over_point = point + (normalv * EPSILON);

        return Computations::new(
            time, object, point, over_point, eyev, normalv, reflectv, inside,
        );
    }

    pub fn shade_hit(&self, comps: &Computations, remaining: usize) -> Color {
        let mut result = Color::new(0.0, 0.0, 0.0);

        for i in 0..self.lights.len() {
            let light = *self.lights[i];
            let in_shadow = self.is_shadowed(comps.over_point, light);

            let shape = comps.object.clone();

            let surface = shape.light_material(
                comps.over_point,
                light,
                comps.eyev,
                comps.normalv,
                in_shadow,
            );

            let reflected = self.reflected_color(comps, remaining);

            result = result + surface + reflected;
        }

        return result;
    }

    pub fn color_at(&self, ray: &Ray, remaining: usize) -> Color {
        // Call intersect to find the intersections of the given ray in this world
        let intersects = self.intersect_world(ray);

        // Find the hit from the resulting intersects
        let hit = Intersection::hit(&intersects);

        if let Some(intersect) = hit {
            let comps = World::prepare_computations(&intersect, ray);

            return self.shade_hit(&comps, remaining);
        }

        Color::black()
    }

    pub fn reflected_color(&self, comps: &Computations, remaining: usize) -> Color {
        // Base case for the recursion caused by parallel mirrors
        if remaining == 0 {
            return Color::black();
        }

        let reflective = comps.object.get_material().reflective();

        if (reflective - 0.0).abs() < EPSILON {
            return Color::black();
        }

        // Reflected ray starts at where the incident ray hit, and is pointed in the direction of reflectv
        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(&reflect_ray, remaining - 1);

        color * reflective
    }

    pub fn is_shadowed(&self, point: Tuple, light: PointLight) -> bool {
        assert!(point.is_point());

        let vec = light.position - point;

        let distance = vec.magnitude(); // Measure the distance from the point to the light source
        let direction = vec.normalize(); // Create a ray pointing towards the light source

        let ray = Ray::new(point, direction);
        let intersections = self.intersect_world(&ray); // Intersect the world with that ray

        let hit = Intersection::hit(&intersections);
        // Check to see if there was a hit, and if so did it occur before the ray reached the light source
        if let Some(intersect) = hit {
            intersect.time < distance
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::plane::Plane;
    use crate::geometry::shape::Shape;
    use crate::geometry::sphere::Sphere;
    use crate::materials::material::Material;
    use crate::materials::phong::Phong;
    use crate::matrices::matrix::Matrix;
    use crate::patterns::solid::Solid;
    use crate::scene::world::World;
    use crate::tuples::color::Color;
    use crate::tuples::intersection::Intersection;
    use crate::tuples::pointlight::PointLight;
    use crate::tuples::ray::Ray;
    use crate::tuples::tuple::Tuple;
    use crate::{EPSILON, REFLECTION_DEPTH};
    use std::f64::consts::SQRT_2;
    use std::sync::Arc;

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

        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());

        let intersection = Intersection::new(4.0, shape.clone());

        // Act
        let result = World::prepare_computations(&intersection, &ray);

        // Assert
        assert_eq!(intersection.time, result.time);
        assert_eq!(true, Arc::ptr_eq(&intersection.object, &result.object));
        assert_eq!(Tuple::point(0.0, 0.0, -1.0), result.point);
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), result.eyev);
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), result.normalv);
    }

    #[test]
    fn given_standard_values_when_the_intersect_occurs_on_the_outside_of_an_object_should_set_inside_to_false(
    ) {
        // Arrange
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());

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

        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());

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

        let shape: Arc<dyn Shape> = world.objects[0].clone();

        let intersection = Intersection::new(4.0, shape.clone());

        // Act
        let comps = World::prepare_computations(&intersection, &ray);
        let result = World::shade_hit(&world, &comps, REFLECTION_DEPTH);

        // Assert
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), result);
    }

    #[test]
    fn given_standard_values_when_shading_the_hits_from_the_inside_should_correctly_color_the_hit()
    {
        // Arrange
        let mut world = World::default();

        world.lights.pop();
        world.lights.push(Arc::new(PointLight::new(
            Tuple::point(0.0, 0.25, 0.0),
            Color::white(),
        )));

        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

        let shape: Arc<dyn Shape> = world.objects[1].clone();

        let intersection = Intersection::new(0.5, shape.clone());

        // Act
        let comps = World::prepare_computations(&intersection, &ray);
        let result = World::shade_hit(&world, &comps, REFLECTION_DEPTH);

        // Assert
        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), result);
    }

    #[test]
    fn given_a_ray_that_misses_when_calling_color_at_should_return_black() {
        // Arrange
        let world = World::default();

        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let result = world.color_at(&ray, REFLECTION_DEPTH);

        // Assert
        assert_eq!(Color::black(), result);
    }

    #[test]
    fn given_a_ray_that_hits_when_calling_color_at_should_return_correct_color_value() {
        // Arrange
        let world = World::default();

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let result = world.color_at(&ray, REFLECTION_DEPTH);

        // Assert
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), result);
    }

    #[test]
    fn given_default_world_when_ray_is_between_outer_and_inner_but_pointed_at_inner_should_color_inner(
    ) {
        // Arrange
        let color = Color::white();
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());
        let material: Arc<dyn Material> = Arc::new(Phong::new(
            Box::new(Solid::default()),
            1.0,
            0.9,
            0.9,
            200.0,
            0.0,
        ));

        let outer = Sphere::new(Arc::new(Matrix::identity(4)), material.clone());
        let inner = Sphere::new(Arc::new(Matrix::scaling(0.5, 0.5, 0.5)), material.clone());

        let objects: Vec<Arc<dyn Shape>> = vec![Arc::new(outer), Arc::new(inner)];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));

        // Act
        let result = world.color_at(&ray, REFLECTION_DEPTH);

        // Assert
        assert_eq!(color, result);
    }

    #[test]
    fn given_default_world_when_nothing_is_collinear_with_point_and_light_should_be_no_shadow() {
        // Arrange
        let world = World::default();

        let point = Tuple::point(0.0, 10.0, 0.0);

        // Act
        let result = world.is_shadowed(point, *world.lights[0]);

        // Assert
        assert_eq!(false, result);
    }

    #[test]
    fn given_default_world_when_object_is_between_point_and_light_should_be_a_shadow() {
        // Arrange
        let world = World::default();

        let point = Tuple::point(10.0, -10.0, 10.0);

        // Act
        let result = world.is_shadowed(point, *world.lights[0]);

        // Assert
        assert_eq!(true, result);
    }

    #[test]
    fn given_default_world_when_object_is_behind_the_light_should_be_no_shadow() {
        // Arrange
        let world = World::default();

        let point = Tuple::point(-20.0, 20.0, -20.0);

        // Act
        let result = world.is_shadowed(point, *world.lights[0]);

        // Assert
        assert_eq!(false, result);
    }

    #[test]
    fn given_default_world_when_object_is_behind_the_point_should_be_no_shadow() {
        // Arrange
        let world = World::default();

        let point = Tuple::point(-2.0, 2.0, -2.0);

        // Act
        let result = world.is_shadowed(point, *world.lights[0]);

        // Assert
        assert_eq!(false, result);
    }

    #[test]
    fn given_world_with_shadows_when_shading_hits_should_be_given_intersection_in_shadow() {
        // Arrange
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::white());
        let material: Arc<dyn Material> = Arc::new(Phong::default());

        let s1 = Arc::new(Sphere::unit());
        let s2 = Arc::new(Sphere::new(
            Arc::new(Matrix::translation(0.0, 0.0, 10.0)),
            material.clone(),
        ));

        let objects: Vec<Arc<dyn Shape>> = vec![s1.clone(), s2.clone()];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let intersection = Intersection::new(4.0, s2.clone());

        let comps = World::prepare_computations(&intersection, &ray);

        // Act
        let result = world.shade_hit(&comps, REFLECTION_DEPTH);

        // Assert
        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }

    #[test]
    fn given_world_with_shadows_when_preparing_computations_should_slightly_offset_the_point() {
        // Arrange
        let material: Arc<dyn Material> = Arc::new(Phong::default());

        let shape = Arc::new(Sphere::new(
            Arc::new(Matrix::translation(0.0, 0.0, 1.0)),
            material.clone(),
        ));

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let intersection = Intersection::new(5.0, shape.clone());

        // Act
        let comps = World::prepare_computations(&intersection, &ray);

        // Assert
        assert_eq!(true, comps.over_point.z < -EPSILON / 2.0);
        assert_eq!(true, comps.point.z > comps.over_point.z);
    }

    #[test]
    fn given_a_plane_when_reflecting_a_ray_at_45_degrees_should_come_out_at_45_degrees() {
        // Arrange
        let shape = Arc::new(Plane::default());

        let ray = Ray::new(
            Tuple::point(0.0, 1.0, -1.0),
            Tuple::vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );

        let intersection = Intersection::new(SQRT_2, shape.clone());

        // Act
        let comps = World::prepare_computations(&intersection, &ray);

        // Assert
        assert_eq!(
            Tuple::vector(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0),
            comps.reflectv
        );
    }

    #[test]
    fn given_a_ray_strikes_a_non_reflective_surface_when_reflecting_color_should_return_black() {
        // Arrange
        let color = Color::white();
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());

        let outer = Arc::new(Sphere::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::new(
                Box::new(Solid::new(Color::new(0.8, 1.0, 0.6))),
                1.0,
                0.7,
                0.2,
                200.0,
                0.0,
            )),
        ));
        let inner = Arc::new(Sphere::new(
            Arc::new(Matrix::scaling(0.5, 0.5, 0.5)),
            Arc::new(Phong::default()),
        ));

        let objects: Vec<Arc<dyn Shape>> = vec![outer.clone(), inner.clone()];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

        let intersection = Intersection::new(1.0, inner.clone());

        // Act
        let comps = World::prepare_computations(&intersection, &ray);
        let color = world.reflected_color(&comps, REFLECTION_DEPTH);

        // Assert
        assert_eq!(Color::black(), color);
    }

    #[test]
    fn given_a_ray_strikes_a_reflective_surface_when_reflecting_color_should_return_correct_value()
    {
        // Arrange
        let color = Color::white();
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());
        let material: Arc<dyn Material> = Arc::new(Phong::new(
            Box::new(Solid::default()),
            0.1,
            0.9,
            0.9,
            200.0,
            0.5,
        ));

        let outer = Arc::new(Sphere::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::new(
                Box::new(Solid::new(Color::new(0.8, 1.0, 0.6))),
                0.1,
                0.7,
                0.2,
                200.0,
                0.0,
            )),
        ));
        let inner = Arc::new(Sphere::new(
            Arc::new(Matrix::scaling(0.5, 0.5, 0.5)),
            Arc::new(Phong::default()),
        ));

        let plane = Arc::new(Plane::new(
            Arc::new(Matrix::translation(0.0, -1.0, 0.0)),
            material.clone(),
        ));

        let objects: Vec<Arc<dyn Shape>> = vec![outer.clone(), inner.clone(), plane.clone()];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );

        let intersection = Intersection::new(SQRT_2, plane.clone());

        // Act
        let comps = World::prepare_computations(&intersection, &ray);
        let color = world.reflected_color(&comps, REFLECTION_DEPTH);

        // Assert
        assert_eq!(Color::new(0.19033, 0.23791, 0.14274), color);
    }

    #[test]
    fn given_a_ray_strikes_a_reflective_surface_when_shading_reflected_hit_should_return_correct_value(
    ) {
        // Arrange
        let color = Color::white();
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());
        let material: Arc<dyn Material> = Arc::new(Phong::new(
            Box::new(Solid::default()),
            0.1,
            0.9,
            0.9,
            200.0,
            0.5,
        ));

        let outer = Arc::new(Sphere::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::new(
                Box::new(Solid::new(Color::new(0.8, 1.0, 0.6))),
                0.1,
                0.7,
                0.2,
                200.0,
                0.0,
            )),
        ));
        let inner = Arc::new(Sphere::new(
            Arc::new(Matrix::scaling(0.5, 0.5, 0.5)),
            Arc::new(Phong::default()),
        ));

        let plane = Arc::new(Plane::new(
            Arc::new(Matrix::translation(0.0, -1.0, 0.0)),
            material.clone(),
        ));

        let objects: Vec<Arc<dyn Shape>> = vec![outer.clone(), inner.clone(), plane.clone()];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );

        let intersection = Intersection::new(SQRT_2, plane.clone());

        // Act
        let comps = World::prepare_computations(&intersection, &ray);
        let color = world.shade_hit(&comps, REFLECTION_DEPTH);

        // Assert
        assert_eq!(Color::new(0.87675, 0.92434, 0.82917), color);
    }

    #[test]
    pub fn given_a_ray_strikes_a_reflective_surface_when_reflection_is_not_allowed_to_recurse_further_should_return_black(
    ) {
        // Arrange
        let color = Color::white();
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());
        let material: Arc<dyn Material> = Arc::new(Phong::new(
            Box::new(Solid::default()),
            0.1,
            0.9,
            0.9,
            200.0,
            0.5,
        ));

        let outer = Arc::new(Sphere::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::new(
                Box::new(Solid::new(Color::new(0.8, 1.0, 0.6))),
                0.1,
                0.7,
                0.2,
                200.0,
                0.0,
            )),
        ));
        let inner = Arc::new(Sphere::new(
            Arc::new(Matrix::scaling(0.5, 0.5, 0.5)),
            Arc::new(Phong::default()),
        ));

        let plane = Arc::new(Plane::new(
            Arc::new(Matrix::translation(0.0, -1.0, 0.0)),
            material.clone(),
        ));

        let objects: Vec<Arc<dyn Shape>> = vec![outer.clone(), inner.clone(), plane.clone()];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );

        let intersection = Intersection::new(SQRT_2, plane.clone());

        // Act
        let comps = World::prepare_computations(&intersection, &ray);
        let color = world.reflected_color(&comps, 0);

        // Assert
        assert_eq!(Color::black(), color);
    }
}
