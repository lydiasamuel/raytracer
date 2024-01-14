use tuples::tuple::Tuple;

use crate::{window::canvas::Canvas, tuples::color::Color};

static EPSILON: f64 = 0.00001;

pub mod tuples;
pub mod matrices;
pub mod window;

pub struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple
}

pub struct Enviroment {
    pub gravity: Tuple,
    pub wind: Tuple
}

pub fn run() {
    let width = 900;
    let height = 550;
    let mut canvas = Canvas::new(900, 550);

    let mut p = Projectile { 
        position: Tuple::point(0.0, 1.0, 0.0), 
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25
    };

    let e = Enviroment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0)
    };

    while p.position.y >= 0.0 {
        let x = p.position.x.ceil() as usize;
        let y = height - (p.position.y.ceil() as usize);

        if x < width && y < height {
            canvas.write_pixel(x, y, Color::new(1.0, 0.8, 0.6)).unwrap();
        }

        p = tick(&e, p);
    }

    print!("{}", canvas.to_ppm().unwrap());
}

pub fn tick(env: &Enviroment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;

    return Projectile { position, velocity };
}