use std::{f64, ops::RangeInclusive};

use nalgebra::{Point3, Unit, UnitVector3, Vector3};

use crate::{
    core::aabb::AxisAlignedBoundingBox, extensions::ri_surrounds::RangeInclusiveSurroundsExtension,
    ray::Ray,
};

use super::hittable_object::{HitRecord, HittableObject};

#[derive(Clone)]
pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    material_id: usize,
    bounding_box: AxisAlignedBoundingBox,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material_id: usize) -> Self {
        assert!(radius > 0.0);
        let radius_vec = Vector3::new(radius, radius, radius);
        let bounding_box = AxisAlignedBoundingBox::new(center - radius_vec, center + radius_vec);
        Sphere {
            center,
            radius,
            material_id,
            bounding_box,
        }
    }

    pub fn center(&self) -> &Point3<f64> {
        &self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// It returns pair `(u, v)`, where
    /// `u` is value `[0, 1]` of angle around the Y axis from X=-1,
    /// `v` is value `[0, 1]` of angle from Y=-1 to Y=1.
    /// `pos` should be a vector from center to surface of the sphere of radius one.
    fn uv_coords(pos: &UnitVector3<f64>) -> (f64, f64) {
        let theta = (-pos.y).acos();
        let phi = (-pos.z).atan2(pos.x) + f64::consts::PI;
        let u = phi / (2.0 * f64::consts::PI);
        let v = theta / f64::consts::PI;
        (u, v)
    }
}

impl HittableObject for Sphere {
    fn hit(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> Option<HitRecord> {
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

        let (u, v) = Self::uv_coords(&unit_outward_normal);

        let hit_record = HitRecord::new(
            hit_point,
            root,
            unit_outward_normal,
            ray,
            self.material_id,
            u,
            v,
        );
        Some(hit_record)
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bounding_box
    }
}
