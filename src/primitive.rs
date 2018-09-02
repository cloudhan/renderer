use ray::*;
use math::*;
use intersect::*;

pub struct Sphere
{
    pub center: Vec3,
    pub radius: f64,
}

impl Intersectable for Sphere
{
    fn intersect(&self, ray: &Ray, t_min: scalar, t_max: scalar, intersection: &mut Intersection) -> bool {

        let oc = ray.origin() - self.center;
        let a = ray.direction().norm_squared();
        let b = oc.dot(ray.direction());
        let c = oc.norm_squared() - self.radius*self.radius;

        let discriminant = b*b - a*c;

        if(discriminant > 0.0) {
            let temp  = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                intersection.set_t(temp);
                let p = ray.point_at(temp);
                intersection.set_point(p);
                intersection.set_normal((p - self.center)/self.radius);
                return true;
            }
            let temp  = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                intersection.set_t(temp);
                let p = ray.point_at(temp);
                intersection.set_point(p);
                intersection.set_normal((p - self.center)/self.radius);
                return true;
            }
            return false;
        }
        else {
            return false;
        }
    
    }
}

impl Intersectable for Vec<Box<Intersectable>>
{
    fn intersect(&self, ray: &Ray, t_min: scalar, t_max: scalar, intersection: &mut Intersection) -> bool {
        let mut temp_intersection = Intersection::new_dummy();
        let mut hit = false;
        let mut closest = t_max;
        for intersectable in self {
            if intersectable.intersect(ray, t_min, closest, &mut temp_intersection) {
                hit = true;
                closest = temp_intersection.t();
                *intersection = temp_intersection;
            }
        }

        return hit;
    }

}


