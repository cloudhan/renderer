use std::unimplemented;

use ray::*;
use math::*;

pub struct Camera
{
    position: Vec3,

    principal_point: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera
{
    pub fn new(position: Vec3, forward: Vec3, focus_offset: Vec3, aspect_ratio: scalar, focal_length: scalar) -> Camera {
        unimplemented!();
    }

    pub fn new_default() -> Camera {
        Camera { 
            position: Vec3::new(0.0, 0.0, 0.0),
            principal_point: Vec3::new(0.0, 0.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
        }
    }
}

pub trait Photographic
{
    fn generate_ray(&self, u: scalar, v: scalar) -> Ray;
}

impl Photographic for Camera{
    fn generate_ray(&self, u: scalar, v: scalar) -> Ray{
        return Ray::new(self.position, self.principal_point + (u - 0.5)*self.horizontal + (v-0.5) * self.vertical - self.position);
    }
}