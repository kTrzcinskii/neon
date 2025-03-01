use nalgebra::{RealField, UnitVector3, Vector3};

pub trait VectorRefractionExtension<T> {
    fn refract(&self, normal: &UnitVector3<T>, etai_over_etat: f64) -> Vector3<T>;
}

impl<T: RealField> VectorRefractionExtension<T> for UnitVector3<T> {
    fn refract(&self, normal: &UnitVector3<T>, etai_over_etat: f64) -> Vector3<T> {
        let cos_theta = -self.dot(normal);
        let out_perpendicular =
            (self.as_ref() + normal.as_ref() * cos_theta) * T::from_f64(etai_over_etat).unwrap();
        let out_parallel = normal.as_ref()
            * (T::from_f64(1.0).unwrap() - out_perpendicular.norm_squared())
                .abs()
                .sqrt();
        out_parallel + out_perpendicular
    }
}
