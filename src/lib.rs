use std::path::Path;

#[derive(Debug)]
pub enum Mode {
    Encrypt(String),
    Decrypt(String),
}
#[derive(Debug)]
pub struct Config {
    pub file_path: String,
    pub mode: Mode,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        if !Path::new(&args[1]).exists() {
            return Err("file not found");
        }
        let file_path = args[1].clone();
        let mode = args[2].clone();
        let mode = match mode.as_str() {
            "encrypt" => Mode::Encrypt(String::from("encrypt")),
            "decrypt" => Mode::Decrypt(String::from("decrypt")),
            _ => return Err("invalid mode"),
        };
        Ok(Config { file_path, mode })
    }
}
