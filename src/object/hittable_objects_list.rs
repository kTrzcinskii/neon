use std::ops::RangeInclusive;

use super::hittable_object::{HitRecord, HittableObject};

pub struct HittableObjectsList {
    items: Vec<Box<dyn HittableObject>>,
}

impl HittableObjectsList {
    pub fn new() -> Self {
        HittableObjectsList { items: vec![] }
    }

    pub fn add(&mut self, item: Box<dyn HittableObject>) {
        self.items.push(item);
    }
}

impl HittableObject for HittableObjectsList {
    fn hit(&self, ray: &crate::ray::Ray, t_range: RangeInclusive<f64>) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t = *t_range.end();

        for item in &self.items {
            let obj_hit = item.hit(ray, *t_range.start()..=closest_t);
            if let Some(hit_record) = obj_hit {
                closest_t = hit_record.t();
                closest_hit = Some(hit_record);
            }
        }

        closest_hit
    }
}

impl From<Vec<Box<dyn HittableObject>>> for HittableObjectsList {
    fn from(value: Vec<Box<dyn HittableObject>>) -> Self {
        HittableObjectsList { items: value }
    }
}

impl Default for HittableObjectsList {
    fn default() -> Self {
        Self::new()
    }
}
