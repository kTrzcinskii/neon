use std::ops::RangeInclusive;

use super::{
    hittable_object::{HitRecord, HittableObject},
    HittableObjectType,
};

pub struct HittableObjectsList<'a> {
    items: Vec<HittableObjectType<'a>>,
}

impl<'a> HittableObjectsList<'a> {
    pub fn new() -> Self {
        HittableObjectsList { items: vec![] }
    }

    pub fn add(&mut self, item: HittableObjectType<'a>) {
        self.items.push(item);
    }
}

impl HittableObject for HittableObjectsList<'_> {
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

impl<'a> From<Vec<HittableObjectType<'a>>> for HittableObjectsList<'a> {
    fn from(value: Vec<HittableObjectType<'a>>) -> Self {
        HittableObjectsList { items: value }
    }
}

impl Default for HittableObjectsList<'_> {
    fn default() -> Self {
        Self::new()
    }
}
