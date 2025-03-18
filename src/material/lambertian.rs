use rgb::Rgb;

use crate::{
    object::hittable_object::HitRecord,
    ray::{ray_generator, Ray},
    texture::{solid_color::SolidColor, NonRecursiveTexture, Texture, TextureType},
};

use super::{Material, MaterialScattering};

#[derive(Clone)]
pub struct Lambertian {
    texture: TextureType,
}

impl Lambertian {
    pub fn new(texture: TextureType) -> Self {
        Self { texture }
    }
}

impl From<Rgb<f64>> for Lambertian {
    fn from(value: Rgb<f64>) -> Self {
        let solid_color = SolidColor::new(value);
        let texture = TextureType::NonRecursive(NonRecursiveTexture::SolidColor(solid_color));
        Self::new(texture)
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialScattering> {
        let scattered_ray = ray_generator::random_ray_on_hemisphere(ray, hit_record);
        let attenuation = self
            .texture
            .color_at(hit_record.u(), hit_record.v(), hit_record.pos());
        let material_scattering = MaterialScattering::new(attenuation, scattered_ray);
        Some(material_scattering)
    }
}
