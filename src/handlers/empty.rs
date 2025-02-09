use crate::{
    http::{Request, Response},
    Result,
};

pub(crate) struct EmptyHandler;

impl EmptyHandler {
    pub(crate) fn handle(request: &Request) -> Result<Response> {
        todo!()
    }
}
