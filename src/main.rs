mod ray;
mod vec;

use crate::ray::Ray;

use crate::vec::write_color;
use crate::vec::{Color, Point3, Vec3};

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin - center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(r: Ray) -> Color {
    let sphere_center = Point3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let t = hit_sphere(sphere_center, 0.5, r);
    if t > 0.0 {
        let normal_vector = (r.at(t) - sphere_center).unit_vector();
        return Color {
            x: normal_vector.x,
            y: normal_vector.y,
            z: normal_vector.z,
        };
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

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    // render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scan lines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray {
                origin,
                direction: lower_left_corner + horizontal * u + vertical * v - origin,
            };
            let c = ray_color(r);
            write_color(c);
        }
    }
}
