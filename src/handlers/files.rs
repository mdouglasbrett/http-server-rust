use crate::{http::Request, Result};

pub struct FileHandler;

impl FileHandler {
    pub fn handle(_request: &Request, _dir: String) -> Result<()> {
        Ok(())
    }
}
