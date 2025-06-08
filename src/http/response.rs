use super::{Encoding, Headers, MimeType, StatusCode};
use crate::{Result, constants::HTTP_VERSION};
use flate2::{write::GzEncoder, Compression};
use std::io::Write;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<Vec<u8>>,
    mime_type: Option<MimeType>,
    encoding: Option<Vec<Encoding>>,
}

impl Response {
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder::new()
    }
    fn validate(&mut self) -> Result<()> {
        if let Some(encoding_vec) = &self.encoding {
            // Encoding -> Check for Gzip
            if encoding_vec.contains(&Encoding::Gzip) {
                let mut b = GzEncoder::new(Vec::new(), Compression::default());
                let uncompressed = self.body.take();
                let _ = b.write_all(&uncompressed.unwrap_or_default());
                self.body = b.finish().ok();
                Ok(())
            } else {
                self.encoding = None;
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    pub fn ok() -> Result<Response> {
        ResponseBuilder::new().build()
    }
    pub fn not_found() -> Result<Response> {
        ResponseBuilder::new()
            .status_code(StatusCode::NotFound)
            .build()
    }
    pub fn created() -> Result<Response> {
        ResponseBuilder::new()
            .status_code(StatusCode::Created)
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
    pub fn as_bytes(&self) -> Vec<u8> {
        let empty: Vec<u8> = Vec::new();
        let content = self.body.as_ref().unwrap_or(&empty);
        if !content.is_empty() {
            let mut response = format!(
                "{} {}\r\n{}: {content_type}\r\n{}: {content_length}\r\n\r\n",
                HTTP_VERSION,
                self.status_code,
                Headers::ContentType,
                Headers::ContentLength,
                content_type = self.mime_type.as_ref().unwrap_or(&MimeType::Unknown),
                content_length = content.len(),
            )
            .as_bytes()
            .to_vec();
            response.extend_from_slice(content);
            response
        } else {
            format!("{} {}\r\n", HTTP_VERSION, self.status_code)
                .as_bytes()
                .to_vec()
        }
    }
}

#[derive(Debug, Default)]
pub struct ResponseBuilder {
    status_code: Option<StatusCode>,
    body: Option<Vec<u8>>,
    mime_type: Option<MimeType>,
    encoding: Option<Vec<Encoding>>,
}

impl ResponseBuilder {
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
    pub fn body(mut self, body: Option<Vec<u8>>) -> Self {
        self.body = body;
        self
    }
    pub fn encoding(mut self, encoding: Option<&String>) -> Self {
        if let Some(encoding_string) = encoding {
            self.encoding = Some(
                encoding_string
                    .split(",")
                    .map(Encoding::from)
                    .collect::<Vec<Encoding>>(),
            );
        }
        self
    }
    pub fn build(self) -> Result<Response> {
        let mut response = Response {
            status_code: self.status_code.unwrap_or(StatusCode::Ok),
            body: self.body,
            mime_type: self.mime_type,
            encoding: self.encoding,
        };
        response.validate()?;
        Ok(response)
    }
}
