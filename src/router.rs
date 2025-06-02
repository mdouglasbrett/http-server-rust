use crate::{file::File, handlers::*, http::Request, Result};
use std::io::{Read, Write};

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

impl Router {
    pub fn new(dir: String) -> Self {
        Router { dir }
    }

    pub fn route<'a, T>(&self, stream: &'a T) -> Result<()>
    where
        &'a T: Write + Read,
    {
        let mut s = stream;
        let req = Request::try_new(s)?;
        let mut arg: HandlerArg<'_, &T, File> = HandlerArg {
            req: &req,
            stream: &mut s,
            target_dir: &self.dir,
            file: None,
        };
        if let Err(e) = match req.route {
            Route::Echo => EchoHandler::handle(arg),
            Route::Files => {
                let file = File;
                arg.file = Some(file);
                FileHandler::handle(arg)
            }
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
