use std::path::Path;

use anyhow::Result;
use image::{ImageReader, RgbImage};
use nalgebra::Point3;
use rgb::Rgb;

use super::Texture;

#[derive(Clone)]
pub struct ImageTexture {
    img: RgbImage,
}

impl ImageTexture {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let img = ImageReader::open(path)?.decode()?.to_rgb8();
        Ok(Self { img })
    }
}

impl Texture for ImageTexture {
    fn color_at(&self, u: f64, v: f64, _: &Point3<f64>) -> Rgb<f64> {
        if self.img.height() == 0 || self.img.width() == 0 {
            panic!("invalid image in texture");
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let x = (u * self.img.width() as f64) as u32;
        let y = (v * self.img.height() as f64) as u32;

        let pixel_u8 = self.img.get_pixel(x, y);

        pixel_u8.0.iter().map(|&c| c as f64 / 255.0).collect()
    }
}
