use nalgebra::Point3;
use rgb::Rgb;

use super::{NonRecursiveTexture, Texture};

pub struct CheckerTexture {
    inversed_scale: f64,
    even_texture: NonRecursiveTexture,
    odd_texture: NonRecursiveTexture,
}

impl CheckerTexture {
    pub fn new(
        scale: f64,
        even_texture: NonRecursiveTexture,
        odd_texture: NonRecursiveTexture,
    ) -> Self {
        Self {
            inversed_scale: 1.0 / scale,
            even_texture,
            odd_texture,
        }
    }
}

impl Texture for CheckerTexture {
    fn color_at(&self, u: f64, v: f64, p: &Point3<f64>) -> Rgb<f64> {
        let x = (p.x * self.inversed_scale).floor() as i64;
        let y = (p.y * self.inversed_scale).floor() as i64;
        let z = (p.z * self.inversed_scale).floor() as i64;

        let is_even = (x + y + z) & 1 == 0;

        match is_even {
            true => self.even_texture.color_at(u, v, p),
            false => self.odd_texture.color_at(u, v, p),
        }
    }
}
