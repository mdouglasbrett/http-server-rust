use crate::constants::defaults;
use lexopt::prelude::*;

#[derive(Debug)]
pub struct Config {
    pub directory: String,
    pub address: String,
}

impl Config {
    pub fn new() -> Self {
        let mut parser = lexopt::Parser::from_env();
        let mut config = Config::default();
        while let Ok(Some(arg)) = parser.next() {
            match arg {
                Short('t') | Long("target_dir") => {
                    if let Ok(val) = parser.value() {
                        if let Ok(parsed_val) = val.parse() {
                            config.directory = parsed_val;
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
            directory: defaults::TARGET_DIR.to_owned(),
            address: defaults::ADDRESS.to_owned(),
        }
    }
}
