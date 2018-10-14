#![allow(unused)]

extern crate openexr;

extern crate rand;
use rand::prelude::*;

mod camera;
mod intersect;
mod material;
mod math;
mod primitive;
mod ray;

use std::fs::File;
use std::io::prelude::*;

use openexr::*;

use camera::*;
use intersect::*;
use material::*;
use math::*;
use primitive::*;
use ray::*;

use std::rc::Rc;

fn color(world: &Vec<Box<Intersectable>>, ray: &Ray, depth_remain: i32) -> Vec3 {
    let mut t = 0.0;
    match world.intersect(&ray, 1e-10, 25500.0) {
        Some(intersection) => {
            let mut scattered = Ray::new_dummy();
            let mut attenuation = Vec3::zeros();

            if (depth_remain > 0 && intersection.material().clone().unwrap().scatter(
                ray,
                &intersection,
                &mut attenuation,
                &mut scattered,
            )) {
                return attenuation.component_mul(&color(world, &scattered, depth_remain - 1));
            } else {
                return Vec3::zeros();
            }
        }
        None => {
            t = 0.5 * ray.direction().y + 1.0;
            return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
        }
    }
}

fn world() -> Vec<Box<Intersectable>> {
    let mut w = Vec::<Box<Intersectable>>::new();
    w.push(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
    }));

    w.push(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    }));

    w.push(Box::new(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0)),
    }));

    w.push(Box::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Dielectric::new(1.5)),
    }));

    w.push(Box::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: -0.45,
        material: Rc::new(Dielectric::new(1.5)),
    }));
    return w;
}

fn main() {
    let width = 400;
    let height = 200;
    let samples = 100;

    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizental = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);

    let my_world = world();

    let cam_position = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);

    let camera = Camera::new(
        &cam_position,
        &lookat,
        &Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (width as scalar) / (height as scalar),
        2.0,
        (lookat - cam_position).norm(),
    );
    let mut rng = thread_rng();

    let mut pixel_data = vec![(0.0f32, 0.0f32, 0.0f32); width * height];

    let mut file = std::fs::File::create("output.exr").unwrap();
    let mut output_file = ScanlineOutputFile::new(
        &mut file,
        Header::new()
            .set_resolution(width as u32, height as u32)
            .add_channel("R", PixelType::FLOAT)
            .add_channel("G", PixelType::FLOAT)
            .add_channel("B", PixelType::FLOAT),
    )
    .unwrap();

    for h in (0..height).rev() {
        for w in 0..width {
            let mut rgb01 = Vec3::zeros();
            for _ in 0..samples {
                let u = (w as scalar + rng.gen::<scalar>()) / width as scalar;
                let v = (h as scalar + rng.gen::<scalar>()) / height as scalar;
                rgb01 += color(&my_world, &camera.generate_ray(u, v), 20);
            }
            rgb01 /= (samples as scalar);

            pixel_data[(height - h -1) * width + w] = (rgb01.x as f32, rgb01.y as f32, rgb01.z as f32);
        }
    }

    let mut fb = FrameBuffer::new(width as u32, height as u32);
    fb.insert_channels(&["R", "G", "B"], &pixel_data);
    output_file.write_pixels(&fb).unwrap();
}
