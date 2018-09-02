mod math;
mod primitive;
mod ray;

use std::fs::File;
use std::io::prelude::*;

use math::*;
use primitive::*;

fn color(world: &Box<Intersectable>, ray: &Ray) -> RGB {
    let mut t = 0.0;
    if world.intersect(&ray, &mut t) {
        return RGB::new(1.0, 0.0, 0.0);
    }
    else{
        t = 0.5 * ray.direction.y + 1.0;
        return (1.0-t) * RGB::new(1.0, 1.0, 1.0) + t * RGB::new(0.5, 0.7, 1.0);
    }




}

fn world() -> Box<Intersectable> {

    return Box::new(Sphere { center: Point::new(0.0, 0.0, -1.0), radius: 0.5 } );
}

fn main() {
    let width = 200;
    let height = 100;

    //camera
    let lower_left = Point::new(-2.0, -1.0, -1.0);
    let horizental = Point::new(4.0, 0.0, 0.0);
    let vertical = Point::new(0.0, 2.0, 0.0);

    let my_world = world();

    let mut f = File::create("./output.ppm").unwrap();
    writeln!(f, "P3\n{} {}\n255", width, height).unwrap();
    for h in (0..height).rev() {
        for w in 0..width {
            let u = w as scalar/width as scalar;
            let v = h as scalar/height as scalar;
            let rgb = 255.99 * color(&my_world, &Ray {
                origin: Point::new(0.0, 0.0, 0.0),
                direction: (lower_left + u*horizental + v*vertical).normalize()
            });

            writeln!(f, "{} {} {}", rgb.x as i32, rgb.y as i32, rgb.z as i32).unwrap();
        }
    }
}
