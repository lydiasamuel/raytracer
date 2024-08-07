use crate::geometry::shape::Shape;
use crate::geometry::sphere::Sphere;
use crate::materials::phong::Phong;
use crate::matrices::matrix::Matrix;
use crate::patterns::solid::Solid;
use crate::scene::computations::Computations;
use crate::tuples::color::Color;
use crate::tuples::intersection::Intersection;
use crate::tuples::point_light::PointLight;
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
                0.0,
                1.0,
            )),
            true,
        );

        let inner = Sphere::new(
            Arc::new(Matrix::scaling(0.5, 0.5, 0.5)),
            Arc::new(Phong::default()),
            true,
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

        result.sort_by(|a, b| a.time().partial_cmp(&b.time()).unwrap());

        return result;
    }

    fn prepare_computations(
        hit_index: usize,
        ray: &Ray,
        intersections: &Vec<Intersection>,
    ) -> Computations {
        let intersection = &intersections[hit_index];

        let time = intersection.time();
        let object = intersection.object();
        let point = ray.position(time);

        let eyev = -ray.direction();
        let mut normalv = object.as_ref().normal_at(point, intersection);

        let mut inside = false;

        if Tuple::dot(normalv, eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        let reflectv = Tuple::reflect(ray.direction(), normalv);

        // EPSILON is used to bump the intersection point slightly in the direction of the surface
        // normal to help prevent self shadowing
        let over_point = point + (normalv * EPSILON);
        let under_point = point - (normalv * EPSILON);

        // Records objects that have been encountered but not yet exited.
        let mut containers: Vec<Arc<dyn Shape>> = vec![];
        let mut n1 = 0.0;
        let mut n2 = 0.0;

        for (i, intersect) in intersections.into_iter().enumerate() {
            // If intersection is the hit
            if i == hit_index {
                if containers.is_empty() {
                    // Otherwise there's no object, so just set it to vacuum
                    n1 = 1.0;
                } else {
                    // Set n1 to the refractive index of the last object in the containers list
                    n1 = containers.last().unwrap().get_material().refractive_index();
                }
            }
            // If the intersection's object is already in the containers list, then this intersection
            // must be exiting the object.
            if let Some(i) = Self::contains_object(intersect, &containers) {
                containers.remove(i); // So remove it
            } else {
                // Otherwise it's entering the object
                containers.push(intersect.object());
            }

            if i == hit_index {
                if containers.is_empty() {
                    n2 = 1.0;
                } else {
                    n2 = containers.last().unwrap().get_material().refractive_index();
                }

                break; // Terminate the loop to stop us repeating ourselves
            }
        }

        return Computations::new(
            time,
            object,
            point,
            over_point,
            under_point,
            n1,
            n2,
            eyev,
            normalv,
            reflectv,
            inside,
        );
    }

    fn contains_object(
        intersection: &Intersection,
        containers: &Vec<Arc<dyn Shape>>,
    ) -> Option<usize> {
        for (i, object) in containers.into_iter().enumerate() {
            if Arc::ptr_eq(&intersection.object(), object) {
                return Some(i);
            }
        }

        None
    }

    pub fn shade_hit(&self, comps: &Computations, remaining: usize) -> Color {
        let mut result = Color::new(0.0, 0.0, 0.0);

        for i in 0..self.lights.len() {
            let light = *self.lights[i];
            let in_shadow = self.is_shadowed(comps.over_point, light);

            let shape = comps.object.clone();
            let material = shape.get_material();

            let surface = shape.light_material(
                comps.over_point,
                light,
                comps.eyev,
                comps.normalv,
                in_shadow,
            );

            let reflected = self.reflected_color(comps, remaining);
            let refracted = self.refracted_color(comps, remaining);

            if material.reflective() > 0.0 && material.transparency() > 0.0 {
                let reflectance = Self::schlick(comps);

                result = result
                    + surface
                    + (reflected * reflectance)
                    + (refracted * (1.0 - reflectance));
            } else {
                result = result + surface + reflected + refracted;
            }
        }

        return result;
    }

    pub fn color_at(&self, ray: &Ray, remaining: usize) -> Color {
        // Call intersect to find the intersections of the given ray in this world
        let intersects = self.intersect_world(ray);

        // Find the hit from the resulting intersects
        let hit = Intersection::hit(&intersects);

        if let Some((i, _)) = hit {
            let comps = World::prepare_computations(i, ray, &intersects);

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

    pub fn refracted_color(&self, comps: &Computations, remaining: usize) -> Color {
        // Similar base case for recursion to reflection, stops infinities
        if remaining == 0 {
            return Color::black();
        }

        let transparency = comps.object.get_material().transparency();

        // Material is opaque so return black
        if (transparency - 0.0).abs() < EPSILON {
            return Color::black();
        }

        // Find the ration of first index of refraction to the second.
        // i.e. the inversion of Snell's Law (describes relationship between angle of incoming ray
        // and the angle of the refracted ray).
        let n_ratio = comps.n1 / comps.n2;

        // Calculate cos(theta_i) is the same as the dot product of the two vectors.
        let cos_i = Tuple::dot(comps.eyev, comps.normalv);

        // Find sin(theta_t)^2 via trigonometric identity
        let sin2_t = (n_ratio * n_ratio) * (1.0 - (cos_i * cos_i));

        // If sin2_t is greater than 1, then there's total internal reflection. This means that
        // light isn't propagated across the interface between the two media, so we don't color it.
        if sin2_t > 1.0 {
            return Color::black();
        }

        // Find cos(theta_t) via trigonometric identity
        let cos_t = (1.0 - sin2_t).sqrt();

        // Compute the direction of the refracted ray
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;

        // Create the refracted ray
        let refract_ray = Ray::new(comps.under_point, direction);

        // Find the color of the refracted ray, making sure to multiply by the transparency value
        // to account for any opacity
        self.color_at(&refract_ray, remaining - 1) * transparency
    }

    // Computes the approximation of the Fresnel Equations and returns the reflectance value between
    // 0.0 and 1.0
    pub fn schlick(comps: &Computations) -> f64 {
        // Find the cosine of the angle between the eye and normal vectors
        let mut cos = Tuple::dot(comps.eyev, comps.normalv);

        // Total internal reflection can only occur if n1 > n2
        if comps.n1 > comps.n2 {
            let n = comps.n1 / comps.n2;
            let sin2_t = (n * n) * (1.0 - (cos * cos));

            if sin2_t > 1.0 {
                return 1.0;
            }

            // Compute cosine of theta_t using trig identity
            let cos_t = (1.0 - sin2_t).cos();

            // When n1 > n2 use cos(theta_t) instead
            cos = cos_t;
        }

        let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
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
        if let Some((i, casts_shadow)) = hit {
            if casts_shadow {
                intersections[i].time() < distance
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::plane::Plane;
    use crate::geometry::shape::Shape;
    use crate::geometry::smooth_triangle::SmoothTriangle;
    use crate::geometry::sphere::Sphere;
    use crate::materials::material::Material;
    use crate::materials::phong::Phong;
    use crate::matrices::matrix::Matrix;
    use crate::patterns::solid::Solid;
    use crate::patterns::test_pattern::TestPattern;
    use crate::scene::computations::Computations;
    use crate::scene::world::World;
    use crate::tuples::color::Color;
    use crate::tuples::intersection::Intersection;
    use crate::tuples::point_light::PointLight;
    use crate::tuples::ray::Ray;
    use crate::tuples::tuple::Tuple;
    use crate::{EPSILON, MAX_RAY_RECURSION_DEPTH};
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

        assert_eq!(4.0, result[0].time());
        assert_eq!(4.5, result[1].time());
        assert_eq!(5.5, result[2].time());
        assert_eq!(6.0, result[3].time());
    }

    #[test]
    fn given_standard_values_when_calling_prepare_computations_should_return_correct_values_for_lighting_function(
    ) {
        // Arrange
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());

        let intersections = vec![Intersection::new(4.0, shape.clone())];

        // Act
        let result = World::prepare_computations(0, &ray, &intersections);

        // Assert
        assert_eq!(intersections[0].time(), result.time);
        assert_eq!(
            true,
            Arc::ptr_eq(&intersections[0].object(), &result.object)
        );
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

        let intersections = vec![Intersection::new(4.0, shape.clone())];

        // Act
        let result = World::prepare_computations(0, &ray, &intersections);

        // Assert
        assert_eq!(false, result.inside);
    }

    #[test]
    fn given_standard_values_when_the_intersect_occurs_on_the_inside_of_an_object_should_set_inside_to_true(
    ) {
        // Arrange
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());

        let intersections = vec![Intersection::new(1.0, shape.clone())];

        // Act
        let result = World::prepare_computations(0, &ray, &intersections);

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

        let intersections = vec![Intersection::new(4.0, shape.clone())];

        // Act
        let comps = World::prepare_computations(0, &ray, &intersections);
        let result = World::shade_hit(&world, &comps, MAX_RAY_RECURSION_DEPTH);

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

        let intersections = vec![Intersection::new(0.5, shape.clone())];

        // Act
        let comps = World::prepare_computations(0, &ray, &intersections);
        let result = World::shade_hit(&world, &comps, MAX_RAY_RECURSION_DEPTH);

        // Assert
        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), result);
    }

    #[test]
    fn given_a_ray_that_misses_when_calling_color_at_should_return_black() {
        // Arrange
        let world = World::default();

        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let result = world.color_at(&ray, MAX_RAY_RECURSION_DEPTH);

        // Assert
        assert_eq!(Color::black(), result);
    }

    #[test]
    fn given_a_ray_that_hits_when_calling_color_at_should_return_correct_color_value() {
        // Arrange
        let world = World::default();

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let result = world.color_at(&ray, MAX_RAY_RECURSION_DEPTH);

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
            0.0,
            1.0,
        ));

        let outer = Sphere::new(Arc::new(Matrix::identity(4)), material.clone(), true);
        let inner = Sphere::new(
            Arc::new(Matrix::scaling(0.5, 0.5, 0.5)),
            material.clone(),
            true,
        );

        let objects: Vec<Arc<dyn Shape>> = vec![Arc::new(outer), Arc::new(inner)];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));

        // Act
        let result = world.color_at(&ray, MAX_RAY_RECURSION_DEPTH);

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
            true,
        ));

        let objects: Vec<Arc<dyn Shape>> = vec![s1.clone(), s2.clone()];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let intersections = vec![Intersection::new(4.0, s2.clone())];

        let comps = World::prepare_computations(0, &ray, &intersections);

        // Act
        let result = world.shade_hit(&comps, MAX_RAY_RECURSION_DEPTH);

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
            true,
        ));

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let intersections = vec![Intersection::new(5.0, shape.clone())];

        // Act
        let comps = World::prepare_computations(0, &ray, &intersections);

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

        let intersections = vec![Intersection::new(SQRT_2, shape.clone())];

        // Act
        let comps = World::prepare_computations(0, &ray, &intersections);

        // Assert
        assert_eq!(
            Tuple::vector(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0),
            comps.reflectv
        );
    }

    #[test]
    fn given_a_ray_strikes_a_non_reflective_surface_when_reflecting_color_should_return_black() {
        // Arrange
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
                0.0,
                1.0,
            )),
            true,
        ));
        let inner = Arc::new(Sphere::new(
            Arc::new(Matrix::scaling(0.5, 0.5, 0.5)),
            Arc::new(Phong::default()),
            true,
        ));

        let objects: Vec<Arc<dyn Shape>> = vec![outer.clone(), inner.clone()];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

        let intersections = vec![Intersection::new(1.0, inner.clone())];

        // Act
        let comps = World::prepare_computations(0, &ray, &intersections);
        let color = world.reflected_color(&comps, MAX_RAY_RECURSION_DEPTH);

        // Assert
        assert_eq!(Color::black(), color);
    }

    #[test]
    fn given_a_ray_strikes_a_reflective_surface_when_reflecting_color_should_return_correct_value()
    {
        // Arrange
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());
        let material: Arc<dyn Material> = Arc::new(Phong::new(
            Box::new(Solid::default()),
            0.1,
            0.9,
            0.9,
            200.0,
            0.5,
            0.0,
            1.0,
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
                0.0,
                1.0,
            )),
            true,
        ));
        let inner = Arc::new(Sphere::new(
            Arc::new(Matrix::scaling(0.5, 0.5, 0.5)),
            Arc::new(Phong::default()),
            true,
        ));

        let plane = Arc::new(Plane::new(
            Arc::new(Matrix::translation(0.0, -1.0, 0.0)),
            material.clone(),
            true,
        ));

        let objects: Vec<Arc<dyn Shape>> = vec![outer.clone(), inner.clone(), plane.clone()];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );

        let intersections = vec![Intersection::new(SQRT_2, plane.clone())];

        // Act
        let comps = World::prepare_computations(0, &ray, &intersections);
        let color = world.reflected_color(&comps, MAX_RAY_RECURSION_DEPTH);

        // Assert
        assert_eq!(Color::new(0.19033, 0.23791, 0.14274), color);
    }

    #[test]
    fn given_a_ray_strikes_a_reflective_surface_when_shading_reflected_hit_should_return_correct_value(
    ) {
        // Arrange
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());
        let material: Arc<dyn Material> = Arc::new(Phong::new(
            Box::new(Solid::default()),
            0.1,
            0.9,
            0.9,
            200.0,
            0.5,
            0.0,
            1.0,
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
                0.0,
                1.0,
            )),
            true,
        ));

        let inner = Arc::new(Sphere::new(
            Arc::new(Matrix::scaling(0.5, 0.5, 0.5)),
            Arc::new(Phong::default()),
            true,
        ));

        let plane = Arc::new(Plane::new(
            Arc::new(Matrix::translation(0.0, -1.0, 0.0)),
            material.clone(),
            true,
        ));

        let objects: Vec<Arc<dyn Shape>> = vec![outer.clone(), inner.clone(), plane.clone()];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );

        let intersections = vec![Intersection::new(SQRT_2, plane.clone())];

        // Act
        let comps = World::prepare_computations(0, &ray, &intersections);
        let color = world.shade_hit(&comps, MAX_RAY_RECURSION_DEPTH);

        // Assert
        assert_eq!(Color::new(0.87675, 0.92434, 0.82917), color);
    }

    #[test]
    pub fn given_a_ray_strikes_a_reflective_surface_when_reflection_is_not_allowed_to_recurse_further_should_return_black(
    ) {
        // Arrange
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());
        let material: Arc<dyn Material> = Arc::new(Phong::new(
            Box::new(Solid::default()),
            0.1,
            0.9,
            0.9,
            200.0,
            0.5,
            0.0,
            1.0,
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
                0.0,
                1.0,
            )),
            true,
        ));
        let inner = Arc::new(Sphere::new(
            Arc::new(Matrix::scaling(0.5, 0.5, 0.5)),
            Arc::new(Phong::default()),
            true,
        ));

        let plane = Arc::new(Plane::new(
            Arc::new(Matrix::translation(0.0, -1.0, 0.0)),
            material.clone(),
            true,
        ));

        let objects: Vec<Arc<dyn Shape>> = vec![outer.clone(), inner.clone(), plane.clone()];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );

        let intersections = vec![Intersection::new(SQRT_2, plane.clone())];

        // Act
        let comps = World::prepare_computations(0, &ray, &intersections);
        let color = world.reflected_color(&comps, 0);

        // Assert
        assert_eq!(Color::black(), color);
    }

    #[test]
    pub fn given_nested_glass_spheres_when_calculating_entry_and_exit_refractive_indices_should_return_correct_values(
    ) {
        // Arrange
        let outer = Arc::new(Sphere::new(
            Arc::new(Matrix::scaling(2.0, 2.0, 2.0)),
            Arc::new(Phong::new(
                Box::new(Solid::default()),
                0.1,
                0.9,
                0.9,
                200.0,
                0.0,
                1.0,
                1.5,
            )),
            true,
        ));

        let inner_left = Arc::new(Sphere::new(
            Arc::new(Matrix::translation(0.0, 0.0, -0.25)),
            Arc::new(Phong::new(
                Box::new(Solid::default()),
                0.1,
                0.9,
                0.9,
                200.0,
                0.0,
                1.0,
                2.0,
            )),
            true,
        ));

        let inner_right = Arc::new(Sphere::new(
            Arc::new(Matrix::translation(0.0, 0.0, 0.25)),
            Arc::new(Phong::new(
                Box::new(Solid::default()),
                0.1,
                0.9,
                0.9,
                200.0,
                0.0,
                1.0,
                2.5,
            )),
            true,
        ));

        let ray = Ray::new(Tuple::point(0.0, 0.0, -4.0), Tuple::vector(0.0, 0.0, 1.0));

        let intersections = vec![
            Intersection::new(2.0, outer.clone()),
            Intersection::new(2.75, inner_left.clone()),
            Intersection::new(3.25, inner_right.clone()),
            Intersection::new(4.75, inner_left.clone()),
            Intersection::new(5.25, inner_right.clone()),
            Intersection::new(6.0, outer.clone()),
        ];

        // Act
        let mut results: Vec<Computations> = vec![];

        for i in 0..intersections.len() {
            results.push(World::prepare_computations(i, &ray, &intersections));
        }

        // Assert
        let expected_results: Vec<(f64, f64)> = vec![
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];

        for i in 0..results.len() {
            let expected = expected_results[i];
            let actual = &results[i];

            assert_eq!(expected.0, actual.n1);
            assert_eq!(expected.1, actual.n2);
        }
    }

    #[test]
    fn given_a_glass_sphere_when_preparing_computations_should_expect_under_point_to_lie_beneath_surface(
    ) {
        // Arrange
        let shape = Arc::new(Sphere::new(
            Arc::new(Matrix::translation(0.0, 0.0, 1.0)),
            Arc::new(Phong::new(
                Box::new(Solid::default()),
                0.1,
                0.9,
                0.9,
                200.0,
                0.0,
                1.0,
                1.5,
            )),
            true,
        ));

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let intersections = vec![Intersection::new(5.0, shape.clone())];

        // Act
        let comps = World::prepare_computations(0, &ray, &intersections);

        // Assert
        assert_eq!(true, comps.under_point.z > EPSILON / 2.0);
        assert_eq!(true, comps.point.z < comps.under_point.z);
    }

    #[test]
    fn given_an_opaque_surface_when_calculating_refracted_color_should_return_black() {
        // Arrange
        let world = World::default();
        let shape = world.objects.first().unwrap().clone();

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let intersections = vec![
            Intersection::new(4.0, shape.clone()),
            Intersection::new(6.0, shape.clone()),
        ];

        // Act
        let comps = World::prepare_computations(0, &ray, &intersections);
        let color = world.refracted_color(&comps, MAX_RAY_RECURSION_DEPTH);

        // Assert
        assert_eq!(Color::black(), color);
    }

    #[test]
    fn given_a_glass_sphere_when_calculating_refracted_color_at_max_recursion_depth_should_return_black(
    ) {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());

        let outer = Arc::new(Sphere::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::new(
                Box::new(Solid::default()),
                0.1,
                0.9,
                0.9,
                200.0,
                0.0,
                1.0,
                1.5,
            )),
            true,
        ));

        let inner = Arc::new(Sphere::new(
            Arc::new(Matrix::scaling(0.5, 0.5, 0.5)),
            Arc::new(Phong::default()),
            true,
        ));

        let objects: Vec<Arc<dyn Shape>> = vec![outer.clone(), inner.clone()];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let intersections = vec![
            Intersection::new(4.0, outer.clone()),
            Intersection::new(6.0, outer.clone()),
        ];

        // Act
        let comps = World::prepare_computations(0, &ray, &intersections);
        let color = world.refracted_color(&comps, 0);

        // Assert
        assert_eq!(Color::black(), color);
    }

    #[test]
    fn given_a_glass_sphere_when_calculating_refracted_color_under_total_internal_reflection_should_return_black(
    ) {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());

        let outer = Arc::new(Sphere::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::new(
                Box::new(Solid::default()),
                0.1,
                0.9,
                0.9,
                200.0,
                0.0,
                1.0,
                1.5,
            )),
            true,
        ));

        let inner = Arc::new(Sphere::new(
            Arc::new(Matrix::scaling(0.5, 0.5, 0.5)),
            Arc::new(Phong::default()),
            true,
        ));

        let objects: Vec<Arc<dyn Shape>> = vec![outer.clone(), inner.clone()];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(
            Tuple::point(0.0, 0.0, SQRT_2 / 2.0),
            Tuple::vector(0.0, 1.0, 0.0),
        );

        let intersections = vec![
            Intersection::new(-SQRT_2 / 2.0, outer.clone()),
            Intersection::new(SQRT_2 / 2.0, outer.clone()),
        ];

        // Act
        let comps = World::prepare_computations(1, &ray, &intersections);
        let color = world.refracted_color(&comps, MAX_RAY_RECURSION_DEPTH);

        // Assert
        assert_eq!(Color::black(), color);
    }

    #[test]
    fn given_a_glass_sphere_when_calculating_refracted_color_should_return_correct_color_with_refracted_ray(
    ) {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());

        let outer = Arc::new(Sphere::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::new(
                Box::new(TestPattern::default()),
                1.0,
                0.9,
                0.9,
                200.0,
                0.0,
                0.0,
                1.0,
            )),
            true,
        ));

        let inner = Arc::new(Sphere::new(
            Arc::new(Matrix::scaling(0.5, 0.5, 0.5)),
            Arc::new(Phong::new(
                Box::new(Solid::default()),
                0.1,
                0.9,
                0.9,
                200.0,
                0.0,
                1.0,
                1.5,
            )),
            true,
        ));

        let objects: Vec<Arc<dyn Shape>> = vec![outer.clone(), inner.clone()];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.1), Tuple::vector(0.0, 1.0, 0.0));

        let intersections = vec![
            Intersection::new(-0.9899, outer.clone()),
            Intersection::new(-0.4899, inner.clone()),
            Intersection::new(0.4899, inner.clone()),
            Intersection::new(0.9899, outer.clone()),
        ];

        // Act
        let comps = World::prepare_computations(2, &ray, &intersections);
        let color = world.refracted_color(&comps, MAX_RAY_RECURSION_DEPTH);

        // Assert
        assert_eq!(Color::new(0.0, 0.998874, 0.047218), color);
    }

    #[test]
    fn given_a_glass_plane_below_default_world_when_shading_hit_should_return_correct_color() {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());

        let outer = Arc::new(Sphere::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::new(
                Box::new(Solid::new(Color::new(0.8, 1.0, 0.6))),
                0.1,
                0.7,
                0.2,
                200.0,
                0.0,
                0.0,
                1.0,
            )),
            true,
        ));

        let inner = Arc::new(Sphere::new(
            Arc::new(Matrix::scaling(0.5, 0.5, 0.5)),
            Arc::new(Phong::default()),
            true,
        ));

        let floor = Arc::new(Plane::new(
            Arc::new(Matrix::translation(0.0, -1.0, 0.0)),
            Arc::new(Phong::new(
                Box::new(Solid::new(Color::white())),
                0.1,
                0.9,
                0.9,
                200.0,
                0.0,
                0.5,
                1.5,
            )),
            true,
        ));

        let ball = Arc::new(Sphere::new(
            Arc::new(Matrix::translation(0.0, -3.5, -0.5)),
            Arc::new(Phong::new(
                Box::new(Solid::new(Color::red())),
                0.5,
                0.9,
                0.9,
                200.0,
                0.0,
                0.0,
                1.0,
            )),
            true,
        ));

        let objects: Vec<Arc<dyn Shape>> =
            vec![outer.clone(), inner.clone(), floor.clone(), ball.clone()];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );

        let intersections = vec![Intersection::new(SQRT_2, floor.clone())];

        // Act
        let comps = World::prepare_computations(0, &ray, &intersections);
        let color = world.shade_hit(&comps, MAX_RAY_RECURSION_DEPTH);

        // Assert
        assert_eq!(Color::new(0.93642, 0.68642, 0.68642), color);
    }

    #[test]
    pub fn given_a_glass_sphere_when_calculating_schlick_approximation_under_total_internal_reflection_should_return_max_value(
    ) {
        // Arrange
        let shape = Arc::new(Sphere::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::new(
                Box::new(Solid::default()),
                0.1,
                0.9,
                0.9,
                200.0,
                0.0,
                1.0,
                1.5,
            )),
            true,
        ));

        let ray = Ray::new(
            Tuple::point(0.0, 0.0, SQRT_2 / 2.0),
            Tuple::vector(0.0, 1.0, 0.0),
        );

        let intersections = vec![
            Intersection::new(-SQRT_2 / 2.0, shape.clone()),
            Intersection::new(SQRT_2 / 2.0, shape.clone()),
        ];

        // Act
        let comps = World::prepare_computations(1, &ray, &intersections);
        let result = World::schlick(&comps);

        // Assert
        assert_eq!(1.0, result);
    }

    #[test]
    pub fn given_a_glass_sphere_when_calculating_schlick_approximation_with_perpendicular_viewing_angle_should_return_small_value(
    ) {
        // Arrange
        let shape = Arc::new(Sphere::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::new(
                Box::new(Solid::default()),
                0.1,
                0.9,
                0.9,
                200.0,
                0.0,
                1.0,
                1.5,
            )),
            true,
        ));

        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));

        let intersections = vec![
            Intersection::new(-1.0, shape.clone()),
            Intersection::new(1.0, shape.clone()),
        ];

        // Act
        let comps = World::prepare_computations(1, &ray, &intersections);
        let result = World::schlick(&comps);

        // Assert
        assert_eq!(true, (result - 0.0597) < EPSILON);
    }

    #[test]
    pub fn given_a_glass_sphere_when_calculating_schlick_approximation_with_small_angle_and_n2_gt_n1_should_return_large_value(
    ) {
        // Arrange
        let shape = Arc::new(Sphere::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::new(
                Box::new(Solid::default()),
                0.1,
                0.9,
                0.9,
                200.0,
                0.0,
                1.0,
                1.5,
            )),
            true,
        ));

        let ray = Ray::new(Tuple::point(0.0, 0.99, -2.0), Tuple::vector(0.0, 0.0, 1.0));

        let intersections = vec![Intersection::new(1.8589, shape.clone())];

        // Act
        let comps = World::prepare_computations(0, &ray, &intersections);
        let result = World::schlick(&comps);

        // Assert
        assert_eq!(true, (result - 0.48873) < EPSILON);
    }

    #[test]
    fn given_a_glass_plane_that_is_reflective_and_transparent_below_default_world_when_shading_hit_should_return_correct_color(
    ) {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());

        let outer = Arc::new(Sphere::new(
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::new(
                Box::new(Solid::new(Color::new(0.8, 1.0, 0.6))),
                0.1,
                0.7,
                0.2,
                200.0,
                0.0,
                0.0,
                1.0,
            )),
            true,
        ));

        let inner = Arc::new(Sphere::new(
            Arc::new(Matrix::scaling(0.5, 0.5, 0.5)),
            Arc::new(Phong::default()),
            true,
        ));

        let floor = Arc::new(Plane::new(
            Arc::new(Matrix::translation(0.0, -1.0, 0.0)),
            Arc::new(Phong::new(
                Box::new(Solid::new(Color::white())),
                0.1,
                0.9,
                0.9,
                200.0,
                0.5,
                0.5,
                1.5,
            )),
            true,
        ));

        let ball = Arc::new(Sphere::new(
            Arc::new(Matrix::translation(0.0, -3.5, -0.5)),
            Arc::new(Phong::new(
                Box::new(Solid::new(Color::red())),
                0.5,
                0.9,
                0.9,
                200.0,
                0.0,
                0.0,
                1.0,
            )),
            true,
        ));

        let objects: Vec<Arc<dyn Shape>> =
            vec![outer.clone(), inner.clone(), floor.clone(), ball.clone()];
        let lights = vec![Arc::new(light)];

        let world = World::new(objects, lights);

        let ray = Ray::new(
            Tuple::point(0.0, 0.0, -3.0),
            Tuple::vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );

        let intersections = vec![Intersection::new(SQRT_2, floor.clone())];

        // Act
        let comps = World::prepare_computations(0, &ray, &intersections);
        let color = world.shade_hit(&comps, MAX_RAY_RECURSION_DEPTH);

        // Assert
        assert_eq!(Color::new(0.93391, 0.69643, 0.69243), color);
    }

    #[test]
    fn given_a_smooth_triangle_when_preparing_the_normal_should_calculate_the_correct_result() {
        // Arrange
        let triangle = Arc::new(SmoothTriangle::default(
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::point(-1.0, 0.0, 0.0),
            Tuple::point(1.0, 0.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
            Tuple::vector(-1.0, 0.0, 0.0),
            Tuple::vector(1.0, 0.0, 0.0),
        ));

        let intersection = Intersection::new_with_uv(1.0, triangle.clone(), 0.45, 0.25);

        let ray = Ray::new(Tuple::point(-0.2, 0.3, -2.0), Tuple::vector(0.0, 0.0, 1.0));

        // Act
        let comps = World::prepare_computations(0, &ray, &vec![intersection]);

        // Assert
        assert_eq!(Tuple::vector(-0.5547, 0.83205, 0.0), comps.normalv);
    }
}
