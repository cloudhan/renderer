use intersect::*;
use material::*;
use math::*;
use ray::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<Scatter>,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray, t_min: scalar, t_max: scalar) -> Option<Intersection> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().norm_squared();
        let b = oc.dot(&ray.direction());
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if (discriminant > 0.0) {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at(temp);
                let mut intersection = Intersection::new_uninitialized();
                intersection
                    .set_t(temp)
                    .set_point(p)
                    .set_normal((p - self.center) / self.radius)
                    .set_material(Some(self.material.clone()));
                return Some(intersection);
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at(temp);
                let mut intersection = Intersection::new_uninitialized();
                intersection
                    .set_t(temp)
                    .set_point(p)
                    .set_normal((p - self.center) / self.radius)
                    .set_material(Some(self.material.clone()));
                return Some(intersection);
            }
            return None;
        } else {
            return None;
        }
    }
}

impl Intersectable for Vec<Box<Intersectable>> {
    fn intersect(&self, ray: &Ray, t_min: scalar, t_max: scalar) -> Option<Intersection> {
        let mut intersection = Intersection::new_uninitialized();
        let mut hit = false;
        let mut closest = t_max;
        for intersectable in self {
            match intersectable.intersect(ray, t_min, closest) {
                Some(temp) => {
                    hit = true;
                    closest = temp.t();
                    intersection = temp;
                }
                None => continue,
            }
        }

        if hit {
            return Some(intersection);
        } else {
            return None;
        }
    }
}
