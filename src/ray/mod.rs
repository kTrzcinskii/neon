pub mod ray_generator;

use nalgebra::{Point3, Unit, UnitVector3, Vector3};

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point3<f64>,
    direction: UnitVector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Self {
        let direction = Unit::new_normalize(direction);
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Point3<f64> {
        &self.origin
    }

    pub fn direction(&self) -> &UnitVector3<f64> {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin() + t * self.direction().into_inner()
    }
}
