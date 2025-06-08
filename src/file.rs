use std::{
    fs::{read, write},
    path::Path,
};

use crate::Result;

#[derive(Debug, Default)]
pub struct File;

impl File {
    pub fn new() -> Self {
        Self
    }
}

pub trait FileAccess {
    fn try_read(&self, p: &Path) -> Result<Vec<u8>>;
    fn try_write(&self, p: &Path, d: &[u8]) -> Result<()>;
}

impl FileAccess for File {
    fn try_read(&self, p: &Path) -> Result<Vec<u8>> {
        let d = read(p)?;
        Ok(d)
    }
    fn try_write(&self, p: &Path, d: &[u8]) -> Result<()> {
        write(p, d)?;
        Ok(())
    }
}
