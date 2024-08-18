use image;
use std::error::Error;

pub fn detect_hidden_data(input_image_path: &str) -> Result<bool, Box<dyn Error>> {
    let img = image::open(input_image_path)?.into_rgba8();
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            for channel in 0..4 {
                if pixel[channel] & 1 != 0 {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}