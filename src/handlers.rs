use crate::{
    errors::AppError,
    file::FileAccess,
    http::{ClientError, Headers, Method, MimeType, Request, Response, ServerError, StatusCode},
    Result,
};
use std::{
    default::Default,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct HandlerArg<'a, T, U>
where
    T: Write,
    U: FileAccess + Default,
{
    pub req: &'a Request,
    pub stream: &'a mut T,
    pub target_dir: &'a PathBuf,
    pub file: Option<U>,
}

#[derive(Debug)]
pub struct ErrorHandlerArg<'a, T>(pub &'a mut T, pub AppError)
where
    T: Write;

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
        U: FileAccess + Default;
}

impl Handler for EchoHandler {
    fn handle<T, U>(r: HandlerArg<T, U>) -> Result<()>
    where
        T: Write,
        U: FileAccess + Default,
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
        U: FileAccess + Default,
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
        U: FileAccess + Default,
    {
        let filename = &r.req.path_parts[1];
        let path = Path::new(r.target_dir).join(filename);
        let file = r.file.unwrap_or_default();
        match r.req.method {
            Method::Get => {
                // As I am checking the file before the match, I don't this this unwrap is so
                // bad?
                if let Ok(body) = file.try_read(&path) {
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
                // See above
                file.try_write(&path, &r.req.body)?;
                let resp = Response::created()?;
                r.stream.write_all(&resp.as_bytes())?;
            }
            _ => return Err(ServerError::Internal.into()),
        }
        Ok(())
    }
}

impl Handler for UserAgentHandler {
    fn handle<T, U>(r: HandlerArg<T, U>) -> Result<()>
    where
        T: Write,
        U: FileAccess + Default,
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
        U: FileAccess + Default,
    {
        let resp = Response::not_found()?;
        r.stream.write_all(&resp.as_bytes())?;
        Ok(())
    }
}

impl ErrorHandler {
    pub fn handle<T>(a: ErrorHandlerArg<T>) -> Result<()>
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

#[cfg(test)]
mod tests {

    mod handlers {

        #[test]
        fn handles_echo() {}

        #[test]
        fn handles_user_agent() {}

        #[test]
        fn handles_empty() {}

        #[test]
        fn handles_read_file() {}

        #[test]
        fn handles_write_file() {}

        #[test]
        fn handles_error() {}
    }
}
