mod window;
mod tuples;

use std::error::Error;

use crate::window::mycanvas::MyCanvas;
use crate::tuples::coord::Point;
use crate::tuples::coord::Vector;
use crate::tuples::color::Color;

#[derive(Debug)]
struct Projectile {
    pub position: Point,
    pub velocity: Vector
}

impl Projectile {
    pub fn new(position: Point, velocity: Vector) -> Projectile {
        return Projectile {
            position,
            velocity
        };
    }
}

#[derive(Debug)]
struct Environment {
    pub gravity: Vector,
    pub wind: Vector
}

impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Environment {
        return Environment {
            gravity,
            wind
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let canvas = MyCanvas::new(900, 550);

    let mut p = Projectile::new(
        Point::new(0.0, 1.0, 0.0), 
        Vector::new(1.0, 1.8, 0.0).normalize() * 11.25
    );
    
    let e = Environment::new(
        Vector::new(0.0, -0.1, 0.0),
        Vector::new(-0.01, 0.0, 0.0)
    );
    
    while p.position.y > 0.0 {
        canvas.draw(Color::new(1.0, 0.8, 0.6), p.position.x as usize, canvas.height - p.position.y as usize);
        p = tick(&e, &p);
    }

    canvas.to_PPM("C:\\Users\\peter\\Downloads\\test.ppm");

    return Ok(());
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;

    return Projectile::new(position, velocity);
}