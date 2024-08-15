use clap::{Arg, Command};

pub struct CliArgs {
    pub file: Option<String>,
    pub message: Option<String>,
    pub password: Option<String>,
}

pub fn get_args() -> CliArgs {
    let matches = Command::new("Shade bit")
        .version("1.0")
        .about("A steganography tool in rust")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Select file")
        )
        .arg(
            Arg::new("message")
                .short('m')
                .long("message")
                .value_name("MESSAGE")
                .help("Embeds message in the file")
        )
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .value_name("PASSWORD")
                .help("Specifies a password for the message")
        )
        .get_matches();


    CliArgs {
        file: matches.get_one::<String>("file").cloned(),
        message: matches.get_one::<String>("message").cloned(),
        password: matches.get_one::<String>("password").cloned(),
    }
    // Create a binding for the default value
    // let file: String = matches
    //     .get_one::<String>("file")
    //     .unwrap_or(&"".to_string())
    //     .clone();

    // // Parse the `count` argument as a `usize`
    // let count: usize = matches
    //     .get_one::<String>("count")
    //     .unwrap_or(&"1".to_string())
    //     .parse()
    //     .expect("Count must be a positive integer");

    // CliArgs { file }
}
