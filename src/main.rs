use std::{env, fs, io::Write};

use anyhow::{bail, Result};
use log::info;
use nalgebra::Vector3;
use neon::ray::Ray;
use rgb::Rgb;

struct Dimensions {
    pub width: u32,
    pub height: u32,
}

impl Dimensions {
    pub fn from_width(width: u32, aspect_ratio: f32) -> Dimensions {
        let height = (width as f32 / aspect_ratio) as u32;
        let height = if height < 1 { 1 } else { height };
        Dimensions { width, height }
    }

    pub fn ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("invalid number of arguments");
    }
    let output_path = &args[1];
    let mut file = fs::File::create(output_path)?;

    // Image data
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    let image_dimensions = Dimensions::from_width(IMAGE_WIDTH, ASPECT_RATIO);
    const MAX_COLOR: u8 = 255;

    // Render
    let mut content = String::new();
    let headline = format!(
        "P3\n{} {}\n{}\n",
        image_dimensions.width, image_dimensions.height, MAX_COLOR
    );
    content.push_str(&headline);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    // We don't use aspect ratio here as it might not be what real ratio between width and height is
    let viewport_width = viewport_height * image_dimensions.ratio();
    let camera_center = Vector3::zeros();

    // Vectors across horizontal and down the vertical viewport edges
    let viewport_horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_vertical = Vector3::new(0.0, -viewport_height, 0.0);

    // Pixel deltas across horizontal and verctial viewport edges
    let pixel_delta_horizontal = viewport_horizontal / image_dimensions.width as f64;
    let pixel_delta_vertical = viewport_vertical / image_dimensions.height as f64;

    // Upper left pixel
    let viewport_upper_left = camera_center
        - Vector3::new(0.0, 0.0, focal_length)
        - viewport_horizontal / 2.0
        - viewport_vertical / 2.0;
    let upper_left_pixel_pos =
        viewport_upper_left + 0.5 * (pixel_delta_horizontal + pixel_delta_vertical);

    info!("Starting rendering");

    for j in 0..image_dimensions.height {
        info!("Scanlines remaining: {}", image_dimensions.height - j);
        for i in 0..IMAGE_WIDTH {
            let pixel_center = upper_left_pixel_pos
                + i as f64 * pixel_delta_horizontal
                + j as f64 * pixel_delta_vertical;
            let ray_dir = pixel_center - camera_center;
            let ray = Ray::new(camera_center.into(), ray_dir);

            let color = ray.color();
            let color: Rgb<u8> = color.iter().map(|c| (c * 255_f64) as u8).collect();
            let line = format!("{} {} {}\n", color.r, color.g, color.b);
            content.push_str(&line);
        }
    }

    info!("Finished rendering");

    file.write_all(content.as_bytes())?;

    Ok(())
}
