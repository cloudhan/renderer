use material::Scatter;
use math::*;
use ray::Ray;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Intersection {
    t: scalar,
    point: Vec3,
    normal: Vec3,
    material: Option<Rc<Scatter>>,
}

impl Intersection {
    pub fn new(t: scalar, point: Vec3, normal: Vec3, material: Option<Rc<Scatter>>) -> Self {
        Intersection {
            t,
            point,
            normal: normal.normalize(),
            material,
        }
    }

    pub fn new_uninitialized() -> Self {
        Intersection {
            t: scalar::infinity(),
            point: zero(),
            normal: zero(),
            material: None,
        }
    }

    pub fn t(&self) -> scalar {
        self.t
    }
    pub fn point(&self) -> &Vec3 {
        &self.point
    }
    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }
    pub fn material(&self) -> &Option<Rc<Scatter>> {
        &self.material
    }

    pub fn set_t(&mut self, new_t: scalar) -> &mut Self {
        self.t = new_t;
        return self;
    }

    pub fn set_point(&mut self, new_point: Vec3) -> &mut Self {
        self.point = new_point;
        return self;
    }
    pub fn set_normal(&mut self, new_normal: Vec3) -> &mut Self {
        self.normal = new_normal;
        return self;
    }
    pub fn set_material(&mut self, new_material: Option<Rc<Scatter>>) -> &mut Self {
        self.material = new_material;
        return self;
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray, t_min: scalar, t_max: scalar) -> Option<Intersection>;
}
