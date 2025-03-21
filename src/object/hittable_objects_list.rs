use std::ops::RangeInclusive;

use crate::core::aabb::AxisAlignedBoundingBox;

use super::{
    hittable_object::{HitRecord, HittableObject},
    HittableObjectType,
};

#[derive(Clone)]
pub struct HittableObjectsList {
    items: Vec<HittableObjectType>,
    bounding_box: AxisAlignedBoundingBox,
}

impl HittableObjectsList {
    pub fn new() -> Self {
        HittableObjectsList {
            items: vec![],
            bounding_box: AxisAlignedBoundingBox::empty(),
        }
    }

    pub fn add(&mut self, item: HittableObjectType) {
        self.bounding_box = AxisAlignedBoundingBox::merge(&self.bounding_box, item.bounding_box());
        self.items.push(item);
    }

    pub fn items(&self) -> &[HittableObjectType] {
        &self.items
    }

    /// Returns flat vector of all elements stored inside `HittableObjectsList`
    pub fn items_flat(&self) -> Vec<HittableObjectType> {
        Self::flatten_hittable_objects_list(self.items())
    }

    /// Returns flat vector of all elements stored inside `objs`
    pub fn flatten_hittable_objects_list(objs: &[HittableObjectType]) -> Vec<HittableObjectType> {
        objs.iter()
            .flat_map(|item| match item {
                HittableObjectType::HittableObjectList(list) => list.items_flat(),
                _ => vec![item.clone()],
            })
            .collect::<Vec<_>>()
    }
}

impl HittableObject for HittableObjectsList {
    fn hit(&self, ray: &crate::ray::Ray, t_range: &RangeInclusive<f64>) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t = *t_range.end();

        for item in &self.items {
            let range = *t_range.start()..=closest_t;
            let obj_hit = item.hit(ray, &range);
            if let Some(hit_record) = obj_hit {
                closest_t = hit_record.t();
                closest_hit = Some(hit_record);
            }
        }

        closest_hit
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bounding_box
    }
}

impl From<Vec<HittableObjectType>> for HittableObjectsList {
    fn from(value: Vec<HittableObjectType>) -> Self {
        let bounding_box = value
            .iter()
            .fold(AxisAlignedBoundingBox::empty(), |acc, ho| {
                AxisAlignedBoundingBox::merge(&acc, ho.bounding_box())
            });
        HittableObjectsList {
            items: value,
            bounding_box,
        }
    }
}

impl Default for HittableObjectsList {
    fn default() -> Self {
        Self::new()
    }
}
