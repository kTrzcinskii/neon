pub mod checker_texture;
pub mod image_texture;
pub mod noise_texture;
pub mod solid_color;

use checker_texture::CheckerTexture;
use image_texture::ImageTexture;
use nalgebra::Point3;
use noise_texture::NoiseTexture;
use rgb::Rgb;
use solid_color::SolidColor;

pub trait Texture {
    fn color_at(&self, u: f64, v: f64, p: &Point3<f64>) -> Rgb<f64>;
}

pub enum TextureType {
    NonRecursive(NonRecursiveTexture),
    CheckerTexture(CheckerTexture),
}

impl Texture for TextureType {
    fn color_at(&self, u: f64, v: f64, p: &Point3<f64>) -> Rgb<f64> {
        match self {
            TextureType::NonRecursive(non_recursive_texture) => {
                non_recursive_texture.color_at(u, v, p)
            }
            TextureType::CheckerTexture(checker_texture) => checker_texture.color_at(u, v, p),
        }
    }
}

pub enum NonRecursiveTexture {
    SolidColor(SolidColor),
    ImageTexture(ImageTexture),
    NoiseTexture(NoiseTexture),
}

impl Texture for NonRecursiveTexture {
    fn color_at(&self, u: f64, v: f64, p: &Point3<f64>) -> Rgb<f64> {
        match self {
            NonRecursiveTexture::SolidColor(solid_color) => solid_color.color_at(u, v, p),
            NonRecursiveTexture::ImageTexture(image_texture) => image_texture.color_at(u, v, p),
            NonRecursiveTexture::NoiseTexture(noise_texture) => noise_texture.color_at(u, v, p),
        }
    }
}
