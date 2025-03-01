use rgb::Rgb;

use crate::{
    object::hittable_object::HitRecord,
    ray::{ray_generator::RayGenerator, Ray},
};

use super::{Material, MaterialScattering};

pub struct Metal {
    albedo: Rgb<f64>,
    /// Reflected ray can be randomized by fuzziness factor,
    /// which is just a radius of sphere inside which ray will end up.
    /// Higher the value stronger the effect, so set to 0.0 if it's not desired.
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Rgb<f64>, fuzziness: f64) -> Self {
        assert!(fuzziness >= 0.0);
        Metal { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialScattering> {
        let reflected_ray = RayGenerator::reflected_ray(ray, hit_record);
        let scattered_ray = RayGenerator::fuzzed_ray(&reflected_ray, self.fuzziness);
        let material_scattering = MaterialScattering::new(self.albedo, scattered_ray);
        // In case when fuzzed ray gets below surface we just don't return it
        // (so surface absorbs it).
        if scattered_ray.direction().dot(hit_record.normal()) > 0.0 {
            Some(material_scattering)
        } else {
            None
        }
    }
}
