use nalgebra::Point3;

use crate::ray::Ray;

use super::hittable_object::{HitRecord, HittableObject};

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Self {
        assert!(radius > 0.0);
        Sphere { center, radius }
    }

    pub fn center(&self) -> &Point3<f64> {
        &self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl HittableObject for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        let acceptable_range = t_min..=t_max;
        if !acceptable_range.contains(&root) {
            root = (h - delta_sqrt) / a;
            if !acceptable_range.contains(&root) {
                return None;
            }
        }

        let hit_point = ray.at(root);
        // This is unit vector thanks to dividing it by radius
        let outward_normal = (hit_point - self.center()) / self.radius();

        let hit_record = HitRecord::new(hit_point, root, outward_normal, ray);
        Some(hit_record)
    }
}
