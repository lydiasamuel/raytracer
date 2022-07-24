use std::rc::Rc;

use crate::Color;
use crate::PointLight;
use crate::Matrix;
use crate::materials::material::Material;
use crate::Point;
use crate::Ray;
use crate::Intersection;
use crate::Vector;
use crate::Shape;

const EPSILON: f64 = 0.00001;

pub struct Plane {
    id: u64,
    transform: Matrix, // Used to translate a point from object space to world space
    material: Box<dyn Material>
}

impl Plane {
    pub fn new(id: u64, transform: Matrix, material: Box<dyn Material>) -> Plane {
        return Plane {
            id,
            transform,
            material
        }
    }
}

impl Shape for Plane {
    fn get_id(&self) -> u64 { 
        return self.id;
    }

    fn transform_ray_to_obj_space(&self, world_ray: &Ray) -> Ray {
        return world_ray.transform(self.transform.inverse());
    }

    fn transform_point_to_obj_space(&self, world_point: &Point) -> Point {
        return world_point.transform(self.transform.inverse());
    }

    fn intersect(self: Rc<Self>, world_ray: &Ray) -> Vec<Intersection> {
        let ray = self.transform_ray_to_obj_space(world_ray);

        /*
            Four cases to consider
            1. Ray is parallel to the plane, and thus will never intersect it
            2. Ray is coplanar with the plane, which is to say the ray's origin is on the plane,
               and the ray's direction is parallel to the plane. Therefore every point on the ray
               intersects the plane. We'll assume this misses.
            3. Ray origin is above the plane
            4. Ray origin is below the plane    
        */

        if ray.direction.y.abs() < EPSILON { // Plane is in xz therefore if there's no y slope so it's parallel
            return Vec::new(); // No intersections in this case
        }
 
        // Ray is either above or below the plane so calculate the intersection time
        let time = -ray.origin.y / ray.direction.y;

        return vec![Intersection::new(time, self.clone())];
    }

    fn normal_at(&self, world_point: &Point) -> Vector {
        return (self.transform.inverse() * Vector::new(0.0, 1.0, 0.0)).unwrap();
    }

    fn light_material(&self, world_point: &Point, light: &PointLight, eyev: &Vector, normalv: &Vector, in_shadow: bool) -> Color {
        let object_point = self.transform_point_to_obj_space(world_point);

        return self.material.lighting(
            world_point, 
            object_point, 
            light,
            eyev,
            normalv,
            in_shadow);
    }
}