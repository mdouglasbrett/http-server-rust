use crate::{
    handlers::Handler,
    http::{Request, Response},
    Result,
};

pub struct EchoHandler;

impl Handler for EchoHandler {
    fn handle(&self, request: &Request) -> Result<Response> {
        todo!()
    }
}
