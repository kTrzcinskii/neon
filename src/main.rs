use std::{env, fs, io::Write};

use anyhow::{bail, Result};
use nalgebra::Point3;
use neon::{
    camera::Camera,
    encoder::{ppm_encoder::PpmEncoder, rendered_image_encoder::RenderedImageEncoder},
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, MaterialType},
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
    // glass-like dielectric
    let material_left = MaterialType::Dielectric(Dielectric::new(1.5));
    let material_bubble = MaterialType::Dielectric(Dielectric::new(1.0 / 1.5));
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
    // Air bubble inside glass sphere
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        &material_left,
    )));
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.35,
        &material_bubble,
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
    const MAX_BOUNCE_DEPTH: u32 = 20;
    const V_FOV: f64 = 90.0;
    let camera = Camera::builder()
        .width(WIDTH)
        .aspect_ratio(ASPECT_RATIO)
        .samples_per_pixel(SAMPLES_PER_PIXEL)
        .max_bounce_depth(MAX_BOUNCE_DEPTH)
        .vertical_fov_angles(V_FOV)
        .build();
    let rendered = camera.render(&world);

    // Encode
    let ppm_encoder = PpmEncoder::new(u8::MAX);
    let rendered_econded = ppm_encoder.encode(&rendered);

    // Save result
    let mut file = fs::File::create(output_path)?;
    file.write_all(&rendered_econded)?;

    Ok(())
}
