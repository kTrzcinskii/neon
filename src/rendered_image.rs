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
    pub pixels: Vec<Vec<Rgb<u8>>>,
    pub dimensions: Dimensions,
}
