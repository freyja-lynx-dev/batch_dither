use magick_rust::{self, magick_wand_genesis, MagickError, MagickWand};
use std::fs;
use std::sync::Once;

static START: Once = Once::new();

struct Size {
    canvas_x: usize,
    canvas_y: usize,
    image_x: usize,
    image_y: usize,
}

fn resize(filepath: &str, desired_size: Size, format: &str) -> Result<Vec<u8>, MagickError> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    wand.read_image(filepath)?;
    wand.fit(desired_size.image_x, desired_size.image_y);
    wand.write_image_blob(format)
}

fn main() {
    let sz = Size {
        canvas_x: 1000,
        canvas_y: 1000,
        image_x: 1000,
        image_y: 1000,
    };
    let fmt = "png";
    match resize("test.jpg", sz, fmt) {
        Ok(bytes) => {
            fs::write(format!("test.{}", fmt), bytes).expect("write failed");
        }
        Err(e) => println!("error: {}", e),
    }
}
