use std::env;

use anyhow::{bail, Result};
use log::{error, info};
use neon::scene::scene_generator;

fn main() -> Result<()> {
    env_logger::init();

    // Parse args
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        bail!("invalid number of arguments");
    }
    let output_path = &args[1];
    let scene = match args[2].as_str() {
        "spheres" => scene_generator::scene_with_spheres(24, 24),
        "moving_spheres" => scene_generator::scene_with_moving_spheres(24, 24),
        "two_checker" => scene_generator::scene_with_two_checker_spheres(),
        "earthmap" => scene_generator::scene_with_earthmap(),
        "perlin_noise" => scene_generator::scene_with_perlin_noise(),
        "quads" => scene_generator::scene_with_quads(),
        "simple_light" => scene_generator::scene_with_simple_light(),
        "cornell_box" => scene_generator::scene_with_cornell_box(),
        "fog_cornell_box" => scene_generator::scene_with_fog_cornell_box(),
        "all_effects" => scene_generator::scene_with_all_effects(),
        _ => bail!("unknown scene"),
    };

    // Render
    info!("Starting rendering");
    let rendered = scene.render();
    info!("Finished rendering");

    // Encode
    if let Err(e) = rendered.save(output_path) {
        error!("Cannot save output file: {}", e);
        bail!(e)
    }

    Ok(())
}
