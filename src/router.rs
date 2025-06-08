use crate::{file::File, handlers::*, http::Request, Result};
use std::{
    io::{BufReader, Read, Write},
    path::PathBuf,
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

impl From<&String> for Route {
    fn from(s: &String) -> Self {
        Route::from(s.as_str())
    }
}

#[derive(Debug)]
pub struct Router {
    dir: PathBuf,
}

impl Router {
    pub fn new(dir: PathBuf) -> Self {
        Router { dir }
    }

    pub fn route<'a, T>(&self, stream: &'a T) -> Result<()>
    where
        &'a T: Write + Read,
    {
        let mut s = stream;

        let mut req_buffer = BufReader::new(s);
        let req = Request::try_from(&mut req_buffer)?;
        let mut arg: HandlerArg<'_, &T, File> = HandlerArg {
            req: &req,
            stream: &mut s,
            target_dir: &self.dir,
            file: None,
        };
        if let Err(e) = match req.route {
            Route::Echo => EchoHandler::handle(arg),
            Route::Files => {
                arg.file = Some(File::new());
                FileHandler::handle(arg)
            }
            Route::UserAgent => UserAgentHandler::handle(arg),
            Route::Empty => EmptyHandler::handle(arg),
            // TODO: _should_ this be represented as NotFound? If we do not know that route, should
            // we not say? It's not exactly the same as a resource not being there...
            Route::Unknown => NotFoundHandler::handle(arg),
        } {
            ErrorHandler::handle(ErrorHandlerArg(&mut s, e))
        } else {
            Ok(())
        }
    }
}
