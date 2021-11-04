use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

use std::ops::Range;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, normal: Vec3, t: f64, ray: &Ray) -> Self {
        let front_face = ray.direction.dot(normal) < 0.0;

        HitRecord {
            point,
            normal: if front_face { normal } else { -normal },
            t: t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord>;
}
