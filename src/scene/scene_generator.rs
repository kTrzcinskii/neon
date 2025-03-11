use nalgebra::{Point3, Vector3};
use rand::Rng;
use rgb::Rgb;

use crate::{
    core::{bvh::BvhTree, camera::Camera},
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, MaterialType},
    object::{
        hittable_objects_list::HittableObjectsList, moving_sphere::MovingSphere, sphere::Sphere,
        HittableObjectType,
    },
    texture::{
        checker_texture::CheckerTexture, image_texture::ImageTexture, noise_texture::NoiseTexture,
        solid_color::SolidColor, NonRecursiveTexture, TextureType,
    },
    utils::random_vector_generator,
};

use super::{Scene, SceneContent};

pub fn scene_with_spheres(rows: usize, cols: usize) -> Scene {
    // Materials
    let mut materials = generate_random_materials(rows, cols);
    let material_ground = MaterialType::Lambertian(Lambertian::from(Rgb::new(0.5, 0.5, 0.5)));
    materials.push(material_ground);
    let glass = MaterialType::Dielectric(Dielectric::new(1.5));
    materials.push(glass);
    let lambertian = MaterialType::Lambertian(Lambertian::from(Rgb::new(0.4, 0.2, 0.1)));
    materials.push(lambertian);
    let metal = MaterialType::Metal(Metal::new(Rgb::new(0.7, 0.6, 0.5), 0.0));
    materials.push(metal);

    // Objects
    let spheres = generate_random_spheres(rows, cols);
    let mut world = HittableObjectsList::from(spheres);
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        materials.len() - 4,
    )));
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        materials.len() - 3,
    )));
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        materials.len() - 2,
    )));
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        materials.len() - 1,
    )));

    let content = SceneContent::new(materials, BvhTree::from(world));
    let camera = build_camera_for_spheres();
    Scene::new(content, camera)
}

pub fn scene_with_moving_spheres(rows: usize, cols: usize) -> Scene {
    // Materials
    let mut materials = generate_random_materials(rows, cols);
    let checker_even = NonRecursiveTexture::SolidColor(SolidColor::new(Rgb::new(0.2, 0.3, 0.1)));
    let checker_odd = NonRecursiveTexture::SolidColor(SolidColor::new(Rgb::new(0.9, 0.9, 0.9)));
    let checker = CheckerTexture::new(0.32, checker_even, checker_odd);
    let material_ground =
        MaterialType::Lambertian(Lambertian::new(TextureType::CheckerTexture(checker)));
    materials.push(material_ground);
    let glass = MaterialType::Dielectric(Dielectric::new(1.5));
    materials.push(glass);
    let lambertian = MaterialType::Lambertian(Lambertian::from(Rgb::new(0.4, 0.2, 0.1)));
    materials.push(lambertian);
    let metal = MaterialType::Metal(Metal::new(Rgb::new(0.7, 0.6, 0.5), 0.0));
    materials.push(metal);

    // Objects
    let spheres = generate_random_moving_spheres(rows, cols);
    let mut world = HittableObjectsList::from(spheres);
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        materials.len() - 4,
    )));
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        materials.len() - 3,
    )));
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        materials.len() - 2,
    )));
    world.add(HittableObjectType::Sphere(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        materials.len() - 1,
    )));

    let content = SceneContent::new(materials, BvhTree::from(world));
    let camera = build_camera_for_spheres();
    Scene::new(content, camera)
}

pub fn scene_with_two_checker_spheres() -> Scene {
    // Materials
    let checker_even = NonRecursiveTexture::SolidColor(SolidColor::new(Rgb::new(0.2, 0.3, 0.1)));
    let checker_odd = NonRecursiveTexture::SolidColor(SolidColor::new(Rgb::new(0.9, 0.9, 0.9)));
    let checker = CheckerTexture::new(0.32, checker_even, checker_odd);
    let material = MaterialType::Lambertian(Lambertian::new(TextureType::CheckerTexture(checker)));
    let materials = vec![material];

    // Objects
    let world = vec![
        HittableObjectType::Sphere(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, 0)),
        HittableObjectType::Sphere(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, 0)),
    ];

    let content = SceneContent::new(materials, BvhTree::from(world));

    // Camera
    const WIDTH: u32 = 1200;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_BOUNCE_DEPTH: u32 = 50;
    const V_FOV: f64 = 20.0;
    const CENTER: Point3<f64> = Point3::new(13.0, 2.0, 3.0);
    const LOOK_AT: Point3<f64> = Point3::new(0.0, 0.0, 0.0);
    let camera = Camera::builder()
        .width(WIDTH)
        .aspect_ratio(ASPECT_RATIO)
        .samples_per_pixel(SAMPLES_PER_PIXEL)
        .max_bounce_depth(MAX_BOUNCE_DEPTH)
        .vertical_fov_angles(V_FOV)
        .center(CENTER)
        .look_at(LOOK_AT)
        .build();

    Scene::new(content, camera)
}

pub fn scene_with_earthmap() -> Scene {
    let earh_texture = ImageTexture::new("assets/earthmap.jpg").unwrap();
    let globe_material = Lambertian::new(TextureType::NonRecursive(
        NonRecursiveTexture::ImageTexture(earh_texture),
    ));
    let materials = vec![MaterialType::Lambertian(globe_material)];
    let globe = HittableObjectType::Sphere(Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, 0));

    let content = SceneContent::new(materials, BvhTree::from(vec![globe]));

    const WIDTH: u32 = 1200;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_BOUNCE_DEPTH: u32 = 50;
    const V_FOV: f64 = 20.0;
    const CENTER: Point3<f64> = Point3::new(12.0, 0.3, 0.0);
    const LOOK_AT: Point3<f64> = Point3::new(0.0, 0.0, 0.0);
    let camera = Camera::builder()
        .width(WIDTH)
        .aspect_ratio(ASPECT_RATIO)
        .samples_per_pixel(SAMPLES_PER_PIXEL)
        .max_bounce_depth(MAX_BOUNCE_DEPTH)
        .vertical_fov_angles(V_FOV)
        .center(CENTER)
        .look_at(LOOK_AT)
        .build();

    Scene::new(content, camera)
}

pub fn scene_with_perlin_noise() -> Scene {
    let materials = vec![MaterialType::Lambertian(Lambertian::new(
        TextureType::NonRecursive(NonRecursiveTexture::NoiseTexture(NoiseTexture::new(4.0))),
    ))];
    let bigger = HittableObjectType::Sphere(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, 0));
    let smaller = HittableObjectType::Sphere(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, 0));

    let content = SceneContent::new(materials, BvhTree::from(vec![bigger, smaller]));

    const WIDTH: u32 = 1200;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_BOUNCE_DEPTH: u32 = 50;
    const V_FOV: f64 = 20.0;
    const CENTER: Point3<f64> = Point3::new(13.0, 2.0, 3.0);
    const LOOK_AT: Point3<f64> = Point3::new(0.0, 0.0, 0.0);
    let camera = Camera::builder()
        .width(WIDTH)
        .aspect_ratio(ASPECT_RATIO)
        .samples_per_pixel(SAMPLES_PER_PIXEL)
        .max_bounce_depth(MAX_BOUNCE_DEPTH)
        .vertical_fov_angles(V_FOV)
        .center(CENTER)
        .look_at(LOOK_AT)
        .build();

    Scene::new(content, camera)
}

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
                MaterialType::Lambertian(Lambertian::from(albedo))
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

fn generate_random_moving_spheres(rows: usize, cols: usize) -> Vec<HittableObjectType> {
    let mut output = Vec::with_capacity(rows * cols);
    let half_rows = (rows as f64 / 2.0) as i32;
    let half_cols = (cols as f64 / 2.0) as i32;
    for i in -half_rows..half_rows {
        for j in -half_cols..half_cols {
            let mut rng = rand::rng();
            let from = Point3::new(
                i as f64 + 0.9 * rng.random::<f64>(),
                0.2,
                j as f64 + 0.9 * rng.random::<f64>(),
            );
            let to = from + Vector3::new(0.0, rng.random::<f64>() / 2.0, 0.0);
            let id = (i + half_rows) as usize * rows + (j + half_rows) as usize;
            let obj = HittableObjectType::MovingSphere(MovingSphere::new(from, to, 0.2, id));
            output.push(obj);
        }
    }
    output
}

fn build_camera_for_spheres() -> Camera {
    const WIDTH: u32 = 1200;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_BOUNCE_DEPTH: u32 = 50;
    const V_FOV: f64 = 20.0;
    const CENTER: Point3<f64> = Point3::new(13.0, 2.0, 3.0);
    const LOOK_AT: Point3<f64> = Point3::new(0.0, 0.0, 0.0);
    const DEFOCUS_ANGLE: f64 = 0.6;
    const FOCUS_DISTANCE: f64 = 10.0;
    Camera::builder()
        .width(WIDTH)
        .aspect_ratio(ASPECT_RATIO)
        .samples_per_pixel(SAMPLES_PER_PIXEL)
        .max_bounce_depth(MAX_BOUNCE_DEPTH)
        .vertical_fov_angles(V_FOV)
        .center(CENTER)
        .look_at(LOOK_AT)
        .defocus_angle(DEFOCUS_ANGLE)
        .focus_distance(FOCUS_DISTANCE)
        .build()
}
