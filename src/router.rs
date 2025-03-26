use std::net::TcpStream;

use crate::{
    errors::ClientError,
    handlers::*,
    http::Request,
    Result,
};

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


#[derive(Debug)]
pub struct Router {
    dir: String,
}

impl Router {
    pub fn new(dir: String) -> Self {
        Router { dir }
    }

    pub fn route<'a>(&self, stream: &'a TcpStream) -> Result<()> {
        let req = Request::try_new(stream)?;
        let arg = HandlerArg {
            req: &req,
            stream,
            target_dir: &self.dir
        };
        match req.route {
            Route::Echo => EchoHandler::handle(arg),
            Route::Files => FileHandler::handle(arg),
            Route::UserAgent => UserAgentHandler::handle(arg),
            Route::Empty => EmptyHandler::handle(arg),
            Route::Unknown => Err(ClientError::NotFound.into()),
        }
    }
}
