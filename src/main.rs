use image::{ImageError, DynamicImage, io::Reader as ImageReader};
use rayon::prelude::*;
use std::path::Path;

fn load_image<P>(path: P) -> Result<DynamicImage, ImageError>
where
    P: AsRef<Path>,
{
    let img = ImageReader::open(path)?.decode()?;
    Ok(img)
}

fn save_image(img: &DynamicImage, path: &str) -> Result<(), ImageError> {
    img.save(path)
}

fn grayscale(img: &DynamicImage) -> DynamicImage {
    img.to_luma8().into()
}

fn brighten(img: &DynamicImage) -> DynamicImage {
    let mut imgbuf = img.to_rgb8();
    imgbuf.par_chunks_mut(3).for_each(|pixel| {
        pixel[0] = pixel[0].saturating_add(50);
        pixel[1] = pixel[1].saturating_add(50);
        pixel[2] = pixel[2].saturating_add(50);
    });
    DynamicImage::ImageRgb8(imgbuf)
}

fn main() {
    let input_path = "images/cat1.jpg";
    let output_gray_path = "images/cat_output.jpg";
    let output_bright_path = "images/cat_bright.jpg";

    match load_image(input_path) {
        Ok(img) => {
            let gray_img = grayscale(&img);
            if let Err(e) = save_image(&gray_img, output_gray_path) {
                eprintln!("failed to save image: {}", e);
            }

            let bright_img = brighten(&img);
            if let Err(e) = save_image(&bright_img, output_bright_path) {
                eprintln!("Failed to save brightened image: {}", e);
            }
        },
    Err(e) => eprintln!("failed to load image: {}", e),
    }
}