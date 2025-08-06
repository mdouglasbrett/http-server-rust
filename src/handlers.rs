// TODO: there is a lot of boilerplate here, is that a code smell? Or is it
// an opportunity to hide some of this behind a macro?
use crate::{
    dir::FileSystemAccess,
    errors::AppError,
    http::{ClientError, Headers, Method, MimeType, Request, Response, ServerError, StatusCode},
    Result,
};
use std::io::Write;

#[derive(Debug)]
pub struct HandlerArg<'a, T>
where
    T: Write,
{
    pub req: &'a Request,
    pub stream: &'a mut T,
}

impl<'a, T> HandlerArg<'a, T>
where
    T: Write,
{
    pub fn new(req: &'a Request, stream: &'a mut T) -> HandlerArg<'a, T> {
        HandlerArg { req, stream }
    }
}

#[derive(Debug)]
pub struct FileHandlerArg<'a, T, U>
where
    T: Write,
    U: FileSystemAccess,
{
    pub req: &'a Request,
    pub stream: &'a mut T,
    pub target_dir: &'a U,
}

impl<'a, T, U> FileHandlerArg<'a, T, U>
where
    T: Write,
    U: FileSystemAccess,
{
    pub fn new(req: &'a Request, stream: &'a mut T, target_dir: &'a U) -> FileHandlerArg<'a, T, U> {
        FileHandlerArg {
            req,
            stream,
            target_dir,
        }
    }
}

#[derive(Debug)]
pub struct ErrorHandlerArg<'a, T>
where
    T: Write,
{
    pub stream: &'a mut T,
    pub err: AppError,
}

impl<'a, T> ErrorHandlerArg<'a, T>
where
    T: Write,
{
    pub fn new(stream: &'a mut T, err: AppError) -> ErrorHandlerArg<'a, T> {
        ErrorHandlerArg { stream, err }
    }
}

pub struct EchoHandler;
pub struct EmptyHandler;
pub struct FileHandler;
pub struct UserAgentHandler;
pub struct NotFoundHandler;
pub struct ErrorHandler;

pub trait Handler {
    fn handle<T>(r: HandlerArg<T>) -> Result<()>
    where
        T: Write;
}

impl Handler for EchoHandler {
    fn handle<T>(r: HandlerArg<T>) -> Result<()>
    where
        T: Write,
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
    fn handle<T>(r: HandlerArg<T>) -> Result<()>
    where
        T: Write,
    {
        let resp = Response::ok()?;
        r.stream.write_all(&resp.as_bytes())?;
        Ok(())
    }
}

impl Handler for UserAgentHandler {
    fn handle<T>(r: HandlerArg<T>) -> Result<()>
    where
        T: Write,
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
    fn handle<T>(r: HandlerArg<T>) -> Result<()>
    where
        T: Write,
    {
        let resp = Response::not_found()?;
        r.stream.write_all(&resp.as_bytes())?;
        Ok(())
    }
}

impl FileHandler {
    pub fn handle<T, U>(r: FileHandlerArg<T, U>) -> Result<()>
    where
        T: Write,
        U: FileSystemAccess,
    {
        let src = &r.req.path_parts[1];
        match r.req.method {
            Method::Get => {
                if let Ok(body) = r.target_dir.try_read(src) {
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
                r.target_dir.try_write(src, &r.req.body)?;
                let resp = Response::created()?;
                r.stream.write_all(&resp.as_bytes())?;
            }
            _ => return Err(ServerError::Internal.into()),
        }
        Ok(())
    }
}

impl ErrorHandler {
    pub fn handle<T>(a: ErrorHandlerArg<T>) -> Result<()>
    where
        T: Write,
    {
        match a.err {
            AppError::Client(ClientError::BadRequest) => {
                let resp = Response::client_error()?;
                a.stream.write_all(&resp.as_bytes())?;
            }
            AppError::Client(ClientError::NotFound) => {
                let resp = Response::not_found()?;
                a.stream.write_all(&resp.as_bytes())?;
            }
            AppError::Server(ServerError::NotImplemented) => {
                let resp = Response::builder()
                    .status_code(crate::http::StatusCode::NotImplemented)
                    .build()?;
                a.stream.write_all(&resp.as_bytes())?;
            }
            _ => {
                let resp = Response::server_error()?;
                a.stream.write_all(&resp.as_bytes())?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    mod handlers {
        use std::collections::HashMap;

        use crate::{handlers::*, http::Request, router::Route};

        #[test]
        fn handles_echo() {
            // TODO: some fixtures?
            let req = Request {
                method: Method::Get,
                route: Route::Echo,
                headers: HashMap::new(),
                body: b"hello".to_vec(),
                path: "/echo/hello".to_owned(),
                path_parts: vec!["echo".to_owned(), "hello".to_owned()],
            };
            let mut stream = Vec::new();
            let arg = HandlerArg::new(&req, &mut stream);
            let _ = EchoHandler::handle(arg);
            let expected = Response::builder()
                .status_code(StatusCode::Ok)
                .body(Some(b"hello".to_vec()))
                .mime_type(MimeType::PlainText)
                .build()
                .unwrap();
            assert_eq!(&expected.as_bytes(), &stream);
        }

        #[test]
        fn handles_user_agent() {
            // TODO: some fixtures?
            let req = Request {
                method: Method::Get,
                route: Route::UserAgent,
                headers: HashMap::from([(Headers::UserAgent, "Test-UA".to_owned())]),
                body: b"Test-UA".to_vec(),
                path: "/user-agent".to_owned(),
                path_parts: vec!["user-agent".to_owned()],
            };
            let mut stream = Vec::new();
            let arg = HandlerArg::new(&req, &mut stream);
            let _ = UserAgentHandler::handle(arg);
            let expected = Response::builder()
                .status_code(StatusCode::Ok)
                .body(Some(b"Test-UA".to_vec()))
                .mime_type(MimeType::PlainText)
                .build()
                .unwrap();
            assert_eq!(&expected.as_bytes(), &stream);
        }

        #[test]
        fn handles_empty() {
            // TODO: some fixtures?
            let req = Request {
                method: Method::Get,
                route: Route::Empty,
                headers: HashMap::new(),
                body: Vec::new(),
                path: "/".to_owned(),
                path_parts: vec!["/".to_owned()],
            };
            let mut stream = Vec::new();
            let arg = HandlerArg::new(&req, &mut stream);
            let _ = EmptyHandler::handle(arg);
            let expected = Response::builder()
                .status_code(StatusCode::Ok)
                .body(None)
                // TODO: do we care about MimeTypes on empty request responses?
                .mime_type(MimeType::PlainText)
                .build()
                .unwrap();
            assert_eq!(&expected.as_bytes(), &stream);
        }

        #[test]
        fn handles_read_file() {
            use crate::dir::FileSystemAccess;

            // TODO: extract this as fixture
            struct MockDir;

            impl FileSystemAccess for MockDir {
                fn try_read(&self, src: &str) -> Result<Vec<u8>> {
                    let _ = src;
                    Ok(b"Hi!".to_vec())
                }
                fn try_write(&self, _src: &str, _d: &[u8]) -> Result<()> {
                    Ok(())
                }
                fn try_create(&self) -> Result<()> {
                    Ok(())
                }
                fn check_dir_exists(&self) -> bool {
                    true
                }
            }
            let req = Request {
                method: Method::Get,
                route: Route::Files,
                headers: HashMap::new(),
                body: Vec::new(),
                path: "/files/test".to_owned(),
                path_parts: vec!["files".to_owned(), "test".to_owned()],
            };
            let mut stream = Vec::new();
            let target_dir = MockDir {};
            let arg = FileHandlerArg::new(&req, &mut stream, &target_dir);
            let _ = FileHandler::handle(arg);
            let expected = Response::builder()
                .status_code(StatusCode::Ok)
                .mime_type(MimeType::OctetStream)
                .body(Some(b"Hi!".to_vec()))
                .build()
                .unwrap();
            assert_eq!(&expected.as_bytes(), &stream);
        }

        #[test]
        fn handles_write_file() {}

        #[test]
        fn handles_error() {}
    }
}
