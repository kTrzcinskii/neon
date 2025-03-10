use nalgebra::Point3;
use rgb::Rgb;

use crate::effects::perlin_noise::PerlinNoise;

use super::Texture;

pub struct NoiseTexture {
    perlin_noise: PerlinNoise,
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            perlin_noise: PerlinNoise::default(),
        }
    }
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self::new()
    }
}

impl Texture for NoiseTexture {
    fn color_at(&self, _: f64, _: f64, p: &Point3<f64>) -> Rgb<f64> {
        Rgb::new(1.0, 1.0, 1.0) * self.perlin_noise.noise(p)
    }
}
