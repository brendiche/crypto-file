use std::path::Path;
use std::error::Error;
use std::io::Write;
use encryptfile as ef;
use rpassword::read_password;


pub enum Mode {
    Encrypt(String),
    Decrypt(String),
}

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

    pub fn get_key() -> String {
        // TODO: add confirmation for key
        println!("Enter the crypto key: ");
        std::io::stdout().flush().unwrap();
        read_password().unwrap()
    }
}

pub fn encrypt(file_path: &str) -> Result<String, Box<dyn Error>>{
    let mut in_file = String::from("./");
    in_file.push_str(file_path);
    let mut c = ef::Config::new();
    let crypto_key = Config::get_key();
    c.input_stream(ef::InputStream::File(in_file.to_owned()))
    .output_stream(ef::OutputStream::File(format!("encrypted_{}", file_path).to_owned()))
    .add_output_option(ef::OutputOption::AllowOverwrite)
    .initialization_vector(ef::InitializationVector::GenerateFromRng)
    .password(ef::PasswordType::Text(crypto_key.to_owned(), ef::scrypt_defaults()))
    .encrypt();
    // find a way to handle errors
    let _ = ef::process(&c).map_err(|e| panic!("error encrypting: {:?}", e));

    Ok(format!("encrypted_{}", file_path))
}

pub fn decrypt(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut c = ef::Config::new();
    let crypto_key = Config::get_key();
    c.input_stream(ef::InputStream::File(file_path.to_owned()))
    .output_stream(ef::OutputStream::File(format!("decrypted_{}", file_path).to_owned()))
    .add_output_option(ef::OutputOption::AllowOverwrite)
    .password(ef::PasswordType::Text(crypto_key.to_owned(), ef::PasswordKeyGenMethod::ReadFromFile))
    .decrypt();
    // find a way to handle errors
    let _ = ef::process(&c).map_err(|e| panic!("error decrypting: {:?}", e));
    println!("decrypting {}", file_path);
    Ok(format!("decrypted_{}", file_path))
}