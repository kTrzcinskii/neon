use std::ops::RangeInclusive;

use hittable_object::HittableObject;
use hittable_objects_list::HittableObjectsList;
use moving_sphere::MovingSphere;
use quad::Quad;
use rotate_y_decorator::RotateYDecorator;
use sphere::Sphere;
use translate_decorator::TranslateDecorator;

use crate::{core::aabb::AxisAlignedBoundingBox, ray::Ray};

pub mod hittable_object;
pub mod hittable_objects_list;
pub mod moving_sphere;
pub mod quad;
pub mod rotate_y_decorator;
pub mod sphere;
pub mod translate_decorator;

#[derive(Clone)]
pub enum HittableObjectType {
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    Quad(Quad),
    HittableObjectList(HittableObjectsList),
    TranslateDecorator(TranslateDecorator),
    RotateYDecorator(RotateYDecorator),
}

impl HittableObject for HittableObjectType {
    fn hit(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> Option<hittable_object::HitRecord> {
        match self {
            HittableObjectType::Sphere(sphere) => sphere.hit(ray, t_range),
            HittableObjectType::MovingSphere(moving_sphere) => moving_sphere.hit(ray, t_range),
            HittableObjectType::Quad(quad) => quad.hit(ray, t_range),
            HittableObjectType::HittableObjectList(hittable_objects_list) => {
                hittable_objects_list.hit(ray, t_range)
            }
            HittableObjectType::TranslateDecorator(translate_decorator) => {
                translate_decorator.hit(ray, t_range)
            }
            HittableObjectType::RotateYDecorator(rotate_y_decorator) => {
                rotate_y_decorator.hit(ray, t_range)
            }
        }
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        match self {
            HittableObjectType::Sphere(sphere) => sphere.bounding_box(),
            HittableObjectType::MovingSphere(moving_sphere) => moving_sphere.bounding_box(),
            HittableObjectType::Quad(quad) => quad.bounding_box(),
            HittableObjectType::HittableObjectList(hittable_objects_list) => {
                hittable_objects_list.bounding_box()
            }
            HittableObjectType::TranslateDecorator(translate_decorator) => {
                translate_decorator.bounding_box()
            }
            HittableObjectType::RotateYDecorator(rotate_y_decorator) => {
                rotate_y_decorator.bounding_box()
            }
        }
    }
}

impl From<Sphere> for HittableObjectType {
    fn from(value: Sphere) -> Self {
        HittableObjectType::Sphere(value)
    }
}

impl From<MovingSphere> for HittableObjectType {
    fn from(value: MovingSphere) -> Self {
        HittableObjectType::MovingSphere(value)
    }
}

impl From<Quad> for HittableObjectType {
    fn from(value: Quad) -> Self {
        HittableObjectType::Quad(value)
    }
}

impl From<HittableObjectsList> for HittableObjectType {
    fn from(value: HittableObjectsList) -> Self {
        HittableObjectType::HittableObjectList(value)
    }
}

impl From<TranslateDecorator> for HittableObjectType {
    fn from(value: TranslateDecorator) -> Self {
        HittableObjectType::TranslateDecorator(value)
    }
}

impl From<RotateYDecorator> for HittableObjectType {
    fn from(value: RotateYDecorator) -> Self {
        HittableObjectType::RotateYDecorator(value)
    }
}
