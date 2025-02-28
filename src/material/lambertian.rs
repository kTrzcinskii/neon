use rgb::Rgb;

use crate::{
    object::hittable_object::HitRecord,
    ray::{ray_generator::RayGenerator, Ray},
};

use super::{Material, MaterialScattering};

pub struct Lambertian {
    albedo: Rgb<f64>,
}

impl Lambertian {
    pub fn new(albedo: Rgb<f64>) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<MaterialScattering> {
        let scattered_ray = RayGenerator::random_ray_on_hemisphere(hit_record);
        let material_scattering = MaterialScattering {
            attenuation: self.albedo,
            scattered_ray,
        };
        Some(material_scattering)
    }
}
