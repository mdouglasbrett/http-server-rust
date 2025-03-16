use crate::{http::{Request, Response}, Result};

pub struct EmptyHandler;

impl EmptyHandler {
    pub fn handle(_request: &Request) -> Result<Response> {
        Response::ok()
    }
}
