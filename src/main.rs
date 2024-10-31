use clap::builder::styling::Color;
use clap::Parser;
use magick_rust::{self, bindings, magick_wand_genesis, MagickError, MagickWand};
use std::fs;
use std::path;
use std::sync::Once;

static START: Once = Once::new();
type ImageBytes = Vec<u8>;

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

fn main() {
    let config = Config::parse();

    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    if let Ok(_) = wand.read_image("test.jpg") {
        wand.fit(config.image_x, config.image_y);
        if let Ok(_) = wand.quantize_image(
            16,
            magick_rust::ColorspaceType::sRGB,
            16,
            magick_rust::DitherMethod::FloydSteinberg,
            false,
        ) {
            if let Ok(img) = wand.write_image_blob("test.png") {
                fs::write(format!("test.{}", config.output_format), img).expect("write failed");
            }
        }
    }
}
