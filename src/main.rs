use std::{env, fs, io::Write};

use anyhow::{bail, Result};
use nalgebra::Point3;
use neon::{
    camera::Camera,
    encoder::{ppm_encoder::PpmEncoder, rendered_image_encoder::RenderedImageEncoder},
    object::{hittable_objects_list::HittableObjectsList, sphere::Sphere},
};

fn main() -> Result<()> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("invalid number of arguments");
    }
    let output_path = &args[1];

    // World
    let mut world = HittableObjectsList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    const WIDTH: u32 = 400;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let camera = Camera::new(WIDTH, ASPECT_RATIO);
    let rendered = camera.render(&world);

    // Encode
    let ppm_encoder = PpmEncoder::new(u8::MAX);
    let rendered_econded = ppm_encoder.encode(&rendered);

    // Save result
    let mut file = fs::File::create(output_path)?;
    file.write_all(&rendered_econded)?;

    Ok(())
}
