use std::error::Error;



fn xor_decrypt(encrypted_message: Vec<u8>, password: &str) -> String {
    if password.is_empty() {
        return encrypted_message.into_iter()
            .map(|m| m as char)
            .collect();
    }
    encrypted_message.into_iter()
        .zip(password.bytes().cycle())
        .map(|(m, p)| (m ^ p) as char)
        .collect()
}


pub fn extract_message_from_image(
    input_image_path: &str,
    password: &Option<String>,
) -> Result<String, Box<dyn Error>> {
    let img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::open(input_image_path)?.into_rgba8();

    let (width, height) = img.dimensions();
    let mut message_bin: Vec<u8> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            for channel in 0..4 {
                let lsb = pixel[channel] & 1;
                message_bin.push(lsb);

                if message_bin.len() % 8 == 0 {
                    let byte = message_bin[message_bin.len() - 8..]
                        .iter()
                        .fold(0, |acc, &b| (acc << 1) | b);

                    if byte == 0 {
                        let encrypted_message = message_bin[..message_bin.len() - 8]
                            .chunks(8)
                            .map(|chunk| {
                                chunk.iter()
                                    .fold(0, |acc, &b| (acc << 1) | b)
                            })
                            .collect::<Vec<u8>>();

                        let password = password.as_deref().unwrap_or("");
                        let decrypted_message = xor_decrypt(encrypted_message, password);

                        return Ok(decrypted_message);
                    }
                }
            }
        }
    }

    Err("No hidden message found".into())
}
