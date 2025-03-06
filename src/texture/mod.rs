pub mod checker_texture;
pub mod solid_color;

use checker_texture::CheckerTexture;
use nalgebra::Point3;
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
}

impl Texture for NonRecursiveTexture {
    fn color_at(&self, u: f64, v: f64, p: &Point3<f64>) -> Rgb<f64> {
        match self {
            NonRecursiveTexture::SolidColor(solid_color) => solid_color.color_at(u, v, p),
        }
    }
}
