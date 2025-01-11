use std::error::Error;
use std::fmt::{Debug, Display, Error as FmtErr, Formatter};
use std::io::Error as IOError;
use std::num::ParseIntError;
use std::sync::{mpsc, PoisonError};

#[derive(Debug, PartialEq)]
pub enum ServerError {
    Internal,
    NotImplemented,
}

impl Error for ServerError {}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), FmtErr> {
        match self {
            Self::Internal => write!(f, "500 Internal Server Error"),
            Self::NotImplemented => write!(f, "501 Not Implemented"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ClientError {
    NotFound,
    BadRequest,
}

impl Error for ClientError {}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), FmtErr> {
        match self {
            Self::NotFound => write!(f, "404 Not Found"),
            Self::BadRequest => write!(f, "400 Bad Request"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AppError {
    Client(ClientError),
    Server(ServerError),
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), FmtErr> {
        write!(f, "{:?}", self)
    }
}

impl From<ClientError> for AppError {
    fn from(error: ClientError) -> Self {
        Self::Client(error)
    }
}

impl From<ServerError> for AppError {
    fn from(error: ServerError) -> Self {
        Self::Server(error)
    }
}

impl From<IOError> for AppError {
    fn from(_error: IOError) -> Self {
        Self::Server(ServerError::Internal)
    }
}

impl From<ParseIntError> for AppError {
    fn from(_error: ParseIntError) -> Self {
        Self::Server(ServerError::Internal)
    }
}

impl From<mpsc::RecvError> for AppError {
    fn from(_error: mpsc::RecvError) -> Self {
        Self::Server(ServerError::Internal)
    }
}

impl<T> From<mpsc::SendError<T>> for AppError {
    fn from(_error: mpsc::SendError<T>) -> Self {
        Self::Server(ServerError::Internal)
    }
}

impl<T> From<PoisonError<T>> for AppError {
    fn from(_error: PoisonError<T>) -> Self {
        Self::Server(ServerError::Internal)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
