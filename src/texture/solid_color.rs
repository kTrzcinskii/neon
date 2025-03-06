use nalgebra::Point3;
use rgb::Rgb;

use super::Texture;

pub struct SolidColor {
    albedo: Rgb<f64>,
}

impl SolidColor {
    pub fn new(albedo: Rgb<f64>) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn color_at(&self, _: f64, _: f64, _: &Point3<f64>) -> Rgb<f64> {
        self.albedo
    }
}
