use math::*;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray {
            origin: origin.clone(),
            direction: direction.normalize(),
        }
    }
    pub fn new_dummy() -> Ray {
        Ray {
            origin: Vec3::new(scalar::infinity(), scalar::infinity(), scalar::infinity()),
            direction: Vec3::new(1.0, 1.0, 1.0).normalize(),
        }
    }
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
    pub fn point_at(&self, t: scalar) -> Vec3 {
        self.origin + t * self.direction
    }
}
