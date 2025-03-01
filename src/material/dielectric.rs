use rgb::Rgb;

use crate::{
    object::hittable_object::HitRecord,
    ray::{ray_generator::RayGenerator, Ray},
};

use super::{Material, MaterialScattering};

pub struct Dielectric {
    /// Refraction index in vacuum (or air), in case if material is inside enclosing media with
    /// its refraction index != 1, then this property should be a ratio of material's refractio index
    /// over media's one.
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialScattering> {
        let attenuation = Rgb::new(1.0, 1.0, 1.0);
        // We try to refract the ray, if we cant it means that there is no solution (theta > 90)
        // and we should reflect it.
        let scattered_ray = if let Some(refracted_ray) =
            RayGenerator::refracted_ray(ray, hit_record, self.refraction_index)
        {
            refracted_ray
        } else {
            RayGenerator::reflected_ray(ray, hit_record)
        };
        let material_scattering = MaterialScattering::new(attenuation, scattered_ray);
        Some(material_scattering)
    }
}
