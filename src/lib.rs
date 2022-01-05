mod window;
mod tuples;
mod matrices;

use std::error::Error;
use std::fs;
use crate::window::mycanvas::MyCanvas;
use crate::tuples::point::Point;
use crate::tuples::vector::Vector;
use crate::tuples::color::Color;
use crate::tuples::projectile::Projectile;
use crate::tuples::environment::Environment;
use crate::matrices::matrix::Matrix;

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let width = 900;
    let height = 550;
    let canvas = MyCanvas::new(width, height);

    let dataA = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let dataB = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.5, 6.0]];
    let matrixA = Matrix::from_rows(&dataA);
    let matrixB = Matrix::from_rows(&dataB);

    if (matrixA == matrixB) {
        println!("YES");
    }
    else {
        println!("NO")
    }

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

    canvas.to_PPM(config.filename.as_str());*/

    return Ok(());
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;

    return Projectile::new(position, velocity);
}