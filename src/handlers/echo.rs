use crate::{
    http::{Request, Response},
    Result,
};

pub(crate) struct EchoHandler;

impl EchoHandler {
    pub(crate) fn handle(request: &Request) -> Result<Response> {
        todo!()
    }
}
