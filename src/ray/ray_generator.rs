use std::ops::Range;

use nalgebra::Vector3;
use rand::Rng;

use crate::{
    extensions::vector_reflection::VectorReflectionExtension, object::hittable_object::HitRecord,
};

use super::Ray;

pub struct RayGenerator;

impl RayGenerator {
    /// Returns a random `Ray` that is on the same hemisphere as
    /// the normal of the `hit_record`.
    pub fn random_ray_on_hemisphere(hit_record: &HitRecord) -> Ray {
        let mut direction = Self::random_unit_vector3();
        if direction.dot(hit_record.normal()) < 0.0 {
            direction = -direction;
        }
        direction += hit_record.normal().into_inner();
        // Fix case of "almost" zero vector
        if direction.iter().all(|x| x.abs() < 1e-8) {
            direction = hit_record.normal().into_inner();
        }
        Ray {
            origin: *hit_record.pos(),
            direction,
        }
    }

    pub fn reflected_ray(ray: &Ray, hit_record: &HitRecord) -> Ray {
        let reflected_direction = ray.direction().reflect(hit_record.normal());
        Ray {
            origin: *hit_record.pos(),
            direction: reflected_direction,
        }
    }

    pub fn fuzzed_ray(ray: &Ray, fuzziness: f64) -> Ray {
        if fuzziness == 0.0 {
            return *ray;
        }
        let fuzzed_direction =
            ray.direction().normalize() + fuzziness * Self::random_unit_vector3();
        Ray {
            direction: fuzzed_direction,
            origin: *ray.origin(),
        }
    }

    fn random_vector3(range: Range<f64>) -> Vector3<f64> {
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
    fn random_unit_vector3() -> Vector3<f64> {
        loop {
            let p = Self::random_vector3(-1.0..1.0);
            let len_squared = p.norm_squared();
            if (1e-160..=1.0).contains(&len_squared) {
                return p / len_squared.sqrt();
            }
        }
    }
}
