use nalgebra::{Point3, Vector3};
use rgb::Rgb;

use crate::object::hittable_object::HittableObject;

#[derive(Default)]
pub struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Self {
        let direction = direction.normalize();
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Point3<f64> {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3<f64> {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin() + t * self.direction()
    }

    pub fn color(&self, object: &impl HittableObject) -> Rgb<f64> {
        let hit_record = object.hit(self, 0.0, f64::MAX);
        if let Some(hit_record) = hit_record {
            let color = 0.5 * (hit_record.normal() + Vector3::new(1.0, 1.0, 1.0));
            return Rgb {
                r: color.x,
                g: color.y,
                b: color.z,
            };
        }

        let white = Vector3::new(1.0, 1.0, 1.0);
        let blue = Vector3::new(0.5, 0.7, 1.0);
        let scale = 0.5 * (self.direction().y + 1.0);
        let color = white.lerp(&blue, scale);
        Rgb {
            r: color.x,
            g: color.y,
            b: color.z,
        }
    }
}
