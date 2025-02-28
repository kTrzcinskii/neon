use std::{env, fs, io::Write};

use anyhow::{bail, Result};
use nalgebra::Point3;
use neon::{
    camera::Camera,
    encoder::{ppm_encoder::PpmEncoder, rendered_image_encoder::RenderedImageEncoder},
    material::{lambertian::Lambertian, metal::Metal, MaterialType},
    object::{hittable_objects_list::HittableObjectsList, sphere::Sphere, HittableObjectType},
};
use rgb::Rgb;

fn main() -> Result<()> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("invalid number of arguments");
    }
    let output_path = &args[1];

    // Materials
    let material_ground = MaterialType::Lambertian(Lambertian::new(Rgb::new(0.8, 0.8, 0.0)));
    let material_center = MaterialType::Lambertian(Lambertian::new(Rgb::new(0.1, 0.2, 0.5)));
    let material_left = MaterialType::Metal(Metal::new(Rgb::new(0.8, 0.8, 0.8), 0.01));
    let material_right = MaterialType::Metal(Metal::new(Rgb::new(0.8, 0.6, 0.2), 0.8));

    // World
    let mut world = HittableObjectsList::new();
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        &material_ground,
    )));
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        &material_center,
    )));
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        &material_left,
    )));
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        &material_right,
    )));

    // Camera
    const WIDTH: u32 = 400;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_BOUNCE_DEPTH: u32 = 10;
    let camera = Camera::new(WIDTH, ASPECT_RATIO, SAMPLES_PER_PIXEL, MAX_BOUNCE_DEPTH);
    let rendered = camera.render(&world);

    // Encode
    let ppm_encoder = PpmEncoder::new(u8::MAX);
    let rendered_econded = ppm_encoder.encode(&rendered);

    // Save result
    let mut file = fs::File::create(output_path)?;
    file.write_all(&rendered_econded)?;

    Ok(())
}
