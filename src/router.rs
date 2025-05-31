use std::net::TcpStream;
use crate::{handlers::*, http::Request, Result};

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

impl From<&String> for Route {
    fn from(s: &String) -> Self {
        Route::from(s.as_str())
    }
}

#[derive(Debug)]
pub struct Router {
    dir: String,
}

impl Router  {
    pub fn new(dir: String) -> Self {
        Router { dir }
    }

    // TODO: <T> where T: Write ? Help with testing, and I'm already using
    // it in the handlers
    pub fn route(&self, stream: TcpStream) -> Result<()> {
        let mut s = stream;
        let req = Request::try_new(&s)?;
        let arg = HandlerArg {
            req: &req,
            stream: &mut s,
            target_dir: &self.dir,
        };
        if let Err(e) = match req.route {
            Route::Echo => EchoHandler::handle(arg),
            Route::Files => FileHandler::handle(arg),
            Route::UserAgent => UserAgentHandler::handle(arg),
            Route::Empty => EmptyHandler::handle(arg),
            Route::Unknown => NotFoundHandler::handle(arg),
        } {
            ErrorHandler::handle((&mut s, e))
        } else {
            Ok(())
        }
    }
}


// TODO: tests
