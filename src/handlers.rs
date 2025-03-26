use crate::{http::{Request, Response, MimeType, Headers}, Result};
use std::net::TcpStream;

#[derive(Debug)]
pub struct HandlerArg<'a> {
    pub req: &'a Request,
    pub stream: &'a TcpStream,
    // TODO: might get away with a &'a str here
    pub target_dir: &'a String,
}

pub struct EchoHandler;
pub struct EmptyHandler;
pub struct FileHandler;
pub struct UserAgentHandler;

pub trait Handler {
    fn handle(r: HandlerArg) -> Result<()>;
}

impl Handler for EchoHandler {
    fn handle(r: HandlerArg) -> Result<()> {
        let body = r.req.body.as_slice();
        let _resp = Response::builder()
            .body(Some(body))
            .encoding(r.req.get_header(Headers::ContentEncoding))
            .mime_type(MimeType::PlainText)
            .build();
        Ok(())
    }
}
impl Handler for EmptyHandler {
    fn handle(r: HandlerArg) -> Result<()> {
        let _resp = Response::ok();
        Ok(())
    }
}
impl Handler for FileHandler {
    fn handle(r: HandlerArg) -> Result<()> {
        Ok(())
    }
}
impl Handler for UserAgentHandler {
    fn handle(r: HandlerArg) -> Result<()> {
        let body = r.req.get_header(Headers::UserAgent).map(|b| b.as_bytes());
        let _resp = Response::builder()
            .body(body)
            .encoding(r.req.get_header(Headers::ContentEncoding))
            .mime_type(MimeType::PlainText)
            .build();
        Ok(())
    }
}
