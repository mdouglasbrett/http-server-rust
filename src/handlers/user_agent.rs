use crate::{
    http::{Headers, MimeType, Request, Response},
    Result,
};

pub struct UserAgentHandler;

impl UserAgentHandler {
    pub fn handle(request: &Request) -> Result<Response> {
        Response::builder()
            .body(request.get_header(Headers::UserAgent))
            .encoding(request.get_header(Headers::ContentEncoding))
            .mime_type(MimeType::PlainText)
            .build()
    }
}
