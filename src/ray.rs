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

    pub fn color(&self) -> Rgb<f64> {
        let t = self.hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let normal = (self.at(t).coords - Vector3::new(0.0, 0.0, -1.0)).normalize();
            let color = 0.5 * Vector3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
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

    // TODO: this probably shouldn't be in this Impl block, move it later
    pub fn hit_sphere(&self, center: &Point3<f64>, radius: f64) -> f64 {
        let oc = center - self.origin();
        // Becasue dotting vector with itself is same as squared length
        let a = self.direction().norm_squared();
        let h = self.direction().dot(&oc);
        let c = oc.norm_squared() - radius * radius;
        let delta = h * h - a * c;
        if delta < 0.0 {
            return -1.0;
        }
        (h - delta.sqrt()) / a
    }
}
