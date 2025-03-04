use std::env;

use anyhow::{bail, Result};
use log::{error, info};
use nalgebra::Point3;
use neon::{camera::Camera, scene::scene_generator};

// TODO: after second book:
// - add loading/saving scenes to files

fn main() -> Result<()> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("invalid number of arguments");
    }
    let output_path = &args[1];

    // Config
    const ROWS: usize = 24;
    const COLS: usize = 24;

    let scene = scene_generator::scene_with_moving_spheres(ROWS, COLS);

    // Camera
    const WIDTH: u32 = 1200;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_BOUNCE_DEPTH: u32 = 50;
    const V_FOV: f64 = 20.0;
    const CENTER: Point3<f64> = Point3::new(13.0, 2.0, 3.0);
    const LOOK_AT: Point3<f64> = Point3::new(0.0, 0.0, 0.0);
    const DEFOCUS_ANGLE: f64 = 0.6;
    const FOCUS_DISTANCE: f64 = 10.0;
    let camera = Camera::builder()
        .width(WIDTH)
        .aspect_ratio(ASPECT_RATIO)
        .samples_per_pixel(SAMPLES_PER_PIXEL)
        .max_bounce_depth(MAX_BOUNCE_DEPTH)
        .vertical_fov_angles(V_FOV)
        .center(CENTER)
        .look_at(LOOK_AT)
        .defocus_angle(DEFOCUS_ANGLE)
        .focus_distance(FOCUS_DISTANCE)
        .build();

    info!("Starting rendering");
    let rendered = camera.render(&scene);
    info!("Finished rendering");

    // Encode
    if let Err(e) = rendered.save(output_path) {
        error!("Cannot save output file: {}", e);
        bail!(e)
    }

    Ok(())
}
