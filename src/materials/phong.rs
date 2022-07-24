use crate::patterns::pattern::Pattern;
use crate::materials::material::Material;
use crate::tuples::light::PointLight;
use crate::Vector;
use crate::Point;
use crate::Color;

pub struct Phong {
    color: Color,
    pattern: Option<Box<dyn Pattern>>,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64
}

impl Phong {
    pub fn new(color: Color, pattern: Option<Box<dyn Pattern>>, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Phong {
        return Phong {
            color,
            pattern,
            ambient, // Light reflected from other objects in the scene
            diffuse, // Light reflected from a matte surface
            specular, // Relection of the light source itself
            shininess
        };
    }

    pub fn default() -> Phong {
        return Phong::new(
            Color::new(1.0, 1.0, 1.0),
            Option::None, 
            0.1, 
            0.9, 
            0.9, 
            200.0);
    }
}

impl Material for Phong {
    fn lighting(&self, world_point: &Point, object_point: Point, light: &PointLight, eyev: &Vector, normalv: &Vector, in_shadow: bool) -> Color {
        let color = match &self.pattern {
            None => self.color,
            Some(p) => p.pattern_at(&object_point)
        };

        // Combine the surface color with the light's color/intensity
        let effective_color = color * light.intensity;

        // Find the direction to the light source
        let light_vector = (light.position - *world_point).normalize();

        // Compute the ambient contribution
        let ambient = effective_color * self.ambient;

        // Diffuse and specular both have a dependency on the light source
        // so if the point is in shadow only use the ambient component.
        if in_shadow {
            return ambient;
        }

        let mut diffuse: Color = Color::new(0.0, 0.0, 0.0);
        let mut specular: Color = Color::new(0.0, 0.0, 0.0);

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = Vector::dot(light_vector, *normalv);

        if light_dot_normal >= 0.0 {
            // Compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflect_vector = (-light_vector).reflect(*normalv);
            let reflect_dot_eye = Vector::dot(reflect_vector, *eyev);

            if reflect_dot_eye > 0.0 {
                // Compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        return ambient + diffuse + specular;
    }
}