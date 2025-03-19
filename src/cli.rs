use anyhow::{bail, Error, Result};

use crate::scene::{scene_generator, Scene};

pub struct Args {
    pub scene: Scene,
    pub output_path: String,
}

pub fn parse_args(cli_args: &[String]) -> Result<Args> {
    if cli_args.len() != 3 && cli_args.len() != 4 {
        bail!("invalid number of arguments");
    }

    let output_path = cli_args[1].clone();

    let samples_per_pixel = if cli_args.len() == 4 {
        Some(
            cli_args[3]
                .parse::<u32>()
                .map_err(|_| Error::msg("invalid 'samples_per_pixel' value"))?,
        )
    } else {
        None
    };

    let scene = match cli_args[2].as_str() {
        "spheres" => scene_generator::scene_with_spheres(24, 24, samples_per_pixel),
        "moving_spheres" => scene_generator::scene_with_moving_spheres(24, 24, samples_per_pixel),
        "two_checker" => scene_generator::scene_with_two_checker_spheres(samples_per_pixel),
        "earthmap" => scene_generator::scene_with_earthmap(samples_per_pixel),
        "perlin_noise" => scene_generator::scene_with_perlin_noise(samples_per_pixel),
        "quads" => scene_generator::scene_with_quads(samples_per_pixel),
        "simple_light" => scene_generator::scene_with_simple_light(samples_per_pixel),
        "cornell_box" => scene_generator::scene_with_cornell_box(samples_per_pixel),
        "fog_cornell_box" => scene_generator::scene_with_fog_cornell_box(samples_per_pixel),
        "all_effects" => scene_generator::scene_with_all_effects(samples_per_pixel),
        _ => bail!("unknown scene"),
    };

    Ok(Args { scene, output_path })
}
