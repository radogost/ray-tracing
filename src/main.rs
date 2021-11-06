mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::random;
use crate::vec::write_color;
use crate::vec::{Color, Point3, Vec3};

use rand::Rng;

use std::rc::Rc;

fn ray_color(r: Ray, world: &HittableList, depth: u32) -> Color {
    if depth == 0 {
        return Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }
    if let Some(hit) = world.hit(&r, 0.001..f64::INFINITY) {
        if let Some((scattered, attenuation)) = hit.material.scatter(&r, &hit) {
            return ray_color(scattered, world, depth - 1) * attenuation;
        } else {
            return Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
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

fn scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Rc::new(Lambertian {
        albedo: Color {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
    });
    let ground = Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        material: ground_material,
    };
    world.add(Rc::new(ground));

    let refpoint = Point3 {
        x: 4.0,
        y: 0.2,
        z: 0.0,
    };

    let mut thread_rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Point3 {
                x: a as f64 + 0.9 * random(),
                y: 0.2,
                z: b as f64 + 0.9 * random(),
            };

            if (center - refpoint).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    //diffuse
                    let albedo: Color = Vec3::random() * Vec3::random();
                    Rc::new(Lambertian { albedo })
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo: Color = Vec3::random_between(0.5, 1.0);
                    let fuzz = thread_rng.gen_range(0.0..0.3);
                    Rc::new(Metal { albedo, fuzz })
                } else {
                    // glass
                    Rc::new(Dielectric {
                        refraction_index: 1.5,
                    })
                };
                let sphere = Sphere {
                    center,
                    radius: 0.2,
                    material: sphere_material,
                };
                world.add(Rc::new(sphere));
            }
        }
    }

    let glass = Dielectric {
        refraction_index: 1.5,
    };
    let glass_sphere = Sphere {
        center: Point3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: Rc::new(glass),
    };
    world.add(Rc::new(glass_sphere));

    let metal = Metal {
        albedo: Color {
            x: 0.7,
            y: 0.6,
            z: 0.5,
        },
        fuzz: 0.0,
    };
    let metal_sphere = Sphere {
        center: Point3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: Rc::new(metal),
    };
    world.add(Rc::new(metal_sphere));

    world
}

fn main() {
    // image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // world
    let world = scene();

    // camera
    let lookfrom = Point3 {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    let lookat = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let vup = Point3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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
