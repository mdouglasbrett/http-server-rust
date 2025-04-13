use lexopt::prelude::*;

const TARGET_DIR: &str = "/tmp/";
const ADDRESS: &str = "127.0.0.1:4221";

pub const HTTP_VERSION: &str = "HTTP/1.1";

#[derive(Debug)]
pub struct Config {
    pub address: String,
    pub directory: String,
}

impl Config {
    pub fn new() -> Self {
        let mut parser = lexopt::Parser::from_env();
        let mut config = Config::default();
        while let Ok(Some(arg)) = parser.next() {
            match arg {
                Short('t') | Long("target_dir") => {
                    if let Ok(val) = parser.value() {
                        if let Ok(parsed_val) = val.parse::<String>() {
                            let dir = format!("{}{}", TARGET_DIR, &parsed_val);
                            config.directory = dir;
                        }
                    }
                }
                Short('a') | Long("address") => {
                    if let Ok(val) = parser.value() {
                        if let Ok(parsed_val) = val.parse() {
                            config.address = parsed_val;
                        }
                    }
                }
                Short('h') | Long("help") => {
                    println!("Usage: cargo run -- [-t | --target_dir=TARGET_DIR] [-a | --adress=ADDRESS]");
                    std::process::exit(0);
                }
                _ => {
                    println!("Error: cargo run -- [-h | --help] for usage");
                    std::process::exit(1);
                }
            }
        }
        config
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            directory: TARGET_DIR.to_owned(),
            address: ADDRESS.to_owned(),
        }
    }
}
