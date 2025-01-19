// TODO: remove this file from nested structure, once
// the deprecated_request_router has been replaced
mod deprecated_request_router;

pub(crate) use deprecated_request_router::request_router;

use crate::{
    errors::ClientError,
    handlers::{EchoHandler, FileHandler, UserAgentHandler},
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
pub(crate) struct Router {
    // TODO: Do I need this dir field here - can I just use it via new and move it?
    dir: String,
}

impl Router {
    pub(crate) fn new(dir: String) -> Self {
        Router { dir }
    }

    pub(crate) fn route<'a>(&self, request: &'a Request) -> Result<Response<'a>> {
        match request.route {
            Route::Echo => EchoHandler::handle(request),
            // TODO: Get rid of the clone
            Route::Files => FileHandler::handle(request, self.dir.clone()),
            Route::UserAgent => UserAgentHandler::handle(request),
            Route::Empty => todo!(),
            Route::Unknown => Err(ClientError::NotFound.into()),
        }
    }
}
