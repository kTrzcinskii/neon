use std::ops::RangeInclusive;

use nalgebra::{Point3, Unit, Vector3};

use crate::{
    core::aabb::{Axis, AxisAlignedBoundingBox},
    ray::Ray,
};

use super::{
    hittable_object::{HitRecord, HittableObject},
    HittableObjectType,
};

#[derive(Clone)]
pub struct RotateYDecorator {
    bbox: AxisAlignedBoundingBox,
    inner: Box<HittableObjectType>,
    sin_theta: f64,
    cos_theta: f64,
}

impl RotateYDecorator {
    /// Important note: `angle` is in degrees
    pub fn new(inner: HittableObjectType, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let initial_bbox = inner.bounding_box();

        let mut min = Point3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = Point3::new(f64::MIN, f64::MIN, f64::MIN);

        (0..2).for_each(|i| {
            (0..2).for_each(|j| {
                (0..2).for_each(|k| {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;

                    let interval_x = initial_bbox.interval(Axis::X);
                    let x = i * interval_x.end() + (1.0 - i) * interval_x.start();

                    let interval_y = initial_bbox.interval(Axis::Y);
                    let y = j * interval_y.end() + (1.0 - j) * interval_y.start();

                    let interval_z = initial_bbox.interval(Axis::Z);
                    let z = k * interval_z.end() + (1.0 - k) * interval_z.start();

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let t = Vector3::new(new_x, y, new_z);

                    min.x = min.x.min(t.x);
                    max.x = max.x.max(t.x);

                    min.y = min.y.min(t.y);
                    max.y = max.y.max(t.y);

                    min.z = min.z.min(t.z);
                    max.z = max.z.max(t.z);
                });
            });
        });

        let final_bbox = AxisAlignedBoundingBox::new(min, max);

        RotateYDecorator {
            bbox: final_bbox,
            inner: Box::new(inner),
            cos_theta,
            sin_theta,
        }
    }

    pub fn inner(&self) -> &HittableObjectType {
        &self.inner
    }
}

impl HittableObject for RotateYDecorator {
    fn hit(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> Option<HitRecord> {
        // Change ray coords from world space to object space
        let origin = Point3::new(
            self.cos_theta * ray.origin().x - self.sin_theta * ray.origin().z,
            ray.origin().y,
            self.sin_theta * ray.origin().x + self.cos_theta * ray.origin().z,
        );

        let direction = Vector3::new(
            self.cos_theta * ray.direction().x - self.sin_theta * ray.direction().z,
            ray.direction().y,
            self.sin_theta * ray.direction().x + self.cos_theta * ray.direction().z,
        );

        let rotated_ray = Ray::new(origin, direction, ray.time());

        let hr = self.inner().hit(&rotated_ray, t_range);

        match hr {
            Some(hit_record) => {
                // Position back from object space to world space
                let new_pos = Point3::new(
                    self.cos_theta * hit_record.pos().x + self.sin_theta * hit_record.pos().z,
                    hit_record.pos().y,
                    -self.sin_theta * hit_record.pos().x + self.cos_theta * hit_record.pos().z,
                );

                let new_normal = Unit::new_normalize(Vector3::new(
                    self.cos_theta * hit_record.normal().x + self.sin_theta * hit_record.normal().z,
                    hit_record.normal().y,
                    -self.sin_theta * hit_record.normal().x
                        + self.cos_theta * hit_record.normal().z,
                ));

                Some(HitRecord::with_changed_pos_and_normal(
                    new_pos,
                    new_normal,
                    &hit_record,
                ))
            }
            None => None,
        }
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }
}
