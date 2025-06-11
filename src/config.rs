use crate::{
    constants::{ADDRESS, TARGET_DIR},
    traits::FileSystemAccess,
    Result,
};
use lexopt::prelude::*;
use std::{
    fs::{create_dir, read, write},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct Dir {
    path: PathBuf,
}

impl Dir {
    pub fn new(p: &str) -> Self {
        Self {
            path: PathBuf::from(p)
        }
    } 
}

impl Default for Dir {
    fn default() -> Self {
        Self::new(TARGET_DIR)
    }
}

impl FileSystemAccess for Dir {
    fn check_dir_exists(&self) -> bool {
        self.path.exists() && self.path.is_dir()
    }
    fn try_read(&self, src: &str) -> Result<Vec<u8>> {
        let d = read(self.path.join(src))?;
        Ok(d)
    }
    fn try_write(&self, src: &str, d: &[u8]) -> Result<()> {
        write(self.path.join(src), d)?;
        Ok(())
    }
    fn try_create(&self) -> Result<()> {
       create_dir(&self.path)?;
       Ok(())
    }
}

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
