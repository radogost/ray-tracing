use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

use std::ops::Range;
use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        let mut res = None;
        let mut closest_so_far = t_range.end;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_range.start..closest_so_far) {
                closest_so_far = hit.t;
                res = Some(hit);
            }
        }

        res
    }
}
