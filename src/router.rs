use crate::{
    dir::FileSystemAccess,
    handlers::*,
    http::{ClientError, Method, Request, ServerError},
    Result,
};
use std::io::{BufReader, Read, Write};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Route {
    Empty,
    Echo,
    UserAgent,
    Files,
    Unknown,
}

impl From<&str> for Route {
    fn from(s: &str) -> Self {
        match s {
            "echo" => Self::Echo,
            "user-agent" => Self::UserAgent,
            "files" => Self::Files,
            "/" => Self::Empty,
            _ => Self::Unknown,
        }
    }
}

enum Operation {
    GetEcho,
    GetUserAgent,
    GetFileContents,
    PostFileContents,
    GetEmpty,
    Unsupported,
    Unknown,
}

impl From<&Request> for Operation {
    fn from(value: &Request) -> Self {
        match (&value.method, &value.route) {
            (Method::Get, Route::Echo) => Self::GetEcho,
            (Method::Get, Route::Files) => Self::GetFileContents,
            (Method::Post, Route::Files) => Self::PostFileContents,
            (Method::Get, Route::UserAgent) => Self::GetUserAgent,
            (Method::Get, Route::Empty) => Self::GetEmpty,
            (Method::Unsupported, _) => Self::Unsupported,
            (Method::Get, Route::Unknown)
            | (Method::Post, Route::Unknown)
            | (Method::Unknown, _)
            | (_, _) => Self::Unknown,
        }
    }
}

impl From<&String> for Route {
    fn from(s: &String) -> Self {
        Route::from(s.as_str())
    }
}

#[derive(Debug)]
pub struct Router<T>
where
    T: FileSystemAccess,
{
    dir: T,
}

impl<T> Router<T>
where
    T: FileSystemAccess,
{
    pub fn new(dir: T) -> Self
    where
        T: FileSystemAccess,
    {
        Router { dir }
    }

    pub fn route<'a, U>(&self, stream: &'a U) -> Result<()>
    where
        &'a U: Write + Read,
    {
        let mut s = stream;

        let mut req_buffer = BufReader::new(s);
        let req = Request::try_from(&mut req_buffer)?;
        let arg: HandlerArg<'_, &U, T> = HandlerArg::new(&req, &mut s, &self.dir);

        if let Err(e) = match Operation::from(&req) {
            Operation::GetEcho => EchoHandler::handle(arg),
            Operation::GetFileContents | Operation::PostFileContents => FileHandler::handle(arg),
            Operation::GetUserAgent => UserAgentHandler::handle(arg),
            Operation::GetEmpty => EmptyHandler::handle(arg),
            Operation::Unsupported => ErrorHandler::handle(ErrorHandlerArg::new(
                &mut s,
                ServerError::NotImplemented.into(),
            )),
            _ => ErrorHandler::handle(ErrorHandlerArg::new(&mut s, ClientError::BadRequest.into())),
        } {
            ErrorHandler::handle(ErrorHandlerArg::new(&mut s, e))
        } else {
            Ok(())
        }
    }
}
