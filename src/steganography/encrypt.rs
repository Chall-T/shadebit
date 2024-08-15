use image::{DynamicImage, GenericImageView, RgbaImage};
use std::error::Error;

pub fn embed_message_in_image(
    input_image_path: &str,
    output_image_path: &str,
    message: &str,
    password: &Option<String>
) -> Result<(), Box<dyn Error>> {
    // Load the image
    let mut img = image::open(input_image_path)?.into_rgba8();

    // Convert the message to binary representation
    let message_bin: Vec<u8> = message.chars()
        .flat_map(|c| {
            let binary_str = format!("{:08b}", c as u8); // Convert character to binary string
            binary_str.chars()
                .map(|b| (b as u8 - b'0')) // Convert binary character to u8 (0 or 1)
                .collect::<Vec<u8>>() // Collect into a Vec<u8>
        })
        .collect(); // Collect all binary data into a Vec<u8>

    let message_len = message_bin.len();

    // Check if the image is large enough to contain the message
    let (width, height) = img.dimensions();
    if message_len > (width * height * 4) as usize {
        return Err("Image is not large enough to hold the message".into());
    }

    let mut message_index = 0;

    // Embed the message into the image
    for y in 0..height {
        for x in 0..width {
            let mut pixel = img.get_pixel_mut(x, y);
            
            for channel in 0..4 {
                if message_index < message_len {
                    let bit = message_bin[message_index] == 1;
                    let mut value = pixel[channel];
                    if bit {
                        value |= 1; // Set the LSB to 1
                    } else {
                        value &= !1; // Set the LSB to 0
                    }
                    pixel[channel] = value;
                    message_index += 1;
                }
            }
        }
    }

    // Save the image with the embedded message
    img.save(output_image_path)?;

    Ok(())
}
