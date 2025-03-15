use std::ops::RangeInclusive;

use nalgebra::{Point3, UnitVector3};

use crate::{core::aabb::AxisAlignedBoundingBox, ray::Ray};

pub trait HittableObject {
    fn hit(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> Option<HitRecord>;
    fn bounding_box(&self) -> &AxisAlignedBoundingBox;
}

pub struct HitRecord {
    pos: Point3<f64>,
    /// This normal vector always points againt the ray
    normal: UnitVector3<f64>,
    t: f64,
    /// True if ray was outside the object
    front_face: bool,
    /// Index in array of `MaterialType`s stored in `Scene`
    material_id: usize,
    /// Horizontal texture coordinate of the hitten object
    u: f64,
    /// Vertical texture coordinate of the hitten object
    v: f64,
}

impl HitRecord {
    /// `outward_normal` is assumed to be unit vector
    pub fn new(
        pos: Point3<f64>,
        t: f64,
        outward_normal: UnitVector3<f64>,
        ray: &Ray,
        material_id: usize,
        u: f64,
        v: f64,
    ) -> Self {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            pos,
            normal,
            t,
            front_face,
            material_id,
            u,
            v,
        }
    }

    pub fn with_changed_pos(new_pos: Point3<f64>, other: &HitRecord) -> Self {
        HitRecord {
            pos: new_pos,
            normal: other.normal,
            t: other.t,
            front_face: other.front_face,
            material_id: other.material_id,
            u: other.u,
            v: other.v,
        }
    }

    pub fn with_changed_pos_and_normal(
        new_pos: Point3<f64>,
        new_normal: UnitVector3<f64>,
        other: &HitRecord,
    ) -> Self {
        HitRecord {
            pos: new_pos,
            normal: new_normal,
            t: other.t,
            front_face: other.front_face,
            material_id: other.material_id,
            u: other.u,
            v: other.v,
        }
    }

    pub fn pos(&self) -> &Point3<f64> {
        &self.pos
    }

    pub fn normal(&self) -> &UnitVector3<f64> {
        &self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn material_id(&self) -> usize {
        self.material_id
    }

    pub fn u(&self) -> f64 {
        self.u
    }

    pub fn v(&self) -> f64 {
        self.v
    }
}
