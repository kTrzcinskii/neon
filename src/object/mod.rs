use std::ops::RangeInclusive;

use hittable_object::HittableObject;
use hittable_objects_list::HittableObjectsList;
use sphere::Sphere;

use crate::ray::Ray;

pub mod hittable_object;
pub mod hittable_objects_list;
pub mod sphere;

pub enum HittableObjectType {
    Sphere(Sphere),
    HittableObjectList(HittableObjectsList),
}

impl HittableObject for HittableObjectType {
    fn hit(&self, ray: &Ray, t_range: RangeInclusive<f64>) -> Option<hittable_object::HitRecord> {
        match self {
            HittableObjectType::Sphere(sphere) => sphere.hit(ray, t_range),
            HittableObjectType::HittableObjectList(hittable_objects_list) => {
                hittable_objects_list.hit(ray, t_range)
            }
        }
    }
}
