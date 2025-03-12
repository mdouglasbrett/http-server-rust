use crate::{
    http::{Request, Response},
    Result,
};

pub struct FileHandler;

impl FileHandler {
    pub fn handle(request: &Request, dir: String) -> Result<Response> {
        todo!()
    }
}
