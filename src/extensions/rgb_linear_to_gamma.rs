use rgb::Rgb;

pub trait RgbLinearToGammaExtension {
    fn linear_to_gamma(&self) -> Self;
}

impl RgbLinearToGammaExtension for Rgb<f64> {
    fn linear_to_gamma(&self) -> Self {
        self.iter().map(|c| c.sqrt()).collect()
    }
}
