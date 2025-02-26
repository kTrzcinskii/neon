use nalgebra::{Point3, Vector3};
use rgb::Rgb;

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

    pub fn color(&self) -> Rgb<u8> {
        let white = Vector3::new(1.0, 1.0, 1.0);
        let blue = Vector3::new(0.5, 0.7, 1.0);
        let scale = 0.5 * (self.direction().y + 1.0);
        let color = white.lerp(&blue, scale);
        let rgb: Vec<_> = color.iter().map(|c| (c * 255_f64) as u8).collect();
        Rgb {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        }
    }
}
