use std::ops::{Add, RangeInclusive};

pub trait RangeInclusiveMoveByOffsetExtension<T> {
    fn move_by_offset(&self, offset: T) -> Self;
}

impl<T> RangeInclusiveMoveByOffsetExtension<T> for RangeInclusive<T>
where
    T: Add<Output = T> + Copy + PartialOrd,
{
    fn move_by_offset(&self, offset: T) -> Self {
        let start = *self.start() + offset;
        let end = *self.end() + offset;
        start..=end
    }
}
