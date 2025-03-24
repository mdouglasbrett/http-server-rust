use super::{Encoding, MimeType, StatusCode};
use crate::Result;

#[derive(Debug)]
pub struct Response<'a> {
    status_code: StatusCode,
    body: Option<&'a [u8]>,
    mime_type: Option<MimeType>,
    content_length: Option<usize>,
    encoding: Option<Vec<Encoding>>,
}

impl<'a> Response<'a> {
    pub fn builder() -> ResponseBuilder<'a> {
        ResponseBuilder::new()
    }
    // TODO: what do we actually want to validate here?
    // Possibly mime type and or encoding?
    // Encoding -> Check for gzip
    fn validate(&self) -> Result<()> {
        Ok(())
    }

    pub fn ok() -> Result<Response<'a>> {
        ResponseBuilder::new().build()
    }
    pub fn not_found() -> Result<Response<'a>> {
        ResponseBuilder::new()
            .status_code(StatusCode::NotFound)
            .build()
    }
    pub fn client_error() -> Result<Response<'a>> {
        ResponseBuilder::new()
            .status_code(StatusCode::ClientError)
            .build()
    }
    pub fn server_error() -> Result<Response<'a>> {
        ResponseBuilder::new()
            .status_code(StatusCode::ServerError)
            .build()
    }
}

#[derive(Debug, Default)]
pub struct ResponseBuilder<'a> {
    status_code: Option<StatusCode>,
    body: Option<&'a [u8]>,
    mime_type: Option<MimeType>,
    encoding: Option<Vec<Encoding>>,
    content_length: Option<usize>
}

impl<'a> ResponseBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = Some(status_code);
        self
    }
    pub fn mime_type(mut self, mime_type: MimeType) -> Self {
        self.mime_type = Some(mime_type);
        self
    }
    // TODO: does this make more sense to just be bytes?
    pub fn body(mut self, body: Option<&'a [u8]>) -> Self {
        self.body = body;
        self
    }
    pub fn encoding(mut self, encoding: Option<&String>) -> Self {
        if let Some(encoding_string) = encoding {
            self.encoding = Some(
                encoding_string
                    .split(",")
                    .map(|e| Encoding::from(e))
                    .collect::<Vec<Encoding>>(),
            );
        }
        self
    }
    pub fn build(self) -> Result<Response<'a>> {
        let response = Response {
            status_code: self.status_code.unwrap_or(StatusCode::Ok),
            body: self.body,
            content_length: if let Some(b) = self.body {
                Some(b.len())
            } else {
                None
            },
            mime_type: self.mime_type,
            encoding: self.encoding,
        };
        response.validate()?;
        Ok(response)
    }
}
