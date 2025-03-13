mod deprecated_response;
mod request;
mod response;

pub use deprecated_response::Response as DeprecatedResponse;
pub use request::{HeaderField, Method, Request};
pub use response::Response;

#[derive(Debug)]
pub enum StatusCode {
    Ok = 200,
    NotFound = 404,
    ServerError = 500,
    ClientError = 400,
    NotImplemented = 501,
}
