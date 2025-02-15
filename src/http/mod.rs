mod deprecated_response;
mod request;
mod response;

pub(crate) use deprecated_response::Response;
pub(crate) use request::{HeaderField, Method, Request};

#[derive(Debug)]
pub enum StatusCode {
    Ok = 200,
    NotFound = 404,
    ServerError = 500,
    ClientError = 400,
    NotImplemented = 501,
}
