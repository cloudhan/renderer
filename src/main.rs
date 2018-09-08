#![allow(unused)]

extern crate rand;
use rand::prelude::*;

mod ray;
mod math;
mod camera;
mod material;
mod intersect;
mod primitive;

use std::fs::File;
use std::io::prelude::*;

use ray::*;
use math::*;
use camera::*;
use material::*;
use primitive::*;
use intersect::*;

use std::rc::Rc;


fn color(world: &Vec<Box<Intersectable>>, ray: &Ray, depth_remain: i32) -> Vec3 {
    let mut t = 0.0;
    let mut intersection = Intersection::new_dummy();
    if world.intersect(&ray, 1e-10, 25500.0, &mut intersection) {
        let mut scattered = Ray::new_dummy();
        let mut attenuation = Vec3::zeros();
        
        if (depth_remain > 0 && intersection.material().unwrap().scatter(ray, &intersection, &mut attenuation, &mut scattered)) {
            return attenuation.component_mul(&color(world, &scattered, depth_remain-1));
        }
        else{
            return Vec3::zeros();
        }
    }
    else {
        t = 0.5 * ray.direction().y + 1.0;
        return (1.0-t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn world() -> Vec<Box<Intersectable>> {
    let mut w = Vec::<Box<Intersectable>>::new();
    w.push(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))) 
    } ));

    w.push(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))
    }));

    w.push(Box::new(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0)) 
    } ));

    w.push(Box::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Dielectric::new(1.5)) 
    } ));

    
    w.push(Box::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: -0.45,
        material: Rc::new(Dielectric::new(1.5)) 
    } ));
    return w;
}

fn main() {
    let width = 400;
    let height = 200;
    let samples = 100;

    let gamma = 2.2;
    let coeff = 1.0/gamma;

    //camera
    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizental = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);

    let my_world = world();

    let cam_position = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);

    let camera = Camera::new(
        &cam_position, &lookat,
        &Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (width as scalar) / (height as scalar),
        2.0,
        (lookat - cam_position).norm(),
    );
    let mut rng = thread_rng();

    let mut f = File::create("./output.ppm").unwrap();
    writeln!(f, "P3\n{} {}\n255", width, height).unwrap();
    for h in (0..height).rev() {
        for w in 0..width {
            let mut rgb01 = Vec3::zeros();
            for _ in 0..samples {
                let u = (w as scalar + rng.gen::<scalar>())/width as scalar;
                let v = (h as scalar + rng.gen::<scalar>())/height as scalar;
                rgb01 += color(&my_world, &camera.generate_ray(u, v), 20);
            }
            rgb01 /= (samples as scalar);

            let mut rgb = Vec3::new(rgb01.x.powf(coeff), rgb01.y.powf(coeff), rgb01.z.powf(coeff));
            rgb = 255.99 * rgb;
            writeln!(f, "{} {} {}", rgb.x as i32, rgb.y as i32, rgb.z as i32).unwrap();
        }
    }
}
