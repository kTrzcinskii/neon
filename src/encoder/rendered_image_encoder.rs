use crate::rendered_image::RenderedImage;

pub trait RenderedImageEncoder {
    fn encode(&self, image: &RenderedImage) -> Vec<u8>;
}
