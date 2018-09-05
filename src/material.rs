use ray::*;
use math::*;
use intersect::*;

extern crate rand;
use self::rand::prelude::*;

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    let mut p = Vec3::zeros();
    loop {
        p = 2.0 * Vec3::new(rng.gen::<scalar>(), rng.gen::<scalar>(),rng.gen::<scalar>())
          - Vec3::new(1.0, 1.0, 1.0);
        if p.norm_squared() < 1.0 { break; };
    }
    return p;
}

fn random_in_unit_hemisphere(rng: &mut ThreadRng, normal: Vec3) -> Vec3 {
    let mut p = Vec3::zeros();
    loop {
        p = 2.0 * Vec3::new(rng.gen::<scalar>(), rng.gen::<scalar>(),rng.gen::<scalar>())
         - Vec3::new(1.0, 1.0, 1.0);
        if p.norm_squared() < 1.0 && p.dot(&normal) > 0.0 { break; };
    }
    return p;
}

pub trait Scatter {
    fn scatter(&self, ray: &Ray, intersection: &Intersection, attenuation: &mut Vec3, scattered_ray: &mut Ray) -> bool;
}

#[derive(Copy, Clone)]
pub struct Lambertian
{
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo: albedo }
    }

    pub fn albedo(&self) -> Vec3 {
        self.albedo
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, ray: &Ray, intersection: &Intersection, attenuation: &mut Vec3, scattered_ray: &mut Ray) -> bool {
        let mut rng = thread_rng();
        let target = intersection.point() + intersection.normal() + random_in_unit_sphere(&mut rng);
        *scattered_ray = Ray::new(intersection.point(), target - intersection.point());
        *attenuation = self.albedo();
        return true;
    }
}

#[derive(Copy, Clone)]
pub struct Metal
{
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo: albedo }
    }

    fn albedo(&self) -> Vec3 {
        self.albedo
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, intersection: &Intersection, attenuation: &mut Vec3, scattered_ray: &mut Ray) -> bool {
        fn reflect(v: &Vec3, unit_normal: &Vec3) -> Vec3 {
            v - 2.0 * v.dot(unit_normal) * unit_normal
        }

        let reflected = reflect(&ray.direction(), &intersection.normal());
        *scattered_ray = Ray::new(intersection.point(), reflected);
        *attenuation = self.albedo();

        return scattered_ray.direction().dot(&intersection.normal()) > 0.0
    }
}