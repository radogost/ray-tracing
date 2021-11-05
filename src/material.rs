use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::random;
use crate::vec::{Color, Vec3};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }
        let scattered_ray = Ray {
            origin: hit.point,
            direction: scatter_direction,
        };
        Some((scattered_ray, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray_in.direction.unit_vector().reflect(hit.normal);
        let reflected = reflected + Vec3::random_in_unit_sphere() * self.fuzz;
        if reflected.dot(hit.normal) > 0.0 {
            let scattered = Ray {
                origin: hit.point,
                direction: reflected,
            };
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // schlick's approximation
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let refraction_ratio = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = (1.0 as f64).min(-unit_direction.dot(hit.normal));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random() {
                unit_direction.reflect(hit.normal)
            } else {
                unit_direction.refract(hit.normal, refraction_ratio)
            };

        let scattered = Ray {
            origin: hit.point,
            direction,
        };

        Some((scattered, attenuation))
    }
}
