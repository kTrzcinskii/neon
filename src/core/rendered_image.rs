use std::path::Path;

use anyhow::{bail, Error, Result};
use image::ColorType;
use rgb::Rgb;

#[derive(Clone, Copy, Default)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

impl Dimensions {
    pub fn from_width(width: u32, aspect_ratio: f64) -> Dimensions {
        let height = (width as f64 / aspect_ratio) as u32;
        let height = if height < 1 { 1 } else { height };
        Dimensions { width, height }
    }

    pub fn ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }

    pub fn all_elements(&self) -> u32 {
        self.width * self.height
    }
}

pub struct RenderedImage {
    /// `pixels` stores all pixels in the image, row by row
    pixels: Vec<Rgb<u8>>,
    dimensions: Dimensions,
}

impl RenderedImage {
    pub fn new(pixels: Vec<Rgb<u8>>, dimensions: Dimensions) -> Result<Self> {
        if pixels.len() != dimensions.all_elements() as _ {
            bail!("`pixels` len doesn't match dimensions");
        }
        Ok(Self { dimensions, pixels })
    }

    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.pixels)
    }

    pub fn save<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        image::save_buffer(
            path,
            self.as_bytes(),
            self.dimensions.width,
            self.dimensions.height,
            ColorType::Rgb8,
        )
        .map_err(Error::from)
    }
}
