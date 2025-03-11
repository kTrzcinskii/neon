use nalgebra::Point3;
use rgb::Rgb;

use crate::effects::perlin_noise::PerlinNoise;

use super::Texture;

pub struct NoiseTexture {
    perlin_noise: PerlinNoise,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            perlin_noise: PerlinNoise::default(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn color_at(&self, _: f64, _: f64, p: &Point3<f64>) -> Rgb<f64> {
        let position = self.scale * p;
        Rgb::new(1.0, 1.0, 1.0) * self.perlin_noise.noise(&position)
    }
}
