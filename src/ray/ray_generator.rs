use rand::Rng;

use crate::{
    extensions::{
        vector_reflection::VectorReflectionExtension, vector_refraction::VectorRefractionExtension,
    },
    object::hittable_object::HitRecord,
    random_vector_generator,
};

use super::Ray;

/// Returns a random `Ray` that is on the same hemisphere as
/// the normal of the `hit_record`.
pub fn random_ray_on_hemisphere(ray: &Ray, hit_record: &HitRecord) -> Ray {
    let mut direction = random_vector_generator::random_unit_vector3_in_sphere();
    if direction.dot(hit_record.normal()) < 0.0 {
        direction = -direction;
    }
    direction += hit_record.normal().into_inner();
    // Fix case of "almost" zero vector
    if direction.iter().all(|x| x.abs() < 1e-8) {
        direction = hit_record.normal().into_inner();
    }
    Ray::new(*hit_record.pos(), direction, ray.time())
}

pub fn reflected_ray(ray: &Ray, hit_record: &HitRecord) -> Ray {
    let reflected_direction = ray.direction().reflect(hit_record.normal());
    Ray::new(*hit_record.pos(), reflected_direction, ray.time())
}

pub fn fuzzed_ray(ray: &Ray, fuzziness: f64) -> Ray {
    if fuzziness == 0.0 {
        return *ray;
    }
    let fuzzed_direction = ray.direction().normalize()
        + fuzziness * random_vector_generator::random_unit_vector3_in_sphere();
    Ray::new(*ray.origin(), fuzzed_direction, ray.time())
}

pub fn refracted_ray(
    ray: &Ray,
    hit_record: &HitRecord,
    material_refraction_index: f64,
) -> Option<Ray> {
    let refraction_index = if hit_record.front_face() {
        1.0 / material_refraction_index
    } else {
        material_refraction_index
    };
    let cos_theta = (-ray.direction().dot(hit_record.normal())).min(1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    let cannot_refract = refraction_index * sin_theta > 1.0;

    let mut rng = rand::rng();
    if cannot_refract || schlick_reflectance(cos_theta, refraction_index) > rng.random() {
        return None;
    }

    let refracted_direction = ray
        .direction()
        .refract(hit_record.normal(), refraction_index);
    Some(Ray::new(*hit_record.pos(), refracted_direction, ray.time()))
}

/// Uses schlick approximation to calculate reflectance, which is probabilty that the light would reflect.
fn schlick_reflectance(cos_theta: f64, refraction_index: f64) -> f64 {
    let r_0_sqrt = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r_0 = r_0_sqrt * r_0_sqrt;
    r_0 + (1.0 - r_0) * (1.0 - cos_theta).powi(5)
}
