mod window;
mod tuples;
mod matrices;
mod geoentity;
mod universe;

use std::rc::Rc;
use std::sync::Mutex;
use std::error::Error;

use crate::window::mycanvas::MyCanvas;
use crate::tuples::point::Point;
use crate::tuples::vector::Vector;
use crate::tuples::color::Color;
use crate::tuples::ray::Ray;
use crate::matrices::matrix::Matrix;
use crate::geoentity::sphere::Sphere;
use crate::tuples::light::PointLight;
use crate::tuples::material::Phong;
use crate::tuples::intersection::Intersection;
use crate::geoentity::intersectable::Intersectable;
use crate::universe::world::World;
use crate::universe::camera::Camera;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
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
    let id_creator = IdentityCreator::new();

    let camera = build_camera();
    let world = build_world(&id_creator);

    let canvas = render(camera, world);

    let file = canvas.to_ppm(config.filename.as_str());

    match file {
        Ok(()) => {},
        Err(error) => panic!("Image could not be saved: {:?}", error)
    }

    return Ok(());
}

pub fn build_camera() -> Camera {
    return Camera::new(
        600, 
        300, 
        std::f64::consts::PI / 3.0,
        Camera::view_transform(
            Point::new(0.0, 1.5, -5.0),
            Point::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 1.0, 0.0))
    );
}

pub fn build_world(id_creator: &IdentityCreator) -> World {
    let mut floor = Sphere::unit(id_creator.get());
    let floor_material = Phong::new(Color::new(1.0, 0.9, 0.9), 0.1, 0.9, 0.0, 200.0);
    floor = floor.set_transform(Matrix::scaling(10.0, 0.01, 10.0));
    floor = floor.set_material(floor_material);

    let mut left_wall = Sphere::unit(id_creator.get());
    let mut left_wall_transform = Matrix::translation(0.0, 0.0, 5.0);
    left_wall_transform = (left_wall_transform * Matrix::rotation_y(-(std::f64::consts::PI / 4.0))).unwrap();
    left_wall_transform = (left_wall_transform * Matrix::rotation_x(std::f64::consts::PI / 2.0)).unwrap();
    left_wall_transform = (left_wall_transform * Matrix::scaling(10.0, 0.01, 10.0)).unwrap();
    left_wall = left_wall.set_transform(left_wall_transform);
    left_wall = left_wall.set_material(floor_material);

    let mut right_wall = Sphere::unit(id_creator.get());
    let mut right_wall_transform = Matrix::translation(0.0, 0.0, 5.0);
    right_wall_transform  = (right_wall_transform  * Matrix::rotation_y(std::f64::consts::PI / 4.0)).unwrap();
    right_wall_transform  = (right_wall_transform * Matrix::rotation_x(std::f64::consts::PI / 2.0)).unwrap();
    right_wall_transform  = (right_wall_transform * Matrix::scaling(10.0, 0.01, 10.0)).unwrap();
    right_wall = right_wall.set_transform(right_wall_transform);
    right_wall = right_wall.set_material(floor_material);

    let mut middle = Sphere::unit(id_creator.get());
    middle = middle.set_transform(Matrix::translation(-0.5, 1.0, 0.5));
    middle = middle.set_material(Phong::new(Color::new(0.1, 1.0, 0.5), 0.1, 0.7, 0.3, 200.0));

    let mut right = Sphere::unit(id_creator.get());
    let mut right_transform = Matrix::translation(1.5, 0.5, -0.5);
    right_transform = (right_transform * Matrix::scaling(0.5, 0.5, 0.5)).unwrap();
    right = right.set_transform(right_transform);
    right = right.set_material(Phong::new(Color::new(0.5, 1.0, 0.1), 0.1, 0.7, 0.3, 200.0));

    let mut left = Sphere::unit(id_creator.get());
    let mut left_transform = Matrix::translation(-1.5, 0.33, -0.75);
    left_transform = (left_transform * Matrix::scaling(0.33, 0.33, 0.33)).unwrap();
    left = left.set_transform(left_transform);
    left = left.set_material(Phong::new(Color::new(1.0, 0.8, 0.1), 0.1, 0.7, 0.3, 200.0));

    let light_source = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));

    let objects: Vec<Rc<dyn Intersectable>> = 
        vec![
            Rc::new(floor), 
            Rc::new(left_wall), 
            Rc::new(right_wall), 
            Rc::new(middle), 
            Rc::new(right), 
            Rc::new(left)
            ];

    let lights = vec![Rc::new(light_source)];

    return World::new(objects, lights);
}

pub fn render(camera: Camera, world: World) -> MyCanvas {
    let canvas = MyCanvas::new(camera.hsize, camera.vsize);

    for y in 0..(camera.vsize - 1) {
        for x in 0..(camera.hsize - 1) {
            let ray = camera.ray_for_pixel(x as f64, y as f64);
            let color = world.color_at(&ray);
            canvas.draw(color, x, y) 
        }
    }

    return canvas;
}