use crate::matrices::matrix::Matrix;
use crate::tuples::ray::Ray;
use crate::tuples::tuple::Tuple;

// Note the camera's canvas will always be 1 unit in front of the camera
pub struct Camera {
    hsize: usize, // Horizontal size (in pixels) of the canvas that the picture will be rendered to
    vsize: usize, // Canvas's vertical size (in pixels)
    field_of_view: f64, // Angle that describes how much the camera can see
    transform: Matrix, // Matrix that describes how the world is oriented relative to the camera
    half_height: f64, // Half the height of the canvas
    half_width: f64, // Half the width of the canvas
    pixel_size: f64, // Size of a single pixel on the canvas
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64, transform: Matrix) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = (hsize as f64) / (vsize as f64);

        let half_width;
        let half_height;

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
            pixel_size,
        }
    }

    // Generates a ray that starts at the camera and hits the X, Y pixel on the
    // canvas in front of the camera
    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // The offset from the edge of the canvas to the pixel's center
        let x_offset = (px as f64 + 0.5) * self.pixel_size;
        let y_offset = (py as f64 + 0.5) * self.pixel_size;

        // The untransformed coordinates of the pixel in world space
        // (remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        // Using the camera matrix, transform the canvas point and the origin
        // and then compute the ray's direction vector.
        // (remember that the canvas is at z = -1)
        let inverse = self.transform.inverse().unwrap();
        let pixel = (inverse.clone() * Tuple::point(world_x, world_y, -1.0)).unwrap();
        let origin = (inverse * Tuple::point(0.0, 0.0, 0.0)).unwrap();
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn height(&self) -> usize {
        self.hsize
    }

    pub fn width(&self) -> usize {
        self.vsize
    }
}

#[cfg(test)]
mod tests {
    use crate::matrices::matrix::Matrix;
    use crate::scene::camera::Camera;
    use crate::tuples::tuple::Tuple;
    use crate::EPSILON;
    use std::f64::consts::{PI, SQRT_2};

    #[test]
    fn given_normal_camera_values_when_creating_a_new_camera_with_a_horizontal_canvas_should_calculate_pixel_value_correctly(
    ) {
        // Arrange
        // Act
        let result = Camera::new(200, 125, PI / 2.0, Matrix::identity(4));

        // Assert
        assert_eq!(true, (0.01 - result.pixel_size) < EPSILON);
    }

    #[test]
    fn given_normal_camera_values_when_creating_a_new_camera_with_a_vertical_canvas_should_calculate_pixel_value_correctly(
    ) {
        // Arrange
        // Act
        let result = Camera::new(125, 200, PI / 2.0, Matrix::identity(4));

        // Assert
        assert_eq!(true, (0.01 - result.pixel_size) < EPSILON);
    }

    #[test]
    fn given_ray_values_that_pass_through_the_center_of_the_canvas_when_calling_ray_for_pixel_should_correctly_hit_canvas_center(
    ) {
        // Arrange
        let camera = Camera::new(201, 101, PI / 2.0, Matrix::identity(4));

        // Act
        let ray = camera.ray_for_pixel(100, 50);

        // Assert
        assert_eq!(Tuple::point(0.0, 0.0, 0.0), ray.origin());
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), ray.direction())
    }

    #[test]
    fn given_ray_values_that_pass_through_a_corner_of_the_canvas_when_calling_ray_for_pixel_should_correctly_hit_canvas_corner(
    ) {
        // Arrange
        let camera = Camera::new(201, 101, PI / 2.0, Matrix::identity(4));

        // Act
        let ray = camera.ray_for_pixel(0, 0);

        // Assert
        assert_eq!(Tuple::point(0.0, 0.0, 0.0), ray.origin());
        assert_eq!(Tuple::vector(0.66519, 0.33259, -0.66851), ray.direction())
    }

    #[test]
    fn given_ray_values_and_a_transformed_camera_when_calling_ray_for_pixel_should_correctly_hit_canvas_transformed_location(
    ) {
        // Arrange
        let transform = Matrix::rotation_y(PI / 4.0) * Matrix::translation(0.0, -2.0, 5.0);

        let camera = Camera::new(201, 101, PI / 2.0, transform.unwrap());

        // Act
        let ray = camera.ray_for_pixel(100, 50);

        // Assert
        assert_eq!(Tuple::point(0.0, 2.0, -5.0), ray.origin());
        assert_eq!(
            Tuple::vector(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0),
            ray.direction()
        )
    }
}
