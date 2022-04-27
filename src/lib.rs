mod window;
mod tuples;
mod matrices;
mod geoentity;

use std::rc::Rc;
use std::sync::Mutex;
use std::error::Error;
use std::fs;

use crate::window::mycanvas::MyCanvas;
use crate::tuples::point::Point;
use crate::tuples::vector::Vector;
use crate::tuples::color::Color;
use crate::tuples::ray::Ray;
use crate::matrices::matrix::Matrix;
use crate::geoentity::sphere::Sphere;
use crate::geoentity::intersected::Intersected;
use crate::tuples::intersection::Intersection;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // --snip--
        if args.len() != 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        return Ok(Config {
            filename
        });
    }
}

pub struct IdentityCreator {
    count: Mutex<u64>
}

impl IdentityCreator {
    pub fn new() -> IdentityCreator {
        return IdentityCreator {
            count: Mutex::new(0)
        }
    }

    pub fn get(&self) -> u64 {
        let mut current = self.count.lock().unwrap();

        let result = *current;
        *current = result + 1;

        return result;
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let id_creator = IdentityCreator::new();

    let wall_z = 10.0;
    let wall_size = 7.0;

    let canvas_pixels = 100;
    let pixel_size = wall_size / (canvas_pixels as f64);
    let half = wall_size / 2.0;

    let canvas = MyCanvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);

    let ray_origin = Point::new(0.0, 0.0, -5.0);
    
    let sphere = Rc::new(Sphere::unit(id_creator.get()));

    // for each row of pixels in the canvas
    for y in 0..canvas_pixels {
        // compute the world y coordinate (top = +half, bottom = -half)
        let world_y = half - (pixel_size * (y as f64));

        // for each pixel in the row
        for x in 0..canvas_pixels {
            // compute the world x coordinate (left = -half, right = half)
            let world_x = -half + (pixel_size * (x as f64));

            // describe the point on the wall that the ray will target
            let position = Point::new(world_x, world_y, wall_z);
            let vector = position - ray_origin;
            let ray = Ray::new(ray_origin, vector.normalize());

            let xs = Sphere::intersect(&sphere, ray);
            let hit = Intersection::hit(&xs);
            
            match hit {
                Some(_) => { canvas.draw(color, x, y) }
                None => {}
            }
        }

    }

    canvas.to_ppm(config.filename.as_str());

    return Ok(());
}