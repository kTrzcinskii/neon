use nalgebra::{RealField, UnitVector3, Vector3};

pub trait VectorReflectionExtension<T> {
    fn reflect(&self, normal: &UnitVector3<T>) -> Vector3<T>;
}

impl<T: RealField> VectorReflectionExtension<T> for Vector3<T> {
    fn reflect(&self, normal: &UnitVector3<T>) -> Vector3<T> {
        self - normal.as_ref() * (T::from_f64(2.0).unwrap() * self.dot(normal))
    }
}
