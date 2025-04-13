use crate::{
    errors::AppError,
    http::{ClientError, Headers, Method, MimeType, Request, Response, ServerError},
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
    fn handle(r: HandlerArg) -> Result<()> {
        // TODO:
        // We are going to need to differentiate based on METHOD
        // Do we need a File type?
        if r.req.method == Method::Get {
            // TODO: how testable is this going to be?
            // How do I abstract the IO part of this? Do I even need to?
        } else {
            let resp = Response::created()?;
            r.stream.write_all(&resp.as_bytes)?;
        }
        Ok(())
    }
}

impl Handler for UserAgentHandler {
    fn handle(r: HandlerArg) -> Result<()> {
        let b = r
            .req
            .get_header(Headers::UserAgent)
            .map(|b| b.as_bytes().to_owned());
        let resp = Response::builder()
            .body(b)
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
    pub fn handle(a: (&mut TcpStream, AppError)) -> Result<()> {
        match a.1 {
            AppError::Client(ClientError::BadRequest) => {
                let resp = Response::client_error()?;
                a.0.write_all(&resp.as_bytes())?;
            }
            AppError::Server(ServerError::NotImplemented) => {
                let resp = Response::builder()
                    .status_code(crate::http::StatusCode::NotImplemented)
                    .build()?;
                a.0.write_all(&resp.as_bytes())?;
            }
            _ => {
                let resp = Response::server_error()?;
                a.0.write_all(&resp.as_bytes())?;
            }
        }
        Ok(())
    }
}
