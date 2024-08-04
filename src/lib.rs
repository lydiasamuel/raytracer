use crate::geometry::plane::Plane;
use crate::materials::phong::Phong;
use crate::matrices::matrix::Matrix;
use crate::patterns::checker::Checker;
use crate::patterns::solid::Solid;
use crate::scene::camera::Camera;
use crate::scene::world::World;
use crate::tuples::color::Color;
use crate::tuples::point_light::PointLight;
use crate::tuples::tuple::Tuple;
use crate::window::canvas::Canvas;

use crate::geometry::shape::Shape;
use crate::scene::obj_file_parser::ObjFileParser;
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
            Tuple::point(0.0, 3.0, -5.0),
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
    let floor_material = Arc::new(Phong::new(
        Box::new(Checker::new(
            Box::new(Solid::new(Color::new(0.9, 0.9, 0.9))),
            Box::new(Solid::new(Color::new(0.8, 0.8, 0.8))),
            Arc::new(Matrix::identity(4)),
        )),
        0.1,
        0.9,
        0.0,
        200.0,
        0.02,
        0.0,
        1.0,
    ));

    let floor = Plane::new(Arc::new(Matrix::identity(4)), floor_material.clone(), false);

    let middle = ObjFileParser::parse_obj_file(
        "tests/obj_files/smooth_teapot.obj".to_string(),
        Arc::new(Matrix::identity(4)),
        Arc::new(Phong::new(
            Box::new(Solid::new(Color::new(0.7, 0.2, 0.5))),
            0.1,
            0.9,
            0.9,
            200.0,
            0.0,
            0.0,
            1.0,
        )),
        true,
    )
    .unwrap();

    let middle_obj = middle.obj_to_group(Arc::new(
        (&(&Matrix::scaling(0.15, 0.15, 0.15) * &Matrix::translation(-0.4, 0.0, 0.4)).unwrap()
            * &Matrix::rotation_x(3.0 * PI / 2.0))
            .unwrap(),
    ));

    middle_obj.clone().divide(5);

    let light_source = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    World::new(
        vec![Arc::new(floor), middle_obj],
        vec![Arc::new(light_source)],
    )
}
