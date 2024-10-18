use std::error::Error;
use std::fmt::{Debug, Display, Error as FmtErr, Formatter};
use std::io::Error as IOError;

#[derive(Debug)]
pub enum RequestError {
    BodyError(String),
    HeaderError(String),
}

impl Error for RequestError {}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtErr> {
        match self {
            RequestError::BodyError(err) => write!(f, "{}", err),
            RequestError::HeaderError(err) => write!(f, "{}", err),
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

impl From<RequestError> for HandlerError {
    fn from(error: RequestError) -> Self {
        Self {
            id: "Request".to_owned(),
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
