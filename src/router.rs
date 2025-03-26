use std::{io::Write, net::TcpStream};

use crate::{
    handlers::*,
    http::{Request, Response},
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

    pub fn route(&self, stream: &mut TcpStream) -> Result<()> {
        let req = Request::try_new(stream)?;
        let arg = HandlerArg {
            req: &req,
            stream,
            target_dir: &self.dir,
        };
        if let Err(e) = match req.route {
            Route::Echo => EchoHandler::handle(arg),
            Route::Files => FileHandler::handle(arg),
            Route::UserAgent => UserAgentHandler::handle(arg),
            Route::Empty => EmptyHandler::handle(arg),
            Route::Unknown => NotFoundHandler::handle(arg),
        } {
            ErrorHandler::handle((stream, e))
        } else {
            Ok(())
        }
    }
}
