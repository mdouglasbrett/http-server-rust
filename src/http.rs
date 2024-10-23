use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
};

use flate2::{write::GzEncoder, Compression};

use crate::errors::{AppError, ClientError, ServerError};
use crate::routes::Route;
use crate::utils::get_path_parts;

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
            Some(_) => Self::Unsupported(ServerError::NotImplemented),
            None => Self::Unknown(ClientError::BadRequest),
        }
    }
}

// TODO: this is a temporary solution. Is there something better?
#[derive(Debug)]
pub enum HeaderField {
    Single(String),
    Multiple(Vec<String>),
}

pub struct Request {
    pub method: Method,
    pub route: Route,
    // https://steveklabnik.com/writing/when-should-i-use-string-vs-str/
    pub path: String,
    pub headers: HashMap<String, HeaderField>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn try_new<T: Read>(value: &mut T) -> Result<Self, AppError>
    {
        let mut buf = BufReader::new(value);
        let mut start_line = String::new();
        let _ = buf.read_line(&mut start_line)?;
        let mut start_parts = start_line.split_whitespace();
        let method = match Method::from(start_parts.next()) {
            Method::Unsupported(err) => {
                return Err(err.into());
            }
            Method::Unknown(err) => {
                return Err(err.into());
            }
            Method::Get => Method::Get,
            Method::Post => Method::Post,
        };
        let path = match start_parts.next() {
            Some(s) => s.to_owned(),
            None => {
                return Err(ClientError::BadRequest.into());
            }
        };
        let path_parts = get_path_parts(path.as_str());

        let route = if path_parts.is_empty() {
            Route::from("/")
        } else {
            Route::from(path_parts[0])
        };

        let mut headers = HashMap::new();

        loop {
            let mut header_line = String::new();
            let _ = buf.read_line(&mut header_line)?;
            let trimmed_header_line = header_line.trim();
            if trimmed_header_line.is_empty() {
                // I think we have reached the body at this point
                break;
            }
            let key_value = trimmed_header_line
                .split_terminator(":")
                .collect::<Vec<&str>>();
            let key = key_value[0];
            let raw_value = key_value[1].trim();
            let value = if key == "Accept-Encoding" {
                HeaderField::Multiple(raw_value.split(", ").map(|s| s.to_owned()).collect())
            } else {
                HeaderField::Single(raw_value.to_owned())
            };
            let _ = headers.insert(key.to_owned(), value);
        }

        let mut body_buf: Vec<u8> = vec![];

        // If there's no content length, do not attempt to parse the body
        if let Some(len) = headers.get("Content-Length") {
            match len {
                HeaderField::Single(len) => {
                    let len = len.parse::<u64>()?;
                    buf.take(len).read_to_end(&mut body_buf)?;
                }
                HeaderField::Multiple(_) => {
                    return Err(ClientError::BadRequest.into());
                }
            }
        }

        Ok(Self {
            route,
            path,
            method,
            headers,
            body: body_buf,
        })
    }
}

pub enum Response {
    Ok(Option<(String, String, Option<String>)>),
    Created,
    ClientError(ClientError),
    ServerError(ServerError),
}

impl Response {
    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Self::Ok(Some((body, mime, encoding))) => {
                // TODO: how do I reliably test this?
                let content = if encoding.is_some() {
                    let mut b = GzEncoder::new(Vec::new(), Compression::default());
                    let _ = b.write_all(body.as_bytes());
                    let compressed_body = b.finish();
                    if let Ok(bytes) = compressed_body {
                        bytes
                    } else {
                        return format!("HTTP/1.1 {}\r\n\r\n", ServerError::Internal)
                            .as_bytes()
                            .to_vec();
                    }
                } else {
                    body.as_bytes().to_vec()
                };
                let mut response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {content_length}\r\n{content_encoding}\r\n",
                content_type = mime,
                content_encoding = match encoding {
                    Some(e) => format!("Content-Encoding: {}\r\n", e),
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
            Self::Ok(None) => "HTTP/1.1 200 OK\r\n\r\n".as_bytes().to_vec(),
            Self::Created => "HTTP/1.1 201 Created\r\n\r\n".as_bytes().to_vec(),
            Self::ServerError(err) => format!("HTTP/1.1 {}\r\n\r\n", err).as_bytes().to_vec(),
            Self::ClientError(err) => format!("HTTP/1.1 {}\r\n\r\n", err).as_bytes().to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    mod response {
        use crate::errors::{ClientError::NotFound, ServerError::NotImplemented};
        use crate::http::Response;
        #[test]
        fn client_error_response() {
            let expected = "HTTP/1.1 404 Not Found\r\n\r\n".as_bytes().to_vec();
            assert_eq!(expected, Response::ClientError(NotFound).to_vec());
        }
        #[test]
        fn server_error_response() {
            let expected = "HTTP/1.1 501 Not Implemented\r\n\r\n".as_bytes().to_vec();
            assert_eq!(expected, Response::ServerError(NotImplemented).to_vec());
        }
        #[test]
        fn created_response() {
            let expected = "HTTP/1.1 201 Created\r\n\r\n".as_bytes().to_vec();
            assert_eq!(expected, Response::Created.to_vec());
        }
        #[test]
        fn ok_response() {
            let expected =
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 3\r\n\r\nabc"
                    .as_bytes()
                    .to_vec();
            assert_eq!(
                expected,
                Response::Ok(Some((
                    String::from("abc"),
                    String::from("text/plain"),
                    None
                )))
                .to_vec()
            );
        }
        #[test]
        fn empty_response() {
            let expected = "HTTP/1.1 200 OK\r\n\r\n".as_bytes().to_vec();
            assert_eq!(expected, Response::Ok(None).to_vec())
        }
    }

    mod request {
        use crate::http::Request;
    }
}
