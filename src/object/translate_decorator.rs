use std::ops::RangeInclusive;

use nalgebra::Vector3;

use crate::{core::aabb::AxisAlignedBoundingBox, ray::Ray};

use super::{
    hittable_object::{HitRecord, HittableObject},
    HittableObjectType,
};

#[derive(Clone)]
pub struct TranslateDecorator {
    bbox: AxisAlignedBoundingBox,
    offset: Vector3<f64>,
    inner: Box<HittableObjectType>,
}

impl TranslateDecorator {
    pub fn new(inner: HittableObjectType, offset: Vector3<f64>) -> Self {
        let bbox = AxisAlignedBoundingBox::moved_by_offset(inner.bounding_box(), &offset);
        TranslateDecorator {
            inner: Box::new(inner),
            offset,
            bbox,
        }
    }

    pub fn inner(&self) -> &HittableObjectType {
        &self.inner
    }
}

impl HittableObject for TranslateDecorator {
    fn hit(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> Option<HitRecord> {
        // Instead of moving object we move ray (camera) in oposite direction
        // So we need to change ray origin
        let moved_ray = Ray::new(
            *ray.origin() - self.offset,
            ray.direction().into_inner(),
            ray.time(),
        );

        let hr = self.inner().hit(&moved_ray, t_range);

        match hr {
            Some(hit_record) => {
                // Move intersection point back to world coordinates
                let new_pos = *hit_record.pos() + self.offset;
                Some(HitRecord::with_changed_pos(new_pos, &hit_record))
            }
            None => None,
        }
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }
}
