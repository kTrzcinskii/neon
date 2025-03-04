use std::ops::RangeInclusive;

use nalgebra::Point3;

use crate::ray::Ray;

pub struct AxisAlignedBoundingBox {
    interval_x: RangeInclusive<f64>,
    interval_y: RangeInclusive<f64>,
    interval_z: RangeInclusive<f64>,
}

impl AxisAlignedBoundingBox {
    pub fn new(start: Point3<f64>, end: Point3<f64>) -> Self {
        let interval_x = Self::find_interval(start.x, end.x);
        let interval_y = Self::find_interval(start.y, end.y);
        let interval_z = Self::find_interval(start.z, end.z);
        AxisAlignedBoundingBox {
            interval_x,
            interval_y,
            interval_z,
        }
    }

    pub fn empty() -> Self {
        let empty_interval = 0.0..=0.0;
        AxisAlignedBoundingBox {
            interval_x: empty_interval.clone(),
            interval_y: empty_interval.clone(),
            interval_z: empty_interval,
        }
    }

    fn find_interval(u: f64, v: f64) -> RangeInclusive<f64> {
        let start = u.min(v);
        let end = u.max(v);
        start..=end
    }

    pub fn intersects_ray(&self, ray: &Ray, t_range: RangeInclusive<f64>) -> bool {
        self.intersects_ray_in_axis(
            ray.direction().x,
            ray.origin().x,
            &t_range,
            &self.interval_x,
        ) || self.intersects_ray_in_axis(
            ray.direction().y,
            ray.origin().y,
            &t_range,
            &self.interval_y,
        ) || self.intersects_ray_in_axis(
            ray.direction().z,
            ray.origin().z,
            &t_range,
            &self.interval_z,
        )
    }

    fn intersects_ray_in_axis(
        &self,
        ray_direction: f64,
        ray_origin: f64,
        t_range: &RangeInclusive<f64>,
        axis: &RangeInclusive<f64>,
    ) -> bool {
        let t0 = (axis.start() - ray_origin) / ray_direction;
        let t1 = (axis.end() - ray_origin) / ray_direction;

        let (t_min, t_max) = if t0 < t1 {
            (t_range.start().max(t0), t_range.end().min(t1))
        } else {
            (t_range.start().max(t1), t_range.end().min(t0))
        };

        t_min < t_max
    }
}
