use std::{error::Error, rc::Rc};

use geometry::{shape::Shape, sphere::Sphere};
use tuples::{intersection::Intersection, ray::Ray, tuple::Tuple};

use crate::{window::canvas::Canvas, tuples::color::Color};

static EPSILON: f64 = 0.00001;

pub mod geometry;
pub mod matrices;
pub mod tuples;
pub mod window;

pub struct Config {
    pub file_path: String,
    pub width: usize,
    pub height: usize
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();
        let width = args[2].clone().parse().unwrap();
        let height = args[3].clone().parse().unwrap();

        return Ok(Config {
            file_path,
            width,
            height
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let canvas_pixels = config.width;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::red();
    
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let shape = Rc::new(Sphere::unit());

    for y in 0..canvas_pixels {
        // Compute the world y coordinate (top = +half, bottom = -half)
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            // Compute the world x coordinate (left = -half, right = half)
            let world_x = -half + pixel_size * x as f64;

            // Describe the point on the wall that the ray will target
            let position = Tuple::point(world_x, world_y, wall_z);

            let world_ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let intersections = shape.clone().intersect(&world_ray);

            if Intersection::hit(&intersections).is_some() {
                canvas.write_pixel(x, y, color)?;
            }
        }
    }

    canvas.write_to_file(config.file_path)?;

    return Ok(());
}
