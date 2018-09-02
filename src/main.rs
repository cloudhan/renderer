#![allow(unused)]

mod ray;
mod math;
mod camera;
mod intersect;
mod primitive;

use std::fs::File;
use std::io::prelude::*;

use ray::*;
use math::*;
use camera::*;
use primitive::*;
use intersect::*;

extern crate rand;
use rand::prelude::*;

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    let mut p = Vec3::zeros();
    loop {
        p = 2.0 * Vec3::new(rng.gen::<scalar>(), rng.gen::<scalar>(),rng.gen::<scalar>()) - Vec3::new(1.0, 1.0, 1.0);
        if p.norm_squared() < 1.0 { break; };
    }
    return p;
}

fn color(world: &Vec<Box<Intersectable>>, ray: &Ray) -> Vec3 {
    let mut rng = thread_rng();
    let mut t = 0.0;
    let mut intersection = Intersection::new_dummy();
    if world.intersect(&ray, 1e-10, 255.0, &mut intersection) {
        let target = intersection.point() + intersection.normal() + random_in_unit_sphere(&mut rng);
        return  0.5 * color(world, &Ray::new(intersection.point(), target - intersection.point()))
    }
    else {
        t = 0.5 * ray.direction().y + 1.0;
        return (1.0-t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn world() -> Vec<Box<Intersectable>> {
    let mut w = Vec::<Box<Intersectable>>::new();
    w.push(Box::new(Sphere { center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5 } ));
    w.push(Box::new(Sphere { center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0}));
    return w;
}

fn main() {
    let width = 400;
    let height = 200;
    let samples = 50;

    let gamma = 2.2;
    let coeff = 1.0/gamma;

    //camera
    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizental = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);

    let my_world = world();

    let camera = Camera::new_default();
    let mut rng = thread_rng();

    let mut f = File::create("./output.ppm").unwrap();
    writeln!(f, "P3\n{} {}\n255", width, height).unwrap();
    for h in (0..height).rev() {
        for w in 0..width {
            let mut rgb01 = Vec3::zeros();
            for _ in 0..samples {
                let u = (w as scalar + rng.gen::<scalar>())/width as scalar;
                let v = (h as scalar + rng.gen::<scalar>())/height as scalar;
                rgb01 += color(&my_world, &camera.generate_ray(u, v));
            }
            rgb01 /= (samples as scalar);

            let mut rgb = Vec3::new(rgb01.x.powf(coeff), rgb01.y.powf(coeff), rgb01.z.powf(coeff));
            rgb = 255.99 * rgb;
            writeln!(f, "{} {} {}", rgb.x as i32, rgb.y as i32, rgb.z as i32).unwrap();
        }
    }
}
