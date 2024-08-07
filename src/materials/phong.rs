use super::material::Material;
use std::sync::Arc;

use crate::geometry::shape::Shape;
use crate::patterns::pattern::Pattern;
use crate::patterns::solid::Solid;

use crate::tuples::{color::Color, point_light::PointLight, tuple::Tuple};

pub struct Phong {
    pattern: Box<dyn Pattern>,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
    transparency: f64,
    refractive_index: f64,
}

impl Phong {
    pub fn new(
        pattern: Box<dyn Pattern>,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        reflective: f64,
        transparency: f64,
        refractive_index: f64,
    ) -> Phong {
        Phong {
            pattern,
            ambient,  // Light reflected from other objects in the scene
            diffuse,  // Light reflected from a matte surface
            specular, // Reflection of the light source itself
            shininess,
            reflective,
            transparency,
            refractive_index,
        }
    }

    pub fn default() -> Phong {
        Phong::new(
            Box::new(Solid::default()),
            0.1,
            0.9,
            0.9,
            200.0,
            0.0,
            0.0,
            1.0,
        )
    }
}

impl Material for Phong {
    fn lighting(
        &self,
        object: Arc<dyn Shape>,
        light: PointLight,
        world_point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        let color = self.pattern.pattern_at_shape(object, world_point);

        // Combine the surface color with the light's color/intensity
        let effective_color = color * light.intensity;

        // Find the direction to the light source
        let light_vector = (light.position - world_point).normalize();

        // Compute the ambient contribution
        let ambient = effective_color * self.ambient;

        // Diffuse and specular both have a dependency on the light source
        // so if the point is in shadow only use the ambient component.
        if in_shadow {
            return ambient;
        }

        let diffuse: Color;
        let specular: Color;

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = Tuple::dot(light_vector, normalv);
        if light_dot_normal < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            // Compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflect_vector = Tuple::reflect(-light_vector, normalv);
            let reflect_dot_eye = Tuple::dot(reflect_vector, eyev);

            if reflect_dot_eye <= 0.0 {
                specular = Color::black();
            } else {
                // Compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        // Add the three contributions together to get the final shading
        ambient + diffuse + specular
    }

    fn reflective(&self) -> f64 {
        self.reflective
    }

    fn transparency(&self) -> f64 {
        self.transparency
    }

    fn refractive_index(&self) -> f64 {
        self.refractive_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::sphere::Sphere;
    use std::f64::consts;

    #[test]
    fn given_default_material_when_creating_it_should_expect_values_to_be_set_correctly() {
        // Arrange
        let ambient = 0.1;
        let diffuse = 0.9;
        let specular = 0.9;
        let shininess = 200.0;
        let reflective = 0.0;

        // Act
        let result = Phong::default();

        // Assert
        assert_eq!(ambient, result.ambient);
        assert_eq!(diffuse, result.diffuse);
        assert_eq!(specular, result.specular);
        assert_eq!(shininess, result.shininess);
        assert_eq!(reflective, result.reflective);
    }

    #[test]
    fn given_default_material_when_eye_between_light_and_surface_should_calculate_resulting_color_correctly(
    ) {
        // Arrange
        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());
        let default = Phong::default();
        let position = Tuple::origin();

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::white());

        // Act
        let result = default.lighting(shape, light, position, eyev, normalv, false);
        let expected = Color::new(1.9, 1.9, 1.9);

        // Assert
        assert_eq!(expected, result);
    }

    #[test]
    fn given_default_material_when_eye_between_light_and_surface_with_eye_offset_by_45_degrees_should_calculate_resulting_color_correctly(
    ) {
        // Arrange
        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());
        let default = Phong::default();
        let position = Tuple::origin();

        let eyev = Tuple::vector(0.0, consts::SQRT_2 / 2.0, -consts::SQRT_2 / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::white());

        // Act
        let result = default.lighting(shape, light, position, eyev, normalv, false);

        // Assert
        let expected = Color::new(1.0, 1.0, 1.0);

        assert_eq!(expected, result);
    }

    #[test]
    fn given_default_material_when_eye_opposite_surface_with_light_offset_by_45_degrees_should_calculate_resulting_color_correctly(
    ) {
        // Arrange
        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());
        let default = Phong::default();
        let position = Tuple::origin();

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::white());

        // Act
        let result = default.lighting(shape, light, position, eyev, normalv, false);

        // Assert
        let expected = Color::new(0.7364, 0.7364, 0.7364);

        assert_eq!(expected, result);
    }

    #[test]
    fn given_default_material_when_eye_in_path_of_reflection_vector_should_calculate_resulting_color_correctly(
    ) {
        // Arrange
        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());
        let default = Phong::default();
        let position = Tuple::origin();

        let eyev = Tuple::vector(0.0, -consts::SQRT_2 / 2.0, -consts::SQRT_2 / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::white());

        // Act
        let result = default.lighting(shape, light, position, eyev, normalv, false);

        // Assert
        let expected = Color::new(1.6364, 1.6364, 1.6364);

        assert_eq!(expected, result);
    }

    #[test]
    fn given_default_material_when_lighting_behind_the_surface_should_calculate_resulting_color_correctly(
    ) {
        // Arrange
        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());
        let default = Phong::default();
        let position = Tuple::origin();

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::white());

        // Act
        let result = default.lighting(shape, light, position, eyev, normalv, false);

        // Assert
        let expected = Color::new(0.1, 0.1, 0.1);

        assert_eq!(expected, result);
    }

    #[test]
    fn given_default_material_when_lighting_surface_in_shadow_should_calculate_resulting_color_correctly(
    ) {
        // Arrange
        let shape: Arc<dyn Shape> = Arc::new(Sphere::unit());
        let default = Phong::default();
        let position = Tuple::origin();

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::white());

        // Act
        let result = default.lighting(shape, light, position, eyev, normalv, true);

        // Assert
        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }
}
