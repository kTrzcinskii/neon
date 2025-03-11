use std::ops::Range;

use nalgebra::{Unit, UnitVector2, UnitVector3, Vector2, Vector3};
use rand::Rng;

pub fn random_vector3(range: Range<f64>) -> Vector3<f64> {
    let mut rng = rand::rng();
    Vector3::new(
        rng.random_range(range.clone()),
        rng.random_range(range.clone()),
        rng.random_range(range),
    )
}

/// Returns Vector3<f64> that is inside a unit sphere.
/// We just keep generating random vectors inside 1-by-1-by cube
/// until we find one that satisfies us.
pub fn random_unit_vector3_in_sphere() -> UnitVector3<f64> {
    loop {
        let p = random_vector3(-1.0..1.0);
        let len_squared = p.norm_squared();
        if (1e-160..=1.0).contains(&len_squared) {
            return Unit::new_unchecked(p / len_squared.sqrt());
        }
    }
}

/// Works the same as `random_unit_vector3_in_sphere` but it's in 2D..
pub fn random_unit_vector2_in_disk() -> UnitVector2<f64> {
    loop {
        let p3 = random_vector3(-1.0..1.0);
        let p = Vector2::new(p3.x, p3.y);
        let len_squared = p.norm_squared();
        if (1e-160..=1.0).contains(&len_squared) {
            return Unit::new_unchecked(p);
        }
    }
}
