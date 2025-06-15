use crate::{constants::TARGET_DIR, Result};
use std::{
    fs::{create_dir, read, write},
    path::PathBuf,
};

pub trait FileSystemAccess {
    fn try_read(&self, src: &str) -> Result<Vec<u8>>;
    fn try_write(&self, src: &str, d: &[u8]) -> Result<()>;
    fn check_dir_exists(&self) -> bool;
    fn try_create(&self) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct Dir {
    path: PathBuf,
}

impl Dir {
    pub fn new(p: &str) -> Self {
        Self {
            path: PathBuf::from(p),
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
        if !self.check_dir_exists() {
            create_dir(&self.path)?;
        }
        Ok(())
    }
}
