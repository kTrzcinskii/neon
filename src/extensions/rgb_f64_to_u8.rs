use rgb::Rgb;

pub trait RgbF64ToU8Extension {
    fn f64_to_u8(&self) -> Rgb<u8>;
}

impl RgbF64ToU8Extension for Rgb<f64> {
    fn f64_to_u8(&self) -> Rgb<u8> {
        self.iter().map(|c| (c * 255_f64) as u8).collect()
    }
}
