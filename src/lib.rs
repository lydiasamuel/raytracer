use crate::geometry::plane::Plane;
use crate::materials::phong::Phong;
use crate::matrices::matrix::Matrix;
use crate::patterns::solid::Solid;
use crate::scene::camera::Camera;
use crate::scene::world::World;
use crate::tuples::color::Color;
use crate::tuples::point_light::PointLight;
use crate::tuples::tuple::Tuple;
use crate::window::canvas::Canvas;

use crate::geometry::cube::Cube;
use crate::geometry::group::Group;
use crate::geometry::shape::Shape;
use crate::geometry::sphere::Sphere;
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
        0.785,
        Matrix::view_transform(
            Tuple::point(-6.0, 6.0, -10.0),
            Tuple::point(6.0, 0.0, 6.0),
            Tuple::vector(-0.45, 1.0, 0.0),
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
    let white_material = Arc::new(Phong::new(
        Box::new(Solid::new(Color::white())),
        0.1,
        0.7,
        0.0,
        200.0,
        0.1,
        0.0,
        1.0,
    ));

    let blue_material = Arc::new(Phong::new(
        Box::new(Solid::new(Color::new(0.537, 0.831, 0.914))),
        0.1,
        0.7,
        0.0,
        200.0,
        0.1,
        0.0,
        1.0,
    ));

    let red_material = Arc::new(Phong::new(
        Box::new(Solid::new(Color::new(0.941, 0.322, 0.388))),
        0.1,
        0.7,
        0.0,
        200.0,
        0.1,
        0.0,
        1.0,
    ));

    let purple_material = Arc::new(Phong::new(
        Box::new(Solid::new(Color::new(0.373, 0.404, 0.550))),
        0.1,
        0.7,
        0.0,
        200.0,
        0.1,
        0.0,
        1.0,
    ));

    let standard_transform =
        (&Matrix::scaling(0.5, 0.5, 0.5) * &Matrix::translation(1.0, -1.0, 1.0)).unwrap();

    let large_object = (&Matrix::scaling(3.5, 3.5, 3.5) * &standard_transform).unwrap();
    let medium_object = (&Matrix::scaling(3.0, 3.0, 3.0) * &standard_transform).unwrap();
    let small_object = (&Matrix::scaling(2.0, 2.0, 2.0) * &standard_transform).unwrap();

    let plane = Arc::new(Plane::new(
        Arc::new((&Matrix::translation(0.0, 0.0, 500.0) * &Matrix::rotation_x(PI / 2.0)).unwrap()),
        Arc::new(Phong::new(
            Box::new(Solid::new(Color::white())),
            1.0,
            0.0,
            0.0,
            200.0,
            0.0,
            0.0,
            1.0,
        )),
        true,
    ));

    let group = Arc::new(Group::default());

    let children: Vec<Arc<dyn Shape>> = vec![
        Arc::new(Sphere::new(
            Arc::new(large_object.clone()),
            Arc::new(Phong::new(
                Box::new(Solid::new(Color::new(0.373, 0.404, 0.550))),
                0.0,
                0.2,
                1.0,
                200.0,
                0.7,
                0.7,
                1.5,
            )),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(4.0, 0.0, 0.0) * &medium_object).unwrap()),
            white_material.clone(),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(8.5, 1.5, -0.5) * &large_object).unwrap()),
            blue_material.clone(),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(0.0, 0.0, 4.0) * &large_object).unwrap()),
            red_material.clone(),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(4.0, 0.0, 4.0) * &small_object).unwrap()),
            white_material.clone(),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(7.5, 0.5, 4.0) * &medium_object).unwrap()),
            purple_material.clone(),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(-0.25, 0.25, 8.0) * &medium_object).unwrap()),
            white_material.clone(),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(4.0, 1.0, 7.5) * &large_object).unwrap()),
            blue_material.clone(),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(10.0, 2.0, 7.5) * &medium_object).unwrap()),
            red_material.clone(),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(8.0, 2.0, 12.0) * &small_object).unwrap()),
            white_material.clone(),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(20.0, 1.0, 9.0) * &small_object).unwrap()),
            white_material.clone(),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(-0.5, -5.0, 0.25) * &large_object).unwrap()),
            blue_material.clone(),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(4.0, -4.0, 0.0) * &large_object).unwrap()),
            red_material.clone(),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(8.5, -4.0, 0.0) * &large_object).unwrap()),
            white_material.clone(),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(0.0, -4.0, 4.0) * &large_object).unwrap()),
            white_material.clone(),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(-0.5, -4.5, 8.0) * &large_object).unwrap()),
            purple_material.clone(),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(0.0, -8.0, 4.0) * &large_object).unwrap()),
            white_material.clone(),
            true,
        )),
        Arc::new(Cube::new(
            Arc::new((&Matrix::translation(-0.5, 8.5, 8.0) * &large_object).unwrap()),
            white_material.clone(),
            true,
        )),
    ];

    group.clone().add_children(children);

    group.clone().divide(1);

    World::new(
        vec![plane, group.clone()],
        vec![
            Arc::new(PointLight::new(
                Tuple::point(50.0, 100.0, -50.0),
                Color::new(1.0, 1.0, 1.0),
            )),
            Arc::new(PointLight::new(
                Tuple::point(-400.0, 50.0, -10.0),
                Color::new(0.2, 0.2, 0.2),
            )),
        ],
    )
}
