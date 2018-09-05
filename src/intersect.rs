use math::*;
use ray::Ray;
use std::rc::Rc;
use std::cell::RefCell;
use material::Scatter;

#[derive(Clone)]
pub struct Intersection
{
    t: scalar,
    point: Vec3,
    normal: Vec3,
    material: Option<Rc<Scatter>>,
}

impl Intersection 
{
    pub fn new(t: scalar, point: Vec3, normal: Vec3, material: Option<Rc<Scatter>>) -> Intersection {
        Intersection { t: t, point: point, normal: normal.normalize(), material: material }
    }

    pub fn new_dummy() -> Intersection {
        Intersection {
            t: scalar::infinity(),
            point: Vec3::new(scalar::infinity(),scalar::infinity(),scalar::infinity()),
            normal: Vec3::new(1.0 , 1.0, 1.0).normalize(),
            material: None,
        }
    }

    pub fn t(&self) -> scalar { self.t }
    pub fn point(&self) -> Vec3 { self.point }
    pub fn normal(&self) -> Vec3 { self.normal }
    pub fn material(&self) -> Option<Rc<Scatter>> { self.material.clone() }

    pub fn set_t(&mut self, new_t: scalar) { self.t = new_t; }
    pub fn set_point(&mut self, new_point: Vec3) { self.point = new_point; }
    pub fn set_normal(&mut self, new_normal: Vec3) { self.normal = new_normal; }
    pub fn set_material(&mut self, new_material: Option<Rc<Scatter>>) { self.material = new_material; }
}

pub trait Intersectable
{
    fn intersect(&self, ray: &Ray, t_min: scalar, t_max: scalar, t: &mut Intersection) -> bool;
}