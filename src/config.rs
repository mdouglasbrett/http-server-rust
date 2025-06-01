use crate::Result;
use lexopt::prelude::*;
use std::path::Path;

const TARGET_DIR: &str = "/tmp";
const ADDRESS: &str = "127.0.0.1:4221";

pub const HTTP_VERSION: &str = "HTTP/1.1";

fn check_directory_exists(dir: &Path) -> bool {
    dir.exists() && dir.is_dir()
}

#[derive(Debug)]
pub struct Config {
    pub address: String,
    // TODO: this can be more specific than a string...
    pub directory: String,
}

impl Config {
    pub fn new() -> Result<Config> {
        let mut parser = lexopt::Parser::from_env();
        let mut config = Config::default();
        while let Ok(Some(arg)) = parser.next() {
            match arg {
                Short('t') | Long("target_dir") => {
                    if let Ok(val) = parser.value() {
                        // TODO: should make this directory handling more robust
                        if let Ok(parsed_val) = val.parse::<String>() {
                            let raw_dir = parsed_val;
                            let dir_string = format!("{}{}", TARGET_DIR, &raw_dir);
                            let dir_path = Path::new(&dir_string);
                            if !check_directory_exists(&dir_path) {
                                // TODO: how do we abstract over the file system to make this
                                // testable
                                // Do we even really do that in Rust?
                                if let Err(e) = std::fs::create_dir(&dir_path) {
                                    return Err(e.into());
                                }
                            }
                            config.directory = dir_string;
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
    fn default() -> Self {
        Self {
            directory: TARGET_DIR.to_owned(),
            address: ADDRESS.to_owned(),
        }
    }
}
