use crate::{
    http::{Headers, MimeType, Request, Response},
    Result,
};

pub struct UserAgentHandler;

impl UserAgentHandler {
    pub fn handle(request: &Request) -> Result<Response> {
        let body = if let Some(b) = request.get_header(Headers::UserAgent) {
            Some(b.as_bytes())
        } else {
            None
        };
        Response::builder()
            .body(body)
            .encoding(request.get_header(Headers::ContentEncoding))
            .mime_type(MimeType::PlainText)
            .build()
    }
}
