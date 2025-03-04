use std::ops::RangeInclusive;

use nalgebra::{Point3, Vector3};

use crate::{object::sphere::Sphere, ray::Ray};

use super::hittable_object::{HitRecord, HittableObject};

/// `MovingSphere` is like `Sphere` struct, with the additional ability to change sphere position
/// over time. It starts in position `from` and ends in position `to`, lineary moving from one to the another.
pub struct MovingSphere {
    from: Point3<f64>,
    direction: Vector3<f64>,
    radius: f64,
    material_id: usize,
}

impl MovingSphere {
    pub fn new(from: Point3<f64>, to: Point3<f64>, radius: f64, material_id: usize) -> Self {
        assert!(radius > 0.0);
        let direction = to - from;
        MovingSphere {
            from,
            direction,
            radius,
            material_id,
        }
    }

    pub fn center_at(&self, time: f64) -> Point3<f64> {
        self.from + time * self.direction
    }
}

impl HittableObject for MovingSphere {
    fn hit(&self, ray: &Ray, t_range: RangeInclusive<f64>) -> Option<HitRecord> {
        let sphere_at_time = Sphere::new(self.center_at(ray.time()), self.radius, self.material_id);
        sphere_at_time.hit(ray, t_range)
    }
}
