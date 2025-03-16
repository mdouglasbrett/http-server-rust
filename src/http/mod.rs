mod deprecated_response;
mod request;
mod response;

pub use deprecated_response::Response as DeprecatedResponse;
pub use request::{HeaderField, Method, Request};
pub use response::{MimeType, Response};

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
    Unknown(String)
}

impl From<&str> for Headers {
    fn from(value: &str) -> Self {
        match value {
            "User-Agent" => Self::UserAgent,
            "Content-Length" => Self::ContentLength,
            "Content-Encoding" => Self::ContentEncoding,
            "Accept-Encoding" => Self::AcceptEncoding,
            "Content-Type" => Self::ContentType,
            _ => Self::Unknown(value.to_owned())
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
