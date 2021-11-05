mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::random;
use crate::vec::write_color;
use crate::vec::{Color, Point3, Vec3};

use std::rc::Rc;

fn ray_color(r: Ray, world: &HittableList, depth: u32) -> Color {
    if depth <= 0 {
        return Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    if let Some(hit) = world.hit(&r, 0.001..f64::INFINITY) {
        let target = hit.point + hit.normal + Vec3::random_unit_vector();
        let ray = Ray {
            origin: hit.point,
            direction: target - hit.point,
        };
        return ray_color(ray, world, depth - 1) * 0.5;
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let start_color = Color {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let end_color = Color {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    };
    start_color * (1.0 - t) + end_color * t
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 50;
    let max_depth = 50;

    // world
    let mut world = HittableList::new();
    let small_sphere = Sphere {
        center: Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    };
    let big_sphere = Sphere {
        center: Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    };
    world.add(Rc::new(small_sphere));
    world.add(Rc::new(big_sphere));

    // camera
    let camera = Camera::new();

    // render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scan lines remaining: {}", j);
        for i in 0..image_width {
            let mut color = Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random()) / (image_width - 1) as f64;
                let v = (j as f64 + random()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                color = color + ray_color(ray, &world, max_depth);
            }
            write_color(color, samples_per_pixel);
        }
    }
}
