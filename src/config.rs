use crate::{
    constants::{ADDRESS, TARGET_DIR},
    dir::{Dir, FileSystemAccess},
    Result,
};
use lexopt::prelude::*;

#[derive(Debug)]
pub struct Config {
    pub address: String,
    pub directory: Dir,
}

impl Config {
    pub fn try_new() -> Result<Config> {
        let mut parser = lexopt::Parser::from_env();
        let mut config = Config::default();
        while let Ok(Some(arg)) = parser.next() {
            match arg {
                Short('t') | Long("target_dir") => {
                    if let Ok(val) = parser.value() {
                        if let Ok(parsed_val) = val.parse::<String>() {
                            let dir = Dir::new(&format!("{TARGET_DIR}{parsed_val}"));
                            dir.try_create()?;
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
                    println!("Usage: cargo run -- [-t | --target_dir=TARGET_DIR] [-a | --address=ADDRESS]");
                    std::process::exit(0);
                }
                _ => {
                    println!("Error: cargo run -- [-h | --help] for usage");
                    std::process::exit(1);
                }
            }
        }
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            address: ADDRESS.to_owned(),
            directory: Dir::default(),
        }
    }
}
