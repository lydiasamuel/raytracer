use crate::matrices::matrix::Matrix;

// Note the camera's canvas will always be 1 unit in front of the camera
pub struct Camera {
    hsize: usize, // Horizontal size (in pixels) of the canvas that the picture will be rendered to
    vsize: usize, // Canvas's vertical size (in pixels)
    field_of_view: f64, // Angle that describes how much the camera can see
    transform: Matrix, // Matrix that describes how the world is oriented relative to the camera
    half_height: f64,
    half_width: f64,
    pixel_size: f64
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64, transform: Matrix) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = (hsize as f64) / (vsize as f64);

        let mut half_width = 0.0;
        let mut half_height = 0.0;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_height = half_view;
            half_width = half_view * aspect;
        }

        let pixel_size = (half_width * 2.0) / (hsize as f64);

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform,
            half_height,
            half_width,
            pixel_size
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::EPSILON;
    use crate::matrices::matrix::Matrix;
    use crate::scene::camera::Camera;

    #[test]
    fn given_normal_camera_values_when_creating_a_new_camera_with_a_horizontal_canvas_should_calculate_pixel_value_correctly() {
        // Arrange
        // Act
        let result = Camera::new(200, 125, PI / 2.0, Matrix::identity(4));

        // Assert
        assert_eq!(true,  (0.01 - result.pixel_size) < EPSILON);
    }

    #[test]
    fn given_normal_camera_values_when_creating_a_new_camera_with_a_vertical_canvas_should_calculate_pixel_value_correctly() {
        // Arrange
        // Act
        let result = Camera::new(125, 200, PI / 2.0, Matrix::identity(4));

        // Assert
        assert_eq!(true, (0.01 - result.pixel_size) < EPSILON);
    }
}
