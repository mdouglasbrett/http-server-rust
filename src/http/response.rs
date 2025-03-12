use super::StatusCode;
use crate::{constants::mime_types::PLAIN_TEXT, Result};

// TODO: almost definitely these fields are going to change
#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
    mime_type: String,
    encoding: Option<String>,
}

impl Response {
    // TODO: expose builder() method that returns the ResponseBuilder type
    // TODO: what do we actually want to validate here?
    // Possibly mime type and or encoding?
    fn validate(&self) -> Result<()> {
        Ok(())
    }


    pub fn ok() -> Result<Response> {
        ResponseBuilder::new().build()
    }
    pub fn not_found() -> Result<Response> {
        ResponseBuilder::new()
            .status_code(StatusCode::NotFound)
            .build()
    }
    pub fn client_error() -> Result<Response> {
        ResponseBuilder::new()
            .status_code(StatusCode::ClientError)
            .build()
    }
    pub fn server_error() -> Result<Response> {
        ResponseBuilder::new()
            .status_code(StatusCode::ServerError)
            .build()
    }
}

#[derive(Debug, Default)]
pub struct ResponseBuilder {
    status_code: Option<StatusCode>,
    body: Option<String>,
    mime_type: Option<String>,
    encoding: Option<String>,
}

impl ResponseBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = Some(status_code);
        self
    }
    pub fn mime_type(mut self, mime_type: String) -> Self {
        self.mime_type = Some(mime_type);
        self
    }
    // TODO, should we just keep this as bytes? How do we recieve it?
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_owned());
        self
    }
    pub fn encoding(mut self, encoding: Option<String>) -> Self {
        self.encoding = encoding;
        self
    }
    pub fn build(self) -> Result<Response> {
        let response = Response {
            status_code: self.status_code.unwrap_or(StatusCode::Ok),
            body: self.body,
            mime_type: self.mime_type.unwrap_or(PLAIN_TEXT.to_owned()),
            encoding: self.encoding,
        };
        response.validate()?;
        Ok(response)
    }
}
