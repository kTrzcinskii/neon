use crate::rendered_image::RenderedImage;

use super::rendered_image_encoder::RenderedImageEncoder;

pub struct PpmEncoder {
    max_color: u8,
}

impl PpmEncoder {
    pub fn new(max_color: u8) -> Self {
        PpmEncoder { max_color }
    }
}

impl RenderedImageEncoder for PpmEncoder {
    fn encode(&self, image: &RenderedImage) -> Vec<u8> {
        let mut content = String::new();
        let headline = format!(
            "P3\n{} {}\n{}\n",
            image.dimensions.width, image.dimensions.height, self.max_color
        );
        content.push_str(&headline);

        for row in &image.pixels {
            for pixel in row {
                let line = format!("{} {} {}\n", pixel.r, pixel.g, pixel.b);
                content.push_str(&line);
            }
        }

        content.bytes().collect()
    }
}
