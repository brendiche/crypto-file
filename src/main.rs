use std::{env, process};

use crypto_file::{Config , Mode};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let result_file = match config.mode {
        Mode::Encrypt(_) => crypto_file::encrypt(&config.file_path).unwrap(),
        Mode::Decrypt(_) => crypto_file::decrypt(&config.file_path).unwrap()
    };

    println!("{:?}", result_file);
}
