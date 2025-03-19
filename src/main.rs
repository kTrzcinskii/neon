use std::env;

use anyhow::{bail, Result};
use log::{error, info};
use neon::cli;

fn main() -> Result<()> {
    env_logger::init();

    // Parse args
    let cli_args: Vec<String> = env::args().collect();
    let args = cli::parse_args(&cli_args)?;

    // Render
    info!("Starting rendering");
    let rendered = args.scene.render();
    info!("Finished rendering");

    // Encode
    if let Err(e) = rendered.save(args.output_path) {
        error!("Cannot save output file: {}", e);
        bail!(e)
    }

    Ok(())
}
