use std::error::Error;
use std::fmt::{Debug, Display, Error as FmtErr, Formatter};


// TODO: make more meaningful errors!!
pub type HandlerError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub enum HttpError {
    BodyError(String),
    HeaderError(String),
}

impl Error for HttpError {}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtErr> {
        match self {
            HttpError::BodyError(err) => write!(f, "{}", err),
            HttpError::HeaderError(err) => write!(f, "{}", err),
        }
    }
}

