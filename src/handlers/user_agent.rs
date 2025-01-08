use crate::{
    handlers::Handler,
    http::{Request, Response},
    Result,
};

pub struct UserAgentHandler;

impl Handler for UserAgentHandler {
    fn handle(&self, request: &Request) -> Result<Response> {
        todo!()
    }
}
