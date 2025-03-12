use crate::{
    http::{Request, ResponseBuilder, Response},
    Result,
};

pub struct EchoHandler;

impl EchoHandler {
    pub fn handle(request: &Request) -> Result<Response> {
        let mut response = ResponseBuilder::new().build();
        response
    }
}
