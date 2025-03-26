// mod deprecated_response;
mod request;
mod response;

// pub use deprecated_response::Response as DeprecatedResponse;
pub use request::Request;
pub use response::Response;
pub use crate::errors::{ClientError, ServerError};

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
    Unknown
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
    Ok = 200,
    NotFound = 404,
    ServerError = 500,
    ClientError = 400,
    NotImplemented = 501,
}

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

#[derive(Debug)]
pub enum MimeType {
    PlainText,
    OctetStream,
    Unknown
}

impl From<&str> for MimeType {
    fn from(value: &str) -> Self {
        match value {
            "text/plain" => Self::PlainText,
            "application/octet-stream" => Self::OctetStream,
            _ => Self::Unknown
        }
    }
}
