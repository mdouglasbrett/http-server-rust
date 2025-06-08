use crate::{
    constants::{ADDRESS, TARGET_DIR},
    Result,
};
use lexopt::prelude::*;
use std::{
    fs::create_dir,
    path::{Path, PathBuf},
};

fn check_directory_exists(dir: &Path) -> bool {
    dir.exists() && dir.is_dir()
}

#[derive(Debug)]
pub struct Config {
    pub address: String,
    pub directory: PathBuf,
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
                            let dir_path =
                                Path::new(&format!("{}{}", TARGET_DIR, parsed_val)).to_owned();
                            if !check_directory_exists(&dir_path) {
                                // TODO: Abstract this away like I did with File?
                                if let Err(e) = create_dir(&dir_path) {
                                    return Err(e.into());
                                }
                            }
                            config.directory = dir_path;
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
            directory: PathBuf::from(TARGET_DIR),
            address: ADDRESS.to_owned(),
        }
    }
}
