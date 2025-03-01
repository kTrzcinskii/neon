use log::info;
use nalgebra::{Point3, Vector2, Vector3};
use rand::Rng;
use rgb::Rgb;
use typed_builder::TypedBuilder;

use crate::{
    extensions::{
        rgb_f64_to_u8::RgbF64ToU8Extension, rgb_linear_to_gamma::RgbLinearToGammaExtension,
    },
    material::Material,
    object::hittable_object::HittableObject,
    ray::Ray,
    rendered_image::{Dimensions, RenderedImage},
};

#[derive(TypedBuilder)]
#[builder(build_method(vis="", name=__build))]
pub struct Camera {
    #[builder(default = 400, setter(into))]
    width: u32,
    #[builder(default, setter(skip))]
    dimensions: Dimensions,
    #[builder(default, setter(into))]
    center: Point3<f64>,
    #[builder(default = 100, setter(into))]
    samples_per_pixel: u32,
    #[builder(default, setter(skip))]
    pixel_samples_scale: f64,
    #[builder(default, setter(skip))]
    upper_left_pixel_pos: Point3<f64>,
    #[builder(default, setter(skip))]
    pixel_delta_horizontal: Vector3<f64>,
    #[builder(default, setter(skip))]
    pixel_delta_vertical: Vector3<f64>,
    #[builder(default = 10, setter(into))]
    max_bounce_depth: u32,
    #[builder(default = 90.0, setter(into))]
    vertical_fov_angles: f64,
    #[builder(default = 16.0 / 9.0, setter(into))]
    aspect_ratio: f64,
}

impl Camera {
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

#[allow(non_camel_case_types)]
impl<
        __width: typed_builder::Optional<u32>,
        __center: typed_builder::Optional<nalgebra::OPoint<f64, nalgebra::Const<3>>>,
        __samples_per_pixel: typed_builder::Optional<u32>,
        __max_bounce_depth: typed_builder::Optional<u32>,
        __vertical_fov_angles: typed_builder::Optional<f64>,
        __aspect_ratio: typed_builder::Optional<f64>,
    >
    CameraBuilder<(
        __width,
        __center,
        __samples_per_pixel,
        __max_bounce_depth,
        __vertical_fov_angles,
        __aspect_ratio,
    )>
{
    pub fn build(self) -> Camera {
        let mut camera = self.__build();

        camera.dimensions = Dimensions::from_width(camera.width, camera.aspect_ratio);

        let focal_length = 1.0;
        let theta = camera.vertical_fov_angles.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        // We don't use aspect ratio here as it might not be what real ratio between width and height is
        let viewport_width = viewport_height * camera.dimensions.ratio();
        let center = Point3::new(0.0, 0.0, 0.0);

        // Vectors across horizontal and down the vertical viewport edges
        let viewport_horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_vertical = Vector3::new(0.0, -viewport_height, 0.0);

        // Pixel deltas across horizontal and verctial viewport edges
        camera.pixel_delta_horizontal = viewport_horizontal / camera.dimensions.width as f64;
        camera.pixel_delta_vertical = viewport_vertical / camera.dimensions.height as f64;

        // Upper left pixel
        let viewport_upper_left = center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_horizontal / 2.0
            - viewport_vertical / 2.0;
        camera.upper_left_pixel_pos = viewport_upper_left
            + 0.5 * (camera.pixel_delta_horizontal + camera.pixel_delta_vertical);

        camera.pixel_samples_scale = 1.0 / camera.samples_per_pixel as f64;

        camera
    }
}
