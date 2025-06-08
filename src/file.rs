use std::{
    fs::{read, write},
    path::PathBuf,
};

use crate::{constants, Result};

#[derive(Debug)]
pub struct File {
    target_dir: PathBuf,
}

impl File {
    pub fn new(target_dir: PathBuf) -> Self {
        Self { target_dir }
    }
}

impl Default for File {
    fn default() -> Self {
        Self {
            target_dir: PathBuf::from(constants::TARGET_DIR),
        }
    }
}

pub trait FileAccess {
    fn try_read(&self, src: &str) -> Result<Vec<u8>>;
    fn try_write(&self, src: &str, d: &[u8]) -> Result<()>;
}

impl FileAccess for File {
    fn try_read(&self, src: &str) -> Result<Vec<u8>> {
        let d = read(self.target_dir.join(src))?;
        Ok(d)
    }
    fn try_write(&self, src: &str, d: &[u8]) -> Result<()> {
        write(self.target_dir.join(src), d)?;
        Ok(())
    }
}
