pub mod ray_generator;

use nalgebra::{Point3, Unit, UnitVector3, Vector3};

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point3<f64>,
    direction: UnitVector3<f64>,
    /// `time` represents moment when ray was sent
    time: f64,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>, time: f64) -> Self {
        let direction = Unit::new_normalize(direction);
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> &Point3<f64> {
        &self.origin
    }

    pub fn direction(&self) -> &UnitVector3<f64> {
        &self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin() + t * self.direction().into_inner()
    }
}
