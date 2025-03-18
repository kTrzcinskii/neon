use rgb::Rgb;

use crate::{
    object::hittable_object::HitRecord,
    ray::{ray_generator, Ray},
    texture::{solid_color::SolidColor, Texture, TextureType},
};

use super::{Material, MaterialScattering};

#[derive(Clone)]
pub struct Isotropic {
    texture: TextureType,
}

impl Isotropic {
    pub fn new(texture: TextureType) -> Self {
        Self { texture }
    }
}

impl From<Rgb<f64>> for Isotropic {
    fn from(value: Rgb<f64>) -> Self {
        let albedo = SolidColor::new(value);
        Self {
            texture: albedo.into(),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialScattering> {
        let scattered_ray = ray_generator::ray_in_random_unit_direction(ray, hit_record);
        let attenuation = self
            .texture
            .color_at(hit_record.u(), hit_record.v(), hit_record.pos());
        Some(MaterialScattering::new(attenuation, scattered_ray))
    }
}
