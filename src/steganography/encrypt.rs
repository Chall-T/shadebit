use image;
use std::error::Error;

fn xor_encrypt(message: &str, password: &str) -> Vec<u8> {
    if password.is_empty() {
        return message.bytes().collect();
    }
    message
        .bytes()
        .zip(password.bytes().cycle())
        .map(|(m, p)| m ^ p)
        .collect()
}


pub fn embed_message_in_image(
    input_image_path: &str,
    output_image_path: &str,
    message: &str,
    password: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    let mut img = image::open(input_image_path)?.into_rgba8();

    let password = password.as_deref().unwrap_or("");
    let encrypted_message = xor_encrypt(message, password);

    let mut terminated_message = encrypted_message.clone();
    terminated_message.push(0);

    let message_len = terminated_message.len();

    let (width, height) = img.dimensions();
    if message_len * 8 > (width * height * 4) as usize {
        return Err("Image is not large enough to hold the message".into());
    }

    let mut message_index = 0;

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel_mut(x, y);

            for channel in 0..4 {
                if message_index < message_len * 8 {
                    let byte_index = message_index / 8;
                    let bit_index = message_index % 8;
                    let bit = (terminated_message[byte_index] >> (7 - bit_index)) & 1;

                    let mut value = pixel[channel];
                    if bit == 1 {
                        value |= 1;
                    } else {
                        value &= !1;
                    }
                    pixel[channel] = value;
                    message_index += 1;
                }
            }
        }
    }

    img.save(output_image_path)?;

    Ok(())
}
