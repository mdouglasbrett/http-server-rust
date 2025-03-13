use crate::{
    http::{Request, Response, StatusCode},
    Result,
};

pub struct EchoHandler;

impl EchoHandler {
    pub fn handle(request: &Request) -> Result<Response> {
        Response::builder()
            .status_code(StatusCode::Ok)
            .body(&request.body)
            .build()
    }
}
