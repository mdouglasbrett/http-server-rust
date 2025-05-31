mod request;
mod response;

use std::fmt::Display;

pub use crate::errors::{ClientError, ServerError};
pub use request::Request;
pub use response::Response;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Unknown(ClientError),
    Unsupported(ServerError),
}

impl From<Option<&str>> for Method {
    fn from(o: Option<&str>) -> Self {
        match o {
            Some("GET") => Self::Get,
            Some("POST") => Self::Post,
            // Maybe tomorrow...
            Some("PUT") | Some("PATCH") | Some("OPTIONS") | Some("HEAD") | Some("DELETE")
            | Some("CONNECT") | Some("TRACE") => Self::Unsupported(ServerError::NotImplemented),
            _ => Self::Unknown(ClientError::BadRequest),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Encoding {
    Gzip,
    Unknown,
}

impl From<&str> for Encoding {
    fn from(value: &str) -> Self {
        match value {
            "gzip" => Self::Gzip,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
pub enum StatusCode {
    Ok,
    Created,
    NotFound,
    ServerError,
    ClientError,
    NotImplemented,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "200 OK"),
            Self::Created => write!(f, "201 Created"),
            Self::ClientError => write!(f, "400 Bad Request"),
            Self::NotFound => write!(f, "404 Not Found"),
            Self::ServerError => write!(f, "500 Internal Server Error"),
            Self::NotImplemented => write!(f, "501 Not Implemented"),
        }
    }
}

// TODO: I can't just use an .into() on these, because of the _ in the from.
// I would have to implement TryFrom and then account for the Error. I am on the fence about
// this...
// TODO: implement TryFrom

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Headers {
    UserAgent,
    ContentLength,
    ContentEncoding,
    AcceptEncoding,
    ContentType,
    Unknown,
}

impl From<&str> for Headers {
    fn from(value: &str) -> Self {
        match value {
            "User-Agent" => Self::UserAgent,
            "Content-Length" => Self::ContentLength,
            "Content-Encoding" => Self::ContentEncoding,
            "Accept-Encoding" => Self::AcceptEncoding,
            "Content-Type" => Self::ContentType,
            _ => Self::Unknown,
        }
    }
}

impl From<String> for Headers {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&String> for Headers {
    fn from(value: &String) -> Self {
        Self::from(value.to_owned())
    }
}

// TODO: I don't think this is great...
impl Display for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserAgent => write!(f, "User-Agent"),
            Self::ContentLength => write!(f, "Content-Length"),
            Self::ContentEncoding => write!(f, "Content-Encoding"),
            Self::AcceptEncoding => write!(f, "Accept-Encoding"),
            Self::ContentType => write!(f, "Content-Type"),
            Self::Unknown => write!(f, ""),
        }
    }
}

#[derive(Debug)]
pub enum MimeType {
    PlainText,
    OctetStream,
    Unknown,
}

impl From<&str> for MimeType {
    fn from(value: &str) -> Self {
        match value {
            "text/plain" => Self::PlainText,
            "application/octet-stream" => Self::OctetStream,
            _ => Self::Unknown,
        }
    }
}

impl Display for MimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlainText => write!(f, "text/plain"),
            Self::OctetStream => write!(f, "application/octet-stream"),
            Self::Unknown => write!(f, ""),
        }
    }
}
