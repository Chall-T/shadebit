mod commands;
mod steganography;

fn main() {
    let args = commands::cli::get_args();

    let file_path = match args.file {
        Some(ref file) => file.clone(), // Clone the value to use it in multiple places
        None => {
            eprintln!("Input file is required!");
            return;
        }
    };

    let message = match args.message {
        Some(ref msg) => msg.clone(), // Clone the message to use it later
        None => {
            eprintln!("Message is required!");
            return;
        }
    };

    let password = args.password.clone(); // Clone password if needed later

    println!("Input file: {}", file_path);
    println!("Message: {}", message);
    if let Some(ref pwd) = password {
        println!("Password: {}", pwd);
    }

    match steganography::encrypt::embed_message_in_image(&file_path, &file_path, &message, &password) {
        Ok(_) => println!("Message embedded successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }

}
