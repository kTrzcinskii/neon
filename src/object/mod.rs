use std::ops::RangeInclusive;

use hittable_object::HittableObject;
use hittable_objects_list::HittableObjectsList;
use moving_sphere::MovingSphere;
use sphere::Sphere;

use crate::{aabb::AxisAlignedBoundingBox, ray::Ray};

pub mod hittable_object;
pub mod hittable_objects_list;
pub mod moving_sphere;
pub mod sphere;

pub enum HittableObjectType {
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    HittableObjectList(HittableObjectsList),
}

impl HittableObject for HittableObjectType {
    fn hit(&self, ray: &Ray, t_range: RangeInclusive<f64>) -> Option<hittable_object::HitRecord> {
        match self {
            HittableObjectType::Sphere(sphere) => sphere.hit(ray, t_range),
            HittableObjectType::MovingSphere(moving_sphere) => moving_sphere.hit(ray, t_range),
            HittableObjectType::HittableObjectList(hittable_objects_list) => {
                hittable_objects_list.hit(ray, t_range)
            }
        }
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        match self {
            HittableObjectType::Sphere(sphere) => sphere.bounding_box(),
            HittableObjectType::MovingSphere(moving_sphere) => moving_sphere.bounding_box(),
            HittableObjectType::HittableObjectList(hittable_objects_list) => {
                hittable_objects_list.bounding_box()
            }
        }
    }
}
