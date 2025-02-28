use log::info;
use nalgebra::{Point3, Vector2, Vector3};
use rand::Rng;
use rgb::Rgb;

use crate::{
    extensions::{
        rgb_f64_to_u8::RgbF64ToU8Extension, rgb_linear_to_gamma::RgbLinearToGammaExtension,
    },
    material::Material,
    object::hittable_object::HittableObject,
    ray::Ray,
    rendered_image::{Dimensions, RenderedImage},
};

pub struct Camera {
    dimensions: Dimensions,
    center: Point3<f64>,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    upper_left_pixel_pos: Point3<f64>,
    pixel_delta_horizontal: Vector3<f64>,
    pixel_delta_vertical: Vector3<f64>,
    max_bounce_depth: u32,
}

impl Camera {
    pub fn new(
        width: u32,
        aspect_ratio: f64,
        samples_per_pixel: u32,
        max_bounce_depth: u32,
    ) -> Self {
        let dimensions = Dimensions::from_width(width, aspect_ratio);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        // We don't use aspect ratio here as it might not be what real ratio between width and height is
        let viewport_width = viewport_height * dimensions.ratio();
        let center = Point3::new(0.0, 0.0, 0.0);

        // Vectors across horizontal and down the vertical viewport edges
        let viewport_horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_vertical = Vector3::new(0.0, -viewport_height, 0.0);

        // Pixel deltas across horizontal and verctial viewport edges
        let pixel_delta_horizontal = viewport_horizontal / dimensions.width as f64;
        let pixel_delta_vertical = viewport_vertical / dimensions.height as f64;

        // Upper left pixel
        let viewport_upper_left = center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_horizontal / 2.0
            - viewport_vertical / 2.0;
        let upper_left_pixel_pos =
            viewport_upper_left + 0.5 * (pixel_delta_horizontal + pixel_delta_vertical);

        Camera {
            dimensions,
            center,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            upper_left_pixel_pos,
            pixel_delta_horizontal,
            pixel_delta_vertical,
            max_bounce_depth,
        }
    }

    pub fn render(&self, world: &impl HittableObject) -> RenderedImage {
        let mut output = vec![vec![]];

        info!("Starting rendering");

        for j in 0..self.dimensions.height {
            info!("Scanlines remaining: {}", self.dimensions.height - j);

            let mut row = vec![];
            for i in 0..self.dimensions.width {
                let mut color = Rgb::<f64>::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.create_ray_around_pixel(i, j);
                    color += self.calculate_color(&ray, world, 0);
                }

                color *= self.pixel_samples_scale;

                row.push(color.linear_to_gamma().f64_to_u8());
            }
            output.push(row);
        }

        info!("Finished rendering");

        RenderedImage {
            pixels: output,
            dimensions: self.dimensions,
        }
    }

    fn calculate_color(&self, ray: &Ray, object: &impl HittableObject, depth: u32) -> Rgb<f64> {
        if depth >= self.max_bounce_depth {
            return Rgb {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            };
        }

        // We start our range at 0.001 to fix the potential rounding issue, where
        // ray would reflect in such a way that it would hit the same sphere once again.
        let hit_record = object.hit(ray, 0.001..=f64::MAX);
        if let Some(hit_record) = hit_record {
            match hit_record.material_type().scatter(ray, &hit_record) {
                Some(material_scattering) => {
                    let next_color = self.calculate_color(
                        material_scattering.scattered_ray(),
                        object,
                        depth + 1,
                    );
                    let final_color: Rgb<f64> = next_color
                        .iter()
                        .zip(material_scattering.attenuation().iter())
                        .map(|(x, y)| x * y)
                        .collect();
                    return final_color;
                }
                None => {
                    return Rgb {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                    };
                }
            }
        }

        let white = Vector3::new(1.0, 1.0, 1.0);
        let blue = Vector3::new(0.5, 0.7, 1.0);
        let scale = 0.5 * (ray.direction().y + 1.0);
        let color = white.lerp(&blue, scale);
        Rgb {
            r: color.x,
            g: color.y,
            b: color.z,
        }
    }

    fn create_ray_around_pixel(&self, pixel_x: u32, pixel_y: u32) -> Ray {
        let offset = Self::sample_square();

        let pixel = self.upper_left_pixel_pos
            + (pixel_x as f64 + offset.x) * self.pixel_delta_horizontal
            + (pixel_y as f64 + offset.y) * self.pixel_delta_vertical;

        let ray_origin = self.center;
        let ray_direction = pixel - self.center;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vector2<f64> {
        let mut rng = rand::rng();
        let f1: f64 = rng.random();
        let f2: f64 = rng.random();
        Vector2::new(f1 - 0.5, f2 - 0.5)
    }
}
