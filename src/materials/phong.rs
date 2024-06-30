use super::material::Material;

use crate::EPSILON;

use crate::tuples::{color::Color, pointlight::PointLight, tuple::Tuple};

#[derive(Debug, Clone)]
pub struct Phong {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Phong {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Phong {
        return Phong {
            color,
            ambient,  // Light reflected from other objects in the scene
            diffuse,  // Light reflected from a matte surface
            specular, // Relection of the light source itself
            shininess,
        };
    }

    pub fn default() -> Phong {
        return Phong::new(Color::white(), 0.1, 0.9, 0.9, 200.0);
    }
}

impl Material for Phong {
    fn lighting(&self, light: &PointLight, point: &Tuple, eyev: &Tuple, normalv: &Tuple) -> Color {
        // Combine the surface color with the light's color/intensity
        let effective_color = self.color * light.intensity;

        // Find the direction to the light source
        let light_vector = (light.position - *point).normalize();

        // Compute the ambient contribution
        let ambient = effective_color * self.ambient;

        let diffuse: Color;
        let specular: Color;

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = Tuple::dot(&light_vector, &normalv);
        if light_dot_normal < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            // Compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflect_vector = Tuple::reflect(&(-light_vector), normalv);
            let reflect_dot_eye = Tuple::dot(&reflect_vector, &eyev);

            if reflect_dot_eye <= 0.0 {
                specular = Color::black();
            } else {
                // Compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        // Add the three contributions together to get the final shading
        return ambient + diffuse + specular;
    }
}

impl PartialEq for Phong {
    fn eq(&self, other: &Self) -> bool {
        if self.color != other.color {
            return false;
        } else if (self.ambient - other.ambient).abs() > EPSILON {
            return false;
        } else if (self.diffuse - other.diffuse).abs() > EPSILON {
            return false;
        } else if (self.specular - other.specular).abs() > EPSILON {
            return false;
        } else if (self.shininess - other.shininess).abs() > EPSILON {
            return false;
        } else {
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts;

    use super::*;

    #[test]
    fn given_default_material_when_creating_it_should_expect_values_to_be_set_correctly() {
        let color = Color::white();
        let ambient = 0.1;
        let diffuse = 0.9;
        let specular = 0.9;
        let shininess = 200.0;

        let result = Phong::default();

        assert_eq!(color, result.color);
        assert_eq!(ambient, result.ambient);
        assert_eq!(diffuse, result.diffuse);
        assert_eq!(specular, result.specular);
        assert_eq!(shininess, result.shininess);
    }

    #[test]
    fn given_default_material_when_eye_between_light_and_surface_should_calculate_resulting_color_correctly(
    ) {
        let default = Phong::default();
        let position = Tuple::origin();

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::white());

        let result = default.lighting(&light, &position, &eyev, &normalv);
        let expected = Color::new(1.9, 1.9, 1.9);

        assert_eq!(expected, result);
    }

    #[test]
    fn given_default_material_when_eye_between_light_and_surface_with_eye_offset_by_45_degrees_should_calculate_resulting_color_correctly(
    ) {
        let default = Phong::default();
        let position = Tuple::origin();

        let eyev = Tuple::vector(0.0, consts::SQRT_2 / 2.0, -consts::SQRT_2 / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::white());

        let result = default.lighting(&light, &position, &eyev, &normalv);
        let expected = Color::new(1.0, 1.0, 1.0);

        assert_eq!(expected, result);
    }

    #[test]
    fn given_default_material_when_eye_opposite_surface_with_light_offset_by_45_degrees_should_calculate_resulting_color_correctly(
    ) {
        let default = Phong::default();
        let position = Tuple::origin();

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::white());

        let result = default.lighting(&light, &position, &eyev, &normalv);
        let expected = Color::new(0.7364, 0.7364, 0.7364);

        assert_eq!(expected, result);
    }

    #[test]
    fn given_default_material_when_eye_in_path_of_reflection_vector_should_calculate_resulting_color_correctly(
    ) {
        let default = Phong::default();
        let position = Tuple::origin();

        let eyev = Tuple::vector(0.0, -consts::SQRT_2 / 2.0, -consts::SQRT_2 / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::white());

        let result = default.lighting(&light, &position, &eyev, &normalv);
        let expected = Color::new(1.6364, 1.6364, 1.6364);

        assert_eq!(expected, result);
    }

    #[test]
    fn given_default_material_when_lighting_behind_the_surface_should_calculate_resulting_color_correctly(
    ) {
        let default = Phong::default();
        let position = Tuple::origin();

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::white());

        let result = default.lighting(&light, &position, &eyev, &normalv);
        let expected = Color::new(0.1, 0.1, 0.1);

        assert_eq!(expected, result);
    }
}
