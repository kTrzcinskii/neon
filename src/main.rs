use std::env;

use anyhow::{bail, Result};
use log::{error, info};
use neon::scene::scene_generator;

fn main() -> Result<()> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("invalid number of arguments");
    }
    let output_path = &args[1];

    // Config
    // const ROWS: usize = 24;
    // const COLS: usize = 24;

    let scene = scene_generator::scene_with_fog_cornell_box();

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
