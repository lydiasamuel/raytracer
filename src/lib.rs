mod window;
mod tuples;
mod matrices;
mod geoentity;

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
    let width = 900;
    let height = 550;
    let canvas = MyCanvas::new(width, height);

    let id_creator = IdentityCreator::new();
    
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let sphere = Sphere::unit(id_creator.get());

    let xs = sphere.intersect(ray);

    println!("{}, {}", xs[0], xs[1]);
  
    /*let mut p = Projectile::new(
        Point::new(0.0, 1.0, 0.0), 
        Vector::new(1.0, 1.8, 0.0).normalize() * 11.25
    );
    
    let e = Environment::new(
        Vector::new(0.0, -0.1, 0.0),
        Vector::new(-0.01, 0.0, 0.0)
    );
    
    while p.position.y > 0.0 {
        canvas.draw(Color::new(1.0, 0.8, 0.6), p.position.x as usize, height - p.position.y as usize);
        p = tick(&e, &p);
    }

    canvas.to_ppm(config.filename.as_str());*/

    return Ok(());
}