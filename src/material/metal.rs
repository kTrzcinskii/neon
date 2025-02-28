use rgb::Rgb;

use crate::{
    object::hittable_object::HitRecord,
    ray::{ray_generator::RayGenerator, Ray},
};

use super::{Material, MaterialScattering};

pub struct Metal {
    albedo: Rgb<f64>,
}

impl Metal {
    pub fn new(albedo: Rgb<f64>) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialScattering> {
        let reflected_ray = RayGenerator::reflected_ray(ray, hit_record);
        let material_scattering = MaterialScattering {
            attenuation: self.albedo,
            scattered_ray: reflected_ray,
        };
        Some(material_scattering)
    }
}
