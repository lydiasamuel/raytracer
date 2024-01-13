use tuples::tuple::Tuple;

static EPSILON: f64 = 0.00001;

pub mod tuples;

pub struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple
}

pub struct Enviroment {
    pub gravity: Tuple,
    pub wind: Tuple
}

pub fn run() {
    let mut p = Projectile { 
        position: Tuple::point(0.0, 1.0, 0.0), 
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize() * 2.0
    };

    let e = Enviroment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0)
    };

    let mut counter = 0;

    while p.position.y >= 0.0 {
        p = tick(&e, p);

        println!("{}", p.position);

        counter += 1;
    }

    println!("It took {} ticks.", counter);
}

pub fn tick(env: &Enviroment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;

    return Projectile { position, velocity };
}