use crate::{
    http::{Headers, MimeType, Request, Response},
    Result,
};

pub struct UserAgentHandler;

impl UserAgentHandler {
    pub fn handle(request: &Request) -> Result<Response> {
        let body = request.get_header(Headers::UserAgent).map(|b| b.as_bytes());
        Response::builder()
            .body(body)
            .encoding(request.get_header(Headers::ContentEncoding))
            .mime_type(MimeType::PlainText)
            .build()
    }
}
