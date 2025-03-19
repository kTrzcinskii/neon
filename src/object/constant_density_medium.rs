use std::ops::RangeInclusive;

use nalgebra::{Unit, Vector3};
use rand::Rng;

use crate::{core::aabb::AxisAlignedBoundingBox, ray::Ray};

use super::{
    hittable_object::{HitRecord, HittableObject},
    HittableObjectType,
};

#[derive(Clone)]
pub struct ConstantDensityMedium {
    phase_function_id: usize,
    boundary: Box<HittableObjectType>,
    negative_inverse_density: f64,
}

impl ConstantDensityMedium {
    pub fn new(boundary: Box<HittableObjectType>, density: f64, phase_function_id: usize) -> Self {
        assert!(density.abs() > 0.0001);
        let negative_inverse_density = -1.0 / density;
        ConstantDensityMedium {
            phase_function_id,
            boundary,
            negative_inverse_density,
        }
    }

    pub fn boundary(&self) -> &HittableObjectType {
        &self.boundary
    }
}

impl HittableObject for ConstantDensityMedium {
    fn hit(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> Option<HitRecord> {
        let full_range = (-f64::MAX)..=f64::MAX;
        let hr1 = self.boundary().hit(ray, &full_range)?;

        let next_range = (hr1.t() + 0.0001)..=f64::MAX;
        let hr2 = self.boundary().hit(ray, &next_range)?;

        let r_min = hr1.t().max(*t_range.start()).max(0.0);
        let r_max = hr2.t().min(*t_range.end());

        if r_min >= r_max {
            return None;
        }

        let ray_length = ray.direction().norm();
        let distance_inside_boundary = (r_max - r_min) * ray_length;

        let mut rng = rand::rng();
        let hit_distance = self.negative_inverse_density * (rng.random::<f64>()).ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = r_min + hit_distance / ray_length;
        let pos = ray.at(t);
        let arbitrary_normal = Unit::new_unchecked(Vector3::new(1.0, 0.0, 0.0));

        let hit_record = HitRecord::new(
            pos,
            t,
            arbitrary_normal,
            ray,
            self.phase_function_id,
            0.0,
            0.0,
        );

        Some(hit_record)
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        self.boundary().bounding_box()
    }
}
