use crate::{
    errors::AppError,
    http::{Headers, MimeType, Request, Response},
    Result,
};
use std::{io::Write, net::TcpStream};

#[derive(Debug)]
pub struct HandlerArg<'a> {
    pub req: &'a Request,
    pub stream: &'a mut TcpStream,
    // TODO: might get away with a &'a str here
    pub target_dir: &'a String,
}

pub struct EchoHandler;
pub struct EmptyHandler;
pub struct FileHandler;
pub struct UserAgentHandler;
pub struct NotFoundHandler;
pub struct ErrorHandler;

pub trait Handler {
    fn handle(r: HandlerArg) -> Result<()>;
}

impl Handler for EchoHandler {
    fn handle(r: HandlerArg) -> Result<()> {
        let body = r.req.body.as_slice();
        let resp = Response::builder()
            .body(Some(body.to_owned()))
            .encoding(r.req.get_header(Headers::ContentEncoding))
            .mime_type(MimeType::PlainText)
            .build()?;
        r.stream.write_all(&resp.as_bytes())?;
        Ok(())
    }
}
impl Handler for EmptyHandler {
    fn handle(r: HandlerArg) -> Result<()> {
        let resp = Response::ok()?;
        r.stream.write_all(&resp.as_bytes())?;
        Ok(())
    }
}
impl Handler for FileHandler {
    fn handle(_r: HandlerArg) -> Result<()> {
        Ok(())
    }
}

// TODO: All of the assignment here! Is this stupid?
impl Handler for UserAgentHandler {
    fn handle(r: HandlerArg) -> Result<()> {
        // TODO: is it worth having this in the map as bytes?
        let body = r.req.get_header(Headers::UserAgent).map(|b| b.to_owned());
        let resp = Response::builder()
            // TODO: need to get the string out
            .body(Some(body.unwrap_or(String::from("")).as_bytes().to_owned()))
            .encoding(r.req.get_header(Headers::ContentEncoding))
            .mime_type(MimeType::PlainText)
            .build()?;
        r.stream.write_all(&resp.as_bytes())?;
        Ok(())
    }
}

impl Handler for NotFoundHandler {
    fn handle(r: HandlerArg) -> Result<()> {
        let resp = Response::not_found()?;
        r.stream.write_all(&resp.as_bytes())?;
        Ok(())
    }
}

impl ErrorHandler {
    pub fn handle(_a: (&mut TcpStream, AppError)) -> Result<()> {
        todo!();
    }
}
