use ray::*;
use math::*;

pub struct Ray
{
    pub origin: Point,
    pub direction: Direction,
}

pub struct Intersection
{
    
}

pub trait Intersectable
{
    fn intersect(&self, ray: &Ray, t: &mut scalar) -> bool;
}

pub struct Sphere
{
    pub center: Point,
    pub radius: f64,
}

impl Intersectable for Sphere
{
    fn intersect(&self, ray: &Ray, t: &mut scalar) -> bool {

        let oc = ray.origin - self.center;
        let a = ray.direction.norm();
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.norm() - self.radius*self.radius;

        let d = b*b - 4.0*a*c;

        return d>0.0;
    }
}