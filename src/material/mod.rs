pub mod lambertian;
pub mod metal;

use lambertian::Lambertian;
use metal::Metal;
use rgb::Rgb;

use crate::{object::hittable_object::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialScattering>;
}

pub enum MaterialType {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material for MaterialType {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialScattering> {
        match self {
            MaterialType::Lambertian(lambertian) => lambertian.scatter(ray, hit_record),
            MaterialType::Metal(metal) => metal.scatter(ray, hit_record),
        }
    }
}

pub struct MaterialScattering {
    attenuation: Rgb<f64>,
    scattered_ray: Ray,
}

impl MaterialScattering {
    pub fn new(attenuation: Rgb<f64>, scattered_ray: Ray) -> Self {
        MaterialScattering {
            attenuation,
            scattered_ray,
        }
    }

    pub fn attenuation(&self) -> &Rgb<f64> {
        &self.attenuation
    }

    pub fn scattered_ray(&self) -> &Ray {
        &self.scattered_ray
    }
}
