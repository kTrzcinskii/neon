use std::ops::RangeInclusive;

use nalgebra::{Point3, Unit, UnitVector3};

use crate::{extensions::ri_surrounds::RangeInclusiveSurroundsExtension, ray::Ray};

use super::hittable_object::{HitRecord, HittableObject};

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    material_id: usize,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material_id: usize) -> Self {
        assert!(radius > 0.0);
        Sphere {
            center,
            radius,
            material_id,
        }
    }

    pub fn center(&self) -> &Point3<f64> {
        &self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl HittableObject for Sphere {
    fn hit(&self, ray: &Ray, t_range: RangeInclusive<f64>) -> Option<HitRecord> {
        let oc = self.center() - ray.origin();
        let a = ray.direction().norm_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.norm_squared() - self.radius() * self.radius();
        let delta = h * h - a * c;
        if delta < 0.0 {
            return None;
        }
        let delta_sqrt = delta.sqrt();

        // Find nearest root that is in acceptable range
        let mut root = (h - delta_sqrt) / a;
        if !t_range.surrounds(&root) {
            root = (h - delta_sqrt) / a;
            if !t_range.surrounds(&root) {
                return None;
            }
        }

        let hit_point = ray.at(root);
        // This is unit vector thanks to dividing it by radius
        let outward_normal = (hit_point - self.center()) / self.radius();
        // We know its normalize so we skip checking part to gain some performance boost
        let unit_outward_normal: UnitVector3<f64> = Unit::new_unchecked(outward_normal);

        let hit_record =
            HitRecord::new(hit_point, root, unit_outward_normal, ray, self.material_id);
        Some(hit_record)
    }
}
