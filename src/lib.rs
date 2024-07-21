use crate::geometry::cone::Cone;
use crate::geometry::cube::Cube;
use crate::geometry::plane::Plane;
use crate::geometry::sphere::Sphere;
use crate::materials::phong::Phong;
use crate::matrices::matrix::Matrix;
use crate::patterns::blended::Blended;
use crate::patterns::checker::Checker;
use crate::patterns::perturbed::Perturbed;
use crate::patterns::solid::Solid;
use crate::patterns::striped::Striped;
use crate::scene::camera::Camera;
use crate::scene::world::World;
use crate::tuples::color::Color;
use crate::tuples::pointlight::PointLight;
use crate::tuples::tuple::Tuple;
use crate::window::canvas::Canvas;

use std::error::Error;
use std::f64::consts::PI;
use std::sync::{mpsc, Arc};
use std::thread;

static MAX_RAY_RECURSION_DEPTH: usize = 5;
static EPSILON: f64 = 0.00001;
static NUM_OF_THREADS: usize = 12;

pub mod geometry;
pub mod materials;
pub mod matrices;
pub mod patterns;
pub mod scene;
pub mod tuples;
pub mod window;

pub struct Config {
    pub file_path: String,
    pub width: usize,
    pub height: usize,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();
        let width = args[2].clone().parse().unwrap();
        let height = args[3].clone().parse().unwrap();

        Ok(Config {
            file_path,
            width,
            height,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let world = Arc::new(build_world());
    let camera = Arc::new(Camera::new(
        config.height,
        config.width,
        PI / 3.0,
        Matrix::view_transform(
            Tuple::point(0.0, 1.5, -5.0),
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        ),
    ));

    let canvas = render(world, camera);

    canvas.write_to_file(config.file_path)?;

    Ok(())
}

pub fn render(world: Arc<World>, camera: Arc<Camera>) -> Canvas {
    // Initialise sending channels for producer consumer
    let (send_channel, receive_channel) = mpsc::channel();

    let width = camera.width();
    let height = camera.height();

    let mut handles = Vec::new();
    for i in 0..NUM_OF_THREADS {
        // Clone send channel and scene info across to thread
        let thread_send_channel = send_channel.clone();
        let thread_world = world.clone();
        let thread_camera = camera.clone();
        let thread_number = i;

        let handle = thread::spawn(move || {
            // Set x to remainder and y to quotient
            let mut x = thread_number % width;
            let mut y = thread_number / width;

            // Stop if we've gone past the bottom of the canvas
            while y < height {
                let ray = thread_camera.ray_for_pixel(x, y);

                // Send back color information to main thread to then write out to canvas
                thread_send_channel
                    .send((x, y, thread_world.color_at(&ray, MAX_RAY_RECURSION_DEPTH)))
                    .unwrap();

                // Increment x by the num of threads and loop if we're past the end of the row
                x += NUM_OF_THREADS;
                if x >= width {
                    x = x % width;
                    y = y + 1;
                }
            }
        });

        handles.push(handle);
    }

    let mut canvas = Canvas::new(camera.width(), camera.height());

    // Expect width * height number of messages from threads
    for _ in 0..(camera.width() * camera.height()) {
        let received = receive_channel.recv().unwrap();
        canvas
            .write_pixel(received.0, received.1, received.2)
            .unwrap();
    }

    for handle in handles {
        handle.join().unwrap();
    }

    canvas
}

pub fn build_world() -> World {
    let floor_pattern = Box::new(Blended::new(
        Box::new(Perturbed::new(
            Box::new(Striped::new(
                Box::new(Solid::new(Color::new(0.98, 0.92, 0.94))),
                Box::new(Solid::new(Color::new(0.2, 0.24, 0.47))),
                Arc::new(Matrix::rotation_y(PI / 2.0)),
            )),
            0.9,
            Arc::new(Matrix::rotation_x(PI / 3.0)),
        )),
        Box::new(Perturbed::new(
            Box::new(Striped::new(
                Box::new(Solid::new(Color::new(0.98, 0.92, 0.94))),
                Box::new(Solid::new(Color::new(0.2, 0.24, 0.47))),
                Arc::new(Matrix::identity(4)),
            )),
            0.9,
            Arc::new(Matrix::rotation_z(PI / 3.0)),
        )),
        Arc::new(Matrix::identity(4)),
    ));

    let floor_material = Arc::new(Phong::new(
        floor_pattern,
        0.1,
        0.9,
        0.0,
        200.0,
        0.02,
        0.0,
        1.0,
    ));

    let floor = Plane::new(Arc::new(Matrix::identity(4)), floor_material.clone(), false);

    let wall_material = Arc::new(Phong::new(
        Box::new(Checker::default()),
        0.8,
        0.4,
        0.0,
        100.0,
        0.0,
        0.0,
        1.0,
    ));

    let left_wall_transform =
        (&Matrix::translation(0.0, 0.0, 20.0) * &Matrix::rotation_x(PI / 2.0)).unwrap();
    let left_wall = Plane::new(Arc::new(left_wall_transform), wall_material.clone(), false);

    let middle = Sphere::new(
        Arc::new(Matrix::translation(-0.5, 1.0, 0.5)),
        Arc::new(Phong::new(
            Box::new(Solid::new(Color::new(0.2, 0.01, 0.3))),
            0.2,
            0.25,
            1.0,
            300.0,
            0.9,
            1.0,
            1.52,
        )),
        true,
    );

    let right = Cube::new(
        Arc::new((&Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scaling(0.5, 0.5, 0.5)).unwrap()),
        Arc::new(Phong::new(
            Box::new(Solid::new(Color::new(0.5, 1.0, 0.1))),
            0.1,
            0.7,
            0.3,
            200.0,
            0.2,
            0.0,
            1.0,
        )),
        true,
    );

    let left = Cone::new(
        Arc::new(
            (&Matrix::translation(-1.5, 1.0, -0.75) * &Matrix::scaling(0.33, 1.0, 0.33)).unwrap(),
        ),
        Arc::new(Phong::new(
            Box::new(Solid::new(Color::new(1.0, 0.8, 0.1))),
            0.1,
            0.7,
            0.3,
            200.0,
            0.1,
            0.0,
            1.0,
        )),
        true,
        -1.0,
        0.0,
        true,
    );

    let light_source = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    World::new(
        vec![
            Arc::new(floor),
            Arc::new(left_wall),
            Arc::new(middle),
            Arc::new(right),
            Arc::new(left),
        ],
        vec![Arc::new(light_source)],
    )
}
