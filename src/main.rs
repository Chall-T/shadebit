mod commands;
mod steganography;
use std::io::{self, Write};

fn read_data_from_image(image_path: &str, password: &Option<String>) -> String{
    if let Ok(false) = steganography::detect::detect_hidden_data(image_path){
        println!("No data found")
    }
    match steganography::decrypt::extract_message_from_image(image_path, password) {
        Ok(message) => message,
        Err(e) => {
            eprintln!("Error extracting message: {}", e);
            String::new()
        }
    }
}

fn write_data_to_image(
    input_image_path: &str,
    output_image_path: &str,
    message: &str,
    password: &Option<String>,
) {
    if let Ok(true) = steganography::detect::detect_hidden_data(input_image_path) {
        print!("Hidden data detected in the image. Do you want to replace it? (Y/N): ");
        io::stdout().flush().unwrap();

        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        let response = response.trim().to_uppercase();

        if response != "Y" && response != "y" {
            println!("Operation aborted.");
            return;
        }
    }
    match steganography::encrypt::embed_message_in_image(
        input_image_path, output_image_path, message, password,
    ) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
    let extracted_message = read_data_from_image(output_image_path, password);
    if message != extracted_message {
        eprintln!(
            "Error! Expectet result:{}, got:{}",
            message, extracted_message
        );
    }
}

fn main() {
    let args = commands::cli::get_args();

    let file_path = match args.file {
        Some(ref file) => file.clone(),
        None => {
            eprintln!("Input file is required!");
            return;
        }
    };
    let password = args.password.clone();

    let message = match args.message {
        Some(ref msg) => msg.clone(),
        None => {
            println!("{}", read_data_from_image(&file_path, &password));
            return;
        }
    };

    write_data_to_image(&file_path, &file_path, &message, &password)


}
