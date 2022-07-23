use crate::materials::phong::Phong;
use crate::Vector;
use std::rc::Rc;

use crate::Matrix;
use crate::Sphere;
use crate::IdentityCreator;
use crate::Point;
use crate::Color;
use crate::PointLight;
use crate::geoentity::shape::Shape;
use crate::Intersection;
use crate::Ray;
use crate::universe::computations::Computations;

const EPSILON: f64 = 0.00001;

pub struct World {
    objects: Vec<Rc<dyn Shape>>,
    lights: Vec<Rc<PointLight>>
}

impl World {
    pub fn new(objects: Vec<Rc<dyn Shape>>, lights: Vec<Rc<PointLight>>) -> World {
        return World {
            objects,
            lights
        }
    }

    pub fn default(id_creator: &IdentityCreator) -> World {
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));

        let outer = Sphere::unit(
            id_creator.get(),
            Matrix::identity(4), 
            Box::new(Phong::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0))
        );
        
        let inner = Sphere::unit(
            id_creator.get(),
            Matrix::scaling(0.5, 0.5, 0.5),
            Box::new(Phong::default())
        );

        let objects: Vec<Rc<dyn Shape>> = vec![Rc::new(outer), Rc::new(inner)];
        let lights = vec![Rc::new(light)];

        return World {
            objects,
            lights
        }
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let mut result = Color::new(0.0, 0.0, 0.0);

        // Call intersect to find the intersections of the given ray in this world
        let intersects = self.intersect(ray);
        
        // Find the hit from the resulting intersects
        let hit = Intersection::hit(&intersects);

        if hit.is_some() {
            let intersect = hit.unwrap();

            let comps = self.prepare_computations(&intersect, ray);

            result = self.shade_hit(&comps);
        }

        return result;
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut result: Vec<Intersection> = vec![];

        for i in 0..self.objects.len() {
            let obj = self.objects[i].clone();
            
            let mut intersects = obj.intersect(ray);
            result.append(&mut intersects);
        }

        result.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        return result;
    }

    pub fn prepare_computations(&self, intersection: &Intersection, ray: &Ray) -> Computations {
        let time = intersection.time;
        let object = intersection.entity.clone();
        let point = ray.position(time);

        let eyev = -ray.direction;
        let mut normalv = object.as_ref().normal_at(&point);

        let mut inside = false;

        if Vector::dot(normalv, eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        // Used to bump slightly in the direction of the surface normal to help prevent self shadowing
        let over_point = point + (normalv * EPSILON);

        return Computations::new(
            time, 
            object,
            point,
            over_point,
            eyev,
            normalv,
            inside
        );
    }

    pub fn shade_hit(&self, comps: &Computations) -> Color {
        let mut result = Color::new(0.0, 0.0, 0.0);

        let material = comps.object.as_ref().material();

        for i in 0..self.lights.len() {
            let light = self.lights[i].as_ref();
            let in_shadow = self.is_shadowed(comps.over_point); // Using bumped point instead which'll help prevent floating point mismatches

            result = result + material.lighting(&comps.over_point, light, &comps.eyev, &comps.normalv, in_shadow);
        }

        return result;
    }

    pub fn is_shadowed(&self, point: Point) -> bool {
        let vec = self.lights[0].position - point; // TODO: support multiple light sources
        
        let distance = vec.magnitude(); // Measure the distance from the point to the light source
        let direction = vec.normalize(); // Create a ray pointing towards the light source
        
        let ray = Ray::new(point, direction);
        let intersections = self.intersect(&ray); // Intersect the world with that ray 
    
        let hit = Intersection::hit(&intersections);
        // Check to see if there was a hit, and if so did it occur before the ray reached the light source
        if hit.is_some() {
            let intersect = hit.unwrap();
            
            return intersect.time < distance;
        }
        
        return false;
    }
}