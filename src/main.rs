use std::{env, fs, io::Write};

use anyhow::{bail, Result};
use log::info;

fn main() -> Result<()> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("invalid number of arguments");
    }
    let output_path = &args[1];
    let mut file = fs::File::create(output_path)?;

    // Image data
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;
    const MAX_COLOR: u8 = 255;

    // Render
    let mut content = String::new();
    let headline = format!("P3\n{} {}\n{}\n", IMAGE_WIDTH, IMAGE_HEIGHT, MAX_COLOR);
    content.push_str(&headline);

    info!("Starting rendering");

    for j in 0..IMAGE_HEIGHT {
        info!("Scanlines remaining: {}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let r = ((i as f64) / (IMAGE_WIDTH as f64 - 1.0) * u8::MAX as f64) as u8;
            let g = ((j as f64) / (IMAGE_HEIGHT as f64 - 1.0) * u8::MAX as f64) as u8;
            let b = 0;
            let line = format!("{} {} {}\n", r, g, b);
            content.push_str(&line);
        }
    }

    info!("Finished rendering");

    file.write_all(content.as_bytes())?;

    Ok(())
}
