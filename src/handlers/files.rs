use crate::{
    handlers::Handler,
    http::{Request, Response},
    Result,
};

pub struct FileHandler;

impl Handler for FileHandler {
    fn handle(&self, request: &Request) -> Result<Response> {
        todo!()
    }
}
