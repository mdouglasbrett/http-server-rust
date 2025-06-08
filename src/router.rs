use crate::{
    file::File,
    handlers::*,
    http::{ClientError, Request},
    Result,
};
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
        let mut arg: HandlerArg<'_, &T, File> = HandlerArg::new(&req, &mut s, None);
        // TODO: should I discriminate on Method straight away?
        // Is this just a massive oversight? LOL
        if let Err(e) = match req.route {
            Route::Echo => EchoHandler::handle(arg),
            Route::Files => {
                arg.file = Some(File::new(self.dir.to_owned()));
                FileHandler::handle(arg)
            }
            Route::UserAgent => UserAgentHandler::handle(arg),
            Route::Empty => EmptyHandler::handle(arg),
            Route::Unknown => {
                ErrorHandler::handle(ErrorHandlerArg::new(&mut s, ClientError::BadRequest.into()))
            }
        } {
            ErrorHandler::handle(ErrorHandlerArg::new(&mut s, e))
        } else {
            Ok(())
        }
    }
}
