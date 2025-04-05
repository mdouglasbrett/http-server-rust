use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read}, net::TcpStream,
};

use crate::{errors::ClientError, router::Route, utils::get_path_parts, Result};

use super::{Headers, Method};

#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: Method,
    pub route: Route,
    // https://steveklabnik.com/writing/when-should-i-use-string-vs-str/
    pub path: String,
    pub headers: HashMap<Headers, String>,
    // Is it better to just have a String?
    pub body: Vec<u8>,
}

// TODO: RequestBuilder
// TODO: this should be implmented as TryFrom 
impl Request {
    pub fn try_new<T: Read>(value: &mut T) -> Result<Self> {
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
            let key = Headers::from(key_value[0]);
            let raw_value = key_value[1].trim();
            let concat_parts = raw_value.replace(", ", ",");
            headers
                .entry(key)
                // in-place mutation
                .and_modify(|val| { *val = format!("{},{}", val, concat_parts); })
                .or_insert(concat_parts.to_owned());
        }

        let mut body_buf: Vec<u8> = vec![];

        // TODO: should this even happen here? we are handling the echo route
        // which, let's be honest we should do in the handler
        if route == Route::Echo && path_parts.len() > 1 {
            body_buf.extend(path_parts[1].as_bytes());
        } else {
            // If there's no content length, do not attempt to parse the body
            if let Some(len) = headers.get(&Headers::ContentLength) {
                let len = len.parse::<u64>()?;
                buf.take(len).read_to_end(&mut body_buf)?;
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
    pub fn get_header(&self, header: Headers) -> Option<&String> {
        // TODO: this reference... this causes knock on effects
        let header_val = self.headers.get(&header);
        header_val
    }
}

#[cfg(test)]
mod tests {

    mod request {
        use crate::errors::{AppError, ClientError};
        use crate::http::request::{Method::Get, Request};
        use crate::router::Route::Echo;
        use std::collections::HashMap;

        #[test]
        fn handles_http_request() {
            let req = b"GET /echo/abc HTTP/1.1\r\n\r\n";
            let expected = Request {
                method: Get,
                route: Echo,
                path: "/echo/abc".to_owned(),
                body: b"abc".to_vec(),
                headers: HashMap::new(),
            };
            assert_eq!(expected, Request::try_new(&mut req.as_slice()).unwrap());
        }

        #[test]
        fn handles_bad_request() {
            let req = b"/echo/abc\r\n\r\n";
            assert_eq!(
                AppError::Client(ClientError::BadRequest),
                Request::try_new(&mut req.as_slice()).unwrap_err()
            );
        }
    }
}
