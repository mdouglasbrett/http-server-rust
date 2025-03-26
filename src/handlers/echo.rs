use crate::{
    http::{Headers, MimeType, Request, Response},
    Result,
};

pub struct EchoHandler;

impl EchoHandler {
    pub fn handle(request: &Request) -> Result<()> {
        let body = request.body.as_slice();
        let _resp = Response::builder()
            .body(Some(body))
            .encoding(request.get_header(Headers::ContentEncoding))
            .mime_type(MimeType::PlainText)
            .build();
        // TODO: write back to the stream
        Ok(())
    }
}
