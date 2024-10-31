use clap::Parser;
use magick_rust::{self, magick_wand_genesis, MagickError, MagickWand};
use std::fs;
use std::sync::Once;

static START: Once = Once::new();

#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
struct Config {
    #[arg(long, default_value_t = 1080)]
    canvas_x: usize,
    #[arg(long, default_value_t = 1080)]
    canvas_y: usize,
    #[arg(long, default_value_t = 1080)]
    image_x: usize,
    #[arg(long, default_value_t = 1080)]
    image_y: usize,
    #[arg(short, long, default_value_t = String::from("png"))]
    output_format: String,
    #[arg(short, long, default_value_t = true)]
    dither: bool,
    #[arg(long, default_value_t = String::from("netsafe"))]
    dither_map: String,
}

fn resize(filepath: &str, desired_size: &Config) -> Result<Vec<u8>, MagickError> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    wand.read_image(filepath)?;
    wand.fit(desired_size.image_x, desired_size.image_y);
    wand.write_image_blob(desired_size.output_format.as_str())
}

fn main() {
    let config = Config::parse();
    match resize("test.jpg", &config) {
        Ok(bytes) => {
            fs::write(format!("test.{}", config.output_format), bytes).expect("write failed");
        }
        Err(e) => println!("error: {}", e),
    }
}
