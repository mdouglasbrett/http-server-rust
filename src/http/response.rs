use super::StatusCode;
use crate::{common::mime_types::PLAIN_TEXT, Result};

// TODO: almost definitely these fields are going to change
#[derive(Debug)]
pub(crate) struct Response {
    pub(crate) status_code: StatusCode,
    pub(crate) body: Option<String>,
    pub(crate) mime_type: String,
    pub(crate) encoding: Option<String>,
}

impl Response {
    pub(crate) fn new() -> ResponseBuilder {
        ResponseBuilder::new()
    }
    // TODO: what do we actually want to validate here?
    // Possibly mime type and or encoding?
    fn validate(&self) -> Result<()> {
        Ok(())
    }
    pub(crate) fn ok() -> Result<Response> {
        ResponseBuilder::new().build()
    }
    pub(crate) fn not_found() -> Result<Response> {
        ResponseBuilder::new()
            .status_code(StatusCode::NotFound)
            .build()
    }
    pub(crate) fn client_error() -> Result<Response> {
        ResponseBuilder::new()
            .status_code(StatusCode::ClientError)
            .build()
    }
    pub(crate) fn server_error() -> Result<Response> {
        ResponseBuilder::new()
            .status_code(StatusCode::ServerError)
            .build()
    }
}

#[derive(Debug)]
pub(crate) struct ResponseBuilder {
    status_code: StatusCode,
    body: Option<String>,
    mime_type: String,
    encoding: Option<String>,
}

impl ResponseBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn default() -> Self {
        Self {
            status_code: StatusCode::Ok,
            mime_type: PLAIN_TEXT.to_owned(),
            body: None,
            encoding: None,
        }
    }

    fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }
    fn mime_type(mut self, mime_type: String) -> Self {
        self.mime_type = mime_type;
        self
    }
    // TODO, should we just keep this as bytes? How do we recieve it?
    fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_owned());
        self
    }
    fn encoding(mut self, encoding: Option<String>) -> Self {
        self.encoding = encoding;
        self
    }
    fn build(self) -> Result<Response> {
        let response = Response {
            status_code: self.status_code,
            body: self.body,
            mime_type: self.mime_type,
            encoding: self.encoding,
        };
        response.validate()?;
        Ok(response)
    }
}
