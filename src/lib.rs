mod tuples;

use std::error::Error;

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
    // projectile starts one unit above the origin.​
    // velocity is normalized to 1 unit/tick.​
    
    let mut p = Projectile::new(
        Point::new(0.0, 1.0, 0.0), 
        Vector::new(1.0, 1.0, 0.0).normalize()
    );
    
    let e = Environment::new(
        Vector::new(0.0, -0.1, 0.0),
        Vector::new(-0.01, 0.0, 0.0)
    );
    
    while p.position.y > 0.0 {
        println!("{:?}", p);
        println!("{:?}", e);
        println!("");
        p = tick(&e, &p);
    }

    return Ok(());
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;

    return Projectile::new(position, velocity);
}
