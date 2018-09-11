use intersect::*;
use math::*;
use ray::*;

extern crate rand;
use self::rand::prelude::*;

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    let mut p = Vec3::zeros();
    loop {
        p = 2.0 * Vec3::new(
            rng.gen::<scalar>(),
            rng.gen::<scalar>(),
            rng.gen::<scalar>(),
        ) - Vec3::new(1.0, 1.0, 1.0);
        if p.norm_squared() < 1.0 {
            break;
        };
    }
    return p;
}

fn random_in_unit_hemisphere(rng: &mut ThreadRng, normal: Vec3) -> Vec3 {
    let mut p = Vec3::zeros();
    loop {
        p = 2.0 * Vec3::new(
            rng.gen::<scalar>(),
            rng.gen::<scalar>(),
            rng.gen::<scalar>(),
        ) - Vec3::new(1.0, 1.0, 1.0);
        if p.norm_squared() < 1.0 && p.dot(&normal) > 0.0 {
            break;
        };
    }
    return p;
}

fn reflect(v: &Vec3, unit_normal: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(unit_normal) * unit_normal
}

fn schlick(cosine: scalar, ref_idx: scalar) -> scalar {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
}

pub trait Scatter {
    fn scatter(
        &self,
        ray: &Ray,
        intersection: &Intersection,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
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
    fn scatter(
        &self,
        ray: &Ray,
        intersection: &Intersection,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool {
        // FIXME:
        let mut rng = thread_rng();
        let target = intersection.point() + intersection.normal() + random_in_unit_sphere(&mut rng);
        *scattered_ray = Ray::new(intersection.point(), &(target - intersection.point()));
        *attenuation = self.albedo();
        return true;
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: scalar,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: scalar) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: fuzz,
        }
    }

    fn albedo(&self) -> Vec3 {
        self.albedo
    }
}

impl Scatter for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        intersection: &Intersection,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool {
        let reflected = reflect(&ray.direction(), &intersection.normal());
        // FIXME:
        let mut rng = thread_rng();
        *scattered_ray = Ray::new(
            intersection.point(),
            &(reflected + self.fuzz * random_in_unit_sphere(&mut rng)),
        );
        *attenuation = self.albedo();

        return scattered_ray.direction().dot(&intersection.normal()) > 0.0;
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    ref_idx: scalar,
}

impl Dielectric {
    pub fn new(ri: scalar) -> Dielectric {
        Dielectric { ref_idx: ri }
    }
}

impl Scatter for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        intersection: &Intersection,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool {
        fn refract(v: &Vec3, n: &Vec3, ni_over_nt: scalar, refracted: &mut Vec3) -> bool {
            let dt = v.dot(n);
            let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
            if (discriminant > 0.0) {
                *refracted = ni_over_nt * (v - n * dt) - n * discriminant.sqrt();
                return true;
            } else {
                return false;
            }
        }

        *attenuation = Vec3::new(1.0, 1.0, 1.0);

        let mut outward_normal = Vec3::zeros();
        let mut ni_over_nt: scalar;
        let mut reflect_prob: scalar;
        let mut cosine: scalar;
        if (ray.direction().dot(&intersection.normal()) > 0.0) {
            outward_normal = -intersection.normal();
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * ray.direction().dot(&intersection.normal());
        } else {
            outward_normal = intersection.normal().clone();
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -ray.direction().dot(&intersection.normal());
        }

        let mut refracted = Vec3::zeros();
        if refract(
            &ray.direction(),
            &outward_normal,
            ni_over_nt,
            &mut refracted,
        ) {
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            reflect_prob = 1.0;
        }

        // FIXME:
        let mut rng = thread_rng();
        if rng.gen::<scalar>() < reflect_prob {
            let reflected = reflect(&ray.direction(), &intersection.normal());
            *scattered_ray = Ray::new(intersection.point(), &reflected);
        } else {
            *scattered_ray = Ray::new(intersection.point(), &refracted);
        }

        return true;
    }
}
