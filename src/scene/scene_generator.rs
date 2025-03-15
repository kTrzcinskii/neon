use nalgebra::{Point3, Vector3};
use rand::Rng;
use rgb::Rgb;

use crate::{
    core::{bvh::BvhTree, camera::Camera},
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
        MaterialType,
    },
    object::{
        moving_sphere::MovingSphere, quad::Quad, rotate_y_decorator::RotateYDecorator,
        sphere::Sphere, translate_decorator::TranslateDecorator, HittableObjectType,
    },
    scene::SceneOptions,
    texture::{
        checker_texture::CheckerTexture, image_texture::ImageTexture, noise_texture::NoiseTexture,
        solid_color::SolidColor,
    },
    utils::random_vector_generator,
};

use super::{Scene, SceneContent};

pub fn scene_with_spheres(rows: usize, cols: usize) -> Scene {
    // Materials
    let mut materials = generate_random_materials(rows, cols);
    let material_ground = Lambertian::from(Rgb::new(0.5, 0.5, 0.5)).into();
    materials.push(material_ground);
    let glass = Dielectric::new(1.5).into();
    materials.push(glass);
    let lambertian = Lambertian::from(Rgb::new(0.4, 0.2, 0.1)).into();
    materials.push(lambertian);
    let metal = Metal::new(Rgb::new(0.7, 0.6, 0.5), 0.0).into();
    materials.push(metal);

    // Objects
    let mut world = generate_random_spheres(rows, cols);
    world.push(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, materials.len() - 4).into());
    world.push(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, materials.len() - 3).into());
    world.push(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, materials.len() - 2).into());
    world.push(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, materials.len() - 1).into());

    let content = SceneContent::new(materials, world.into());
    let camera = build_camera_for_spheres();
    Scene::new(content, camera, Default::default())
}

pub fn scene_with_moving_spheres(rows: usize, cols: usize) -> Scene {
    // Materials
    let mut materials = generate_random_materials(rows, cols);
    let checker_even = SolidColor::new(Rgb::new(0.2, 0.3, 0.1));
    let checker_odd = SolidColor::new(Rgb::new(0.9, 0.9, 0.9));
    let checker = CheckerTexture::new(0.32, checker_even.into(), checker_odd.into());
    let material_ground = Lambertian::new(checker.into()).into();
    materials.push(material_ground);
    let glass = Dielectric::new(1.5).into();
    materials.push(glass);
    let lambertian = Lambertian::from(Rgb::new(0.4, 0.2, 0.1)).into();
    materials.push(lambertian);
    let metal = Metal::new(Rgb::new(0.7, 0.6, 0.5), 0.0).into();
    materials.push(metal);

    // Objects
    let mut world = generate_random_moving_spheres(rows, cols);
    world.push(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, materials.len() - 4).into());
    world.push(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, materials.len() - 3).into());
    world.push(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, materials.len() - 2).into());
    world.push(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, materials.len() - 1).into());

    let content = SceneContent::new(materials, world.into());
    let camera = build_camera_for_spheres();
    Scene::new(content, camera, Default::default())
}

pub fn scene_with_two_checker_spheres() -> Scene {
    // Materials
    let checker_even = SolidColor::new(Rgb::new(0.2, 0.3, 0.1));
    let checker_odd = SolidColor::new(Rgb::new(0.9, 0.9, 0.9));
    let checker = CheckerTexture::new(0.32, checker_even.into(), checker_odd.into());
    let material = Lambertian::new(checker.into()).into();
    let materials = vec![material];

    // Objects
    let world = vec![
        HittableObjectType::Sphere(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, 0)),
        HittableObjectType::Sphere(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, 0)),
    ];

    let content = SceneContent::new(materials, world.into());

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

    Scene::new(content, camera, Default::default())
}

pub fn scene_with_earthmap() -> Scene {
    let earh_texture = ImageTexture::new("assets/earthmap.jpg").unwrap();
    let globe_material = Lambertian::new(earh_texture.into()).into();
    let materials = vec![globe_material];
    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, 0).into();

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

    Scene::new(content, camera, Default::default())
}

pub fn scene_with_perlin_noise() -> Scene {
    let materials = vec![Lambertian::new(NoiseTexture::new(4.0).into()).into()];
    let bigger = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, 0).into();
    let smaller = Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, 0).into();

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

    Scene::new(content, camera, Default::default())
}

pub fn scene_with_quads() -> Scene {
    let left_red: MaterialType = Lambertian::from(Rgb::new(1.0, 0.2, 0.2)).into();
    let back_green: MaterialType = Lambertian::from(Rgb::new(0.2, 1.0, 0.2)).into();
    let right_blue: MaterialType = Lambertian::from(Rgb::new(0.2, 0.2, 1.0)).into();
    let upper_orange: MaterialType = Lambertian::from(Rgb::new(1.0, 0.5, 0.0)).into();
    let lower_teal: MaterialType = Lambertian::from(Rgb::new(0.2, 0.8, 0.8)).into();
    let materials = vec![left_red, back_green, right_blue, upper_orange, lower_teal];

    let left: HittableObjectType = Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vector3::new(0.0, 0.0, -4.0),
        Vector3::new(0.0, 4.0, 0.0),
        0,
    )
    .into();
    let back: HittableObjectType = Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 4.0, 0.0),
        1,
    )
    .into();
    let right: HittableObjectType = Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vector3::new(0.0, 0.0, 4.0),
        Vector3::new(0.0, 4.0, 0.0),
        2,
    )
    .into();
    let upper: HittableObjectType = Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 4.0),
        3,
    )
    .into();
    let lower: HittableObjectType = Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -4.0),
        4,
    )
    .into();
    let objects = vec![left, back, right, upper, lower];

    let content = SceneContent::new(materials, objects.into());

    const WIDTH: u32 = 800;
    const ASPECT_RATIO: f64 = 1.0;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_BOUNCE_DEPTH: u32 = 50;
    const V_FOV: f64 = 80.0;
    const CENTER: Point3<f64> = Point3::new(0.0, 0.0, 9.0);
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

    Scene::new(content, camera, Default::default())
}

pub fn scene_with_simple_light() -> Scene {
    let perlin_texture = Lambertian::new(NoiseTexture::new(4.0).into()).into();
    // Brighter than (1,1,1) to light things around it
    let light = DiffuseLight::from(Rgb::new(4.0, 4.0, 4.0)).into();
    let materials = vec![perlin_texture, light];

    let ground = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, 0).into();
    let main_object = Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, 0).into();
    let quad_light = Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vector3::new(2.0, 0.0, 0.0),
        Vector3::new(0.0, 2.0, 0.0),
        1,
    )
    .into();
    let sphere_light = Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, 1).into();
    let world = vec![ground, main_object, quad_light, sphere_light];

    let content = SceneContent::new(materials, world.into());

    const WIDTH: u32 = 1200;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_BOUNCE_DEPTH: u32 = 50;
    const V_FOV: f64 = 20.0;
    const CENTER: Point3<f64> = Point3::new(26.0, 3.0, 6.0);
    const LOOK_AT: Point3<f64> = Point3::new(0.0, 2.0, 0.0);
    let camera = Camera::builder()
        .width(WIDTH)
        .aspect_ratio(ASPECT_RATIO)
        .samples_per_pixel(SAMPLES_PER_PIXEL)
        .max_bounce_depth(MAX_BOUNCE_DEPTH)
        .vertical_fov_angles(V_FOV)
        .center(CENTER)
        .look_at(LOOK_AT)
        .build();

    const BACKGROUND_COLOR: Rgb<f64> = Rgb::new(0.0, 0.0, 0.0);
    let options = SceneOptions::builder().background(BACKGROUND_COLOR).build();

    Scene::new(content, camera, options)
}

pub fn scene_with_cornell_box() -> Scene {
    let light = DiffuseLight::from(Rgb::new(15.0, 15.0, 15.0)).into();
    let red = Lambertian::from(Rgb::new(0.65, 0.05, 0.05)).into();
    let white = Lambertian::from(Rgb::new(0.73, 0.73, 0.73)).into();
    let green = Lambertian::from(Rgb::new(0.12, 0.45, 0.15)).into();
    let materials = vec![light, red, white, green];

    let green_quad = Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        3,
    )
    .into();

    let red_quad = Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        1,
    )
    .into();

    let white_bottom_quad = Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        2,
    )
    .into();

    let white_mid_quad = Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vector3::new(-555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -555.0),
        2,
    )
    .into();

    let white_upper_quad = Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        2,
    )
    .into();

    let light_source = Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vector3::new(-130.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -105.0),
        0,
    )
    .into();

    let cuboid_bigger = Quad::cuboid(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        2,
    )
    .into();
    let cuboid_bigger = RotateYDecorator::new(cuboid_bigger, 15.0).into();
    let cuboid_bigger =
        TranslateDecorator::new(cuboid_bigger, Vector3::new(265.0, 0.0, 295.0)).into();

    let cuboid_smaller = Quad::cuboid(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        2,
    )
    .into();
    let cuboid_smaller = RotateYDecorator::new(cuboid_smaller, -18.0).into();
    let cuboid_smaller =
        TranslateDecorator::new(cuboid_smaller, Vector3::new(130.0, 0.0, 65.0)).into();

    let world = vec![
        green_quad,
        red_quad,
        white_bottom_quad,
        white_mid_quad,
        white_upper_quad,
        light_source,
        cuboid_smaller,
        cuboid_bigger,
    ];

    let content = SceneContent::new(materials, world.into());

    const WIDTH: u32 = 800;
    const ASPECT_RATIO: f64 = 1.0;
    const SAMPLES_PER_PIXEL: u32 = 1500;
    const MAX_BOUNCE_DEPTH: u32 = 80;
    const V_FOV: f64 = 40.0;
    const CENTER: Point3<f64> = Point3::new(278.0, 278.0, -800.0);
    const LOOK_AT: Point3<f64> = Point3::new(278.0, 278.0, 0.0);
    let camera = Camera::builder()
        .width(WIDTH)
        .aspect_ratio(ASPECT_RATIO)
        .samples_per_pixel(SAMPLES_PER_PIXEL)
        .max_bounce_depth(MAX_BOUNCE_DEPTH)
        .vertical_fov_angles(V_FOV)
        .center(CENTER)
        .look_at(LOOK_AT)
        .build();

    let options = SceneOptions::builder()
        .background(Rgb::new(0.0, 0.0, 0.0))
        .build();

    Scene::new(content, camera, options)
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
                Lambertian::from(albedo).into()
            } else if choose_material < 0.95 {
                let albedo: Rgb<f64> = random_vector_generator::random_vector3(0.5..1.0)
                    .into_iter()
                    .copied()
                    .collect();
                let fuzziness: f64 = rng.random();
                Metal::new(albedo, fuzziness).into()
            } else {
                Dielectric::new(1.5).into()
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
            let obj = Sphere::new(center, 0.2, id).into();
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
            let obj = MovingSphere::new(from, to, 0.2, id).into();
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
