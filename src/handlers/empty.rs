use crate::{
    http::{Request, Response},
    Result,
};

pub struct EmptyHandler;

impl EmptyHandler {
    pub fn handle(request: &Request) -> Result<Response> {
        Response::ok()
    }
}
