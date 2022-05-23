use std::rc::Rc;

use crate::Matrix;
use crate::Phong;
use crate::Sphere;
use crate::IdentityCreator;
use crate::Point;
use crate::Color;
use crate::PointLight;
use crate::geoentity::intersectable::Intersectable;
use crate::Intersection;
use crate::Ray;

pub struct World {
    objects: Vec<Rc<dyn Intersectable>>,
    lights: Vec<Rc<PointLight>>
}

impl World {
    pub fn new(objects: Vec<Rc<dyn Intersectable>>, lights: Vec<Rc<PointLight>>) -> World {
        return World {
            objects,
            lights
        }
    }

    pub fn default(id_creator: &IdentityCreator) -> World {
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));

        let mut sphere_A = Sphere::unit(id_creator.get());
        sphere_A = sphere_A.set_material(
            Phong::new(Color::new(0.8, 1.0, 0.6), 
            0.1, 
            0.7, 
            0.2, 
            200.0));

        let mut sphere_B = Sphere::unit(id_creator.get());
        sphere_B = sphere_B.set_transform(Matrix::scaling(0.5, 0.5, 0.5));

        let objects: Vec<Rc<dyn Intersectable>> = vec![Rc::new(sphere_A), Rc::new(sphere_B)];
        let lights = vec![Rc::new(light)];

        return World {
            objects,
            lights
        }
    }

    pub fn intersect(self, ray: &Ray) -> Vec<Intersection> {
        let mut result: Vec<Intersection> = vec![];

        for obj in self.objects {
            let mut intersects = obj.clone().intersect(ray);
            result.append(&mut intersects);
        }

        // result.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        return result;
    }
}