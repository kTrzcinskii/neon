use std::env;

use anyhow::{bail, Result};
use nalgebra::Point3;
use neon::{
    camera::Camera,
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, MaterialType},
    object::{hittable_objects_list::HittableObjectsList, sphere::Sphere, HittableObjectType},
    random_vector_generator,
    scene::Scene,
};
use rand::Rng;
use rgb::Rgb;

fn generate_random_materials(rows: usize, cols: usize) -> Vec<MaterialType> {
    let count = rows * cols;
    (0..count)
        .map(|_| {
            let mut rng = rand::rng();
            let choose_material: f64 = rng.random();
            if choose_material < 0.8 {
                let color_vec = random_vector_generator::random_vector3(0.0..1.0);
                let albedo: Rgb<f64> = color_vec
                    .iter()
                    .zip(color_vec.iter())
                    .map(|(x, y)| x * y)
                    .collect();
                MaterialType::Lambertian(Lambertian::new(albedo))
            } else if choose_material < 0.95 {
                let albedo: Rgb<f64> = random_vector_generator::random_vector3(0.5..1.0)
                    .into_iter()
                    .copied()
                    .collect();
                let fuzziness: f64 = rng.random();
                MaterialType::Metal(Metal::new(albedo, fuzziness))
            } else {
                MaterialType::Dielectric(Dielectric::new(1.5))
            }
        })
        .collect()
}

fn generate_random_spheres(rows: usize, cols: usize) -> Vec<HittableObjectType> {
    let mut output = Vec::with_capacity(rows * cols);
    let half_rows = (rows as f64 / 2.0) as i32;
    let half_cols = (cols as f64 / 2.0) as i32;
    for i in -half_rows..half_rows {
        for j in -half_cols..half_cols {
            let mut rng = rand::rng();
            let center = Point3::new(
                i as f64 + 0.9 * rng.random::<f64>(),
                0.2,
                j as f64 + 0.9 * rng.random::<f64>(),
            );
            let id = (i + half_rows) as usize * rows + (j + half_rows) as usize;
            let obj = HittableObjectType::Sphere(Sphere::new(center, 0.2, id));
            output.push(obj);
        }
    }
    output
}

// TODO: before starting new book:
// - create scene module and scene generator instead of hard coding it in main

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

    // Materials
    let material_ground = MaterialType::Lambertian(Lambertian::new(Rgb::new(0.5, 0.5, 0.5)));
    let mut random_materials = generate_random_materials(ROWS, COLS);
    random_materials.push(material_ground);

    // World
    let spheres = generate_random_spheres(ROWS, COLS);
    let mut world = HittableObjectsList::from(spheres);
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        random_materials.len() - 1,
    )));

    let glass = MaterialType::Dielectric(Dielectric::new(1.5));
    random_materials.push(glass);
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        random_materials.len() - 1,
    )));

    let lambertian = MaterialType::Lambertian(Lambertian::new(Rgb::new(0.4, 0.2, 0.1)));
    random_materials.push(lambertian);
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        random_materials.len() - 1,
    )));

    let metal = MaterialType::Metal(Metal::new(Rgb::new(0.7, 0.6, 0.5), 0.0));
    random_materials.push(metal);
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        random_materials.len() - 1,
    )));

    let scene = Scene::new(
        random_materials,
        HittableObjectType::HittableObjectList(world),
    );

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
    let rendered = camera.render(&scene);

    // Encode
    if let Err(e) = rendered.save(output_path) {
        eprintln!("There was an error while saving output file: {}", e);
    }

    Ok(())
}
