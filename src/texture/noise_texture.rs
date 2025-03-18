use nalgebra::Point3;
use rgb::Rgb;

use crate::effects::perlin_noise::PerlinNoise;

use super::Texture;

#[derive(Clone)]
pub struct NoiseTexture {
    perlin_noise: PerlinNoise,
    scale: f64,
    turbulance_depth: usize,
    turbulance_factor: f64,
}

impl NoiseTexture {
    const DEFAULT_TURBULANCE_DEPTH: usize = 7;
    const DEFAULT_TURBULANCE_FACTOR: f64 = 10.0;

    pub fn new(scale: f64) -> Self {
        Self {
            perlin_noise: PerlinNoise::default(),
            scale,
            turbulance_depth: Self::DEFAULT_TURBULANCE_DEPTH,
            turbulance_factor: Self::DEFAULT_TURBULANCE_FACTOR,
        }
    }

    pub fn with_turbulance_params(
        scale: f64,
        turbulance_depth: usize,
        turbulance_factor: f64,
    ) -> Self {
        Self {
            perlin_noise: PerlinNoise::default(),
            scale,
            turbulance_depth,
            turbulance_factor,
        }
    }
}

impl Texture for NoiseTexture {
    fn color_at(&self, _: f64, _: f64, p: &Point3<f64>) -> Rgb<f64> {
        let base_color = Rgb::new(0.5, 0.5, 0.5);
        let noise = (self.scale * p.z
            + self.turbulance_factor * self.perlin_noise.turbulance(p, self.turbulance_depth))
        .sin()
            + 1.0;
        base_color * noise
    }
}
