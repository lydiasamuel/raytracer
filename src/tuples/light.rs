use super::{color::Color, tuple::Tuple};

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color
    
}

impl Light {
    pub fn new(position: Tuple, intensity: Color) -> Light {
        if !position.is_point() {
            panic!("Error: Light must be created with a point tuple variant.");
        }

        return Light {
            position,
            intensity
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_normal_values_for_a_light_source_when_creating_it_should_expect_values_to_be_set_correctly() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Tuple::origin();

        let result = Light::new(position, intensity);

        assert_eq!(intensity, result.intensity);
        assert_eq!(position, result.position);
    }
}