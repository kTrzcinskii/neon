use std::ops::RangeInclusive;

use nalgebra::{Point3, Unit, UnitVector3, Vector3};

use crate::{core::aabb::AxisAlignedBoundingBox, ray::Ray};

use super::hittable_object::{HitRecord, HittableObject};

/// `Quad` represents 2D quadrilateral (actually parallelogram, but quad sounds better).
/// Representation is as follows:
///
/// - `start` is the starting corner
///
/// - `start + u` is corner adjacent to `start`
///
/// - `start + v` is another corner adjacent to `start`
///
/// - `start + u + v` is the last corner
///
/// Quad lies on plane that is represented by equation: `Ax + By + Cz = D`
///
/// We know that `normal = (A, B, C)`, so we explicitly need to just store `D` parameter.
#[derive(Clone)]
pub struct Quad {
    start: Point3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    bounding_box: AxisAlignedBoundingBox,
    material_id: usize,
    normal: UnitVector3<f64>,
    plane_d: f64,
    w: Vector3<f64>,
}

impl Quad {
    pub fn new(start: Point3<f64>, u: Vector3<f64>, v: Vector3<f64>, material_id: usize) -> Self {
        let bb_diagonal_1 = AxisAlignedBoundingBox::new(start, start + u + v);
        let bb_diagonal_2 = AxisAlignedBoundingBox::new(start + u, start + v);
        let bounding_box = AxisAlignedBoundingBox::merge(&bb_diagonal_1, &bb_diagonal_2);
        let n = u.cross(&v);
        let w = n / n.dot(&n);
        let normal = Unit::new_normalize(n);
        let plane_d = normal.dot(&start.coords);
        Self {
            start,
            u,
            v,
            bounding_box,
            material_id,
            normal,
            plane_d,
            w,
        }
    }
}

impl HittableObject for Quad {
    fn hit(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction());

        // Ray is parallel to the plane on which quad lies
        const DELTA: f64 = 1e-8;
        if denom.abs() < DELTA {
            return None;
        }

        // Check if point is on the plane in time range
        let t = (self.plane_d - self.normal.dot(&ray.origin().coords)) / denom;
        if !t_range.contains(&t) {
            return None;
        }

        // Check if point lies inside quad
        let pos = ray.at(t);
        let hitpoint_vector = pos - self.start;
        let alpha = self.w.dot(&hitpoint_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&hitpoint_vector));

        // quad is from start to start + u + v
        // meaning that any point that satisfies start + alhpa * u + beta * v is in the quad,
        // where alpha and beta must be in range [0, 1]
        let coord_range = 0.0..=1.0;
        if !coord_range.contains(&alpha) || !coord_range.contains(&beta) {
            return None;
        }

        let hit_record = HitRecord::new(pos, t, self.normal, ray, self.material_id, alpha, beta);

        Some(hit_record)
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bounding_box
    }
}
