use nalgebra::{Point3, Vector3};

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
}
