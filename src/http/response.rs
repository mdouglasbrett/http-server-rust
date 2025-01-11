use flate2::{write::GzEncoder, Compression};
use std::io::Write;

use crate::{
    common::headers::{CONTENT_ENCONDING, CONTENT_LENGTH, CONTENT_TYPE},
    errors::{ClientError, ServerError},
};

pub enum Response<'a> {
    Ok(Option<(&'a [u8], String, Option<String>)>),
    Created,
    ClientError(ClientError),
    ServerError(ServerError),
}

impl<'a> Response<'a> {
    // TODO: is this idiomatic?
    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Self::Ok(Some((body, mime, encoding))) => {
                // TODO: how do I reliably test this?
                let content = if encoding.is_some() {
                    let mut b = GzEncoder::new(Vec::new(), Compression::default());
                    let _ = b.write_all(body);
                    let compressed_body = b.finish();
                    if let Ok(bytes) = compressed_body {
                        bytes
                    } else {
                        return format!("HTTP/1.1 {}\r\n\r\n", ServerError::Internal)
                            .as_bytes()
                            .to_vec();
                    }
                } else {
                    body.to_vec()
                };
                let mut response = format!(
                "HTTP/1.1 200 OK\r\n{}: {content_type}\r\n{}: {content_length}\r\n{content_encoding}\r\n",
                CONTENT_TYPE,
                CONTENT_LENGTH,
                content_type = mime,
                content_encoding = match encoding {
                    Some(e) => format!("{}: {}\r\n",CONTENT_ENCONDING, e),
                    None => "".to_owned()
                },
                content_length = content.len(),)
                    .as_bytes()
                    .to_vec();
                if !content.is_empty() {
                    response.extend_from_slice(&content);
                }

                response
            }
            Self::Ok(None) => b"HTTP/1.1 200 OK\r\n\r\n".to_vec(),
            Self::Created => b"HTTP/1.1 201 Created\r\n\r\n".to_vec(),
            Self::ServerError(err) => format!("HTTP/1.1 {}\r\n\r\n", err).as_bytes().to_vec(),
            Self::ClientError(err) => format!("HTTP/1.1 {}\r\n\r\n", err).as_bytes().to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {

    mod response {
        use crate::common::mime_types;
        use crate::errors::{ClientError::NotFound, ServerError::NotImplemented};
        use crate::http::Response;
        #[test]
        fn client_error_response() {
            let expected = b"HTTP/1.1 404 Not Found\r\n\r\n".to_vec();
            assert_eq!(expected, Response::ClientError(NotFound).to_vec());
        }
        #[test]
        fn server_error_response() {
            let expected = b"HTTP/1.1 501 Not Implemented\r\n\r\n".to_vec();
            assert_eq!(expected, Response::ServerError(NotImplemented).to_vec());
        }
        #[test]
        fn created_response() {
            let expected = b"HTTP/1.1 201 Created\r\n\r\n".to_vec();
            assert_eq!(expected, Response::Created.to_vec());
        }
        #[test]
        fn ok_response() {
            let expected =
                b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 3\r\n\r\nabc"
                    .to_vec();
            assert_eq!(
                expected,
                Response::Ok(Some((b"abc", String::from(mime_types::PLAIN_TEXT), None))).to_vec()
            );
        }
        #[test]
        fn empty_response() {
            let expected = b"HTTP/1.1 200 OK\r\n\r\n".to_vec();
            assert_eq!(expected, Response::Ok(None).to_vec())
        }
    }
}
