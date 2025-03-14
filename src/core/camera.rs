use std::{
    sync::mpsc::{channel, Receiver},
    thread::{self, JoinHandle},
};

use indicatif::{ProgressBar, ProgressStyle};
use nalgebra::{Point3, Unit, UnitVector3, Vector2, Vector3};
use rand::Rng;
use rayon::prelude::*;
use rgb::Rgb;
use typed_builder::TypedBuilder;

use crate::{
    core::rendered_image::{Dimensions, RenderedImage},
    extensions::{
        rgb_f64_to_u8::RgbF64ToU8Extension, rgb_linear_to_gamma::RgbLinearToGammaExtension,
    },
    material::Material,
    object::hittable_object::HittableObject,
    ray::Ray,
    scene::{SceneContent, SceneOptions},
    utils::random_vector_generator,
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
    #[builder(default = Point3::new(0.0, 0.0, -1.0), setter(into))]
    look_at: Point3<f64>,
    #[builder(default = Vector3::new(0.0, 1.0, 0.0), setter(into))]
    relative_up: Vector3<f64>,
    #[builder(default = UnitVector3::new_unchecked(Vector3::new(1.0, 0.0, 0.0)), setter(skip))]
    right: UnitVector3<f64>,
    #[builder(default = UnitVector3::new_unchecked(Vector3::new(0.0, 1.0, 0.0)), setter(skip))]
    up: UnitVector3<f64>,
    #[builder(default = UnitVector3::new_unchecked(Vector3::new(0.0, 0.0, -1.0)), setter(skip))]
    at: UnitVector3<f64>,
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
    /// Variation angle of rays through each pixel
    #[builder(default = 0.0, setter(into))]
    defocus_angle: f64,
    /// Distance from camera center to plane of perfect focus
    #[builder(default = 10.0, setter(into))]
    focus_distance: f64,
    #[builder(default, setter(skip))]
    defocus_disk: DefocusDisk,
}

impl Camera {
    pub fn render(
        &self,
        scene_content: &SceneContent,
        scene_options: &SceneOptions,
    ) -> RenderedImage {
        let (tx, rx) = channel::<()>();

        let progress_handler = self.spawn_progress_thread(rx);

        let pixels: Vec<Rgb<u8>> = (0..self.dimensions.height)
            .into_par_iter()
            .flat_map(|j| {
                (0..self.dimensions.width)
                    .into_par_iter()
                    .map(|i| {
                        let color = (0..self.samples_per_pixel)
                            .map(|_| {
                                let ray = self.create_ray_around_pixel(i, j);
                                self.calculate_color(&ray, scene_content, scene_options, 0)
                            })
                            .fold(Rgb::new(0.0, 0.0, 0.0), |acc, color| acc + color);
                        tx.send(()).unwrap();
                        (color * self.pixel_samples_scale)
                            .linear_to_gamma()
                            .f64_to_u8()
                    })
                    .collect::<Vec<Rgb<u8>>>()
            })
            .collect();

        progress_handler.join().unwrap();

        RenderedImage::new(pixels, self.dimensions).unwrap()
    }

    fn calculate_color(
        &self,
        ray: &Ray,
        scene_content: &SceneContent,
        scene_options: &SceneOptions,
        depth: u32,
    ) -> Rgb<f64> {
        if depth >= self.max_bounce_depth {
            return Rgb {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            };
        }

        // We start our range at 0.001 to fix the potential rounding issue, where
        // ray would reflect in such a way that it would hit the same sphere once again.
        let full_range = 0.001..=f64::MAX;
        let hit_record = scene_content.bvh().hit(ray, &full_range);
        if let Some(hit_record) = hit_record {
            let material = scene_content
                .material_by_id(hit_record.material_id())
                .unwrap();
            let emitted_color = material.emitted(hit_record.u(), hit_record.v(), hit_record.pos());
            match material.scatter(ray, &hit_record) {
                Some(material_scattering) => {
                    let next_color = self.calculate_color(
                        material_scattering.scattered_ray(),
                        scene_content,
                        scene_options,
                        depth + 1,
                    );
                    let scattered_color: Rgb<f64> = next_color
                        .iter()
                        .zip(material_scattering.attenuation().iter())
                        .map(|(x, y)| x * y)
                        .collect();
                    return scattered_color + emitted_color;
                }
                None => return emitted_color,
            }
        }

        // Ray hit nothing - just return background color
        *scene_options.background()
    }

    /// Create ray originating in a defocus disk and directed and random pixel around
    /// viewport pixel (i, j), with random time in range [0, 1.0]
    fn create_ray_around_pixel(&self, pixel_x: u32, pixel_y: u32) -> Ray {
        let offset = Self::sample_square();

        let pixel = self.upper_left_pixel_pos
            + (pixel_x as f64 + offset.x) * self.pixel_delta_horizontal
            + (pixel_y as f64 + offset.y) * self.pixel_delta_vertical;

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel - ray_origin;

        let mut rng = rand::rng();
        let ray_time = rng.random();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn sample_square() -> Vector2<f64> {
        let mut rng = rand::rng();
        let f1: f64 = rng.random();
        let f2: f64 = rng.random();
        Vector2::new(f1 - 0.5, f2 - 0.5)
    }

    fn defocus_disk_sample(&self) -> Point3<f64> {
        let p = random_vector_generator::random_unit_vector2_in_disk();
        self.center
            + p.x * self.defocus_disk.horizontal_radius
            + p.y * self.defocus_disk.vertical_radius
    }

    fn spawn_progress_thread(&self, rx: Receiver<()>) -> JoinHandle<()> {
        let all_elements = self.dimensions.all_elements();
        let pb = ProgressBar::new(all_elements as _);
        pb.set_style(
            ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}")
                .unwrap()
                .progress_chars("#>-"),
        );

        thread::spawn(move || {
            let mut counter = 0;
            for _ in 0..all_elements {
                rx.recv().unwrap();
                counter += 1;
                pb.set_position(counter);
            }
            pb.finish_with_message("rendered");
        })
    }
}

#[allow(non_camel_case_types)]
impl<
        __width: typed_builder::Optional<u32>,
        __center: typed_builder::Optional<nalgebra::OPoint<f64, nalgebra::Const<3>>>,
        __look_at: typed_builder::Optional<nalgebra::OPoint<f64, nalgebra::Const<3>>>,
        __relative_up: typed_builder::Optional<
            nalgebra::Matrix<
                f64,
                nalgebra::Const<3>,
                nalgebra::Const<1>,
                nalgebra::ArrayStorage<f64, 3, 1>,
            >,
        >,
        __samples_per_pixel: typed_builder::Optional<u32>,
        __max_bounce_depth: typed_builder::Optional<u32>,
        __vertical_fov_angles: typed_builder::Optional<f64>,
        __aspect_ratio: typed_builder::Optional<f64>,
        __defocus_angle: typed_builder::Optional<f64>,
        __focus_distance: typed_builder::Optional<f64>,
    >
    CameraBuilder<(
        __width,
        __center,
        __look_at,
        __relative_up,
        __samples_per_pixel,
        __max_bounce_depth,
        __vertical_fov_angles,
        __aspect_ratio,
        __defocus_angle,
        __focus_distance,
    )>
{
    pub fn build(self) -> Camera {
        let mut camera = self.__build();

        camera.dimensions = Dimensions::from_width(camera.width, camera.aspect_ratio);

        let theta = camera.vertical_fov_angles.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * camera.focus_distance;
        // We don't use aspect ratio here as it might not be what real ratio between width and height is
        let viewport_width = viewport_height * camera.dimensions.ratio();

        // Calculate camera vectors
        camera.at = Unit::new_normalize(camera.center - camera.look_at);
        camera.right = Unit::new_normalize(camera.relative_up.cross(&camera.at));
        camera.up = Unit::new_normalize(camera.at.cross(&camera.right));

        // Vectors across horizontal and down the vertical viewport edges
        let viewport_horizontal = viewport_width * camera.right.into_inner();
        let viewport_vertical = -viewport_height * camera.up.into_inner();

        // Pixel deltas across horizontal and verctial viewport edges
        camera.pixel_delta_horizontal = viewport_horizontal / camera.dimensions.width as f64;
        camera.pixel_delta_vertical = viewport_vertical / camera.dimensions.height as f64;

        // Upper left pixel
        let viewport_upper_left = camera.center
            - camera.focus_distance * camera.at.into_inner()
            - viewport_horizontal / 2.0
            - viewport_vertical / 2.0;
        camera.upper_left_pixel_pos = viewport_upper_left
            + 0.5 * (camera.pixel_delta_horizontal + camera.pixel_delta_vertical);

        camera.pixel_samples_scale = 1.0 / camera.samples_per_pixel as f64;

        // Calculate defocus disk
        let defocus_radius =
            camera.focus_distance * (camera.defocus_angle / 2.0).to_radians().tan();
        camera.defocus_disk = DefocusDisk {
            horizontal_radius: camera.right.into_inner() * defocus_radius,
            vertical_radius: camera.up.into_inner() * defocus_radius,
        };

        camera
    }
}

#[derive(Default)]
struct DefocusDisk {
    horizontal_radius: Vector3<f64>,
    vertical_radius: Vector3<f64>,
}
