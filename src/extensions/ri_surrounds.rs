use std::ops::RangeInclusive;

pub trait RangeInclusiveSurroundsExtension<T> {
    fn surrounds(&self, value: &T) -> bool;
}

impl<T: PartialOrd> RangeInclusiveSurroundsExtension<T> for RangeInclusive<T> {
    fn surrounds(&self, value: &T) -> bool {
        value > self.start() && value < self.end()
    }
}
