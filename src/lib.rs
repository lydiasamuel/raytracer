use crate::geometry::plane::Plane;
use std::error::Error;
use std::f64::consts::PI;
use std::sync::{mpsc, Arc};
use std::thread;

use crate::geometry::sphere::Sphere;
use crate::materials::phong::Phong;
use crate::matrices::matrix::Matrix;
use crate::patterns::blended::Blended;
use crate::patterns::perturbed::Perturbed;
use crate::patterns::solid::Solid;
use crate::patterns::striped::Striped;
use crate::scene::camera::Camera;
use crate::scene::world::World;
use crate::tuples::color::Color;
use crate::tuples::pointlight::PointLight;
use crate::tuples::tuple::Tuple;
use crate::window::canvas::Canvas;

static EPSILON: f64 = 0.00001;
static THREADS: usize = 8;

pub mod geometry;
pub mod materials;
pub mod matrices;
mod patterns;
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
    // Figure out chunk size using row-wise 1D partitioning
    let chunk_size = camera.height() / THREADS;
    // Account for case where image height isn't a perfect multiple of threads
    let leftover_chunk_size = camera.height() % THREADS;

    let mut handles = Vec::new();
    for i in 0..THREADS {
        // Clone send channel and scene info across to thread
        let thread_send_channel = send_channel.clone();
        let thread_world = world.clone();
        let thread_camera = camera.clone();

        let handle = thread::spawn(move || {
            let num_of_rows;

            if i == THREADS - 1 {
                num_of_rows = chunk_size + leftover_chunk_size;
            } else {
                num_of_rows = chunk_size;
            }

            let row_start = i * chunk_size;
            // Iterate over the partition of rows given to this thread
            for y in row_start..(row_start + num_of_rows) {
                for x in 0..thread_camera.width() {
                    let ray = thread_camera.ray_for_pixel(x, y);
                    // Send back color information to main thread to then write out to canvas
                    thread_send_channel
                        .send((x, y, thread_world.color_at(&ray)))
                        .unwrap();
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
    let floor_pattern =
        Box::new(Blended::new(
            Box::new(Perturbed::new(
                    Box::new(Striped::new(
                        Box::new(Solid::new(Color::white())),
                        Box::new(Solid::new(Color::red())),
                        Matrix::rotation_y(PI / 2.0)
                    )),
                    1.4,
                    Matrix::identity(4)
            )),
            Box::new(Perturbed::new(
                Box::new(Striped::new(
                    Box::new(Solid::new(Color::white())),
                    Box::new(Solid::new(Color::red())),
                    Matrix::identity(4)
                )),
                1.4,
                Matrix::scaling(2.0, 2.0, 2.0)
            )),
            Matrix::identity(4)
        ));

    let floor_material = Arc::new(Phong::new(
        floor_pattern,
        0.1,
        0.9,
        0.0,
        200.0,
    ));

    let floor = Plane::new(Matrix::identity(4), floor_material.clone());

    let middle = Sphere::new(
        Matrix::translation(-0.5, 1.0, 0.5),
        Arc::new(Phong::new(
            Box::new(Solid::new(Color::new(0.1, 1.0, 0.5))),
            0.1,
            0.7,
            0.3,
            200.0,
        )),
    );

    let right = Sphere::new(
        (Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5)).unwrap(),
        Arc::new(Phong::new(
            Box::new(Solid::new(Color::new(0.5, 1.0, 0.1))),
            0.1,
            0.7,
            0.3,
            200.0,
        )),
    );

    let left = Sphere::new(
        (Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33)).unwrap(),
        Arc::new(Phong::new(
            Box::new(Solid::new(Color::new(1.0, 0.8, 0.1))),
            0.1,
            0.7,
            0.3,
            200.0,
        )),
    );

    let light_source = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    World::new(
        vec![
            Arc::new(floor),
            Arc::new(middle),
            Arc::new(right),
            Arc::new(left),
        ],
        vec![Arc::new(light_source)],
    )
}
