use math::*;
use ray::*;

extern crate rand;
use self::rand::prelude::*;

pub struct Camera {
    position: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,

    w: Vec3,
    u: Vec3,
    v: Vec3,

    lens_radius: scalar,
}

impl Camera {
    pub fn new(
        position: &Vec3,
        lookat: &Vec3,
        up: &Vec3,
        vfov: scalar,
        aspect_ratio: scalar,
        aperture: scalar,
        focal_dist: scalar,
    ) -> Camera {
        let theta = vfov * scalar::PI() / 180.0;
        let half_height = scalar::tan(theta / 2.0);
        let half_width = aspect_ratio * half_height;

        let w = (position - lookat).normalize(); // negative forward
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);

        return Camera {
            position: position.clone(),
            lower_left: position
                - half_width * focal_dist * u
                - half_height * focal_dist * v
                - focal_dist * w,

            horizontal: 2.0 * half_width * focal_dist * u,
            vertical: 2.0 * half_height * focal_dist * v,
            w: w,
            u: u,
            v: v,

            lens_radius: aperture / 2.0,
        };
    }
}

pub trait Photographic {
    fn generate_ray(&self, u: scalar, v: scalar) -> Ray;
}

impl Photographic for Camera {
    fn generate_ray(&self, u: scalar, v: scalar) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        return Ray::new(
            &(self.position + offset),
            &(self.lower_left + u * self.horizontal + v * self.vertical - self.position - offset),
        );
    }
}

fn random_in_unit_disk() -> Vec3 {
    //FIXME:
    let mut rng = thread_rng();
    let mut res = Vec3::zeros();
    loop {
        res = Vec3::new(rng.gen::<scalar>() - 1.0, rng.gen::<scalar>() - 1.0, 0.0);
        if res.norm_squared() < 1.0 {
            break;
        }
    }
    return res;
}
