use crate::Ray;
use crate::Vector;
use crate::Point;
use crate::Matrix;

// Note the camera's canvas will always be 1 unit in front of the camera
pub struct Camera {
    pub hsize: usize,
    half_height: f64,
    pub vsize: usize,
    half_width: f64,
    field_of_view: f64,
    transform: Matrix,
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
        }
        else {
            half_height = half_view;
            half_width = half_view * aspect;
        }

        let pixel_size = (half_width * 2.0) / (hsize as f64);

        return Camera {
            hsize,
            half_height,
            vsize,
            half_width,
            field_of_view,
            transform,
            pixel_size
        }
    }

    pub fn default() -> Camera {
        return Camera::new(250, 250, std::f64::consts::PI / 2.0, Matrix::identity(4));
    }

    // Generate a matrix that moves the camera around the scene, so instead of a fixed screen you cast
    // rays at (and the objects appear as coloured shadows). Now you can move that around and take 
    // pictures from different points.
    // from: Where the camera is in the scene
    // to: The point at which the camera is looking in the scene
    // up: Which direction is up
    pub fn view_transform(from: Point, to: Point, up: Vector) -> Matrix {
        // Compute the forward vector by: to - from, then normalizing the result
        let forward = (to - from).normalize();
        let upn = up.normalize();

        // Compute the left vector by: forward x (up normalized)
        let left = Vector::cross(forward, upn);

        // Compute the true up by: left x forward. This allows the original up to be only approximately up
        // which makes framing scenes alot easier, since the precise calc isn't needed.
        let true_up = Vector::cross(left, forward);

        let orientation = Matrix::from_rows(
            &vec![
                vec![left.x, left.y, left.z, 0.0],
                vec![true_up.x, true_up.y, true_up.z, 0.0],
                vec![-forward.x, -forward.y, -forward.z, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ]
        );
        
        // Append the translation that moves the scene into place befor orienting it.
        let result = orientation * Matrix::translation(-from.x, -from.y, -from.z);

        return result.unwrap();
    }

    // Generates a ray that starts at the camera and hits the X, Y pixel on the
    // canvas in front of the camera
    pub fn ray_for_pixel(&self, px: f64, py: f64) -> Ray {
        // The offset from the edge of the canvas to the pixel's center
        let x_offset = (px + 0.5) * self.pixel_size;
        let y_offset = (py + 0.5) * self.pixel_size;

        // The untransformed coordinates of the pixel in world space
        // (remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        // Using the camera matrix, transform the canvas point and the origin
        // and then compute the ray's direction vector.
        // (remember that the canvas is at z = -1)
        let pixel = (self.transform.inverse() * Point::new(world_x, world_y, -1.0)).unwrap();
        let origin = (self.transform.inverse() * Point::new(0.0, 0.0, 0.0)).unwrap();
        let direction = (pixel - origin).normalize();

        return Ray::new(origin, direction);
    }
}