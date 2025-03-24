use crate::{
    http::{Headers, MimeType, Request, Response},
    Result,
};

pub struct EchoHandler;

impl EchoHandler {
    pub fn handle(request: &Request) -> Result<Response> {
        let body = request.body.as_slice();
        Response::builder()
            .body(Some(body))
            .encoding(request.get_header(Headers::ContentEncoding))
            .mime_type(MimeType::PlainText)
            .build()
    }
}
