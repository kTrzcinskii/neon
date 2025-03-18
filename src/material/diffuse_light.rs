use nalgebra::Point3;
use rgb::Rgb;

use crate::{
    object::hittable_object::HitRecord,
    ray::Ray,
    texture::{solid_color::SolidColor, Texture, TextureType},
};

use super::{Material, MaterialScattering};

#[derive(Clone)]
pub struct DiffuseLight {
    texture: TextureType,
}

impl DiffuseLight {
    pub fn new(texture: TextureType) -> Self {
        Self { texture }
    }
}

impl From<Rgb<f64>> for DiffuseLight {
    fn from(value: Rgb<f64>) -> Self {
        let solid = SolidColor::new(value);
        DiffuseLight {
            texture: solid.into(),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<MaterialScattering> {
        None
    }

    fn emitted(&self, u: f64, v: f64, pos: &Point3<f64>) -> Rgb<f64> {
        self.texture.color_at(u, v, pos)
    }
}
