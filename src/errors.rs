// TODO: I expect I can handle some of these 'From' implementations
// in a more generic way...
use std::error::Error;
use std::fmt::{Debug, Display, Error as FmtErr, Formatter};
use std::io::Error as IOError;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ServerError {
    Internal,
    NotImplemented,
}

impl Error for ServerError {}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtErr> {
        match self {
            Self::Internal => write!(f, "500 Internal Server Error"),
            Self::NotImplemented => write!(f, "501 Not Implemented"),
        }
    }
}

#[derive(Debug)]
pub enum ClientError {
    NotFound,
    BadRequest,
}

impl Error for ClientError {}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtErr> {
        match self {
            Self::NotFound => write!(f, "404 Not Found"),
            Self::BadRequest => write!(f, "400 Bad Request"),
        }
    }
}

#[derive(Debug)]
pub struct RequestError {
    id: String,
    message: String,
}

impl Error for RequestError {}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtErr> {
        write!(f, "Error id: {}, message: {}", self.id, self.message)
    }
}

impl From<ClientError> for RequestError {
    fn from(error: ClientError) -> Self {
        Self {
            id: "ClientError".to_owned(),
            message: error.to_string(),
        }
    }
}

impl From<ServerError> for RequestError {
    fn from(error: ServerError) -> Self {
        Self {
            id: "ServerError".to_owned(),
            message: error.to_string(),
        }
    }
}

impl From<IOError> for RequestError {
    fn from(error: IOError) -> Self {
        Self {
            id: "IO".to_owned(),
            message: error.to_string(),
        }
    }
}

impl From<ParseIntError> for RequestError {
    fn from(error: ParseIntError) -> Self {
        Self {
            id: "ParseInt".to_owned(),
            message: error.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct HandlerError {
    id: String,
    message: String,
}

impl Error for HandlerError {}

impl Display for HandlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtErr> {
        write!(f, "Error id: {}, message: {}", self.id, self.message)
    }
}

impl From<ClientError> for HandlerError {
    fn from(error: ClientError) -> Self {
        Self {
            id: "ClientError".to_owned(),
            message: error.to_string(),
        }
    }
}

impl From<ServerError> for HandlerError {
    fn from(error: ServerError) -> Self {
        Self {
            id: "ServerError".to_owned(),
            message: error.to_string(),
        }
    }
}

impl From<IOError> for HandlerError {
    fn from(error: IOError) -> Self {
        Self {
            id: "IO".to_owned(),
            message: error.to_string(),
        }
    }
}
