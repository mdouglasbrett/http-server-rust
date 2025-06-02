use crate::{
    errors::AppError,
    // TODO: rename this trait
    file::FileHandler as FH,
    http::{ClientError, Headers, Method, MimeType, Request, Response, ServerError, StatusCode},
    Result,
};
use std::{io::Write, path::Path};

#[derive(Debug)]
pub struct HandlerArg<'a, T, U>
where
    T: Write,
    U: FH,
{
    pub req: &'a Request,
    pub stream: &'a mut T,
    // TODO: might get away with a &'a str here
    pub target_dir: &'a String,
    pub file: Option<U>,
}

pub struct EchoHandler;
pub struct EmptyHandler;
pub struct FileHandler;
pub struct UserAgentHandler;
pub struct NotFoundHandler;
pub struct ErrorHandler;

pub trait Handler {
    fn handle<T, U>(r: HandlerArg<T, U>) -> Result<()>
    where
        T: Write,
        U: FH;
}

impl Handler for EchoHandler {
    fn handle<T, U>(r: HandlerArg<T, U>) -> Result<()>
    where
        T: Write,
        U: FH,
    {
        let body = r.req.body.as_slice();
        let resp = Response::builder()
            .body(Some(body.to_owned()))
            .encoding(r.req.get_header(Headers::AcceptEncoding))
            .mime_type(MimeType::PlainText)
            .build()?;
        r.stream.write_all(&resp.as_bytes())?;
        Ok(())
    }
}
impl Handler for EmptyHandler {
    fn handle<T, U>(r: HandlerArg<T, U>) -> Result<()>
    where
        T: Write,
        U: FH,
    {
        let resp = Response::ok()?;
        r.stream.write_all(&resp.as_bytes())?;
        Ok(())
    }
}

impl Handler for FileHandler {
    fn handle<T, U>(r: HandlerArg<T, U>) -> Result<()>
    where
        T: Write,
        U: FH,
    {
        let filename = &r.req.path_parts[1];
        let path = Path::new(r.target_dir).join(filename);
        if r.file.is_some() {
            match r.req.method {
                Method::Get => {
                    if let Ok(body) = r.file.unwrap().try_read(&path) {
                        let resp = Response::builder()
                            .status_code(StatusCode::Ok)
                            .body(Some(body))
                            .encoding(r.req.get_header(Headers::AcceptEncoding))
                            .mime_type(MimeType::OctetStream)
                            .build()?;
                        r.stream.write_all(&resp.as_bytes())?;
                    } else {
                        return Err(ClientError::NotFound.into());
                    }
                }
                Method::Post => {
                    r.file.unwrap().try_write(&path, &r.req.body)?;
                    let resp = Response::created()?;
                    r.stream.write_all(&resp.as_bytes())?;
                }
                _ => return Err(ServerError::Internal.into()),
            }
        } else {
            return Err(ServerError::Internal.into());
        }
        Ok(())
    }
}

impl Handler for UserAgentHandler {
    fn handle<T, U>(r: HandlerArg<T, U>) -> Result<()>
    where
        T: Write,
        U: FH,
    {
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
    fn handle<T, U>(r: HandlerArg<T, U>) -> Result<()>
    where
        T: Write,
        U: FH,
    {
        let resp = Response::not_found()?;
        r.stream.write_all(&resp.as_bytes())?;
        Ok(())
    }
}

impl ErrorHandler {
    // TODO: make this arg a concrete type?
    pub fn handle<T>(a: (&mut T, AppError)) -> Result<()>
    where
        T: Write,
    {
        match a.1 {
            AppError::Client(ClientError::BadRequest) => {
                let resp = Response::client_error()?;
                a.0.write_all(&resp.as_bytes())?;
            }
            AppError::Client(ClientError::NotFound) => {
                let resp = Response::not_found()?;
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
