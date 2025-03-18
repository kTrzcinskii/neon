pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

use dielectric::Dielectric;
use diffuse_light::DiffuseLight;
use isotropic::Isotropic;
use lambertian::Lambertian;
use metal::Metal;
use nalgebra::Point3;
use rgb::Rgb;

use crate::{object::hittable_object::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialScattering>;
    fn emitted(&self, _: f64, _: f64, _: &Point3<f64>) -> Rgb<f64> {
        // By default material emmits no light - just black color
        Rgb::new(0.0, 0.0, 0.0)
    }
}

pub enum MaterialType {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Isotropic(Isotropic),
}

impl Material for MaterialType {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialScattering> {
        match self {
            MaterialType::Lambertian(lambertian) => lambertian.scatter(ray, hit_record),
            MaterialType::Metal(metal) => metal.scatter(ray, hit_record),
            MaterialType::Dielectric(dielectric) => dielectric.scatter(ray, hit_record),
            MaterialType::DiffuseLight(diffuse_light) => diffuse_light.scatter(ray, hit_record),
            MaterialType::Isotropic(isotropic) => isotropic.scatter(ray, hit_record),
        }
    }

    fn emitted(&self, u: f64, v: f64, pos: &Point3<f64>) -> Rgb<f64> {
        match self {
            MaterialType::Lambertian(lambertian) => lambertian.emitted(u, v, pos),
            MaterialType::Metal(metal) => metal.emitted(u, v, pos),
            MaterialType::Dielectric(dielectric) => dielectric.emitted(u, v, pos),
            MaterialType::DiffuseLight(diffuse_light) => diffuse_light.emitted(u, v, pos),
            MaterialType::Isotropic(isotropic) => isotropic.emitted(u, v, pos),
        }
    }
}

impl From<Lambertian> for MaterialType {
    fn from(value: Lambertian) -> Self {
        Self::Lambertian(value)
    }
}

impl From<Metal> for MaterialType {
    fn from(value: Metal) -> Self {
        Self::Metal(value)
    }
}

impl From<Dielectric> for MaterialType {
    fn from(value: Dielectric) -> Self {
        Self::Dielectric(value)
    }
}

impl From<DiffuseLight> for MaterialType {
    fn from(value: DiffuseLight) -> Self {
        Self::DiffuseLight(value)
    }
}

impl From<Isotropic> for MaterialType {
    fn from(value: Isotropic) -> Self {
        Self::Isotropic(value)
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
