use std::path::Path;

use crate::Result;

#[derive(Debug)]
pub struct File;

pub trait FileHandler {
    fn try_read(p: &Path) -> Result<()>;
    fn try_write(p: &Path) -> Result<()>;
}

impl FileHandler for File {
    fn try_read(p: &Path) -> Result<()> {
        todo!();
    }
    fn try_write(p: &Path) -> Result<()> {
        todo!();
    }
}
