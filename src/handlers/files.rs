use crate::{
    http::{Request, Response},
    Result,
};

pub(crate) struct FileHandler;

impl FileHandler {
    pub(crate) fn handle(request: &Request, dir: String) -> Result<Response> {
        todo!()
    }
}
