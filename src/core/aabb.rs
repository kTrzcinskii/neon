use std::{cmp::Ordering, ops::RangeInclusive};

use nalgebra::Point3;

use crate::ray::Ray;

#[derive(Clone)]
pub struct AxisAlignedBoundingBox {
    interval_x: RangeInclusive<f64>,
    interval_y: RangeInclusive<f64>,
    interval_z: RangeInclusive<f64>,
}

pub enum Axis {
    X,
    Y,
    Z,
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

    /// Merges two `AxisAlignedBoundingBox`s into one containing them both.
    /// Created `AxisAlignedBoundingBox` is as small as possible.
    pub fn merge(b1: &AxisAlignedBoundingBox, b2: &AxisAlignedBoundingBox) -> Self {
        let interval_x = Self::merge_ranges(&b1.interval_x, &b2.interval_x);
        let interval_y = Self::merge_ranges(&b1.interval_y, &b2.interval_y);
        let interval_z = Self::merge_ranges(&b1.interval_z, &b2.interval_z);
        Self {
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

    pub fn longest_axis(&self) -> Axis {
        let len_x = self.interval_x.end() - self.interval_x.start();
        let len_y = self.interval_y.end() - self.interval_y.start();
        let len_z = self.interval_z.end() - self.interval_z.start();
        if len_x > len_y {
            if len_x > len_z {
                Axis::X
            } else {
                Axis::Z
            }
        } else if len_y > len_z {
            Axis::Y
        } else {
            Axis::Z
        }
    }

    fn find_interval(u: f64, v: f64) -> RangeInclusive<f64> {
        let start = u.min(v);
        let end = u.max(v);
        start..=end
    }

    pub fn intersects_ray(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> bool {
        let mut t_min = *t_range.start();
        let mut t_max = *t_range.end();

        let (t_min_x, t_max_x) =
            self.intersects_ray_in_axis(ray.direction().x, ray.origin().x, &Axis::X);

        t_min = t_min.max(t_min_x);
        t_max = t_max.min(t_max_x);

        if t_max <= t_min {
            return false;
        }

        let (t_min_y, t_max_y) =
            self.intersects_ray_in_axis(ray.direction().y, ray.origin().y, &Axis::Y);

        t_min = t_min.max(t_min_y);
        t_max = t_max.min(t_max_y);

        if t_max <= t_min {
            return false;
        }

        let (t_min_z, t_max_z) =
            self.intersects_ray_in_axis(ray.direction().z, ray.origin().z, &Axis::Z);

        t_min = t_min.max(t_min_z);
        t_max = t_max.min(t_max_z);

        t_max > t_min
    }

    pub fn compare_by_axis(
        b1: &AxisAlignedBoundingBox,
        b2: &AxisAlignedBoundingBox,
        axis: &Axis,
    ) -> Ordering {
        match axis {
            Axis::X => Self::compare_by_x(b1, b2),
            Axis::Y => Self::compare_by_y(b1, b2),
            Axis::Z => Self::compare_by_z(b1, b2),
        }
    }

    fn compare_by_x(b1: &AxisAlignedBoundingBox, b2: &AxisAlignedBoundingBox) -> Ordering {
        b1.interval_x.start().total_cmp(b2.interval_x.start())
    }

    fn compare_by_y(b1: &AxisAlignedBoundingBox, b2: &AxisAlignedBoundingBox) -> Ordering {
        b1.interval_y.start().total_cmp(b2.interval_y.start())
    }

    fn compare_by_z(b1: &AxisAlignedBoundingBox, b2: &AxisAlignedBoundingBox) -> Ordering {
        b1.interval_z.start().total_cmp(b2.interval_z.start())
    }

    /// Returns points (t0, t1) in which ray intersects with axis
    /// we can assume that `t0 < t1`
    fn intersects_ray_in_axis(
        &self,
        ray_direction: f64,
        ray_origin: f64,
        axis: &Axis,
    ) -> (f64, f64) {
        let interval = match axis {
            Axis::X => &self.interval_x,
            Axis::Y => &self.interval_y,
            Axis::Z => &self.interval_z,
        };

        let t0 = (interval.start() - ray_origin) / ray_direction;
        let t1 = (interval.end() - ray_origin) / ray_direction;

        let t_min = t0.min(t1);
        let t_max = t0.max(t1);
        (t_min, t_max)
    }

    /// Creates the smalles range that contains both ranges
    fn merge_ranges(r1: &RangeInclusive<f64>, r2: &RangeInclusive<f64>) -> RangeInclusive<f64> {
        let start = r1.start().min(*r2.start());
        let end = r1.end().max(*r2.end());
        start..=end
    }
}
