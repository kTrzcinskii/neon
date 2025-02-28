use std::ops::RangeInclusive;

use nalgebra::{Point3, UnitVector3};

use crate::{material::MaterialType, ray::Ray};

pub trait HittableObject {
    fn hit(&self, ray: &Ray, t_range: RangeInclusive<f64>) -> Option<HitRecord>;
}

pub struct HitRecord<'a> {
    pos: Point3<f64>,
    /// This normal vector always points againt the ray
    normal: UnitVector3<f64>,
    t: f64,
    /// True if ray was outside the object
    front_face: bool,
    material: &'a MaterialType,
}

impl<'a> HitRecord<'a> {
    /// `outward_normal` is assumed to be unit vector
    pub fn new(
        pos: Point3<f64>,
        t: f64,
        outward_normal: UnitVector3<f64>,
        ray: &Ray,
        material: &'a MaterialType,
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
            material,
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

    pub fn material_type(&self) -> &MaterialType {
        self.material
    }
}
