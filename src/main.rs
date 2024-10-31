use clap::Parser;
use magick_rust::{self, bindings, magick_wand_genesis, MagickError, MagickWand};
use std::fs;
use std::path;
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
    color_map: String,
    #[arg(long, default_value_t = String::from("floyd"))]
    dither_type: String,
    /* TO-DO -- see if there's a way to provide defaults for this */
    #[arg(long)]
    input_dir: path::PathBuf,
    #[arg(long)]
    output_dir: path::PathBuf,
}

fn resize(filepath: &str, config: &Config) -> Result<Vec<u8>, MagickError> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    wand.read_image(filepath)?;
    wand.fit(config.image_x, config.image_y);
    wand.write_image_blob(config.output_format.as_str())
}
/* STUB: does not do anything */
fn dither(filepath: &str, config: &Config) -> Result<Vec<u8>, MagickError> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let _dtype = match config.dither_type.as_str() {
        "floyd" => bindings::DitherMethod_FloydSteinbergDitherMethod,
        _ => bindings::DitherMethod_UndefinedDitherMethod,
    };
    let wand = MagickWand::new();
    wand.read_image(filepath)?;
    wand.write_image_blob(config.output_format.as_str())
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
